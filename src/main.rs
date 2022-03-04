extern crate openssl;
#[macro_use]
extern crate diesel_migrations;
extern crate diesel;
extern crate whisper;

use std::env;
use std::time::Instant;

use actix::*;
use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{http, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use dotenv::dotenv;
use serde_json::from_str;
use session::WsChatSession;
use whisper::db::DbExecutor;
mod server;
mod session;

embed_migrations!("./migrations");

/// Entry point for our websocket route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();

    dotenv().ok();

    let gateway_database_url =
        env::var("GATEWAY_DATABASE_URL").expect("GATEWAY_DATABASE_URL must be set");
    let gateway_manager = ConnectionManager::<PgConnection>::new(gateway_database_url);

    let gateway_pool = Pool::builder()
        .build(gateway_manager)
        .expect("Failed to create pool.");

    let own_database_url = env::var("OWN_DATABASE_URL").expect("OWN_DATABASE_URL must be set");
    let own_manager = ConnectionManager::<PgConnection>::new(own_database_url);

    let own_pool = Pool::builder()
        .build(own_manager)
        .expect("Failed to create pool.");

    embedded_migrations::run(&own_pool.get().expect("cant get connection pool")).unwrap();

    let own_pool_clone = own_pool.clone();
    let addr = Data::new(SyncArbiter::start(12, move || {
        DbExecutor(own_pool_clone.clone(), gateway_pool.clone())
    }));

    // // Start chat server actor
    let server = server::ChatServer::new(own_pool.clone()).start();

    // Create Http server with websocket support
    HttpServer::new(move || {
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
            .app_data(web::Data::new(addr.clone()))
            .app_data(web::Data::new(server.clone()))
            .route("/ws/", web::get().to(chat_route))
    })
    .bind(("0.0.0.0", 3335))?
    .run()
    .await
}
