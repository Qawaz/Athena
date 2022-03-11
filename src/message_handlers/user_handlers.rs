use crate::errors::ServiceError;
use crate::models::profile::Profile;
use crate::models::user::{Counters, ProfileAPI, User};
use crate::models::user_requests::GetUserByIDReq;
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
