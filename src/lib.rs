pub mod models;
pub mod schema;

use std::path::PathBuf;

use diesel::prelude::*;
// use dotenvy::dotenv;
// use std::env;

pub fn establish_connection(db: PathBuf) -> SqliteConnection {
    // dotenv().ok();

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let database_url = "C:/Users/alexa/Desktop/Garage/Workbench/2025-09-26_MusicCliInRust/music.db";
    SqliteConnection::establish(&db.to_string_lossy().into_owned())
        .unwrap_or_else(|_| panic!("Error connecting to {}", &db.to_string_lossy().into_owned()))
}