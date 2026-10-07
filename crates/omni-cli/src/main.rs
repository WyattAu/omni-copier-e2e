//! Example CLI: parse a `PubId` and echo it normalized. Demonstrates the
//! workspace pattern — binaries compose on crates, and the logic lives in
//! `lib.rs` so it stays testable.

use clap::Parser;

/// Validate an identifier using omni-core.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// The identifier to validate.
    id: String,
}

fn main() {
    let args = Args::parse();
    match omni_cli::run(&args.id) {
        Ok(line) => println!("{line}"),
        Err(line) => {
            eprintln!("{line}");
            std::process::exit(1);
        }
    }
}
