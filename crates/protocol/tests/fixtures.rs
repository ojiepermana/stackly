//! Checks the shared JSON fixtures in `protocols/fixtures/` against the Rust
//! contract types. The Swift tests in `clients/swift` read the same files.

use std::path::PathBuf;

use serde_json::Value;
use stackly_protocol::ProtocolVersion;

fn read_fixture(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../protocols/fixtures")
        .join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("cannot read fixture {}: {err}", path.display()))
}

#[test]
fn protocol_version_fixture_decodes_to_current_version() {
    let decoded: ProtocolVersion = serde_json::from_str(&read_fixture("protocol-version.json"))
        .expect("the fixture is valid ProtocolVersion JSON");

    assert_eq!(decoded, ProtocolVersion::CURRENT);
}

#[test]
fn protocol_version_encodes_to_fixture_value() {
    let fixture: Value = serde_json::from_str(&read_fixture("protocol-version.json"))
        .expect("the fixture is valid JSON");

    let encoded =
        serde_json::to_value(ProtocolVersion::CURRENT).expect("ProtocolVersion always serializes");

    assert_eq!(encoded, fixture);
}
