use crate::schema::jwt_tokens;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Jwt {
    pub id: i32,
    pub user_id: i32,
    pub access_token: String,
    pub access_token_expires_at: NaiveDateTime,
    pub refresh_token: String,
    pub refresh_token_expires_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "jwt_tokens"]
pub struct CreateJWT {
    pub user_id: i32,
    pub access_token: String,
    pub access_token_expires_at: NaiveDateTime,
    pub refresh_token: String,
    pub refresh_token_expires_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct RevokeTokenRequest {
    pub refresh_token: String,
}