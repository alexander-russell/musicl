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
        Commands::Play(args) => handle_play(&db, args)?,

        Commands::Add { path } => handle_add(path)?,
        Commands::Sync { organise_library } => handle_sync(organise_library)?,
        Commands::Playlist { name } => handle_playlist(name)?,
        Commands::Archive { pattern } => handle_archive(pattern)?,
        Commands::Unarchive { pattern } => handle_unarchive(pattern)?,
        Commands::Remove { pattern } => handle_remove(pattern)?,
    }

    Ok(())
}

/// Handle the `play` command
fn handle_play(db: &str, args: PlayArgs) -> Result<()> {
    let mut matches = find_matches(db, &args.pattern)?;

    if args.track && matches.len() > 1 && !args.multiple_tracks {
        anyhow::bail!(
            "Multiple matches found, but --track without --multiple-tracks only allows one"
        );
    }

    // apply `--first N`
    if let Some(n) = args.first {
        matches.truncate(n as usize);
    }

    // apply `--shuffle`
    if args.shuffle {
        use rand::seq::SliceRandom;
        let mut rng = rand::rng();
        matches.shuffle(&mut rng);
    }

    // apply `--reverse`
    if args.reverse {
        matches.reverse();
    }

    // apply `--repeat`
    if args.repeat {
        let orig = matches.clone();
        for _ in 1..20 {
            matches.extend_from_slice(&orig);
        }
    }

    // apply `--backing` (append 20 random items from db)
    if args.backing {
        use rand::seq::IteratorRandom;
        let all_tracks: Vec<_> = db.lines().map(|s| s.to_string()).collect();
        let mut rng = rand::rng();
        let backing_tracks: Vec<_> = all_tracks.into_iter().choose_multiple(&mut rng, 20);
        matches.extend(backing_tracks);
    }

    for track in matches {
        println!("{}", track);
    }

    Ok(())
}

fn handle_add(path: PathBuf) -> Result<()> {
    println!("Adding: {:?}", path);
    Ok(())
}

fn handle_sync(organise_library: bool) -> Result<()> {
    println!("Syncing database...");
    if organise_library {
        println!("  and organising library")
    }
    Ok(())
}

fn handle_playlist(name: Option<String>) -> Result<()> {
    println!("Managing playlist: {:?}", name);
    Ok(())
}

fn handle_archive(pattern: String) -> Result<()> {
    println!("Archiving tracks: {}", pattern);
    Ok(())
}

fn handle_unarchive(pattern: String) -> Result<()> {
    println!("Unarchiving tracks: {}", pattern);
    Ok(())
}

fn handle_remove(pattern: String) -> Result<()> {
    println!("Removing tracks: {}", pattern);
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
