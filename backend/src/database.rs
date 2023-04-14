use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use std::{env, error::Error};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DbError = Box<dyn Error + Send + Sync>;

pub fn connect_sqlite() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: DbPool = Pool::builder()
        .build(manager)
        .expect("Failed to connect to database");

    pool
}
