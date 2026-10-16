//! macOS adapters for the Stackly platform traits.
//!
//! This is the only crate where `unsafe` code may live. Every `unsafe` block
//! needs a `// SAFETY:` comment, and `cargo xtask check-deps` rejects this
//! opt out anywhere else.
#![allow(unsafe_code)]

use std::path::PathBuf;

use stackly_platform::{DirectoryResolver, PlatformError};

/// Resolves Stackly directories under the current user's `~/Library`.
#[derive(Debug, Default, Clone, Copy)]
pub struct MacosDirectoryResolver;

impl DirectoryResolver for MacosDirectoryResolver {
    // provisional: F4 replaces this with the final resolver.
    fn app_support_dir(&self) -> Result<PathBuf, PlatformError> {
        let home = std::env::home_dir()
            .filter(|path| path.is_absolute())
            .ok_or(PlatformError::HomeDirUnavailable)?;
        Ok(home.join("Library/Application Support/Stackly"))
    }
}
