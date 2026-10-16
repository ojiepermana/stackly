//! OS neutral platform ports for Stackly.
//!
//! Adapters such as `stackly-platform-macos` implement these traits, and the
//! use cases in `stackly-core` depend only on them. Nothing here calls an OS
//! API, so the crate builds for every target.

use std::path::PathBuf;

// provisional: F4 designs the full resolver (registry, runtimes, instances,
// logs, cache, retained, socket path) and how tests point it at a test root.
/// Resolves the per user directories Stackly owns.
pub trait DirectoryResolver {
    /// Returns the root of Stackly's per user data, for example
    /// `~/Library/Application Support/Stackly` on macOS.
    ///
    /// # Errors
    ///
    /// Returns [`PlatformError::HomeDirUnavailable`] when the current user's
    /// home directory cannot be determined.
    fn app_support_dir(&self) -> Result<PathBuf, PlatformError>;
}

/// Error returned by a platform adapter.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum PlatformError {
    /// The current user's home directory could not be determined.
    #[error("cannot determine the home directory of the current user")]
    HomeDirUnavailable,
}
