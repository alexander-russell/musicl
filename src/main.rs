use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;

/// Music library manager
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Path to the music library database
    #[arg(short, long)]
    db: PathBuf,

    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    /// Play tracks from the library
    Play(commands::play::PlayArgs),

    /// Add new music files to the library
    Add {
        /// Path to new music file or folder
        path: PathBuf,
    },

    /// Synchronise database with files
    Sync {
        /// Loop tracks
        #[arg(long)]
        organise_library: bool,
    },

    /// Build a playlist
    Playlist {
        /// Playlist name
        name: Option<String>,
    },

    /// Archive a track
    Archive { pattern: String },

    /// Unarchive a track
    Unarchive { pattern: String },

    /// Remove a track
    Remove { pattern: String },
}

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