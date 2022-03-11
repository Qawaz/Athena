use crate::schema::users;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

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
