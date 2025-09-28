use anyhow::Result;
use clap::Args;

use crate::ctx::Ctx;

#[derive(Args)]
pub struct SyncArgs {
    /// Loop tracks
    #[arg(long)]
    organise_library: bool,
}

pub fn handle(ctx: &mut Ctx, args: SyncArgs) -> Result<()> {
    let _ = ctx;
    println!("Syncing database...");
    if args.organise_library {
        println!("  and organising library")
    }
    Ok(())
}
