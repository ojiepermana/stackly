/// Facts about the running daemon build.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct DaemonInfo {
    /// The Stackly version, shared by every crate in the workspace.
    pub version: &'static str,
}

/// Returns facts about this daemon build.
#[must_use]
pub const fn daemon_info() -> DaemonInfo {
    DaemonInfo {
        version: env!("CARGO_PKG_VERSION"),
    }
}
