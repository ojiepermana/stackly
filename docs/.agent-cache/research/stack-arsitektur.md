# Landscape check: stack & arsitektur

Checked 2026-10-16 by a researcher subagent (5 searches, 8 fetches). Reuse until 2026-11-15 unless asked for latest.

1. Rust stable 1.99.1 (releases.rs). 1.99.0 reportedly 2026-10-01 (third party only). Beta 1.100.0 dated 2026-11-12. Edition 2024 stable since 1.85 (memory, not confirmed this run).
2. typeshare latest crate 1.0.5, published 2026-01-02 (crates.io API). Maintenance not confirmed. Swift output with serde tag/content from search snippet only. Alternative: Specta Swift exporter (docs not opened). serde-generate targets binary formats, poor fit for JSON.
3. YAML: serde_yml unsound and unmaintained, RUSTSEC-2025-0068 (2025-09-11), all versions. Advisory names serde_norway and serde_yaml_ng as maintained forks. Popularity unverified. saphyr / serde-saphyr unverified.
4. Xcode 27 shipped 2026-09-14 with Swift 6.4 and macOS 27 SDK, needs macOS 26.6+ (third party only). GitHub hosted arm64 labels: macos-26 (macos-latest), macos-26-xlarge, macos-15, macos-15-xlarge; macos-14 deprecated. No macos-27 label; xcode-27 labels are preview.
5. jiff 0.2.38 (2026-10-06), not 1.0. Other crate versions not checked.

## Links (opened and confirmed)

- https://releases.rs/
- https://crates.io/api/v1/crates/typeshare
- https://crates.io/api/v1/crates/jiff
- https://api.osv.dev/v1/vulns/RUSTSEC-2025-0068
- https://github.com/actions/runner-images
