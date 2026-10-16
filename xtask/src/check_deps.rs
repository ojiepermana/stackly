//! `cargo xtask check-deps`: the dependency direction and `unsafe` rules
//! from spec 0001 ("Aturan arah dependency").
//!
//! Only direct dependencies are checked, each with its kind (normal, dev,
//! build). Transitive dependencies are left alone: portability is proven by
//! the cross target `cargo check`, not by a list of banned crates.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};
use cargo_metadata::{Dependency, DependencyKind, MetadataCommand, Package};

/// Crates that must build for every target, Windows included.
const PORTABLE: [&str; 4] = [
    "stackly-domain",
    "stackly-platform",
    "stackly-protocol",
    "stackly-core",
];
/// Crates that depend on no other Stackly crate.
const LEAVES: [&str; 2] = ["stackly-domain", "stackly-protocol"];
/// Adapters the portable crates never see.
const ADAPTERS: [&str; 2] = ["stackly-platform-macos", "stackly-storage"];
/// Crates whose source may opt out of the workspace `unsafe_code` lint.
/// Add `stackly-storage` only once it is proven to need `unsafe`.
const UNSAFE_ALLOWED: [&str; 1] = ["stackly-platform-macos"];
const MACOS_ADAPTER: &str = "stackly-platform-macos";
const MACOS_ONLY_TARGET: &str = r#"cfg(target_os = "macos")"#;
const TESTKIT: &str = "stackly-testkit";
const UNSAFE_LINT: &str = "unsafe_code";

pub fn run(root: &Path) -> Result<()> {
    let metadata = MetadataCommand::new()
        .manifest_path(root.join("Cargo.toml"))
        .no_deps()
        .exec()
        .context("cannot read `cargo metadata`")?;
    let packages = metadata.workspace_packages();

    let mut violations = check_workspace_lints(root)?;
    for package in &packages {
        for dependency in &package.dependencies {
            check_dependency(package.name.as_str(), dependency, &mut violations);
        }
        check_lints_inherited(package, &mut violations)?;
        if !UNSAFE_ALLOWED.contains(&package.name.as_str()) {
            check_unsafe_opt_out(package, &mut violations)?;
        }
    }

    if violations.is_empty() {
        println!(
            "check-deps: {} crates follow the dependency and unsafe rules",
            packages.len()
        );
        return Ok(());
    }
    for violation in &violations {
        eprintln!("  {violation}");
    }
    bail!(
        "{} dependency rule violation(s), see spec 0001",
        violations.len()
    )
}

fn check_dependency(package: &str, dependency: &Dependency, violations: &mut Vec<String>) {
    let name = dependency.name.as_str();
    let target = dependency.target.as_ref().map(ToString::to_string);
    let mut deny = |rule: &str| {
        violations.push(format!(
            "{package} has a {} dependency on {name}: {rule}",
            kind_label(dependency.kind)
        ));
    };

    if PORTABLE.contains(&package) {
        if ADAPTERS.contains(&name) {
            deny("portable crates never depend on an adapter");
        }
        if target.as_deref().is_some_and(is_apple_target) {
            deny("portable crates never take a dependency that only exists on Apple targets");
        }
    }
    if LEAVES.contains(&package) {
        if is_stackly_crate(name) {
            deny("this crate depends on no other Stackly crate");
        }
        if matches!(name, "tokio" | "libc") {
            deny("tokio and libc are never direct dependencies of this crate");
        }
    }
    if package == "stackly-core" && matches!(name, "stackly-protocol" | "stackly-storage") {
        deny("stacklyd maps domain types to DTOs, so core never sees the protocol or storage");
    }
    if package == "stackly-cli" && matches!(name, "stackly-core" | "stackly-storage") {
        deny("the CLI never runs provider binaries or writes the registry");
    }
    if package == TESTKIT && name == "stackly-core" {
        deny("testkit never depends on core, so core tests see one copy of each type");
    }
    if name == TESTKIT && dependency.kind != DependencyKind::Development {
        deny("stackly-testkit is only ever a dev dependency");
    }
    if name == MACOS_ADAPTER && target.as_deref() != Some(MACOS_ONLY_TARGET) {
        deny("declare it under [target.'cfg(target_os = \"macos\")'.dependencies]");
    }
}

/// The workspace lint table must keep `unsafe_code` denied.
fn check_workspace_lints(root: &Path) -> Result<Vec<String>> {
    let manifest = read_manifest(&root.join("Cargo.toml"))?;
    let level = manifest
        .get("workspace")
        .and_then(|workspace| workspace.get("lints"))
        .and_then(|lints| lints.get("rust"))
        .and_then(|rust| rust.get(UNSAFE_LINT))
        .and_then(toml::Value::as_str);
    Ok(match level {
        Some("deny") => Vec::new(),
        _ => vec![format!(
            "workspace Cargo.toml must set [workspace.lints.rust] {UNSAFE_LINT} = \"deny\""
        )],
    })
}

/// Every member inherits the workspace lints instead of writing its own.
fn check_lints_inherited(package: &Package, violations: &mut Vec<String>) -> Result<()> {
    let manifest = read_manifest(package.manifest_path.as_std_path())?;
    let inherits = manifest
        .get("lints")
        .and_then(|lints| lints.get("workspace"))
        .and_then(toml::Value::as_bool);
    if inherits != Some(true) {
        violations.push(format!(
            "{} must inherit the workspace lints with `[lints] workspace = true`",
            package.name
        ));
    }
    Ok(())
}

/// Only the crates in [`UNSAFE_ALLOWED`] may switch the `unsafe_code` lint
/// back on in source.
fn check_unsafe_opt_out(package: &Package, violations: &mut Vec<String>) -> Result<()> {
    let dir = package
        .manifest_path
        .parent()
        .context("a manifest path always has a parent directory")?;
    let mut files = Vec::new();
    collect_rust_files(dir.as_std_path(), &mut files)?;
    for file in files {
        let source =
            fs::read_to_string(&file).with_context(|| format!("cannot read {}", file.display()))?;
        if opts_out_of_unsafe_lint(&source) {
            violations.push(format!(
                "{} re-enables `{UNSAFE_LINT}` in {}; only {} may",
                package.name,
                file.display(),
                UNSAFE_ALLOWED.join(", ")
            ));
        }
    }
    Ok(())
}

fn collect_rust_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("cannot read {}", dir.display()))? {
        let path = entry?.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|name| name == "target") {
                continue;
            }
            collect_rust_files(&path, files)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
    Ok(())
}

/// True when an `allow(...)` or `expect(...)` attribute names the unsafe
/// lint, whatever the spacing or the other lints listed with it.
fn opts_out_of_unsafe_lint(source: &str) -> bool {
    let compact: String = source.chars().filter(|c| !c.is_whitespace()).collect();
    ["allow(", "expect("].iter().any(|attribute| {
        compact.match_indices(attribute).any(|(start, _)| {
            let args = &compact[start + attribute.len()..];
            let end = args.find(')').unwrap_or(args.len());
            args[..end].split(',').any(|lint| lint == UNSAFE_LINT)
        })
    })
}

fn read_manifest(path: &Path) -> Result<toml::Table> {
    let text =
        fs::read_to_string(path).with_context(|| format!("cannot read {}", path.display()))?;
    text.parse()
        .with_context(|| format!("cannot parse {}", path.display()))
}

fn is_stackly_crate(name: &str) -> bool {
    name.starts_with("stackly-") || name == "stacklyd"
}

fn is_apple_target(target: &str) -> bool {
    ["macos", "ios", "apple"]
        .iter()
        .any(|apple| target.contains(apple))
}

fn kind_label(kind: DependencyKind) -> &'static str {
    match kind {
        DependencyKind::Normal => "normal",
        DependencyKind::Development => "dev",
        DependencyKind::Build => "build",
        _ => "unknown kind of",
    }
}
