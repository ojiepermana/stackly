//! Pattern checks shared by the record rules and the guard.

/// The prefixes a PRD ID may carry.
const PRD_PREFIXES: [&str; 5] = ["AC", "REQ", "NFR", "VAL", "BR"];

/// One or more rule violations found while validating a value.
///
/// The message lists every violation, so the wrapper can report all of them
/// at once instead of one per run.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{}", .violations.join("; "))]
#[non_exhaustive]
pub struct RuleViolations {
    /// Each violated rule, as a lowercase sentence without a trailing period.
    pub violations: Vec<String>,
}

/// Collects violations, then turns them into a result.
#[derive(Debug, Default)]
pub(crate) struct Violations(Vec<String>);

impl Violations {
    pub(crate) fn check(&mut self, ok: bool, rule: impl FnOnce() -> String) {
        if !ok {
            self.0.push(rule());
        }
    }

    pub(crate) fn into_result(self) -> Result<(), RuleViolations> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(RuleViolations { violations: self.0 })
        }
    }
}

/// True when `id` matches `^(AC|REQ|NFR|VAL|BR)-[0-9]{3}$`, for example
/// `AC-011` or `NFR-005`.
#[must_use]
pub fn is_prd_id(id: &str) -> bool {
    let Some((prefix, number)) = id.split_once('-') else {
        return false;
    };
    PRD_PREFIXES.contains(&prefix)
        && number.len() == 3
        && number.bytes().all(|b| b.is_ascii_digit())
}

/// True when `version` is exact, matching
/// `^[0-9]+(\.[0-9]+)+([+-][0-9A-Za-z.]+)?$`: `17.4` and `8.4.6+build.1`
/// pass, selectors such as `17`, `@17`, or `^17` do not.
#[must_use]
pub fn is_exact_version(version: &str) -> bool {
    let (core, suffix) = match version.find(['+', '-']) {
        Some(at) => (&version[..at], Some(&version[at + 1..])),
        None => (version, None),
    };
    let mut parts = 0;
    for part in core.split('.') {
        if part.is_empty() || !part.bytes().all(|b| b.is_ascii_digit()) {
            return false;
        }
        parts += 1;
    }
    let suffix_ok = suffix.is_none_or(|suffix| {
        !suffix.is_empty()
            && suffix
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'.')
    });
    parts >= 2 && suffix_ok
}

/// True when `digest` is a SHA-256 written as 64 lowercase hex characters
/// with no prefix, the same form as `sha256` in the runtime inventory.
#[must_use]
pub fn is_sha256_hex(digest: &str) -> bool {
    digest.len() == 64
        && digest
            .bytes()
            .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// Checks a `covers` list: at least one ID, every ID a PRD ID, no duplicates.
pub(crate) fn check_covers<S: AsRef<str>>(covers: &[S], violations: &mut Violations) {
    violations.check(!covers.is_empty(), || {
        "covers must list at least one PRD ID".to_owned()
    });
    for (index, id) in covers.iter().enumerate() {
        let id = id.as_ref();
        violations.check(is_prd_id(id), || {
            format!(
                "covers entry `{id}` is not a PRD ID (AC, REQ, NFR, VAL, or BR plus three digits)"
            )
        });
        violations.check(
            !covers[..index].iter().any(|seen| seen.as_ref() == id),
            || format!("covers lists `{id}` twice"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prd_ids_need_a_known_prefix_and_three_digits() {
        assert!(is_prd_id("AC-011"));
        assert!(is_prd_id("REQ-051"));
        assert!(is_prd_id("BR-001"));
        assert!(!is_prd_id("AC-11"));
        assert!(!is_prd_id("AC-0111"));
        assert!(!is_prd_id("OQ-011"));
        assert!(!is_prd_id("ac-011"));
        assert!(!is_prd_id("AC011"));
    }

    #[test]
    fn exact_versions_reject_selectors() {
        assert!(is_exact_version("17.4"));
        assert!(is_exact_version("2.15.0"));
        assert!(is_exact_version("25.8.1.5101"));
        assert!(is_exact_version("8.4.6+build.1"));
        assert!(is_exact_version("9.1.2-rc1"));
        assert!(!is_exact_version("17"));
        assert!(!is_exact_version("@17"));
        assert!(!is_exact_version("^17.0"));
        assert!(!is_exact_version("17."));
        assert!(!is_exact_version("17.4-"));
        assert!(!is_exact_version("17.4+a_b"));
    }

    #[test]
    fn digests_are_64_lowercase_hex_characters() {
        let digest = "a".repeat(64);
        assert!(is_sha256_hex(&digest));
        assert!(!is_sha256_hex(&"A".repeat(64)));
        assert!(!is_sha256_hex(&format!("sha256:{}", "a".repeat(57))));
        assert!(!is_sha256_hex(&"a".repeat(63)));
    }

    #[test]
    fn covers_reports_empty_unknown_and_duplicate_ids() {
        let mut violations = Violations::default();
        check_covers(&["AC-011", "OQ-001", "AC-011"], &mut violations);
        let err = violations.into_result().unwrap_err();
        assert_eq!(err.violations.len(), 2);

        let mut violations = Violations::default();
        check_covers::<&str>(&[], &mut violations);
        assert!(violations.into_result().is_err());
    }
}
