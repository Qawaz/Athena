use crate::db::DbExecutor;
use crate::errors::ServiceError;
use crate::models::user::User;
use crate::models::user_requests::GetUserByIDReq;
use crate::schema::users::dsl::*;
use actix::{Handler, Message, SyncContext};
use diesel::prelude::*;

impl Message for GetUserByIDReq {
    type Result = Result<User, ServiceError>;
}

impl Handler<GetUserByIDReq> for DbExecutor {
    type Result = Result<User, ServiceError>;

    fn handle(
        &mut self,
        user_id_request: GetUserByIDReq,
        _: &mut SyncContext<Self>,
    ) -> Self::Result {
        let conn: &PgConnection = &self.1.get().unwrap();

        let target_user = users.find(user_id_request.id).first(conn)?;

        Ok(target_user)
    }
}
