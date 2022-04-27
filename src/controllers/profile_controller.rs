use actix::Addr;
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder, ResponseError,
};

use crate::{
    db::DbExecutor,
    errors::ServiceError,
    extractors::jwt_data_decode::Auth,
    models::profile::{GetUserProfile, SetStatusRequest},
};

#[get("/{id}")]
async fn get_user_profile((id, addr): (Path<usize>, Data<Addr<DbExecutor>>)) -> impl Responder {
    let actix_message = addr.send(GetUserProfile { id: *id }).await;
    let result = actix_message.unwrap();

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => ServiceError::error_response(&error),
    }
}

#[post("/set-status")]
async fn set_status(
    (mut set_status_request, sub, addr): (Json<SetStatusRequest>, Auth, Data<Addr<DbExecutor>>),
) -> impl Responder {
    set_status_request.set_sender_id_from_jwt(sub.user_id);

    let actix_message = addr.send(set_status_request.into_inner()).await;
    let result = actix_message.unwrap();

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => ServiceError::error_response(&error),
    }
}
