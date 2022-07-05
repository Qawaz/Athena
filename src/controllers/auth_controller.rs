use crate::models::token::VerifyTokenRequest;
use crate::{
    db::DbExecutor,
    errors::ServiceError,
    models::{auth::LoginRequest, token::RevokeTokenRequest, user::CreateUser},
};
use actix::Addr;
use actix_web::ResponseError;
use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use actix_web_validator::Json;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Secret Keeper greats you")
}

#[post("/signup")]
pub async fn register(
    (new_user, addr): (Json<CreateUser>, Data<Addr<DbExecutor>>),
) -> impl Responder {
    let actix_message = addr.send(new_user.into_inner()).await;
    let result = actix_message.unwrap();

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => ServiceError::error_response(&error),
    }
}

#[post("/signin")]
async fn login((creds, addr): (web::Json<LoginRequest>, Data<Addr<DbExecutor>>)) -> impl Responder {
    println!("{:?}", creds);

    let actix_message = addr.send(creds.into_inner()).await;
    let result = actix_message.unwrap();

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => ServiceError::error_response(&error),
    }
}

#[post("/verify-token")]
async fn verify_token(
    (verify_token_request, addr): (web::Json<VerifyTokenRequest>, Data<Addr<DbExecutor>>),
) -> impl Responder {
    let actix_message = addr.send(verify_token_request.into_inner()).await;
    let result = actix_message.unwrap();

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => ServiceError::error_response(&error),
    }
}

#[post("/revoke-token")]
async fn revoke_token(
    (revoke_token_request, addr): (web::Json<RevokeTokenRequest>, Data<Addr<DbExecutor>>),
) -> impl Responder {
    let actix_message = addr.send(revoke_token_request.into_inner()).await;
    let result = actix_message.unwrap();

    web::Json(result.ok())
}
