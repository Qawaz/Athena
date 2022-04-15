use std::io::Write;

use actix::Addr;
use actix_multipart::Multipart;
use actix_web::{
    get, post,
    web::{self, Data, Json, Path},
    Error, HttpResponse, Responder, ResponseError,
};
use futures::TryStreamExt;
use uuid::Uuid;

use crate::{
    db::DbExecutor,
    errors::ServiceError,
    extractors::jwt_data_decode::Auth,
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

#[post("/user/set-avatar")]
async fn set_avatar(
    (mut form_data, addr, sub): (Multipart, Data<Addr<DbExecutor>>, Auth),
) -> Result<HttpResponse, Error> {
    while let Some(mut field) = form_data.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

        let parts: Vec<&str> = filename.split('.').collect();

        let res = match parts.last() {
            Some(v) => match *v {
                "png" => mime::IMAGE_PNG,
                "jpg" => mime::IMAGE_JPEG,
                &_ => mime::TEXT_PLAIN,
            },
            None => mime::TEXT_PLAIN,
        };

        println!("file extension is: {:?}", res.subtype());

        println!("{:?}", filename);
        let filepath = format!(
            "./tmp/{}",
            format!("{}.{}", sub.user_id.to_string(), res.subtype().to_owned())
        );

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
        // let actix_message = addr.send(GetUserByIDReq { id: *id }).await;
        // let result = actix_message.unwrap();
    }
    Ok(HttpResponse::Ok().into())

    // match result {
    //     Ok(response) => HttpResponse::Ok().json(response),
    //     Err(error) => ServiceError::error_response(&error),
    // }
}
