//! `cargo xtask build-app [--release]`: builds the macOS app with xcodebuild.

use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};

const PROJECT: &str = "apps/macos/Stackly.xcodeproj";
const SCHEME: &str = "Stackly";
/// Kept under `target/` so app builds stay out of git and out of
/// `~/Library/Developer/Xcode/DerivedData`.
const DERIVED_DATA: &str = "target/xcode";

/// The Xcode build configuration.
pub enum Configuration {
    Debug,
    Release,
}

impl Configuration {
    fn name(&self) -> &'static str {
        match self {
            Self::Debug => "Debug",
            Self::Release => "Release",
        }
    }
}

pub fn run(root: &Path, configuration: Configuration) -> Result<()> {
    let config = configuration.name();
    let status = Command::new("xcodebuild")
        .current_dir(root)
        .args([
            "-project",
            PROJECT,
            "-scheme",
            SCHEME,
            "-configuration",
            config,
        ])
        .args(["-destination", "platform=macOS,arch=arm64"])
        .args(["-derivedDataPath", DERIVED_DATA, "build"])
        .status()
        .context("cannot run xcodebuild; install Xcode 27 and select it with `xcode-select`")?;
    if !status.success() {
        bail!("xcodebuild failed with {status}");
    }
    println!("built {DERIVED_DATA}/Build/Products/{config}/Stackly.app");
    Ok(())
}
