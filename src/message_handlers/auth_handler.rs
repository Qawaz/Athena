use crate::auth::auth::{create_jwt, Role};
use crate::diesel::ExpressionMethods;
use crate::schema::jwt_tokens::dsl::*;
use crate::{models::token::RevokeTokenRequest, schema::users::dsl::*};
use actix::{Handler, Message, SyncContext};
use blake3::Hasher;
use chrono::NaiveDateTime;
use diesel::RunQueryDsl;
use diesel::{result::Error, PgConnection, QueryDsl};
use jsonwebtoken::{Algorithm, Header};

use crate::{
    db::DbExecutor,
    errors::ServiceError,
    models::{
        auth::{LoginRequest, LoginResponse},
        user::{User, CreateUserResponse},
    },
    schema::users::username,
};

impl Message for LoginRequest {
    type Result = Result<LoginResponse, ServiceError>;
}

impl Handler<LoginRequest> for DbExecutor {
    type Result = Result<LoginResponse, ServiceError>;

    fn handle(&mut self, creds: LoginRequest, _: &mut SyncContext<Self>) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();

        let mut hasher = Hasher::new();

        hasher.update(&creds.password.as_bytes());

        let mut found_users = users
            .filter(username.eq(&creds.username))
            .load::<User>(conn)?;

        let mut header = Header::new(Algorithm::HS512);
        header.kid = Some("blabla".to_owned());

        if let Some(user) = found_users.pop() {
            if user.password.as_deref().unwrap_or("")
                == hasher
                    .finalize()
                    .to_hex()
                    .chars()
                    .collect::<String>()
                    .to_string()
            {
                let (get_access_token, get_refresh_token, expires) =
                    create_jwt(&user.id, &Role::from_str("User"), conn)
                        .map_err(|_e| ServiceError::InternalServerError)?;

                return Ok(LoginResponse {
                    access_token: get_access_token,
                    refresh_token: get_refresh_token,
                    expires: NaiveDateTime::from_timestamp(expires, 0)
                        .format("%Y-%m-%d %H:%M:%S.%f")
                        .to_string(),
                    user: CreateUserResponse {
                        user_id: user.id,
                        username: user.username.as_deref().unwrap_or("").to_string(),
                        email: user.email.as_deref().unwrap_or("").to_string(),
                        avatar: user.avatar.as_deref().unwrap_or("").to_string(),
                    },
                });
            }
        }

        Err(ServiceError::Unauthorized)
    }
}

impl Message for RevokeTokenRequest {
    type Result = Result<String, Error>;
}

impl Handler<RevokeTokenRequest> for DbExecutor {
    type Result = Result<String, Error>;

    fn handle(
        &mut self,
        revoke_token_request: RevokeTokenRequest,
        _: &mut SyncContext<Self>,
    ) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();

        let target =
            diesel::delete(jwt_tokens.filter(refresh_token.eq(revoke_token_request.refresh_token)))
                .execute(conn)?;

        if target == 1 {
            Ok("token revoked successfully".to_string())
        } else {
            Err(Error::NotFound)
        }
    }
}