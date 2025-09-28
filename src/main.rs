mod cli;
mod commands;

// use diesel::prelude::*;
// use musicl::models::*;
// use diesel_demo::*;

mod models;
mod schema; // required for diesel
use crate::models::Song;
use diesel::prelude::*;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Commands};
use musicl::establish_connection;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let db = std::fs::read_to_string(&cli.db)
        .with_context(|| format!("could not read file `{}`", cli.db.display()))?;

    // use musicl::schema::songs::dsl::*;

    let connection = &mut establish_connection();

    use crate::schema::songs::dsl::*;

    // let results = songs
    //     .limit(5)
    //     .load(connection)
    //     .expect("Error loading posts");
    let results = songs.load::<Song>(connection).expect("Error loading songs");

    println!("Displaying {} songs", results.len());
    for song in results {
        println!("{}: {}", song.id, song.path);
    }

    // let db: Vec<String> = results.into_iter().map(|s| s.path).collect();

    match cli.command {
        Commands::Play(args) => commands::play::handle(&db, args)?,
        Commands::Add(args) => commands::add::handle(&db, args)?,
        Commands::Sync(args) => commands::sync::handle(&db, args)?,
        Commands::Playlist(args) => commands::playlist::handle(&db, args)?,
        Commands::Archive(args) => commands::archive::handle(&db, args)?,
        Commands::Unarchive(args) => commands::unarchive::handle(&db, args)?,
        Commands::Remove(args) => commands::remove::handle(&db, args)?,
    }

    Ok(())
}