use anyhow::{Context, Result};
use clap::Parser;

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

    // NOTE: progress bar
    let pb = indicatif::ProgressBar::new(content.lines().count() as u64);

    for line in content.lines() {
        if line.contains(&args.pattern) {
            pb.println(format!("[+] finished #{}", line));
            println!("{}", line);
        }
    }
    pb.finish_with_message("done");
    Ok(())
}
