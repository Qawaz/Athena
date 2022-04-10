use actix::Addr;
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder, ResponseError,
};

use crate::{
    db::DbExecutor,
    errors::ServiceError,
    models::user_requests::{GetMultipleUsers, GetUserByIDReq},
};

#[get("/id/{id}")]
async fn get_user_by_id((id, addr): (Path<i32>, Data<Addr<DbExecutor>>)) -> impl Responder {
    let actix_message = addr.send(GetUserByIDReq { id: *id }).await;
    let result = actix_message.unwrap();

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => ServiceError::error_response(&error),
    }
}

#[post("/multiple")]
async fn get_multiple_users(
    (users_ids, addr): (Json<GetMultipleUsers>, Data<Addr<DbExecutor>>),
) -> impl Responder {
    println!("Executing multiple users");

    let actix_message = addr.send(users_ids.into_inner()).await;
    let result = actix_message.unwrap();

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => ServiceError::error_response(&error),
    }
}
