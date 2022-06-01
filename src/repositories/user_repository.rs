use crate::schema::users::dsl::*;
use diesel;
use diesel::prelude::*;

pub fn update_avatar(user_id: &i32, avatar_url: &str, conn: &PgConnection) -> QueryResult<usize> {
    diesel::update(users.filter(id.eq(user_id)))
        .set(avatar.eq(avatar_url))
        .execute(conn)
}
