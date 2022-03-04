extern crate diesel;

use actix::Actor;
use actix::SyncContext;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub struct DbExecutor(
    pub Pool<ConnectionManager<PgConnection>>,
    pub Pool<ConnectionManager<PgConnection>>,
);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
