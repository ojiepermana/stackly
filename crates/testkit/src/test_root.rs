use std::io;
use std::path::Path;

use crate::FakeDirectoryResolver;

/// Prefix of every test root directory, so a leftover one is easy to spot.
const PREFIX: &str = "stackly-test-";

/// A fresh, empty directory that belongs to one test and is deleted when the
/// value is dropped.
///
/// Use it instead of `~/Library/Application Support/Stackly`. Cleanup removes
/// only this directory and what the test created inside it: a symlink inside
/// is removed, never followed. The path is canonical (on macOS the temporary
/// directory sits behind the `/var` to `/private/var` symlink), so checks that
/// a path lies under the root compare like with like.
///
/// ```
/// use stackly_testkit::TestRoot;
///
/// let root = TestRoot::new()?;
/// std::fs::write(root.path().join("registry.db"), b"")?;
/// let path = root.path().to_path_buf();
/// drop(root);
/// assert!(!path.exists());
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Debug)]
pub struct TestRoot {
    dir: tempfile::TempDir,
}

impl TestRoot {
    /// Creates a new test root under the system temporary directory.
    ///
    /// # Errors
    ///
    /// Returns the I/O error when the temporary directory cannot be resolved
    /// or the test root cannot be created in it.
    pub fn new() -> io::Result<Self> {
        let base = std::env::temp_dir().canonicalize()?;
        let dir = tempfile::Builder::new().prefix(PREFIX).tempdir_in(base)?;
        Ok(Self { dir })
    }

    /// Returns the canonical path of this test root.
    #[must_use]
    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    /// Returns a directory resolver whose app support directory is this
    /// test root.
    #[must_use]
    pub fn resolver(&self) -> FakeDirectoryResolver {
        FakeDirectoryResolver::new(self.path())
    }
}

#[cfg(test)]
mod tests {
    use stackly_platform::DirectoryResolver;

    use super::*;

    #[test]
    fn each_test_root_is_a_new_empty_directory() {
        let first = TestRoot::new().unwrap();
        let second = TestRoot::new().unwrap();

        assert_ne!(first.path(), second.path());
        assert_eq!(std::fs::read_dir(first.path()).unwrap().count(), 0);
    }

    #[test]
    fn test_root_path_is_canonical() {
        let root = TestRoot::new().unwrap();

        assert_eq!(root.path().canonicalize().unwrap(), root.path());
    }

    #[test]
    fn resolver_points_at_the_test_root() {
        let root = TestRoot::new().unwrap();

        assert_eq!(root.resolver().app_support_dir().unwrap(), root.path());
    }

    #[cfg(unix)]
    #[test]
    fn dropping_the_root_removes_a_symlink_but_not_its_target() {
        let outside = TestRoot::new().unwrap();
        let kept = outside.path().join("kept.txt");
        std::fs::write(&kept, b"outside the test root").unwrap();

        let root = TestRoot::new().unwrap();
        let link = root.path().join("link");
        std::os::unix::fs::symlink(outside.path(), &link).unwrap();
        let root_path = root.path().to_path_buf();
        drop(root);

        assert!(!root_path.exists());
        assert!(kept.exists());
    }
}
