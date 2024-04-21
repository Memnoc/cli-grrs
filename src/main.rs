use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
    let file = match File::open(&args.path) {
        Ok(file) => file,
        Err(e) => {
            return Err(io::Error::new(
                e.kind(),
                format!("Failed to open the file '{}': {}", args.path.display(), e),
            ))
        }
    };

    let reader = BufReader::new(file);

    // Read the file line by line
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                return Err(io::Error::new(
                    e.kind(),
                    "Error reading a line from tne file",
                ))
            }
        };
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
