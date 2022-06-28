use crate::errors::ServiceError;
use crate::models::profile::{CreateProfile, Profile};
use crate::models::user::{
    Counters, CreateUser, ProfileAPI, SetAvatarRequest, User, UserAPIWithoutCounters,
};
use crate::models::user_requests::{GetMultipleUsers, GetUserByIDReq};
use crate::schema::profiles::dsl::*;
use crate::schema::users::dsl::*;
use crate::{db::DbExecutor, models::user::UserAPI};
use actix::{Handler, Message, SyncContext};
use blake3::Hasher;
use diesel::prelude::*;
use diesel::sql_types::Integer;

impl Message for CreateUser {
    type Result = Result<User, ServiceError>;
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<User, ServiceError>;

    fn handle(&mut self, mut new_user: CreateUser, _: &mut SyncContext<Self>) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();

        let mut hasher = Hasher::new();

        hasher.update(&new_user.password.as_bytes());

        new_user.password = hasher.finalize().to_hex().chars().collect();

        let inserted_user: User = diesel::insert_into(users)
            .values(&new_user)
            .get_result(connection)?;

        // Create also user profile on Whisper App
        let _create_whisper_profile = diesel::insert_into(profiles)
            .values(CreateProfile {
                user_id: inserted_user.id,
                status: "".to_string(),
                description: "".to_string(),
            })
            .execute(connection)?;

        Ok(inserted_user)
    }
}

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
        let connection: &PgConnection = &self.0.get().unwrap();

        let user: User = users.find(user_id_request.id).first(connection)?;
        let profile = Profile::belonging_to(&user)
            .select((status, description))
            .distinct()
            .first::<ProfileAPI>(connection)
            .unwrap();

        use diesel::sql_types::BigInt;
        #[derive(Debug, QueryableByName)]
        struct Count {
            #[sql_type = "BigInt"]
            count: i64,
        }

        let following = diesel::sql_query("SELECT COUNT(*) FROM followers where user_id = $1")
            .bind::<Integer, _>(user_id_request.id)
            .load::<Count>(connection)
            .expect("message error")
            .pop()
            .expect("no rows")
            .count;

        let followers = diesel::sql_query("SELECT COUNT(*) FROM followers where following = $1")
            .bind::<Integer, _>(user_id_request.id)
            .load::<Count>(connection)
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
        let connection: &PgConnection = &self.0.get().unwrap();

        use crate::schema::users::dsl::*;

        let selected_users = users
            .filter(id.eq_any(request.ids))
            .order_by(created_at.asc())
            .get_results::<User>(connection)?;

        let users_profiles = Profile::belonging_to(&selected_users)
            .load::<Profile>(connection)?
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
        let connection: &PgConnection = &self.0.get().unwrap();

        let update_avatar = diesel::update(users.find(request.user_id))
            .set(avatar.eq(request.avatar))
            .execute(connection);

        match update_avatar {
            Ok(response) => println!("Avatar updated: {:?}", response),
            Err(error) => println!("Could not update avatar: {:?}", error),
        }

        Ok(())
    }
}
