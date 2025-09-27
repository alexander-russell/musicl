use crate::PlayArgs;
use anyhow::Result;

pub fn handle(db: &str, args: PlayArgs) -> Result<()> {
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

fn find_matches(content: &str, pattern: &str) -> Result<Vec<String>> {
    let mut matches = Vec::new();
    for line in content.lines() {
        if line.contains(pattern) {
            matches.push(line.to_string());
        }
    }
    Ok(matches)
}
