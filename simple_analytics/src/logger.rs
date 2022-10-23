
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::prelude::*;

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

pub fn create_analytic(conn: &mut SqliteConnection, value_user_agent: &str, value_ip: Option<&str>, value_metadata: &str) -> usize {
    use crate::schema::analytics;
    use crate::models::NewAnalytic;

    let entry = NewAnalytic { user_agent: value_user_agent, ip: value_ip.unwrap_or(""), metadata: value_metadata };

    diesel::insert_into(analytics::table)
        .values(&entry)
        .execute(conn)
        .expect("Error saving new entry")
}
