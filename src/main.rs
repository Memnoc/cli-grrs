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

fn main() {
    let args = Cli::parse();

    // FIX: not an efficient implementation
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");

    // Open the file specified in the path
    let file = File::open(&args.path).expect("could not open file");
    let reader = BufReader::with_capacity(10, file);

    // Read the file line by line
    for line_result in reader.lines() {
        let line = line_result.expect("could not read line");
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
