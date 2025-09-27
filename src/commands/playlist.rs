use anyhow::{Result};

pub fn handle(name: Option<String>) -> Result<()> {
    println!("Managing playlist: {:?}", name);
    Ok(())
}
