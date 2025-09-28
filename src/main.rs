mod cli;
mod commands;

use clap::Parser;
use anyhow::{Context, Result};
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let db = std::fs::read_to_string(&cli.db)
        .with_context(|| format!("could not read file `{}`", cli.db.display()))?;

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