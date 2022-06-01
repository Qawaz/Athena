use crate::diesel::ExpressionMethods;
use crate::errors::ServiceError;
use crate::models::settings::{ChangeEmailRequest, ChangePasswordRequest};
use crate::schema::users::dsl::*;
use crate::utils::hash::hash_string;
use crate::{db::DbExecutor, models::settings::ChangeUsernameRequest};

use actix::{Handler, Message, SyncContext};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

impl Message for ChangePasswordRequest {
    type Result = Result<String, ServiceError>;
}

impl Handler<ChangePasswordRequest> for DbExecutor {
    type Result = Result<String, ServiceError>;

    fn handle(
        &mut self,
        password_request: ChangePasswordRequest,
        _: &mut SyncContext<Self>,
    ) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();

        let target = users
            .filter(id.eq(password_request.uid.unwrap()))
            .filter(password.eq(hash_string(password_request.current_password)));

        let updated_row = diesel::update(target)
            .set(password.eq(hash_string(password_request.new_password)))
            .execute(conn)
            .expect("cant updated password");

        if updated_row == 0 {
            return Err(ServiceError::Forbidden);
        }

        Ok("Password updated successfully".to_string())
    }
}

impl Message for ChangeUsernameRequest {
    type Result = Result<String, ServiceError>;
}

impl Handler<ChangeUsernameRequest> for DbExecutor {
    type Result = Result<String, ServiceError>;

    fn handle(
        &mut self,
        change_username_request: ChangeUsernameRequest,
        _: &mut SyncContext<Self>,
    ) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();

        let target = users
            .filter(id.eq(change_username_request.uid.unwrap()))
            .filter(password.eq(hash_string(change_username_request.password)));

        let updated_row = diesel::update(target)
            .set(username.eq(change_username_request.new_username))
            .execute(conn)
            .expect("cant change username");

        if updated_row == 0 {
            return Err(ServiceError::Forbidden);
        }

        Ok("Username updated successfully".to_string())
    }
}

impl Message for ChangeEmailRequest {
    type Result = Result<String, ServiceError>;
}

impl Handler<ChangeEmailRequest> for DbExecutor {
    type Result = Result<String, ServiceError>;

    fn handle(
        &mut self,
        change_email_request: ChangeEmailRequest,
        _: &mut SyncContext<Self>,
    ) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();

        let target = users
            .filter(id.eq(change_email_request.uid.unwrap()))
            .filter(password.eq(hash_string(change_email_request.password)));

        let updated_row = diesel::update(target)
            .set(email.eq(change_email_request.new_email))
            .execute(conn)
            .expect("cant change email");

        if updated_row == 0 {
            return Err(ServiceError::Forbidden);
        }

        Ok("Email updated successfully".to_string())
    }
}