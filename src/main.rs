use clap::Parser;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    // Open the file specified in the path
    let file = File::open(&args.path)?;
    let reader = BufReader::new(file);

    // Read the file line by line
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
