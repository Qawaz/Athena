use actix::Addr;
use actix_multipart::Multipart;
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Error, HttpResponse, Responder, ResponseError,
};
use aws_sdk_s3::{types::ByteStream, Client};
use futures::TryStreamExt;
use url::Url;
use uuid::Uuid;

use crate::{
    db::DbExecutor,
    errors::ServiceError,
    extractors::jwt_data_decode::Auth,
    libs::aws::{create_object, get_object},
    models::{
        user::{SetAvatarRequest, SetAvatarResponse},
        user_requests::{GetMultipleUsers, GetUserByIDReq}, settings::{ChangePasswordRequest, ChangeUsernameRequest},
    },
};

#[post("/change-password")]
async fn change_password(
    (mut password_request, sub, addr): (Json<ChangePasswordRequest>, Auth, Data<Addr<DbExecutor>>),
) -> impl Responder {
    password_request.uid = Some(sub.user_id);

    let actix_message = addr.send(password_request.into_inner()).await;
    let result = actix_message.unwrap();

    match result {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(error) => ResponseError::error_response(&error),
    }
}

#[post("/change-username")]
async fn change_username(
    (mut change_username_request, sub, addr): (
        Json<ChangeUsernameRequest>,
        Auth,
        Data<Addr<DbExecutor>>,
    ),
) -> impl Responder {
    change_username_request.uid = Some(sub.user_id);

    let actix_message = addr.send(change_username_request.into_inner()).await;
    let result = actix_message.unwrap();

    match result {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(error) => ResponseError::error_response(&error),
    }
}

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

    let actix_message = addr.send(users_ids.into_inner()).await;
    let result = actix_message.unwrap();

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => ServiceError::error_response(&error),
    }
}

#[post("/user/set-avatar")]
async fn set_avatar(
    (mut form_data, addr, sub, s3_client): (Multipart, Data<Addr<DbExecutor>>, Auth, Data<Client>),
) -> Result<HttpResponse, Error> {
    let mut image_response_uri = String::new();

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

        println!(
            "filename is:{:?} and the extenstion filename is: {:?}",
            filename,
            res.subtype()
        );

        let mut data = Vec::new();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            data.extend_from_slice(chunk.as_ref())
        }

        let bst = ByteStream::from(data);

        let file_key = format!(
            "{}{}{}.{}",
            "user-",
            sub.user_id.to_string(),
            "-avatar",
            res.subtype()
        );

        let bucket = "messenger";

        let _upload = create_object(
            &s3_client,
            bucket,
            bst,
            &file_key,
            &res.subtype().to_string(),
        )
        .await;

        image_response_uri = get_object(&s3_client, bucket, &file_key, 150)
            .await?
            .uri()
            .to_string();

        let object_url = Url::parse(image_response_uri.as_ref()).unwrap();

        if let Some(domain) = object_url.domain() {
            let _db_update_avatar = addr
                .send(SetAvatarRequest {
                    user_id: sub.user_id,
                    avatar: format!("{}://{}{}", object_url.scheme(), domain, object_url.path()),
                })
                .await;
        }
    }

    Ok(HttpResponse::Ok()
        .json(SetAvatarResponse {
            avatar: image_response_uri,
        })
        .into())
}
