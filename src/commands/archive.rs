use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct ArchiveArgs {
    /// Database ID of the track
    #[arg(long, conflicts_with = "pattern")]
    id: Option<u32>,

    /// Pattern to search library
    #[arg(long)]
    pattern: Option<String>,
}

pub fn handle(db: &str, args: ArchiveArgs) -> Result<()> {
    let _ = db;
    println!("Archiving tracks: {:?}", args.pattern);
    Ok(())
}
