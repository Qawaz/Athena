use crate::errors::ServiceError;
use crate::models::profile::Profile;
use crate::models::user::{Counters, ProfileAPI, SetAvatarRequest, User, UserAPIWithoutCounters};
use crate::models::user_requests::{GetMultipleUsers, GetUserByIDReq};
use crate::schema::profiles::dsl::*;
use crate::schema::users::dsl::*;
use crate::{db::DbExecutor, models::user::UserAPI};
use actix::{Handler, Message, SyncContext};
use diesel::prelude::*;
use diesel::sql_types::Integer;

impl Message for GetUserByIDReq {
    type Result = Result<UserAPI, ServiceError>;
}

impl Handler<GetUserByIDReq> for DbExecutor {
    type Result = Result<UserAPI, ServiceError>;

    fn handle(
        &mut self,
        user_id_request: GetUserByIDReq,
        _: &mut SyncContext<Self>,
    ) -> Self::Result {
        let gateway_conn: &PgConnection = &self.1.get().unwrap();
        let own_conn: &PgConnection = &self.0.get().unwrap();

        let user: User = users.find(user_id_request.id).first(gateway_conn)?;
        let profile = Profile::belonging_to(&user)
            .select((status, description))
            .distinct()
            .first::<ProfileAPI>(own_conn)
            .unwrap();

        use diesel::sql_types::BigInt;
        #[derive(Debug, QueryableByName)]
        struct Count {
            #[sql_type = "BigInt"]
            count: i64,
        }

        let following = diesel::sql_query("SELECT COUNT(*) FROM followers where user_id = $1")
            .bind::<Integer, _>(user_id_request.id)
            .load::<Count>(own_conn)
            .expect("message error")
            .pop()
            .expect("no rows")
            .count;

        let followers = diesel::sql_query("SELECT COUNT(*) FROM followers where following = $1")
            .bind::<Integer, _>(user_id_request.id)
            .load::<Count>(own_conn)
            .expect("message error")
            .pop()
            .expect("no rows")
            .count;

        Ok(UserAPI {
            user,
            profile,
            counters: Counters {
                feeds: following,
                followers,
            },
        })
    }
}

impl Message for GetMultipleUsers {
    type Result = Result<Vec<UserAPIWithoutCounters>, ServiceError>;
}

impl Handler<GetMultipleUsers> for DbExecutor {
    type Result = Result<Vec<UserAPIWithoutCounters>, ServiceError>;

    fn handle(&mut self, request: GetMultipleUsers, _: &mut SyncContext<Self>) -> Self::Result {
        let own_conn: &PgConnection = &self.0.get().unwrap();
        let gateway_conn: &PgConnection = &self.1.get().unwrap();

        use crate::schema::users::dsl::*;

        let selected_users = users
            .filter(id.eq_any(request.ids))
            .order_by(created_at.asc())
            .get_results::<User>(gateway_conn)?;

        let users_profiles = Profile::belonging_to(&selected_users)
            .load::<Profile>(own_conn)?
            .grouped_by(&selected_users);

        let data = selected_users
            .into_iter()
            .zip(users_profiles)
            .collect::<Vec<_>>();

        let users_with_profile: Vec<UserAPIWithoutCounters> = data
            .into_iter()
            .map(|(user, profile)| {
                let user_status = if !profile.is_empty() {
                    profile[0].status.to_owned()
                } else {
                    Some("".to_string())
                };

                let user_description = if !profile.is_empty() {
                    profile[0].description.to_owned()
                } else {
                    Some("".to_string())
                };

                UserAPIWithoutCounters {
                    user,
                    profile: ProfileAPI {
                        status: user_status,
                        description: user_description,
                    },
                }
            })
            .collect();

        Ok(users_with_profile)
    }
}

impl Message for SetAvatarRequest {
    type Result = Result<(), ServiceError>;
}

impl Handler<SetAvatarRequest> for DbExecutor {
    type Result = Result<(), ServiceError>;

    fn handle(&mut self, request: SetAvatarRequest, _: &mut SyncContext<Self>) -> Self::Result {
        let gateway_conn: &PgConnection = &self.1.get().unwrap();

        let update_avatar = diesel::update(users.find(request.user_id))
            .set(avatar.eq(request.avatar))
            .execute(gateway_conn);

        match update_avatar {
            Ok(response) => println!("Avatar updated: {:?}", response),
            Err(error) => println!("Could not update avatar: {:?}", error),
        }

        Ok(())
    }
}
