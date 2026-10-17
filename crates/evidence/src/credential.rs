//! Fixture credentials: fake secrets with a prefix the wrapper can scan for.

use std::fmt;

use uuid::Uuid;

/// The prefix of every fixture credential. The wrapper scans run output for
/// it, and a match fails the run.
pub const FIXTURE_CREDENTIAL_PREFIX: &str = "stackly-fixture-credential-";

/// A fake secret for tests, different on every call, never a real one.
///
/// `Debug` prints `FixtureCredential(****)` so an assertion message never
/// leaks it. Pass [`FixtureCredential::value`] to the code under test, and
/// never write the value as a literal in a test.
///
/// ```
/// use stackly_evidence::FixtureCredential;
///
/// let password = FixtureCredential::new();
/// let log = "connected as stackly";
/// password.assert_absent(log, "the connection log");
/// assert_eq!(format!("{password:?}"), "FixtureCredential(****)");
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct FixtureCredential {
    value: String,
}

impl FixtureCredential {
    /// Creates `stackly-fixture-credential-<32 lowercase hex>`, with the
    /// random part taken from a new UUID v7.
    #[must_use]
    pub fn new() -> Self {
        Self {
            value: format!("{FIXTURE_CREDENTIAL_PREFIX}{}", Uuid::now_v7().simple()),
        }
    }

    /// Returns the secret, to hand to the code under test.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Panics with `fixture credential found in <what>` when `haystack`
    /// holds any fixture credential, without printing the value.
    ///
    /// # Panics
    ///
    /// Panics when [`FIXTURE_CREDENTIAL_PREFIX`] appears in `haystack`.
    pub fn assert_absent(&self, haystack: &str, what: &str) {
        assert!(
            !haystack.contains(FIXTURE_CREDENTIAL_PREFIX),
            "fixture credential found in {what}"
        );
    }
}

impl Default for FixtureCredential {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for FixtureCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixtureCredential(****)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_credential_is_the_prefix_plus_32_lowercase_hex() {
        let first = FixtureCredential::new();
        let second = FixtureCredential::new();
        let random = first
            .value()
            .strip_prefix(FIXTURE_CREDENTIAL_PREFIX)
            .unwrap();

        assert_eq!(random.len(), 32);
        assert!(
            random
                .bytes()
                .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
        );
        assert_ne!(first, second);
    }

    #[test]
    fn debug_never_prints_the_value() {
        let credential = FixtureCredential::new();
        assert!(!format!("{credential:?}").contains(credential.value()));
    }

    #[test]
    #[should_panic(expected = "fixture credential found in the daemon log")]
    fn assert_absent_panics_without_printing_the_value() {
        let credential = FixtureCredential::new();
        let log = format!("password={}", credential.value());
        credential.assert_absent(&log, "the daemon log");
    }
}
