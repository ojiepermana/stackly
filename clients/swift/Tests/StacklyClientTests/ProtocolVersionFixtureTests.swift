import Foundation
import StacklyClient
import Testing

/// Checks the generated `ProtocolVersion` against the fixture the Rust tests
/// also read, comparing decoded values rather than raw JSON text.
struct ProtocolVersionFixtureTests {
    @Test func fixtureDecodesToCurrentVersion() throws {
        let decoded = try JSONDecoder().decode(
            ProtocolVersion.self, from: Fixture.data("protocol-version.json"))

        #expect(decoded == ProtocolVersion(major: 0, minor: 1))
    }

    @Test func encodedValueDecodesLikeTheFixture() throws {
        let fixture = try JSONDecoder().decode(
            ProtocolVersion.self, from: Fixture.data("protocol-version.json"))

        let encoded = try JSONEncoder().encode(ProtocolVersion(major: 0, minor: 1))
        let roundTripped = try JSONDecoder().decode(ProtocolVersion.self, from: encoded)

        #expect(roundTripped == fixture)
    }
}
