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
        repeat: bool, // `loop` is a reserved word

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
            lyric: _,
            first,
            sample: _,
            shuffle,
            reverse,
            repeat,
            backing,
            include_archive: _,
        } => {
            // println!("Playing tracks matching: {}", pattern);
            let mut matches = find_matches(&db, &pattern)?;

            // enforce `--track` and `--multiple-tracks`
            if track && matches.len() > 1 && !multiple_tracks {
                anyhow::bail!(
                    "Multiple matches found, but --track without --multiple-tracks only allows one"
                );
            }

            // apply `--first N`
            if let Some(n) = first {
                matches.truncate(n as usize);
            }

            // apply `--shuffle`
            if shuffle {
                use rand::seq::SliceRandom;
                let mut rng = rand::rng();
                matches.shuffle(&mut rng);
            }

            // apply `--reverse`
            if reverse {
                matches.reverse();
            }

            // apply `--loop` (repeat 20 times)
            if repeat {
                let orig = matches.clone();
                for _ in 1..20 {
                    matches.extend_from_slice(&orig);
                }
            }

            // apply `--backing` (append 20 random items from db)
            if backing {
                use rand::seq::IteratorRandom;
                let all_tracks: Vec<_> = db.lines().map(|s| s.to_string()).collect();
                let mut rng = rand::rng();
                let backing_tracks: Vec<_> = all_tracks.into_iter().choose_multiple(&mut rng, 20);
                matches.extend(backing_tracks);
            }

            for track in matches {
                println!("{}", track);
            }
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
