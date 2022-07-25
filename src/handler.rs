use actix_ws::Message;
use futures::{
    future::{select, Either},
    StreamExt,
};
use std::time::{Duration, Instant};
use tokio::{pin, sync::mpsc, time::interval};

use crate::server::ChatServerHandle;
/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Echo text & binary messages received from the client, respond to ping messages, and monitor
/// connection health to detect network issues and free up resources.
pub async fn chat_ws(
    chat_server: ChatServerHandle,
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
    jwt_user_id: usize,
) {
    println!("connected");

    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();

    // unwrap: chat server is not dropped before the HTTP server
    let session_id = chat_server.connect(conn_tx, jwt_user_id).await;

    let close_reason = loop {
        // most of the futures we process need to be stack-pinned to work with select()

        let tick = interval.tick();
        pin!(tick);

        let msg_rx = conn_rx.recv();
        pin!(msg_rx);

        // TODO: nested select is pretty gross for readability on the match
        let messages = select(msg_stream.next(), msg_rx);
        pin!(messages);

        match select(messages, tick).await {
            // commands & messages received from client
            Either::Left((Either::Left((Some(Ok(msg)), _)), _)) => {
                println!("msg: {msg:?}");

                match msg {
                    Message::Ping(bytes) => {
                        last_heartbeat = Instant::now();
                        // unwrap:
                        session.pong(&bytes).await.unwrap();
                    }

                    Message::Pong(_) => {
                        last_heartbeat = Instant::now();
                    }

                    Message::Text(text) => {
                        chat_server
                            .send_message(session_id, jwt_user_id, text.to_string())
                            .await;
                    }

                    Message::Binary(_bin) => {
                        println!("unexpected binary message");
                    }

                    Message::Close(reason) => break reason,

                    _ => {
                        break None;
                    }
                }
            }

            // client WebSocket stream error
            Either::Left((Either::Left((Some(Err(err)), _)), _)) => {
                println!("{}", err);
                break None;
            }

            // client WebSocket stream ended
            Either::Left((Either::Left((None, _)), _)) => break None,

            // chat messages received from other room participants
            Either::Left((Either::Right((Some(chat_msg), _)), _)) => {
                session.text(chat_msg).await.unwrap();
            }

            // all connection's message senders were dropped
            Either::Left((Either::Right((None, _)), _)) => unreachable!(
                "all connection message senders were dropped; chat server may have panicked"
            ),

            // heartbeat internal tick
            Either::Right((_inst, _)) => {
                // if no heartbeat ping/pong received recently, close the connection
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    println!(
                        "client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting"
                    );
                    break None;
                }

                // send heartbeat ping
                let _ = session.ping(b"").await;
            }
        };
    };

    chat_server.disconnect(session_id);

    // attempt to close connection gracefully
    let _ = session.close(close_reason).await;
}

// pub struct WsChatSession {
//     /// unique session id
//     pub id: usize,
//     /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
//     /// otherwise we drop connection.
//     pub hb: Instant,
//     /// Chat server
//     pub addr: Addr<ChatServer>,
// }

// impl Actor for WsChatSession {
//     type Context = ws::WebsocketContext<Self>;

//     /// Method is called on actor start.
//     /// We register ws session with ChatServer
//     fn started(&mut self, ctx: &mut Self::Context) {
//         // we'll start heartbeat process on session start.
//         self.hb(ctx);

//         // register self in chat server. `AsyncContext::wait` register
//         // future within context, but context waits until this future resolves
//         // before processing any other events.
//         // HttpContext::state() is instance of WsChatSessionState, state is shared
//         // across all routes within application
//         let addr = ctx.address();
//         self.addr
//             .send(Connect {
//                 id: self.id,
//                 addr: addr.recipient(),
//             })
//             .into_actor(self)
//             .then(|res, act, ctx| {
//                 match res {
//                     Ok(res) => act.id = res,
//                     // something is wrong with chat server
//                     _ => ctx.stop(),
//                 }
//                 fut::ready(())
//             })
//             .wait(ctx);
//     }

//     fn stopping(&mut self, _: &mut Self::Context) -> Running {
//         // notify chat server
//         self.addr.do_send(Disconnect { id: self.id });
//         Running::Stop
//     }
// }

// impl WsChatSession {
//     /// helper method that sends ping to client every second.
//     ///
//     /// also this method checks heartbeats from client
//     fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
//         ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
//             // check client heartbeats
//             if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
//                 // heartbeat timed out
//                 println!("Websocket Client heartbeat failed, disconnecting!");

//                 // notify chat server
//                 act.addr.do_send(Disconnect { id: act.id });

//                 // stop actor
//                 ctx.stop();

//                 // don't try to send a ping
//                 return;
//             }

//             ctx.ping(b"");
//         });
//     }
// }

// /// Handle messages from chat server, we simply send it to peer websocket
// impl Handler<Message> for WsChatSession {
//     type Result = ();

//     fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
//         ctx.text(msg.0);
//     }
// }

// /// WebSocket message handler
// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
//     fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         let msg = match msg {
//             Err(_) => {
//                 ctx.stop();
//                 return;
//             }
//             Ok(msg) => msg,
//         };

//         match msg {
//             ws::Message::Ping(msg) => {
//                 self.hb = Instant::now();
//                 ctx.pong(&msg);
//             }
//             ws::Message::Pong(_) => {
//                 self.hb = Instant::now();
//             }
//             ws::Message::Text(text) => {
//                 let m = text.trim();

//                 let decode_message: Value = serde_json::from_str(m).unwrap();

//                 match decode_message["event"].as_str() {
//                     Some("message") => {
//                         let mut private_message: PrivateMessage =
//                             serde_json::from_str(&text).unwrap();

//                         private_message.data.set_sender_id_from_jwt(self.id);

//                         self.addr.do_send(private_message)
//                     }
//                     Some("delivery-report") => {
//                         let mut delivery_report: DeliveryReport =
//                             serde_json::from_str(&text).unwrap();

//                         delivery_report.data.set_sender_id_from_jwt(self.id);

//                         println!("the fucking incoming delivery-report happend {:?}", self.id);

//                         self.addr.do_send(delivery_report)
//                     }
//                     _ => println!("Unknown Action"),
//                 }
//             }
//             ws::Message::Binary(_) => println!("Unexpected binary"),
//             ws::Message::Close(reason) => {
//                 ctx.close(reason);
//                 ctx.stop();
//             }
//             ws::Message::Continuation(_) => {
//                 ctx.stop();
//             }
//             ws::Message::Nop => (),
//         }
//     }
// }
