# Verify: Stack & arsitektur · spec 0001 · updated 2026-10-16

_Langkah diturunkan dari Done when fitur 1 dan keputusan di spec 0001. Spec ini tidak punya ID `AC-N` sendiri, jadi tiap langkah menunjuk ID Trace dari scope (`REQ-001`, `REQ-050`, `NFR-010`, `OQ-005`) atau bagian spec yang dibuktikannya. `/check verify` menjalankan langkah ini; `/test` mengunci yang tahan lama. Jalankan dari root repo di Mac Apple Silicon dengan macOS 27 dan Xcode 27._

## UI / manual

- [x] Buka `target/xcode/Build/Products/Debug/Stackly.app` setelah `cargo xtask build-app` → satu jendela muncul dengan placeholder `RootView` bertuliskan "Stackly", lalu app bisa ditutup normal → Proyek Xcode dan package Swift
- [ ] Buka `apps/macos/Stackly.xcodeproj` di Xcode 27 → scheme `Stackly` ada, `StacklyUI` tampil sebagai package lokal, dan pratinjau `#Preview` di `RootView.swift` tampil → Proyek Xcode dan package Swift

## Commands

- [x] `rustc --version` di root repo → `rustc 1.99.1`, dipasang otomatis dari `rust-toolchain.toml` → Toolchain Rust
- [x] `cargo build --workspace --locked` → build bersih untuk 9 anggota workspace → REQ-050
- [x] `cargo clippy --workspace --all-targets --locked -- -D warnings` → tanpa warning → Konfigurasi workspace
- [x] `cargo test --workspace --locked` → 2 test fixture `ProtocolVersion` dan 1 doc test `InstanceId` lulus → OQ-005
- [x] `./target/debug/stacklyd --version` dan `./target/debug/stackly --version` → keduanya mencetak `0.1.0` dari `[workspace.package]` → Konfigurasi workspace
- [x] `./target/debug/stacklyd` → satu baris log JSON di stderr dengan `version`, `protocol_major`, `protocol_minor`, dan `app_support_dir` berakhiran `Library/Application Support/Stackly`, exit 0 → REQ-001
- [x] `HOME=relative/home ./target/debug/stacklyd` → log error "cannot resolve the Stackly data directory", exit 1 (fail fast saat startup). Catatan: `HOME` kosong atau tidak diset tetap berhasil, karena Rust membaca home dari database user → REQ-001
- [x] `cargo xtask codegen --check` → "2 generated files are up to date" → OQ-005
- [x] Ubah satu doc comment di `crates/protocol/src/version.rs`, jalankan `cargo xtask codegen --check` → gagal dan menyebut `StacklyProtocol.swift` serta `protocol-version.schema.json`; kembalikan perubahannya → OQ-005
- [x] `cargo xtask check-deps` → "9 crates follow the dependency and unsafe rules" → Aturan arah dependency
- [x] Tambahkan sementara `stackly-protocol.workspace = true` ke `[dependencies]` di `crates/core/Cargo.toml`, jalankan `cargo xtask check-deps` → gagal dengan pesan core never sees the protocol; kembalikan perubahannya → Aturan arah dependency
- [x] `cargo check --locked --target aarch64-pc-windows-msvc -p stackly-domain -p stackly-platform -p stackly-protocol -p stackly-core` → build bersih → NFR-010
- [x] `cargo clippy --locked --target aarch64-pc-windows-msvc -p stacklyd -p stackly-cli -- -D warnings` → bersih, membuktikan cabang non macOS di daemon bisa dikompilasi → NFR-010
- [x] `swift test --package-path clients/swift` → 2 test Swift Testing lulus dengan fixture yang sama → OQ-005
- [x] `cargo xtask build-app` lalu `lipo -archs target/xcode/Build/Products/Debug/Stackly.app/Contents/MacOS/Stackly` → hanya `arm64` → Proyek Xcode dan package Swift
- [x] `codesign -dv target/xcode/Build/Products/Debug/Stackly.app` → `Signature=adhoc`, `TeamIdentifier=not set`; `codesign -d --entitlements - <app>` tanpa entitlement sandbox → Proyek Xcode dan package Swift

## Acceptance-criteria coverage

- REQ-001 (hanya daemon yang memegang registry dan proses): langkah `stacklyd` start dan fail fast, ditambah aturan CLI di `check-deps`
- REQ-050 (workspace Rust dan proyek SwiftUI bisa build): langkah build Rust, `build-app`, dan buka app
- NFR-010 (core siap untuk Windows): kedua langkah cek lintas target `aarch64-pc-windows-msvc`
- OQ-005 (encoding IPC dan tooling kontrak): fixture Rust dan Swift, `codegen --check`, dan probe drift
- PRD bagian 15 (arah dependency): `check-deps` dan probe pelanggarannya
