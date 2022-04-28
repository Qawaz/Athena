use crate::models::user::User;
use crate::schema::profiles;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Associations, Deserialize)]
#[belongs_to(User)]
pub struct Profile {
    pub id: i32,
    pub user_id: i32,
    pub status: Option<String>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

pub struct GetUserProfile {
    pub id: usize,
}

#[derive(Debug, Deserialize)]
pub struct SetStatusRequest {
    pub sender: i32,
    pub status: String,
}

impl SetStatusRequest {
    pub fn set_sender_id_from_jwt(&mut self, sender_id: i32) {
        self.sender = sender_id
    }
}
