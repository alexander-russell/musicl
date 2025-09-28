use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct AddArgs {
    /// Path to new music file or folder
    path: PathBuf,
}

pub fn handle(db: &str, args: AddArgs) -> Result<()> {
    let _ = db;
    println!("Adding: {:?}", args.path);
    Ok(())
}
