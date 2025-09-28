use anyhow::Result;
use clap::Args;

use crate::ctx::Ctx;
use crate::models::Song;
use crate::schema::songs::dsl::*;
use diesel::{prelude::*};

#[derive(Args)]
pub struct PlayArgs {
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

pub fn handle(ctx: &mut Ctx, args: PlayArgs) -> Result<()> {
    // let songs = "str";
    // let mut matches = find_matches(songs, &args.pattern)?;

    let mut matches = filter_songs(ctx.connection, args.pattern);

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
    // if args.repeat {
    //     let orig = matches.clone();
    //     for _ in 1..20 {
    //         matches.extend_from_slice(&orig);
    //     }
    // }

    // apply `--backing` (append 20 random items from db)
    // if args.backing {
    //     use rand::seq::IteratorRandom;
    //     let all_tracks: Vec<_> = db.lines().map(|s| s.to_string()).collect();
    //     let mut rng = rand::rng();
    //     let backing_tracks: Vec<_> = all_tracks.into_iter().choose_multiple(&mut rng, 20);
    //     matches.extend(backing_tracks);
    // }

    for song in matches {
        println!("{}", song.path);
    }

    Ok(())
}

fn filter_songs(connection: & mut SqliteConnection, pattern: String) -> Vec<Song> {
    songs
        .filter(path.like(format!("%{}%", pattern)))
        .load::<Song>(connection)
        .expect("DB filter_songs failed")
}