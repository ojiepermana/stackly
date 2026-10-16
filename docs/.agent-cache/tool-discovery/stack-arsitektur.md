# Tool discovery: stack & arsitektur

Date: 2026-10-16. Reuse until 2026-11-15, then subtract installed and declined before offering again.

## Installed (project, `.claude/skills/`, tracked in `skills-lock.json`)

- rust-best-practices (apollographql/skills), Rust idioms
- rust-async-patterns (wshobson/agents), tokio and async patterns
- swiftui-expert-skill (avdlee/swiftui-agent-skill), SwiftUI state, layout, macOS
- swift-concurrency (avdlee/swift-concurrency-agent-skill), Sendable, actors, Swift 6
- swift-testing-pro (twostraws/swift-testing-agent-skill), Swift Testing

## Declined

- rust-engineer (jeffallan/claude-skills): installed then removed. Flagged Critical by the Gen scan (content reviewed, looked benign), written for edition 2021, overlaps the two Rust skills above.
- github-actions (callstackincubator/agent-skills): installed then removed. React Native simulator builds only, not a fit.
- github-actions-templates (wshobson/agents): not picked.

## Skipped candidates (shown, not picked)

- rust-cli, rust-observability (full-stack-skills/rust-skills), serde-code-review (existential-birds/beagle), tokio-patterns (geoffjay/claude-plugins)
- swiftui-pro (twostraws), xcode-build-fixer (avdlee), macos-design-guidelines (ehmo)
- none credible found for: rusqlite, cargo-deny, swift-format, SMAppService, Keychain

## MCP servers (engineer chose to connect; engineer connects them)

- XcodeBuildMCP (maintainer to verify: cameroncooke or getsentry)
- GitHub MCP server (github/github-mcp-server)
- SQLite MCP (@modelcontextprotocol/server-sqlite, or a read only one such as StacklokLabs/sqlite-mcp); point it only at test roots
