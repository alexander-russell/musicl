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

#[derive(clap::Args)]
struct PlayArgs {
    /// Pattern to search library
    pattern: String,

    /// Search by track
    #[arg(short = 't', long)]
    track: bool,

    /// Accept multiple tracks
    #[arg(short = 'm', long)]
    multiple_tracks: bool,

    /// Search by lyric
    #[arg(short = 'l', long)]
    lyric: bool,

    /// Select the first n tracks
    #[arg(short = 'f', long)]
    first: Option<u32>,

    /// Randomly select n tracks
    #[arg(short = 's', long)]
    sample: Option<u32>,

    /// Shuffle tracks
    #[arg(long)]
    shuffle: bool,

    /// Reverse track order
    #[arg(long)]
    reverse: bool,

    /// Loop tracks
    #[arg(long)]
    repeat: bool,

    /// Add backing tracks
    #[arg(long)]
    backing: bool,

    /// Include archived tracks
    #[arg(long)]
    include_archive: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Play tracks from the library
    Play(PlayArgs),

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
        Commands::Add { path } => add::handle(path)?,
        Commands::Sync { organise_library } => sync::handle(organise_library)?,
        Commands::Playlist { name } => playlist::handle(name)?,
        Commands::Archive { pattern } => archive::handle(pattern)?,
        Commands::Unarchive { pattern } => unarchive::handle(pattern)?,
        Commands::Remove { pattern } => remove::handle(pattern)?,
    }

    Ok(())
}

mod add {
    use super::*;
    pub fn handle(path: PathBuf) -> Result<()> {
        println!("Adding: {:?}", path);
        Ok(())
    }
}

mod sync {
    use super::*;
    pub fn handle(organise_library: bool) -> Result<()> {
        println!("Syncing database...");
        if organise_library {
            println!("  and organising library")
        }
        Ok(())
    }
}

mod playlist {
    use super::*;
    pub fn handle(name: Option<String>) -> Result<()> {
        println!("Managing playlist: {:?}", name);
        Ok(())
    }
}

mod archive {
    use super::*;
    pub fn handle(pattern: String) -> Result<()> {
        println!("Archiving tracks: {}", pattern);
        Ok(())
    }
}

mod unarchive {
    use super::*;
    pub fn handle(pattern: String) -> Result<()> {
        println!("Unarchiving tracks: {}", pattern);
        Ok(())
    }
}

mod remove {
    use super::*;
    pub fn handle(pattern: String) -> Result<()> {
        println!("Removing tracks: {}", pattern);
        Ok(())
    }
}
