//! `cargo xtask evidence ...`: test evidence (spec 0002).
//!
//! `evidence run` is the only way a test run becomes evidence. The other
//! subcommands (`check`, `new`, `baseline`) arrive with the next build steps
//! of spec 0002.

mod collect;
mod context;
mod run;

use std::process::ExitCode;

use anyhow::{Result, bail};

pub fn run(args: &[&str]) -> Result<ExitCode> {
    match args {
        ["run", rest @ ..] => run::run(rest),
        _ => bail!(
            "unknown evidence task `{}`; use `cargo xtask evidence run --kind <fixture|native> [--out <folder>] -- <command>`",
            args.join(" ")
        ),
    }
}
