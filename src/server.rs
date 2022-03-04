use actix::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use serde::{Deserialize, Serialize};
use whisper::{
    message_repository::create_message,
    models::{self, CreateMessage},
};

use std::collections::{HashMap, HashSet};

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
    pub to_user_id: usize,
    pub user_id: usize,
    pub content: String,
    pub created_at: Option<String>,
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

/// `ChatServer` manages chat rooms and responsible for coordinating chat
/// session. implementation is super primitive
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<String, HashSet<usize>>,
    own_pool: Pool<ConnectionManager<PgConnection>>,
}

impl ChatServer {
    pub fn new(own_pool: Pool<ConnectionManager<PgConnection>>) -> ChatServer {
        // default room
        let mut rooms = HashMap::new();
        rooms.insert("Main".to_owned(), HashSet::new());

        ChatServer {
            sessions: HashMap::new(),
            rooms,
            own_pool,
        }
    }
}

impl ChatServer {
    fn send_private_message(&self, saved_message: &models::Message) {
        let broadcast_private_message = PrivateMessage {
            event: "message".to_owned(),
            data: PrivateMessageContent {
                local_id: Some(0),
                id: saved_message.id as usize,
                user_id: saved_message.user_id as usize,
                to_user_id: saved_message.to_user_id as usize,
                content: saved_message.content.clone(),
                created_at: Some(saved_message.created_at.to_string()),
            },
        };

        if let Some(addr) = self.sessions.get(&(saved_message.to_user_id as usize)) {
            let _ = addr.do_send(Message(
                serde_json::to_string(&broadcast_private_message).unwrap(),
            ));
        }
    }

    fn return_assigned_message_to_owner(
        &self,
        saved_message: &models::Message,
        local_id: Option<usize>,
    ) {
        let assigned_message = PrivateMessage {
            event: "assigned_message".to_owned(),
            data: PrivateMessageContent {
                local_id,
                id: saved_message.id as usize,
                user_id: saved_message.user_id as usize,
                to_user_id: saved_message.to_user_id as usize,
                content: saved_message.content.clone(),
                created_at: Some(saved_message.created_at.to_string()),
            },
        };

        if let Some(addr) = self.sessions.get(&(saved_message.user_id as usize)) {
            let _ = addr.do_send(Message(serde_json::to_string(&assigned_message).unwrap()));
        }
    }
    /// Send message to all users in the room
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        let _ = addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // notify all users in same room
        self.send_message(&"Main".to_owned(), "Someone joined", 0);

        // register session with random id
        self.sessions.insert(msg.id, msg.addr);

        // auto join session to Main room
        self.rooms
            .entry("Main".to_owned())
            .or_insert_with(HashSet::new)
            .insert(msg.id);

        // send id back
        msg.id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        let mut rooms: Vec<String> = Vec::new();

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all rooms
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
    }
}

/// Handler for private message.
impl Handler<PrivateMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: PrivateMessage, _: &mut Context<Self>) {
        let saved_message = create_message(
            CreateMessage {
                user_id: *&msg.data.user_id as i32,
                to_user_id: *&msg.data.to_user_id as i32,
                content: (*msg.data.content).to_string(),
            },
            &self.own_pool.get().unwrap(),
        )
        .unwrap();

        // broadcast to target
        self.send_private_message(&saved_message);

        // return to owner with assigned id and date
        self.return_assigned_message_to_owner(&saved_message, msg.data.local_id)
    }
}

/// Handler for `ListRooms` message.
impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        let mut rooms = Vec::new();

        for key in self.rooms.keys() {
            rooms.push(key.to_owned())
        }

        MessageResult(rooms)
    }
}

/// Join room, send disconnect message to old room
/// send join message to new room
impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id, name } = msg;
        let mut rooms = Vec::new();

        // remove session from all rooms
        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&id) {
                rooms.push(n.to_owned());
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }

        self.rooms
            .entry(name.clone())
            .or_insert_with(HashSet::new)
            .insert(id);

        self.send_message(&name, "Someone connected", id);
    }
}
