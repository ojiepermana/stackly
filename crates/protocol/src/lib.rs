//! The Stackly IPC contract: DTOs for requests, responses, and events.
//!
//! These types are the single source for the generated Swift client and for
//! the JSON Schemas in `protocols/schemas/`. Run `cargo xtask codegen` after
//! any change, and keep the shared fixtures in `protocols/fixtures/` in step.
//! The crate depends on no other Stackly crate: `stacklyd` maps domain types
//! to these DTOs.

mod version;

pub use version::ProtocolVersion;

/// Returns the JSON Schema of every contract type, keyed by file stem.
///
/// `cargo xtask codegen` writes each one to
/// `protocols/schemas/<stem>.schema.json`.
#[must_use]
pub fn json_schemas() -> Vec<(&'static str, schemars::Schema)> {
    vec![("protocol-version", schemars::schema_for!(ProtocolVersion))]
}
