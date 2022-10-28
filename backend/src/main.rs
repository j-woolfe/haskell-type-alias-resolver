// Simple CLI interface for functions provided in htar

use htar::{run_on_file, start_web_server};

// CLI library
use clap::Parser as CLIParser;

use std::path::PathBuf;

#[derive(CLIParser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Runs as a web server on port 3000
    #[clap(short, long, action)]
    server: bool,

    /// Path of Haskell file to analyse
    #[clap(short, long, value_parser, value_name = "FILE")]
    path: Option<PathBuf>,

    /// Target type alias
    #[clap(short, long, value_parser, value_name = "TYPE")]
    target: Option<String>,

    /// Enable human readable output
    #[clap(short = 'r', long, action)]
    human_readable: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.server {
        start_web_server().await;
    } else {
        // Run in command line mode
        match (args.path, args.target) {
            (None, _) => {
                println!("Missing path to source file (use -p)")
            }
            (_, None) => {
                println!("Missing target type (use -t)")
            }
            (Some(path), Some(target_type)) => {
                let replacement_data = run_on_file(path, target_type);

                if args.human_readable {
                    println!("{}", replacement_data)
                } else {
                    println!("{}", serde_json::to_string(&replacement_data).unwrap())
                }
            }
        }
    }
}
