use actix::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use tokio::sync::{mpsc, oneshot};
use whisper::{
    models::{
        self,
        delivery_report::DeliveryReport,
        message::{CreateMessage, NewMessagesArray, NewMessagesArrayContent},
    },
    repositories::message_repository::{
        create_message, get_unreceived_new_messages, update_delivery_message_status,
    },
};

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for chat server communications

/// New chat session is created
#[derive(Debug, Message)]
#[rtype(usize)]
pub struct Connect {
    pub id: usize,
    pub addr: Recipient<Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}
#[derive(Message, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct PrivateMessage {
    pub event: String,
    pub data: PrivateMessageContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivateMessageContent {
    pub local_id: Option<usize>,
    pub id: usize,
    pub sender: usize,
    pub receiver: usize,
    pub content: String,
    pub created_at: Option<String>,
}

impl PrivateMessageContent {
    pub fn set_sender_id_from_jwt(&mut self, user_id: usize) {
        self.sender = user_id
    }
}

/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
    /// Room name
    pub room: String,
}

/// List of available rooms
pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

/// Join room, if room does not exists create new one.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    /// Client id
    pub id: usize,
    /// Room name
    pub name: String,
}

pub type Msg = String;

pub type SessionID = usize;

/// Room ID.
pub type RoomId = String;

#[derive(Debug)]
enum Command {
    Connect {
        conn_tx: mpsc::UnboundedSender<Msg>,
        res_tx: oneshot::Sender<SessionID>,
        jwt_user_id: usize,
    },

    Disconnect {
        conn: SessionID,
    },

    Message {
        msg: Msg,
        session_id: SessionID,
        jwt_user_id: usize,
        res_tx: oneshot::Sender<()>,
    },
}

/// `ChatServer` manages chat rooms and responsible for coordinating chat
/// session. implementation is super primitive
pub struct ChatServer {
    sessions: HashMap<SessionID, mpsc::UnboundedSender<Msg>>,
    rooms: HashMap<RoomId, HashSet<SessionID>>,
    own_pool: Pool<ConnectionManager<PgConnection>>,
    cmd_rx: mpsc::UnboundedReceiver<Command>,
}

impl ChatServer {
    pub fn new(own_pool: Pool<ConnectionManager<PgConnection>>) -> (Self, ChatServerHandle) {
        // default room
        let mut rooms = HashMap::new();
        rooms.insert("Main".to_owned(), HashSet::new());

        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        (
            Self {
                sessions: HashMap::new(),
                rooms,
                own_pool,
                cmd_rx,
            },
            ChatServerHandle { cmd_tx },
        )
    }
}

impl ChatServer {
    /// Send message to target user
    // fn send_message(&self, room: &str, message: &str, skip_id: usize) {
    //     if let Some(sessions) = self.rooms.get(room) {
    //         for id in sessions {
    //             if *id != skip_id {
    //                 if let Some(addr) = self.sessions.get(id) {
    //                     let _ = addr.do_send(Message(message.to_owned()));
    //                 }
    //             }
    //         }
    //     }
    // }

    // Send verified delivery report
    fn send_verified_delivery_report(&self, delivery_report: DeliveryReport) {
        let query_result = update_delivery_message_status(
            &delivery_report.data.ids,
            &self.own_pool.get().unwrap(),
        )
        .unwrap();

        if query_result == 1 {
            if let Some(tx) = self.sessions.get(&delivery_report.data.sender) {
                let _send = tx.send(serde_json::to_string(&delivery_report).unwrap());
            }
        };
    }

    /// Send verified delivery report
    fn send_new_unreceived_messages(&self, user_id: usize) {
        if let Some(tx) = self.sessions.get(&user_id) {
            let get_new_messages = get_unreceived_new_messages(
                user_id.try_into().unwrap(),
                &self.own_pool.get().unwrap(),
            );

            let new_messages_counts = get_new_messages.as_ref().unwrap().iter().count();
            println!("get fucking new messages {:?} :", get_new_messages);

            if new_messages_counts > 0 {
                let _send = tx.send(
                    serde_json::to_string(&NewMessagesArray {
                        data: NewMessagesArrayContent {
                            messages: get_new_messages.unwrap(),
                        },
                        ..Default::default()
                    })
                    .unwrap(),
                );
            }
        }
    }

    async fn connect(&mut self, tx: mpsc::UnboundedSender<Msg>, jwt_user_id: usize) -> SessionID {
        println!("Someone joined: {:?}", tx);

        // register session with random connection ID
        let id = thread_rng().gen::<usize>();
        self.sessions.insert(jwt_user_id, tx);

        self.send_new_unreceived_messages(jwt_user_id);

        // send id back
        id
    }

    /// Unregister connection from room map and broadcast disconnection message.
    async fn disconnect(&mut self, conn_id: SessionID) {
        println!("Someone disconnected");

        let mut rooms: Vec<String> = Vec::new();

        // remove sender
        if self.sessions.remove(&conn_id).is_some() {
            // remove session from all rooms
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&conn_id) {
                    rooms.push(name.to_owned());
                }
            }
        }
    }

    pub async fn send_private_message(&self, private_message: PrivateMessage) {
        let saved_message = create_message(
            CreateMessage {
                sender: *&private_message.data.sender as i32,
                receiver: *&private_message.data.receiver as i32,
                content: (*private_message.data.content).to_string(),
            },
            &self.own_pool.get().unwrap(),
        )
        .unwrap();

        let broadcast_private_message = PrivateMessage {
            event: "message".to_owned(),
            data: PrivateMessageContent {
                local_id: Some(0),
                id: saved_message.id as usize,
                sender: saved_message.sender as usize,
                receiver: saved_message.receiver as usize,
                content: saved_message.content.clone(),
                created_at: Some(saved_message.created_at.to_string()),
            },
        };

        // send to target
        if let Some(tx) = self.sessions.get(&(saved_message.receiver as usize)) {
            let _send_message = tx.send(serde_json::to_string(&broadcast_private_message).unwrap());
        }

        // return to owner with assigned id and date
        self.return_assigned_message_to_owner(&saved_message, private_message.data.local_id)
    }

    fn return_assigned_message_to_owner(
        &self,
        saved_message: &models::message::Message,
        local_id: Option<usize>,
    ) {
        let assigned_message = PrivateMessage {
            event: "assigned_message".to_owned(),
            data: PrivateMessageContent {
                local_id,
                id: saved_message.id as usize,
                sender: saved_message.sender as usize,
                receiver: saved_message.receiver as usize,
                content: saved_message.content.clone(),
                created_at: Some(saved_message.created_at.to_string()),
            },
        };

        if let Some(tx) = self.sessions.get(&(saved_message.sender as usize)) {
            let _send_receive_message = tx.send(serde_json::to_string(&assigned_message).unwrap());
        }
    }

    pub async fn run(mut self) -> tokio::io::Result<()> {
        while let Some(cmd) = self.cmd_rx.recv().await {
            match cmd {
                Command::Connect {
                    conn_tx,
                    res_tx,
                    jwt_user_id,
                } => {
                    let conn_id = self.connect(conn_tx, jwt_user_id).await;
                    let _ = res_tx.send(conn_id);
                }

                Command::Disconnect { conn } => {
                    self.disconnect(conn).await;
                }

                Command::Message {
                    session_id: _,
                    msg,
                    jwt_user_id,
                    res_tx,
                } => {
                    // self.send_message(conn, msg).await;
                    let messsage = msg.trim();

                    let decode_message: Value = serde_json::from_str(messsage).unwrap();

                    match decode_message["event"].as_str() {
                        Some("message") => {
                            let mut private_message: PrivateMessage =
                                serde_json::from_str(&msg).unwrap();

                            private_message.data.set_sender_id_from_jwt(jwt_user_id);

                            self.send_private_message(private_message).await;
                        }
                        Some("delivery-report") => {
                            let mut delivery_report: DeliveryReport =
                                serde_json::from_str(&msg).unwrap();

                            delivery_report.data.set_sender_id_from_jwt(jwt_user_id);

                            // println!(
                            //     "the fucking incoming delivery-report happend {:?}",
                            //     self.id
                            // );

                            self.send_verified_delivery_report(delivery_report)
                        }
                        _ => println!("Unknown Action"),
                    }
                    let _ = res_tx.send(());
                }
            }
        }

        Ok(())
    }
}

/// Handle and command sender for chat server.
///
/// Reduces boilerplate of setting up response channels in WebSocket handlers.
#[derive(Clone)]
pub struct ChatServerHandle {
    cmd_tx: mpsc::UnboundedSender<Command>,
}

impl ChatServerHandle {
    /// Register client message sender and obtain connection ID.
    pub async fn connect(
        &self,
        conn_tx: mpsc::UnboundedSender<String>,
        jwt_user_id: usize,
    ) -> SessionID {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: chat server should not have been dropped
        self.cmd_tx
            .send(Command::Connect {
                conn_tx,
                res_tx,
                jwt_user_id,
            })
            .unwrap();

        // unwrap: chat server does not drop out response channel
        res_rx.await.unwrap()
    }

    /// Broadcast message to current room.
    pub async fn send_message(&self, session_id: SessionID, jwt_user_id: usize, msg: String) {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: chat server should not have been dropped
        self.cmd_tx
            .send(Command::Message {
                msg: msg.into(),
                session_id,
                jwt_user_id,
                res_tx,
            })
            .unwrap();

        // unwrap: chat server does not drop our response channel
        res_rx.await.unwrap();
    }

    /// Unregister message sender and broadcast disconnection message to current room.
    pub fn disconnect(&self, conn: SessionID) {
        // unwrap: chat server should not have been dropped
        self.cmd_tx.send(Command::Disconnect { conn }).unwrap();
    }
}
