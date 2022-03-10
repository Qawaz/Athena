use actix::Addr;
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder, ResponseError,
};

use crate::{db::DbExecutor, errors::ServiceError, models::user_requests::GetUserByIDReq};

#[get("/id/{id}")]
async fn get_user_by_id((id, addr): (Path<i32>, Data<Addr<DbExecutor>>)) -> impl Responder {
    let actix_message = addr.send(GetUserByIDReq { id: *id }).await;
    let result = actix_message.unwrap();

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => ServiceError::error_response(&error),
    }
}
