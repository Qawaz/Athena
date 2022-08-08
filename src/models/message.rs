use crate::schema::messages;
use diesel::{Insertable, Queryable};
use serde::Serialize;

#[derive(Debug, Queryable, Insertable, Identifiable, Serialize)]
pub struct Message {
    pub id: i32,
    pub sender: i32,
    pub receiver: i32,
    pub content: String,
    pub delivered: bool,
    pub deleted_delivered: bool,
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
    pub sender: i32,
    pub receiver: i32,
    pub content: String,
}

// Structs for new deleted message

#[derive(Debug, Serialize)]
pub struct NewDeletedMessagesArray {
    pub event: String,
    pub data: NewDeletedMessagesArrayContent,
}

impl Default for NewDeletedMessagesArray {
    fn default() -> NewDeletedMessagesArray {
        NewDeletedMessagesArray {
            event: "new-deleted-messages-array".to_string(),
            data: NewDeletedMessagesArrayContent {
                messages_ids: Vec::new(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NewDeletedMessagesArrayContent {
    pub messages_ids: Vec<i32>,
}
