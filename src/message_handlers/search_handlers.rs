use crate::db::DbExecutor;
use crate::errors::ServiceError;
use crate::models::user::{ProfileAPI, User, UserAPIWithoutCounters};
use crate::schema::users::dsl::*;
use crate::{controllers::search_controller::SearchUsersQueryStrings, models::profile::Profile};
use actix::{Handler, Message, SyncContext};
use diesel::prelude::*;

impl Message for SearchUsersQueryStrings {
    type Result = Result<Vec<UserAPIWithoutCounters>, ServiceError>;
}

impl Handler<SearchUsersQueryStrings> for DbExecutor {
    type Result = Result<Vec<UserAPIWithoutCounters>, ServiceError>;

    fn handle(
        &mut self,
        query_strings: SearchUsersQueryStrings,
        _: &mut SyncContext<Self>,
    ) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();

        let pattern = format!("%{}%", query_strings.username);

        let found_users = users
            .filter(username.like(pattern))
            .load::<User>(conn)?;

        let profile = Profile::belonging_to(&found_users)
            .load::<Profile>(conn)?
            .grouped_by(&found_users);

        let data: Vec<(User, Vec<Profile>)> =
            found_users.into_iter().zip(profile).collect::<Vec<_>>();

        let found_users_with_profile: Vec<UserAPIWithoutCounters> = data
            .into_iter()
            .map(|(user, profile)| {
                let status = if !profile.is_empty() {
                    profile[0].status.to_owned()
                } else {
                    Some("".to_string())
                };

                let description = if !profile.is_empty() {
                    profile[0].description.to_owned()
                } else {
                    Some("".to_string())
                };

                UserAPIWithoutCounters {
                    user,
                    profile: ProfileAPI {
                        status,
                        description,
                    },
                }
            })
            .collect();

        Ok(found_users_with_profile)
    }
}
