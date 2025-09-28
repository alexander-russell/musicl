use anyhow::{Result};
use clap::Args;

#[derive(Args)]
pub struct UnarchiveArgs {
    /// Database ID of the track
    #[arg(long, conflicts_with = "pattern")]
    id: Option<u32>,

    /// Pattern to search library
    #[arg(long)]
    pattern: Option<String>,
}

pub fn handle(db: &str, args: UnarchiveArgs) -> Result<()> {
    let _ = db;
    println!("Unarchiving tracks: {:?}", args.pattern);
    Ok(())
}
