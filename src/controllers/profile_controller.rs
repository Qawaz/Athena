use actix::Addr;
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder, ResponseError,
};

use crate::{db::DbExecutor, errors::ServiceError, models::profile::GetUserProfile};

#[get("/{id}")]
async fn get_user_profile((id, addr): (Path<usize>, Data<Addr<DbExecutor>>)) -> impl Responder {
    let actix_message = addr.send(GetUserProfile { id: *id }).await;
    let result = actix_message.unwrap();

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => ServiceError::error_response(&error),
    }
}
