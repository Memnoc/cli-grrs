use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,          // pattern to look for
    path: std::path::PathBuf, // path of file to read
}

fn main() {
    println!("== GRRS ==");

    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
