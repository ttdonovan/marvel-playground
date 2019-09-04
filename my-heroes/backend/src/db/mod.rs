use diesel::{
    prelude::*,
    sqlite::SqliteConnection,
};
use dotenv::dotenv;

use std::env;

pub mod models;
pub mod operations;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").
        expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connection to {}", database_url))
}