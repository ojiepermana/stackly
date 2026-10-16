//! `cargo xtask codegen [--check]`: Swift contract types and JSON Schemas,
//! both generated from `stackly-protocol`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};

/// The `typeshare-cli` release the committed Swift file was generated with.
const TYPESHARE_CLI_VERSION: &str = "1.13.4";
const PROTOCOL_SRC: &str = "crates/protocol/src";
const TYPESHARE_CONFIG: &str = "typeshare.toml";
const SWIFT_OUTPUT: &str = "clients/swift/Sources/StacklyClient/Generated/StacklyProtocol.swift";
const SCHEMA_DIR: &str = "protocols/schemas";
const SCHEMA_SUFFIX: &str = ".schema.json";

/// Whether to write the generated files or only compare them.
pub enum Mode {
    Write,
    Check,
}

/// One generated file: its path relative to the repo root and its contents.
struct Generated {
    path: PathBuf,
    contents: String,
}

pub fn run(root: &Path, mode: Mode) -> Result<()> {
    check_typeshare_version()?;
    let mut generated = vec![generate_swift(root)?];
    generated.extend(generate_schemas()?);
    match mode {
        Mode::Write => write(root, &generated),
        Mode::Check => check(root, &generated),
    }
}

fn check_typeshare_version() -> Result<()> {
    let install = format!("cargo install --locked typeshare-cli --version {TYPESHARE_CLI_VERSION}");
    let output = Command::new("typeshare")
        .arg("--version")
        .output()
        .with_context(|| format!("cannot run `typeshare`; install it with `{install}`"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let found = stdout.split_whitespace().nth(1).unwrap_or("unknown");
    if !output.status.success() || found != TYPESHARE_CLI_VERSION {
        bail!(
            "typeshare-cli {TYPESHARE_CLI_VERSION} is required, found {found}; install it with `{install}`"
        );
    }
    Ok(())
}

fn generate_swift(root: &Path) -> Result<Generated> {
    let staging = tempfile::tempdir().context("cannot create a staging directory")?;
    let output = staging.path().join("StacklyProtocol.swift");
    let status = Command::new("typeshare")
        .arg(root.join(PROTOCOL_SRC))
        .args(["--lang", "swift"])
        .arg("--config-file")
        .arg(root.join(TYPESHARE_CONFIG))
        .arg("--output-file")
        .arg(&output)
        // typeshare logs every step at info level; keep only real problems.
        .env("RUST_LOG", "error")
        .status()
        .context("cannot run `typeshare`")?;
    if !status.success() {
        bail!("typeshare failed with {status}");
    }
    let contents =
        fs::read_to_string(&output).with_context(|| format!("cannot read {}", output.display()))?;
    Ok(Generated {
        path: PathBuf::from(SWIFT_OUTPUT),
        contents,
    })
}

fn generate_schemas() -> Result<Vec<Generated>> {
    stackly_protocol::json_schemas()
        .into_iter()
        .map(|(stem, schema)| {
            let mut contents = serde_json::to_string_pretty(&schema)
                .with_context(|| format!("cannot serialize the {stem} schema"))?;
            contents.push('\n');
            Ok(Generated {
                path: Path::new(SCHEMA_DIR).join(format!("{stem}{SCHEMA_SUFFIX}")),
                contents,
            })
        })
        .collect()
}

fn write(root: &Path, generated: &[Generated]) -> Result<()> {
    for file in generated {
        let path = root.join(&file.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("cannot create {}", parent.display()))?;
        }
        fs::write(&path, &file.contents)
            .with_context(|| format!("cannot write {}", path.display()))?;
        println!("wrote {}", file.path.display());
    }
    for stale in stale_schemas(root, generated)? {
        fs::remove_file(root.join(&stale))
            .with_context(|| format!("cannot remove {}", stale.display()))?;
        println!("removed {}", stale.display());
    }
    Ok(())
}

fn check(root: &Path, generated: &[Generated]) -> Result<()> {
    let mut drifted = Vec::new();
    for file in generated {
        let on_disk = fs::read_to_string(root.join(&file.path)).ok();
        if on_disk.as_deref() != Some(file.contents.as_str()) {
            drifted.push(file.path.clone());
        }
    }
    drifted.extend(stale_schemas(root, generated)?);
    if drifted.is_empty() {
        println!(
            "codegen: {} generated files are up to date",
            generated.len()
        );
        return Ok(());
    }
    for path in &drifted {
        eprintln!("  out of date: {}", path.display());
    }
    bail!(
        "generated files drifted from stackly-protocol; run `cargo xtask codegen` and commit the result"
    )
}

/// Schema files on disk that no contract type generates any more.
fn stale_schemas(root: &Path, generated: &[Generated]) -> Result<Vec<PathBuf>> {
    let dir = root.join(SCHEMA_DIR);
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut stale = Vec::new();
    for entry in fs::read_dir(&dir).with_context(|| format!("cannot read {}", dir.display()))? {
        let name = entry?.file_name();
        let Some(name) = name.to_str() else { continue };
        let path = Path::new(SCHEMA_DIR).join(name);
        if name.ends_with(SCHEMA_SUFFIX) && !generated.iter().any(|file| file.path == path) {
            stale.push(path);
        }
    }
    Ok(stale)
}
