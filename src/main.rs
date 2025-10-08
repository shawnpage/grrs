use clap::Parser;

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
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    // let pattern = std::env::args().nth(1).unwrap();
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    //
    // println!("pattern: {:?}, path: {:?}", pattern, path);
}
