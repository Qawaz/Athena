use super::schema::messages;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Insertable)]
pub struct Message {
    pub id: i32,
    pub user_id: i32,
    pub to_user_id: i32,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct CreateMessage {
    pub user_id: i32,
    pub to_user_id: i32,
    pub content: String,
}
