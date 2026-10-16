//! Repo tasks for Stackly, run as `cargo xtask <task>`.

mod build_app;
mod check_deps;
mod codegen;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Result, bail};

const USAGE: &str = "\
Usage: cargo xtask <task>

Tasks:
  codegen              Regenerate the Swift contract types and the JSON Schemas
  codegen --check      Fail when the committed generated files are out of date
  check-deps           Check crate dependency direction and the unsafe policy
  build-app            Build the macOS app with xcodebuild (scheme Stackly, Debug)
  build-app --release  Same, with the Release configuration
";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let root = repo_root();
    match args.as_slice() {
        ["codegen"] => codegen::run(&root, codegen::Mode::Write),
        ["codegen", "--check"] => codegen::run(&root, codegen::Mode::Check),
        ["check-deps"] => check_deps::run(&root),
        ["build-app"] => build_app::run(&root, build_app::Configuration::Debug),
        ["build-app", "--release"] => build_app::run(&root, build_app::Configuration::Release),
        ["help" | "--help" | "-h"] => {
            print!("{USAGE}");
            Ok(())
        }
        [] => bail!("no task given\n\n{USAGE}"),
        _ => bail!("unknown task `{}`\n\n{USAGE}", args.join(" ")),
    }
}

/// The repository root, one level above this crate.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask sits one directory below the repository root")
        .to_path_buf()
}
