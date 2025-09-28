use anyhow::{Result};
use clap::Args;

use crate::ctx::Ctx;

#[derive(Args)]
pub struct UnarchiveArgs {
    /// Database ID of the track
    #[arg(long, conflicts_with = "pattern")]
    id: Option<u32>,

    /// Pattern to search library
    #[arg(long)]
    pattern: Option<String>,
}

pub fn handle(ctx: &mut Ctx, args: UnarchiveArgs) -> Result<()> {
    let _ = ctx;
    println!("Unarchiving tracks: {:?}", args.pattern);
    Ok(())
}
