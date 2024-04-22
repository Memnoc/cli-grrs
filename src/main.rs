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

fn find_matches(content: &str, pattern: &str) {
    let pb = indicatif::ProgressBar::new(content.lines().count() as u64);
    for line in content.lines() {
        if line.contains(pattern) {
            pb.println(format!("[+] finished #{}", line));
            println!("{}", line);
        }
        pb.inc(1);
        pb.finish_with_message("done");
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern);
    Ok(())
}
