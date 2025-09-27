use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Music library manager
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Path to the music library database
    #[arg(short, long)]
    pub db: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Play tracks from the library
    Play(crate::commands::play::PlayArgs),

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