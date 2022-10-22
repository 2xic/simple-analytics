//pub mod nested;

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel::r2d2::{ConnectionManager, Pool};
//use models;

pub fn establish_connection() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
/*    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
*/
    let pool = Pool::builder()
    .build(ConnectionManager::<SqliteConnection>::new(database_url))
    .unwrap();


//    pool.max_size(1);

    return pool;
}
