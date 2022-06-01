use crate::schema::users;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
pub struct User {
    #[serde(rename(serialize = "user_id"))]
    pub id: i32,
    pub solana_pubkey: Option<String>,
    pub ethereum_pubkey: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub avatar: Option<String>,
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Validate, Insertable, Serialize)]
#[table_name = "users"]
pub struct CreateUser {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub avatar: String,
}

#[derive(Debug, Serialize)]
pub struct UserAPI {
    #[serde(flatten)]
    pub user: User,

    pub profile: ProfileAPI,

    pub counters: Counters,
}

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct ProfileAPI {
    pub status: Option<String>,
    pub description: Option<String>,
}
#[derive(Debug, Clone, Serialize, Queryable)]
pub struct Counters {
    pub feeds: i64,
    pub followers: i64,
}
#[derive(Debug, Serialize)]
pub struct UserAPIWithoutCounters {
    #[serde(flatten)]
    pub user: User,

    // #[serde(flatten)]
    pub profile: ProfileAPI,
}

pub struct SetAvatarRequest {
    pub user_id: i32,
    pub avatar: String,
}

#[derive(Debug, Serialize)]
pub struct SetAvatarResponse {
    pub avatar: String,
}
