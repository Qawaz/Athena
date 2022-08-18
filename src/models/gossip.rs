use crate::schema::gossips;
use diesel::{Insertable, Queryable};
use serde::Serialize;

#[derive(Debug, Queryable, Insertable, Identifiable, Serialize)]
pub struct Gossip {
    pub id: i32,
    pub user_id: i32,
    pub kind: String,
    pub target_id: i32,
    pub last_message_id: i32,
    pub unread_messages: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

enum GossipKind {
    CONVERSATION(String),
    GROUP(String),
    CHANNEL(String),
    GUILD(String),
}

#[derive(Insertable)]
#[table_name = "gossips"]
pub struct CreateGossip {
    pub user_id: i32,
    pub target_id: i32,
    pub last_message_id: i32,
    pub kind: String,
}
