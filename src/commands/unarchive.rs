use anyhow::{Result};

pub fn handle(pattern: String) -> Result<()> {
    println!("Unarchiving tracks: {}", pattern);
    Ok(())
}
