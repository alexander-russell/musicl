use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct PlaylistArgs {
    /// Playlist name
    name: Option<String>,

    /// Database ID of the track
    #[arg(long, conflicts_with = "pattern")]
    id: Option<u32>,

    /// Pattern to search library
    #[arg(long)]
    pattern: Option<String>,

    /// Remove tracks instead of adding
    #[arg(long)]
    remove: bool,
}

pub fn handle(db: &str, args: PlaylistArgs) -> Result<()> {
    let _ = db;
    println!("Managing playlist: {:?}", args.name);
    Ok(())
}
