pub mod models;
pub mod schema;

use diesel::prelude::*;
// use dotenvy::dotenv;
// use std::env;

pub fn establish_connection() -> SqliteConnection {
    // dotenv().ok();

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "C:/Users/alexa/Desktop/Garage/Workbench/2025-09-26_MusicCliInRust/music.db";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}