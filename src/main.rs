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
use actix_web::{get, http, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use dotenv::dotenv;
use session::WsChatSession;
use whisper::controllers::profile_controller::get_user_profile;
use whisper::controllers::search_controller::search_users;
use whisper::controllers::user_controller::{get_multiple_users, get_user_by_id};
use whisper::db::DbExecutor;
use whisper::extractors::http_auth_extractor::http_auth_extract;
use whisper::extractors::jwt_data_decode::Auth;
mod server;
mod session;

embed_migrations!("./migrations");

/// Entry point for our websocket route
#[get("/ws/")]
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
    sub: Auth,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsChatSession {
            id: sub.user_id as usize,
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

    let own_database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
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
    let server = Data::new(server::ChatServer::new(own_pool.clone()).start());

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

        let auth = HttpAuthentication::bearer(http_auth_extract);

        App::new()
            .wrap(cors)
            .app_data(addr.clone())
            .app_data(server.clone())
            .service(web::scope("/search").service(search_users))
            .service(web::scope("/profiles").service(get_user_profile))
            .service(
                web::scope("/users")
                    .service(get_user_by_id)
                    .service(get_multiple_users),
            )
            .service(web::scope("").wrap(auth).service(chat_route))
    })
    .bind(("localhost", 3335))?
    .run()
    .await
}
