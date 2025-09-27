use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    Play {
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
        r#loop: bool, // `loop` is a reserved word

        /// Add backing tracks
        #[arg(long)]
        backing: bool,

        /// Include archived tracks
        #[arg(long)]
        include_archive: bool,
    },

    /// Add new music files to the library
    Add {
        /// Path to new music file or folder
        path: PathBuf,
    },

    /// Sync/update the library
    Sync,

    /// Manage playlists
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
        Commands::Play {
            pattern,
            track,
            multiple_tracks,
            lyric,
            first,
            sample,
            shuffle,
            reverse,
            r#loop,
            backing,
            include_archive,
        } => {
            // println!("Playing tracks matching: {}", pattern);
            let matches = find_matches(&db, &pattern)?;
            for track in matches {
                println!("{}", track);
            }
            // TODO: implement playback
        }
        Commands::Add { path } => {
            println!("Adding: {:?}", path);
            // TODO: implement add
        }
        Commands::Sync => {
            println!("Syncing library...");
            // TODO: implement sync
        }
        Commands::Playlist { name } => {
            println!("Managing playlist: {:?}", name);
            // TODO: implement playlist authoring
        }
        Commands::Archive { pattern } => {
            println!("Archiving tracks: {}", pattern);
            // TODO: implement archive
        }
        Commands::Unarchive { pattern } => {
            println!("Unarchiving tracks: {}", pattern);
            // TODO: implement unarchive
        }
        Commands::Remove { pattern } => {
            println!("Removing tracks: {}", pattern);
            // TODO: implement remove
        }
    }

    Ok(())
}

fn find_matches(content: &str, pattern: &str) -> Result<Vec<String>> {
    let mut matches = Vec::new();
    for line in content.lines() {
        if line.contains(pattern) {
            matches.push(line.to_string());
        }
    }
    Ok(matches)
}
