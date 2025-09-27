use anyhow::{Result};

pub fn handle(pattern: String) -> Result<()> {
    println!("Archiving tracks: {}", pattern);
    Ok(())
}
