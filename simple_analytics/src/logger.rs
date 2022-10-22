
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn establish_connection() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Pool::builder()
    .max_size(1)
    .build(ConnectionManager::<SqliteConnection>::new(database_url))
    .unwrap();

    assert!(pool.max_size() == 1);    

    return pool;
}
