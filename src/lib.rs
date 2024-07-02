mod error;
pub mod reset;
pub mod servers;

pub use error::Error;

pub type Result<T, E = crate::error::Error> = std::result::Result<T, E>;

use std::env;

// add the `r2d2` feature for diesel
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use dotenvy::dotenv;
// set an alias, so we don't have to keep writing out this long type
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub type DbPoolConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn etablish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create a pool.")
}
