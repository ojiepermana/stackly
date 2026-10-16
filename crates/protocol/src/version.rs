use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Version of the IPC protocol a peer speaks.
///
/// `major` changes when the wire breaks, `minor` when it only gains optional
/// additions.
#[typeshare]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolVersion {
    /// Bumped for every breaking change to the wire.
    pub major: u32,
    /// Bumped for every compatible addition.
    pub minor: u32,
}

impl ProtocolVersion {
    /// The protocol version this build speaks.
    pub const CURRENT: Self = Self { major: 0, minor: 1 };
}
