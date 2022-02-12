use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

use actix::*;
use actix_cors::Cors;
use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;

use serde_json::{from_str, Value};

mod server;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Entry point for our websocket route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    println!("catching req here: {:?}", req.headers().get("user_id"));

    ws::start(
        WsChatSession {
            id: from_str::<usize>(req.headers().get("user-id").unwrap().to_str().unwrap())
                .unwrap_or(0),
            hb: Instant::now(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

///  Displays and affects state
async fn get_count(count: web::Data<Arc<AtomicUsize>>) -> impl Responder {
    let current_count = count.fetch_add(1, Ordering::SeqCst);
    format!("Visitors: {}", current_count)
}

struct WsChatSession {
    /// unique session id
    id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    /// Chat server
    addr: Addr<server::ChatServer>,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                id: self.id,
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<server::Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        // println!("WEBSOCKET MESSAGE: {:?}", msg);

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();

                // let qq = r#"{"event": "message","data":"{\"user_id\":5,\"to_user_id\":5, \"content\": \"i am not f ok\"}"}"#;
                // let decode_message: server::PrivateMessage = serde_json::from_str(m).unwrap();
                let decode_message: Value = serde_json::from_str(m).unwrap();

                match decode_message["event"].as_str() {
                    Some("message") => {
                        println!("We Got a message: {}", decode_message);

                        let private_message: server::PrivateMessage =
                            serde_json::from_str(&text).unwrap();

                        self.addr.do_send(private_message)
                    }
                    _ => println!("Unknown Action"),
                }
                // if m.starts_with('/') {
                //     let v: Vec<&str> = m.splitn(2, ' ').collect();
                //     match v[0] {
                //         "/list" => {
                //             // Send ListRooms message to chat server and wait for
                //             // response
                //             println!("List rooms");
                //             self.addr
                //                 .send(server::ListRooms)
                //                 .into_actor(self)
                //                 .then(|res, _, ctx| {
                //                     match res {
                //                         Ok(rooms) => {
                //                             for room in rooms {
                //                                 ctx.text(room);
                //                             }
                //                         }
                //                         _ => println!("Something is wrong"),
                //                     }
                //                     fut::ready(())
                //                 })
                //                 .wait(ctx)
                //             // .wait(ctx) pauses all events in context,
                //             // so actor wont receive any new messages until it get list
                //             // of rooms back
                //         }
                //         "/join" => {
                //             if v.len() == 2 {
                //                 self.room = v[1].to_owned();
                //                 self.addr.do_send(server::Join {
                //                     id: self.id,
                //                     name: self.room.clone(),
                //                 });

                //                 ctx.text("joined");
                //             } else {
                //                 ctx.text("!!! room name is required");
                //             }
                //         }
                //         "/name" => {
                //             if v.len() == 2 {
                //                 self.name = Some(v[1].to_owned());
                //             } else {
                //                 ctx.text("!!! name is required");
                //             }
                //         }
                //         _ => ctx.text(format!("!!! unknown command: {:?}", m)),
                //     }
                // } else {
                //     let msg = if let Some(ref name) = self.name {
                //         format!("{}: {}", name, m)
                //     } else {
                //         m.to_owned()
                //     };
                //     // send message to chat server
                //     self.addr.do_send(server::ClientMessage {
                //         id: self.id,
                //         msg,
                //         room: self.room.clone(),
                //     })
                // }
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl WsChatSession {
    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(server::Disconnect { id: act.id });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // App state
    // We are keeping a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));

    // Start chat server actor
    let server = server::ChatServer::new(app_state.clone()).start();

    // Create Http server with websocket support
    HttpServer::new(move || {
        // Define CORS
        let cors = Cors::default()
            .allow_any_method()
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(Logger::default())
            // .wrap_fn(|req, srv| {
            //     println!(
            //         "Hi from start. You requested: {:?} and the request {:?}",
            //         req.headers().get("user_id"),
            //         req.head()
            //     );
            //     srv.call(req).map(|res| {
            //         println!("Hi from response");
            //         res
            //     })
            // })
            .data(app_state.clone())
            .data(server.clone())
            // redirect to websocket.html
            .service(web::resource("/").route(web::get().to(|| {
                HttpResponse::Found()
                    .header("LOCATION", "/static/websocket.html")
                    .finish()
            })))
            .route("/count/", web::get().to(get_count))
            // websocket
            .service(web::resource("/ws/").to(chat_route))
            // static resources
            .service(fs::Files::new("/static/", "static/"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
