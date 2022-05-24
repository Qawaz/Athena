use crate::errors::ServiceError;
use crate::models::token::Jwt;
use crate::schema::jwt_tokens::dsl::jwt_tokens;
use crate::{diesel::RunQueryDsl, models::token::CreateJWT};
use chrono::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{env, fmt};

#[derive(Clone, PartialEq)]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i32,
    pub roles: Vec<String>,
    pub exp: usize,
}

pub fn create_jwt(
    uid: &i32,
    role: &Role,
    conn: &PgConnection,
) -> Result<(String, String, i64), ServiceError> {
    dotenv().ok();

    let jwt_access_token_secret =
        env::var("JWT_ACCESS_TOKEN_SECRET").expect("JWT_ACCESS_TOKEN_SECRET must be set");

    let jwt_refresh_token_secret =
        env::var("JWT_REFRESH_TOKEN_SECRET").expect("JWT_REFRESH_TOKEN_SECRET must be set");

    let access_token_expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(180))
        .expect("valid timestamp")
        .timestamp();

    let refresh_token_expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(360))
        .expect("valid timestamp")
        .timestamp();

    let access_token_claims = Claims {
        sub: *uid,
        roles: vec![role.to_string()],
        exp: access_token_expiration as usize,
    };

    let refresh_token_claims = Claims {
        sub: *uid,
        roles: vec![role.to_string()],
        exp: refresh_token_expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    let access_token = encode(
        &header,
        &access_token_claims,
        &EncodingKey::from_secret(jwt_access_token_secret.as_bytes()),
    )
    .map_err(|_| ServiceError::JWTTokenCreationError);

    let refresh_token = encode(
        &header,
        &refresh_token_claims,
        &EncodingKey::from_secret(jwt_refresh_token_secret.as_bytes()),
    )
    .map_err(|_| ServiceError::JWTTokenCreationError);

    diesel::insert_into(jwt_tokens)
        .values(CreateJWT {
            user_id: 2,
            access_token: access_token.as_ref().unwrap().to_string(),
            access_token_expires_at: NaiveDateTime::from_timestamp(access_token_expiration, 0),
            refresh_token: refresh_token.as_ref().unwrap().to_string(),
            refresh_token_expires_at: NaiveDateTime::from_timestamp(refresh_token_expiration, 0),
        })
        .get_result::<Jwt>(conn)
        .expect("cant insert jwt token");

    Ok((
        access_token.unwrap(),
        refresh_token.unwrap(),
        access_token_expiration,
    ))
}

pub async fn authorize(token: &str) -> Result<Vec<String>, ServiceError> {
    dotenv().ok();

    let jwt_access_token_secret =
        env::var("JWT_ACCESS_TOKEN_SECRET").expect("JWT_ACCESS_TOKEN_SECRET must be set");

    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&jwt_access_token_secret.into_bytes()),
        &Validation::new(Algorithm::HS512),
    );
    // .map_err(|_| ServiceError::Unauthorized);

    // if role == Role::Admin && Role::from_str(&decoded.claims.role) != Role::Admin {
    //     return Err(ServiceError::InternalServerError);
    // }
    match decoded {
        Ok(data) => Ok(data.claims.roles),
        Err(error) => match *error.kind() {
            ErrorKind::InvalidToken => Err(ServiceError::InvalidToken),
            ErrorKind::InvalidIssuer => Err(ServiceError::JWTTokenCreationError),
            _ => Err(ServiceError::InternalServerError),
        },
    }
}