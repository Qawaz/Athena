use crate::models::user::User;
use crate::schema::profiles;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Identifiable, Serialize, Associations, Deserialize)]
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
