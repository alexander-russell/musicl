use anyhow::Result;
use clap::Args;
use std::path::PathBuf;


#[derive(Args)]
pub struct SetupArgs {
    /// Path to new music library database file to be created
    new_db: PathBuf,
}

pub fn handle(args: SetupArgs) -> Result<()> {
    println!("Setting things up: {:?}", args.new_db);
    Ok(())
}
