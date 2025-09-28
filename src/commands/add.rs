use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use crate::ctx::Ctx;

#[derive(Args)]
pub struct AddArgs {
    /// Path to new music file or folder
    path: PathBuf,
}

pub fn handle(ctx: &mut Ctx, args: AddArgs) -> Result<()> {
    let _ = ctx;
    println!("Adding: {:?}", args.path);
    Ok(())
}
