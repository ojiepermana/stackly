# Stackly

Stackly installs, runs, and watches native developer tools (PostgreSQL, MySQL, NATS, ClickHouse, Valkey, Traefik) on one Mac: many versions and many instances, no Docker, no cloud account. One Rust daemon per user (`stacklyd`) owns the registry and every process. The Rust CLI (`stackly`) and the SwiftUI app only talk to it over IPC. There is no code yet; the next step is the feature 1 scaffold.

## Stack

- **Language / Runtime**: Rust 1.99.1, edition 2024, pinned in `rust-toolchain.toml` (core, daemon, CLI). Swift 6 language mode with full strict concurrency on Xcode 27 (macOS app). Target is macOS 27 on Apple Silicon only (`aarch64-apple-darwin`).
- **Framework**: tokio (multi thread runtime), SwiftUI, clap v4 derive.
- **Key dependencies**: serde_json over a private Unix socket (no TCP listener); typeshare and schemars, which generate Swift types and JSON Schema from `stackly-protocol`; rusqlite `bundled` with one writer thread; thiserror; tracing with JSON output; jiff (UTC, RFC 3339); uuid v7; serde_yaml_ng (never serde_yml).
- **Package manager**: cargo workspace with every third party version in `[workspace.dependencies]` (add a crate only when code uses it), local SwiftPM packages `StacklyClient` and `StacklyUI`, and `cargo xtask` as the task runner. Full decision: [spec 0001](docs/specs/0001-stack-arsitektur/index.md).

## Build approach

Journey (satu user journey dibangun lengkap, termasuk semua state, error, dan polish, sebelum journey berikutnya dimulai).

## Commands

Planned in spec 0001. None of them work until the scaffold lands.

```bash
cargo test -p <crate> [test_name_filter]
cargo xtask codegen          # rerun after any change in stackly-protocol
cargo xtask codegen --check  # fails when committed generated files drift
cargo xtask check-deps       # crate dependency direction and unsafe rules
cargo xtask build-app        # xcodebuild, scheme Stackly
swift test --package-path clients/swift
cargo check --target aarch64-pc-windows-msvc -p stackly-domain -p stackly-platform -p stackly-protocol -p stackly-core
```

## Specs

Stored in `docs/specs/`. Format: `docs/specs/NNNN-slug/index.md`, with the reasoning in `rationale.md`. Specs replace the ADR folder the PRD proposes. What to build, in which order, and each feature's status live in `docs/scope/` (`index.md` holds the legend). The suggested flow per feature is `/architect` when it is tagged needs a decision, then `/develop`, `/check verify`, and `/test`; features tagged `· GA` add `/check review` and `/document`; `/sync` after a change lands. A feature's next step is its first unticked box.

`prd.md` is input for writing scope and specs only. Never take requirements, decisions, or acceptance criteria from it while building. If a scope or spec is missing something, update it through `/scope` or `/architect`, or ask. A spec beats an Agent Skill when they conflict, and scope and spec beat the mockup on behavior.

## Rules

- Clean Architecture mapped onto crates. `stackly-domain` (entities, invariants, state machines) is the center and imports no Stackly crate, tokio, or libc. `stackly-core` holds thin use cases that call ports, such as the OS traits in `stackly-platform`. `stackly-platform-macos` and `stackly-storage` are adapters. `stacklyd`, `stackly`, and the SwiftUI app are presentation. Dependencies point inward only, and `cargo xtask check-deps` enforces the full list in spec 0001 ("Aturan arah dependency").
- Domain, platform, protocol, and core never touch macOS APIs and must pass `cargo check` for `aarch64-pc-windows-msvc`.
- Only the daemon writes the registry, starts processes, and creates IDs and paths. The CLI never runs provider binaries, and the GUI never links Rust.
- Boundaries carry DTOs from `stackly-protocol`, never domain types; `stacklyd` does the mapping. The wire is camelCase JSON behind a 4 byte big endian length (1 MiB control frames, 64 KiB log batches, checked before allocating). Enums with data use `tag = "type"`, `content = "content"`. Generated files are never edited by hand, and the fixtures in `protocols/fixtures/` are the shared contract.
- Errors use thiserror in libraries with stable categories in `stackly-protocol`, and CLI exit codes come from those categories. anyhow only in `xtask` and tests. The daemon and CLI check env vars, paths, and config once at startup and fail fast with a clear message.
- No `unwrap()`, Swift force unwrap, or `try!` outside tests; `expect("...")` only with a message that names the invariant. `unsafe` only in `stackly-platform-macos`, each block with a `// SAFETY:` comment.
- Every public item in library crates and Swift packages has a doc comment. Code, comments, doc comments, test names, error messages, and commits are in English; scope, spec, and review docs stay in Indonesian. Commits follow `type(scope): subject`.
- Tests come after each build (`/test`). Domain gets plain unit tests with no fakes. Core gets unit tests against `stackly-testkit` fakes, not mocked adapters. Adapters, daemon, and supervisor get integration tests in each crate's `tests/` against real systems, each with its own ephemeral test root, ports, and guards that reap child processes; never the real Application Support folder, never global state.
- Keep PRD IDs (`REQ`, `AC`, `NFR`, `OQ`, and the rest) stable in specs and tests. Test status starts at `NOT_RUN`, a fixture pass is not native runtime evidence, and missing hardware, binaries, or signing is recorded as `BLOCKED`.
- UI meets WCAG AA contrast, has VoiceOver labels, works fully from the keyboard, and follows Dynamic Type.

## Tooling

Recorded here, installed by `/develop coding standards & tooling` (feature 2).

- Format with rustfmt and swift-format (`.swift-format`). Clippy runs with default lints plus `clippy::unwrap_used`, and warnings fail (`-D warnings`). Library crates turn on `missing_docs`.
- Pre commit: a plain git hook, no Node or extra tools, running `cargo fmt --check`, clippy on the changed crates, and `swift-format lint`. Tests run in CI.
- CI on GitHub Actions as spec 0001 describes: Linux for fmt, clippy, tests, `cargo-deny`, and the portability check (clippy and tests there use `--workspace --exclude stackly-platform-macos`); `macos-26` for Rust jobs on darwin; one self hosted macOS 27 Mac for Swift and real runtime jobs. Builds never remap source paths.
- `typeshare-cli` is pinned in xtask. Install the matching one with `cargo install --locked typeshare-cli --version <v>`.
- Markdown follows `.markdownlint.jsonc`: no line length limit, only `<a>` as inline HTML, compact tables.

## Git

- integration: on
- branch prefix: feat/
- commit: per-milestone

## Agent skills

- [rust-best-practices](.claude/skills/rust-best-practices/): `apollographql/skills`, idiomatic Rust, ownership, error handling, tests
- [rust-async-patterns](.claude/skills/rust-async-patterns/): `wshobson/agents`, tokio, async traits, concurrency

Declined: rust-engineer, github-actions (callstackincubator), github-actions-templates, rust-cli, rust-observability, serde-code-review, tokio-patterns

MCP servers: GitHub MCP server (recommended), SQLite MCP pointed only at test roots (recommended)

## Context files

- [apps/macos/AGENTS.md](apps/macos/AGENTS.md): SwiftUI app and `StacklyUI`, Swift conventions and skills, mockup rules
- [clients/swift/AGENTS.md](clients/swift/AGENTS.md): `StacklyClient` package, generated contract types, fixture tests

_Drafted by /audit from the repo, worth a quick human pass. Edit freely: once a line stops matching this draft, later runs treat it as curated and will flag rather than overwrite it._
