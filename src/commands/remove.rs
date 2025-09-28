use anyhow::{Result};
use clap::Args;

use crate::ctx::Ctx;

#[derive(Args)]
pub struct RemoveArgs {
    /// Database ID of the track
    #[arg(long, conflicts_with = "pattern")]
    id: Option<u32>,

    /// Pattern to search library
    #[arg(long)]
    pattern: Option<String>,
}

pub fn handle(ctx: &mut Ctx, args: RemoveArgs) -> Result<()> {
    let _ = ctx;
    println!("Removing tracks: {:?}", args.id);
    Ok(())
}
