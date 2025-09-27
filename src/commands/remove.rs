use anyhow::{Result};

pub fn handle(pattern: String) -> Result<()> {
    println!("Removing tracks: {}", pattern);
    Ok(())
}
