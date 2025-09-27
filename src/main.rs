use clap::Parser;
use anyhow::{Context, Result};

mod cli;
mod commands;

use cli::{Cli, Commands};


fn main() -> Result<()> {
    let cli = Cli::parse();

    let db = std::fs::read_to_string(&cli.db)
        .with_context(|| format!("could not read file `{}`", cli.db.display()))?;

    match cli.command {
        Commands::Play(args) => commands::play::handle(&db, args)?,
        Commands::Add { path } => commands::add::handle(path)?,
        Commands::Sync { organise_library } => commands::sync::handle(organise_library)?,
        Commands::Playlist { name } => commands::playlist::handle(name)?,
        Commands::Archive { pattern } => commands::archive::handle(pattern)?,
        Commands::Unarchive { pattern } => commands::unarchive::handle(pattern)?,
        Commands::Remove { pattern } => commands::remove::handle(pattern)?,
    }

    Ok(())
}