use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct SyncArgs {
    /// Loop tracks
    #[arg(long)]
    organise_library: bool,
}

pub fn handle(db: &str, args: SyncArgs) -> Result<()> {
    let _ = db;
    println!("Syncing database...");
    if args.organise_library {
        println!("  and organising library")
    }
    Ok(())
}
