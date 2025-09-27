use anyhow::{Result};

pub fn handle(organise_library: bool) -> Result<()> {
    println!("Syncing database...");
    if organise_library {
        println!("  and organising library")
    }
    Ok(())
}
