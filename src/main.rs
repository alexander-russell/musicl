use clap::Parser;
use anyhow::{Context, Result};


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the music library data
    db: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.db)
        .with_context(|| format!("could not read file `{}`", args.db.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)
                .with_context(|| format!("could not write line {}", line))?;
        }
    }

    Ok(())
}