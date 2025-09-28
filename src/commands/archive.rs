use anyhow::Result;
use clap::Args;

use crate::ctx::Ctx;

#[derive(Args)]
pub struct ArchiveArgs {
    /// Database ID of the track
    #[arg(long, conflicts_with = "pattern")]
    id: Option<u32>,

    /// Pattern to search library
    #[arg(long)]
    pattern: Option<String>,
}

pub fn handle(ctx: &mut Ctx, args: ArchiveArgs) -> Result<()> {
    let _ = ctx;
    println!("Archiving tracks: {:?}", args.pattern);
    Ok(())
}
