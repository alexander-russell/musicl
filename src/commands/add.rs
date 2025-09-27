use anyhow::{Result};
use std::path::PathBuf;

pub fn handle(path: PathBuf) -> Result<()> {
    println!("Adding: {:?}", path);
    Ok(())
}
