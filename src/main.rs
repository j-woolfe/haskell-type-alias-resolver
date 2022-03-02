mod alias;

use crate::alias::alias_replacement;

// CLI library
use clap::Parser as CLIParser;
use std::fs::read_to_string;

#[derive(CLIParser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    type_sig: String,

    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let source_code = read_to_string(args.path).unwrap();
    let source = source_code.as_bytes();

    let json = alias_replacement(&args.type_sig, source);

    println!("{}", json.to_string());
}
