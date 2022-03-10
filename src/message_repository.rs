use crate::models::message::{CreateMessage, Message};
use crate::schema::messages;
use crate::schema::messages::dsl::*;
use diesel;
use diesel::prelude::*;

pub fn create_message(message: CreateMessage, conn: &PgConnection) -> QueryResult<Message> {
    diesel::insert_into(messages::table)
        .values(&message)
        .get_result(conn)
}

pub fn get_messages(connection: &PgConnection) -> QueryResult<Vec<Message>> {
    messages.limit(5).load::<Message>(&*connection)
}
