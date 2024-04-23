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

fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> Result<(), std::io::Error> {
    let pb = indicatif::ProgressBar::new(content.lines().count() as u64);
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
            pb.println(format!("[+] finished #{}", line));
            pb.inc(1);
        }
    }
    pb.finish_with_message("done");
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let _ = find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let _ = find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n"); // byte string literal &[u8]
}
