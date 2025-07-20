// src/main.rs
/*
 * Main executable for FutureCrypto
 */

use clap::Parser;
use futurecrypto::{Result, run};

#[derive(Parser)]
#[command(version, about = "FutureCrypto - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
