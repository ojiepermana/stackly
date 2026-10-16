use std::fmt;
use std::str::FromStr;

use uuid::Uuid;

/// Length of a hyphenated UUID, the only text form an ID accepts.
const HYPHENATED_LEN: usize = 36;

/// Stable identifier of an instance, always a UUID version 7.
///
/// Only the daemon creates new IDs. The text form is the lowercase hyphenated
/// UUID, and parsing accepts any letter case.
///
/// ```
/// use stackly_domain::InstanceId;
///
/// let id = InstanceId::generate();
/// let parsed: InstanceId = id.to_string().to_uppercase().parse()?;
/// assert_eq!(parsed, id);
/// # Ok::<(), stackly_domain::InvalidInstanceId>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InstanceId(Uuid);

impl InstanceId {
    /// Creates a new, time ordered ID from the current system time.
    #[must_use]
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    /// Wraps an existing UUID, for example one read back from the registry.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidInstanceId::WrongVersion`] when the UUID is not
    /// version 7.
    pub fn from_uuid(uuid: Uuid) -> Result<Self, InvalidInstanceId> {
        match uuid.get_version_num() {
            7 => Ok(Self(uuid)),
            other => Err(InvalidInstanceId::WrongVersion(other)),
        }
    }

    /// Returns the underlying UUID.
    #[must_use]
    pub const fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for InstanceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0.hyphenated(), f)
    }
}

impl FromStr for InstanceId {
    type Err = InvalidInstanceId;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != HYPHENATED_LEN {
            return Err(InvalidInstanceId::Malformed);
        }
        let uuid = Uuid::try_parse(s).map_err(|_| InvalidInstanceId::Malformed)?;
        Self::from_uuid(uuid)
    }
}

/// Error returned when text or a UUID is not a valid [`InstanceId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum InvalidInstanceId {
    /// The text is not a hyphenated UUID.
    #[error("instance ID is not a hyphenated UUID")]
    Malformed,
    /// The UUID is well formed but not version 7.
    #[error("instance ID must be a UUID version 7, found version {0}")]
    WrongVersion(usize),
}
