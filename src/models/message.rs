use crate::schema::messages;
use diesel::{Insertable, Queryable};
use serde::Serialize;

#[derive(Debug, Queryable, Insertable, Identifiable, Serialize)]
pub struct Message {
    pub id: i32,
    pub user_id: i32,
    pub to_user_id: i32,
    pub content: String,
    pub delivered: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize)]
pub struct NewMessagesArray {
    pub event: String,
    pub data: NewMessagesArrayContent,
}

impl Default for NewMessagesArray {
    fn default() -> NewMessagesArray {
        NewMessagesArray {
            event: "new-messages-array".to_string(),
            data: NewMessagesArrayContent {
                messages: Vec::new(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NewMessagesArrayContent {
    pub messages: Vec<Message>,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct CreateMessage {
    pub user_id: i32,
    pub to_user_id: i32,
    pub content: String,
}
