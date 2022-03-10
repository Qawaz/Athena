use actix::Addr;
use actix_web::{
    post,
    web::{Data, Query},
    HttpResponse, Responder, ResponseError,
};
use serde::Deserialize;

use crate::{db::DbExecutor, errors::ServiceError};

#[derive(Deserialize)]
pub struct SearchUsersQueryStrings {
    pub username: String,
}

#[post("/users")]
async fn search_users(
    (query_strings, addr): (Query<SearchUsersQueryStrings>, Data<Addr<DbExecutor>>),
) -> impl Responder {
    let actix_message = addr.send(query_strings.into_inner()).await;
    let result = actix_message.unwrap();

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => ServiceError::error_response(&error),
    }
}
