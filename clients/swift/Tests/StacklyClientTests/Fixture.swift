import Foundation

/// Shared JSON fixtures in `protocols/fixtures/` at the repository root.
enum Fixture {
    /// The repository root, found from this file's path. Builds must never
    /// remap source paths (`-file-prefix-map`), or this breaks.
    private static let repositoryRoot = URL(filePath: #filePath)
        .deletingLastPathComponent()  // Fixture.swift
        .deletingLastPathComponent()  // StacklyClientTests
        .deletingLastPathComponent()  // Tests
        .deletingLastPathComponent()  // swift
        .deletingLastPathComponent()  // clients

    /// Reads one fixture file by name, for example `protocol-version.json`.
    static func data(_ name: String) throws -> Data {
        try Data(contentsOf: repositoryRoot.appending(path: "protocols/fixtures/\(name)"))
    }
}
