//! `stackly`, the Stackly command line client.
//!
//! The CLI only talks to `stacklyd` over IPC. It never runs provider binaries
//! and never writes the registry. There are no commands yet, so it prints its
//! help.

use clap::{CommandFactory, Parser};

/// The Stackly command line client.
#[derive(Debug, Parser)]
#[command(name = "stackly", version, about)]
struct Cli {}

fn main() -> std::io::Result<()> {
    let _cli = Cli::parse();
    Cli::command().print_help()
}
