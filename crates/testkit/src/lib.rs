//! Fakes and fixtures for Stackly tests.
//!
//! Add this crate only as a dev dependency. It never depends on
//! `stackly-core`, so core unit tests see a single copy of every type.
//!
//! Integration tests take their own [`TestRoot`] and ports from
//! [`allocate_port`], so they never touch the real per user folders, never
//! share state, and can run in parallel.

mod port;
mod test_root;

use std::path::{Path, PathBuf};

use stackly_platform::{DirectoryResolver, PlatformError};

pub use port::allocate_port;
pub use test_root::TestRoot;

/// A [`DirectoryResolver`] that points at a test root instead of the real
/// per user folders.
#[derive(Debug, Clone)]
pub struct FakeDirectoryResolver {
    root: PathBuf,
}

impl FakeDirectoryResolver {
    /// Creates a resolver whose app support directory is `root`.
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Returns the test root this resolver points at.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }
}

impl DirectoryResolver for FakeDirectoryResolver {
    fn app_support_dir(&self) -> Result<PathBuf, PlatformError> {
        Ok(self.root.clone())
    }
}
