use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use crate::ctx::Ctx;

#[derive(Args)]
pub struct SetupArgs {
    /// Path to new music library database file to be created
    new_db: PathBuf,
}

pub fn handle(ctx: &mut Ctx, args: SetupArgs) -> Result<()> {
    let _ = ctx;
    println!("Setting things up: {:?}", args.new_db);
    Ok(())
}
