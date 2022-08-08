use crate::models::message::{CreateMessage, Message};
use crate::schema::messages;
use crate::schema::messages::dsl::*;
use chrono::Utc;
use diesel;
use diesel::prelude::*;

pub fn create_message(message: CreateMessage, conn: &PgConnection) -> QueryResult<Message> {
    diesel::insert_into(messages::table)
        .values(&message)
        .get_result(conn)
}

pub fn get_unreceived_new_messages(
    target_user_id: i32,
    conn: &PgConnection,
) -> QueryResult<Vec<Message>> {
    messages
        .filter(receiver.eq(target_user_id))
        .filter(delivered.eq(false))
        .get_results(conn)
}

pub fn get_unreceived_new_deleted_messages(
    target_user_id: i32,
    conn: &PgConnection,
) -> QueryResult<Vec<i32>> {
    messages
        .select(id)
        .filter(receiver.eq(target_user_id))
        .filter(deleted_delivered.eq(false))
        .get_results(conn)
}

pub fn update_delivery_message_status(
    message_ids: &Vec<i32>,
    conn: &PgConnection,
) -> QueryResult<usize> {
    diesel::update(messages.filter(id.eq_any(message_ids)))
        .set(delivered.eq(true))
        .execute(conn)
}

pub fn get_messages(connection: &PgConnection) -> QueryResult<Vec<Message>> {
    messages.limit(5).load::<Message>(&*connection)
}

pub fn get_messages_by_ids(
    message_ids: &Vec<i32>,
    connection: &PgConnection,
) -> QueryResult<Vec<Message>> {
    messages
        .filter(id.eq_any(message_ids))
        .load::<Message>(&*connection)
}

pub fn delete_messages(message_ids: &Vec<i32>, connection: &PgConnection) -> QueryResult<usize> {
    diesel::update(messages.filter(id.eq_any(message_ids)))
        .set(deleted_at.eq(Utc::now().naive_utc()))
        .execute(connection)
}
