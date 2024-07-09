use anyhow::{Context, Result};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    // Create a new progress bar with a length of the total lines in the file
    let pb = ProgressBar::new(content.lines().count() as u64);

    // Create the style separately
    let style = ProgressStyle::default_bar()
        .template("[{spinner:.green}] [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
        .unwrap_or_else(|e| {
            eprintln!("Failed to set progress bar style: {}", e);
            ProgressStyle::default_bar()
        })
        .progress_chars("#>-");

    // Apply the style to the progress bar
    pb.set_style(style);

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
        pb.inc(1);
    }

    // Mark the progress bar as finished
    pb.finish_with_message("Search complete");

    Ok(())
}