use crate::db::DbExecutor;
use crate::errors::ServiceError;
use crate::models::profile::{GetUserProfile, Profile};
use crate::schema::profiles::dsl::*;
use actix::{Handler, Message, SyncContext};
use diesel::prelude::*;

impl Message for GetUserProfile {
    type Result = Result<Profile, ServiceError>;
}

impl Handler<GetUserProfile> for DbExecutor {
    type Result = Result<Profile, ServiceError>;

    fn handle(
        &mut self,
        profile_request: GetUserProfile,
        _: &mut SyncContext<Self>,
    ) -> Self::Result {
        let own_conn: &PgConnection = &self.0.get().unwrap();

        let profile = profiles
            .filter(user_id.eq(profile_request.id as i32))
            .first(own_conn)?;

        Ok(profile)
    }
}
