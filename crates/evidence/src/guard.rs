//! The Rust test guard: it writes a `NOT_RUN` record when a test starts and
//! the final result when the test ends.

use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::fs;
use std::io::{self, Write};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use jiff::Timestamp;
use uuid::Uuid;

use crate::rules::{Violations, check_covers, is_exact_version, is_sha256_hex};
use crate::{
    EvidenceRecord, Kind, Language, Method, PANIC_ACTUAL, RECORD_SCHEMA, RUST_DEFAULT_ACTUAL,
    Source, Status, Subject, TestIdentity, UNFINISHED_ACTUAL, write_json_atomic,
};

/// The absolute path of the run folder. Only the wrapper sets it.
pub const EVIDENCE_DIR_ENV: &str = "STACKLY_EVIDENCE_DIR";
/// `fixture` or `native`, the kind of the run. Only the wrapper sets it.
pub const EVIDENCE_KIND_ENV: &str = "STACKLY_EVIDENCE_KIND";

thread_local! {
    /// Whether this thread already holds an [`Evidence::start`] guard.
    static GUARD_HELD: Cell<bool> = const { Cell::new(false) };
}

/// Where a test was written. Build it with [`test_site!`](crate::test_site).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TestSite {
    /// `CARGO_PKG_NAME` of the crate under test.
    pub package: &'static str,
    /// `CARGO_CRATE_NAME` of the test target.
    pub target: &'static str,
    /// `file!()` of the guard.
    pub file: &'static str,
    /// `line!()` of the guard.
    pub line: u32,
    /// `module_path!()` of the guard, the fallback test name.
    pub module_path: &'static str,
}

/// Captures the [`TestSite`] of the line it is written on.
#[macro_export]
macro_rules! test_site {
    () => {
        $crate::TestSite {
            package: ::core::env!("CARGO_PKG_NAME"),
            target: ::core::env!("CARGO_CRATE_NAME"),
            file: ::core::file!(),
            line: ::core::line!(),
            module_path: ::core::module_path!(),
        }
    };
}

/// Creates the [`Evidence`] guard of a test that proves PRD IDs.
///
/// Write it as the first statement of the test, so it is dropped last and
/// sees a panic from any other value's drop. End the test with
/// [`Evidence::pass`], [`Evidence::pass_with`], or [`Evidence::blocked`].
///
/// ```no_run
/// use stackly_evidence::{Kind, evidence};
///
/// let ev = evidence!(
///     covers = ["AC-011"],
///     kind = Kind::Fixture,
///     expected = "a normal stop ends the dummy server with SIGTERM only",
/// );
/// // arrange, act, assert as usual
/// ev.pass();
/// ```
#[macro_export]
macro_rules! evidence {
    (covers = [$($id:expr),+ $(,)?], kind = $kind:expr, expected = $expected:expr $(,)?) => {
        $crate::Evidence::start($crate::test_site!(), &[$($id),+], $kind, $expected)
    };
}

/// The result a test recorded before its guard is dropped.
#[derive(Debug)]
enum Outcome {
    Open,
    Pass(Option<String>),
    Blocked { reason: String, owner: String },
}

/// The folder [`Evidence::artifact_dir`] hands out.
#[derive(Debug)]
enum Artifacts {
    InRun(PathBuf),
    Temporary(tempfile::TempDir),
}

impl Artifacts {
    fn path(&self) -> &Path {
        match self {
            Self::InRun(path) => path,
            Self::Temporary(dir) => dir.path(),
        }
    }
}

/// The evidence guard of one test.
///
/// Created by [`evidence!`](crate::evidence), it writes a `NOT_RUN` record
/// at once. `pass`, `pass_with`, and `blocked` only note the intended
/// result; `Drop` writes the final record: `PASS` or `BLOCKED` as noted when
/// the thread is not panicking, `FAIL` otherwise. Without
/// `STACKLY_EVIDENCE_DIR` it writes nothing but still checks its arguments.
///
/// The guard is neither `Send` nor `Sync`, so `Drop` always runs on the
/// test thread, where [`std::thread::panicking`] means something.
#[derive(Debug)]
pub struct Evidence {
    run_dir: Option<PathBuf>,
    record: RefCell<EvidenceRecord>,
    outcome: RefCell<Outcome>,
    artifacts: RefCell<Option<Artifacts>>,
    holds_thread_slot: bool,
    _not_send: PhantomData<*const ()>,
}

impl Evidence {
    /// Starts the guard of the current test. Use [`evidence!`](crate::evidence)
    /// instead of calling this directly.
    ///
    /// # Panics
    ///
    /// Panics when an ID in `covers` is not a PRD ID, `expected` is empty,
    /// the evidence environment is set but invalid, `kind` differs from the
    /// kind of the run, this thread already holds a guard, or the `NOT_RUN`
    /// record cannot be written.
    #[must_use = "bind the guard with `let ev = evidence!(...)`; dropping it at once records FAIL"]
    pub fn start(site: TestSite, covers: &[&str], kind: Kind, expected: &str) -> Self {
        let run = run_target(
            std::env::var_os(EVIDENCE_DIR_ENV),
            std::env::var_os(EVIDENCE_KIND_ENV),
        )
        .unwrap_or_else(|message| panic!("{message}"));
        if let Some((_, run_kind)) = &run {
            check_kind(kind, *run_kind).unwrap_or_else(|message| panic!("{message}"));
        }
        assert!(
            !GUARD_HELD.with(Cell::get),
            "only one evidence guard per test: this thread already holds one"
        );
        let guard = Self::create(run.map(|(dir, _)| dir), site, covers, kind, expected, true);
        GUARD_HELD.with(|held| held.set(true));
        guard
    }

    /// Starts a guard that writes to `run_dir` instead of the folder named
    /// by the environment, and does not take the one guard per thread
    /// slot. It exists to test the guard itself.
    ///
    /// # Panics
    ///
    /// Panics when an ID in `covers` is not a PRD ID, `expected` is empty,
    /// or the `NOT_RUN` record cannot be written.
    #[must_use = "bind the guard to a variable; dropping it at once records FAIL"]
    pub fn with_dir(
        run_dir: impl Into<PathBuf>,
        site: TestSite,
        covers: &[&str],
        kind: Kind,
        expected: &str,
    ) -> Self {
        Self::create(Some(run_dir.into()), site, covers, kind, expected, false)
    }

    fn create(
        run_dir: Option<PathBuf>,
        site: TestSite,
        covers: &[&str],
        kind: Kind,
        expected: &str,
        holds_thread_slot: bool,
    ) -> Self {
        let mut violations = Violations::default();
        check_covers(covers, &mut violations);
        violations.check(!expected.trim().is_empty(), || {
            "expected must not be empty".to_owned()
        });
        if let Err(err) = violations.into_result() {
            panic!("evidence!: {err}");
        }

        let record = EvidenceRecord {
            schema: RECORD_SCHEMA.to_owned(),
            id: Uuid::now_v7(),
            covers: covers.iter().map(|id| (*id).to_owned()).collect(),
            kind,
            method: Method::Automated,
            test: TestIdentity {
                language: Language::Rust,
                package: Some(site.package.to_owned()),
                target: Some(site.target.to_owned()),
                name: derive_name(std::thread::current().name(), &site),
                file: Some(site.file.to_owned()),
                line: Some(site.line),
            },
            status: Status::NotRun,
            expected: expected.to_owned(),
            actual: None,
            reason: None,
            owner: None,
            subjects: Vec::new(),
            steps: None,
            reviewer: None,
            started_at: Timestamp::now(),
            finished_at: None,
            evidence_path: "output.log".to_owned(),
            context: None,
        };
        if let Some(dir) = &run_dir {
            write_record(dir, &record).unwrap_or_else(|err| {
                panic!(
                    "evidence!: cannot write the NOT_RUN record to {}: {err}",
                    dir.display()
                )
            });
        }
        Self {
            run_dir,
            record: RefCell::new(record),
            outcome: RefCell::new(Outcome::Open),
            artifacts: RefCell::new(None),
            holds_thread_slot,
            _not_send: PhantomData,
        }
    }

    /// The record ID, also the name of its raw record file.
    #[must_use]
    pub fn id(&self) -> Uuid {
        self.record.borrow().id
    }

    /// Notes that the test passed, with `actual` set to
    /// `all assertions passed`. A native test uses [`Evidence::pass_with`].
    ///
    /// # Panics
    ///
    /// Panics when the result was already noted.
    pub fn pass(&self) {
        self.settle(Outcome::Pass(None));
    }

    /// Notes that the test passed, with the value it observed.
    ///
    /// # Panics
    ///
    /// Panics when `actual` is empty or the result was already noted.
    pub fn pass_with(&self, actual: &str) {
        assert!(
            !actual.trim().is_empty(),
            "pass_with needs the observed value"
        );
        self.settle(Outcome::Pass(Some(actual.to_owned())));
    }

    /// Notes that a prerequisite (binary, hardware, signing) is missing.
    ///
    /// # Panics
    ///
    /// Panics when `reason` or `owner` is empty, or the result was already
    /// noted.
    pub fn blocked(&self, reason: &str, owner: &str) {
        assert!(
            !reason.trim().is_empty() && !owner.trim().is_empty(),
            "blocked needs a reason and the owner who can unblock it"
        );
        self.settle(Outcome::Blocked {
            reason: reason.to_owned(),
            owner: owner.to_owned(),
        });
    }

    fn settle(&self, outcome: Outcome) {
        let mut current = self.outcome.borrow_mut();
        assert!(
            matches!(*current, Outcome::Open),
            "the result of this test is already noted; call pass, pass_with, or blocked once"
        );
        *current = outcome;
    }

    /// Adds a provider or runtime the test ran against. Call it as soon as
    /// the runtime is found, with the version read from the running binary,
    /// never a literal.
    ///
    /// # Panics
    ///
    /// Panics when `version` is not exact, `digest` is not 64 lowercase hex
    /// characters, or a native test passes [`Source::Fixture`].
    pub fn subject(&self, provider: &str, version: &str, digest: Option<&str>, source: Source) {
        assert!(
            is_exact_version(version),
            "subject {provider}: version `{version}` is not exact"
        );
        assert!(
            digest.is_none_or(is_sha256_hex),
            "subject {provider}: the digest must be 64 lowercase hex characters"
        );
        let mut record = self.record.borrow_mut();
        assert!(
            !(record.kind == Kind::Native && source == Source::Fixture),
            "subject {provider}: a native test never uses Source::Fixture"
        );
        record.subjects.push(Subject {
            provider: provider.to_owned(),
            version: version.to_owned(),
            digest: digest.map(str::to_owned),
            source,
        });
    }

    /// Returns a folder for logs and files the test wants to keep.
    ///
    /// In a run it is `artifacts/<id>/` in the run folder, and the record's
    /// `evidencePath` points there. Without a run it is a temporary folder
    /// removed when the guard is dropped, so the test needs no branch.
    ///
    /// # Panics
    ///
    /// Panics when the folder cannot be created.
    #[must_use]
    pub fn artifact_dir(&self) -> PathBuf {
        let mut artifacts = self.artifacts.borrow_mut();
        if let Some(existing) = artifacts.as_ref() {
            return existing.path().to_path_buf();
        }
        let created = match &self.run_dir {
            Some(dir) => {
                let mut record = self.record.borrow_mut();
                let path = dir.join("artifacts").join(record.id.to_string());
                fs::create_dir_all(&path).unwrap_or_else(|err| {
                    panic!(
                        "cannot create the artifact folder {}: {err}",
                        path.display()
                    )
                });
                record.evidence_path = format!("artifacts/{}/", record.id);
                Artifacts::InRun(path)
            }
            None => Artifacts::Temporary(
                tempfile::Builder::new()
                    .prefix("stackly-evidence-")
                    .tempdir()
                    .unwrap_or_else(|err| {
                        panic!("cannot create a temporary artifact folder: {err}")
                    }),
            ),
        };
        let path = created.path().to_path_buf();
        *artifacts = Some(created);
        path
    }
}

impl Drop for Evidence {
    fn drop(&mut self) {
        if self.holds_thread_slot {
            // Ignored: the slot is gone anyway when the thread is ending.
            let _ = GUARD_HELD.try_with(|held| held.set(false));
        }
        let Some(dir) = self.run_dir.clone() else {
            return;
        };
        let outcome = std::mem::replace(self.outcome.get_mut(), Outcome::Open);
        let record = self.record.get_mut();
        finish(record, outcome, std::thread::panicking(), Timestamp::now());
        if let Err(err) = write_record(&dir, record) {
            // Drop never panics: report the failure and leave NOT_RUN on disk.
            let _ = writeln!(
                io::stderr(),
                "evidence: cannot write the final record {}: {err}; it stays NOT_RUN",
                record.id
            );
        }
    }
}

/// Turns the noted outcome into the final status.
fn finish(record: &mut EvidenceRecord, outcome: Outcome, panicking: bool, now: Timestamp) {
    let (status, actual, reason, owner) = match (panicking, outcome) {
        (true, _) => (Status::Fail, Some(PANIC_ACTUAL.to_owned()), None, None),
        (false, Outcome::Pass(actual)) => (
            Status::Pass,
            Some(actual.unwrap_or_else(|| RUST_DEFAULT_ACTUAL.to_owned())),
            None,
            None,
        ),
        (false, Outcome::Blocked { reason, owner }) => {
            (Status::Blocked, None, Some(reason), Some(owner))
        }
        (false, Outcome::Open) => (Status::Fail, Some(UNFINISHED_ACTUAL.to_owned()), None, None),
    };
    record.status = status;
    record.actual = actual;
    record.reason = reason;
    record.owner = owner;
    record.finished_at = Some(now);
}

/// The test name: the libtest thread name, which is the test path inside
/// its target, or `module_path:line` when the thread has no useful name.
fn derive_name(thread_name: Option<&str>, site: &TestSite) -> String {
    match thread_name {
        Some(name) if name != "main" => name.to_owned(),
        _ => format!("{}:{}", site.module_path, site.line),
    }
}

/// Reads the evidence environment: no run, or a run folder and kind.
fn run_target(
    dir: Option<OsString>,
    kind: Option<OsString>,
) -> Result<Option<(PathBuf, Kind)>, String> {
    let Some(dir) = dir else {
        return Ok(None);
    };
    let dir = PathBuf::from(dir);
    if !dir.is_absolute() {
        return Err(format!(
            "{EVIDENCE_DIR_ENV} must be an absolute path, found `{}`",
            dir.display()
        ));
    }
    if !dir.is_dir() {
        return Err(format!(
            "{EVIDENCE_DIR_ENV} points at `{}`, which is not a folder",
            dir.display()
        ));
    }
    let kind = kind
        .as_deref()
        .and_then(|value| value.to_str())
        .and_then(Kind::parse)
        .ok_or_else(|| {
            format!(
                "{EVIDENCE_KIND_ENV} must be `fixture` or `native` when {EVIDENCE_DIR_ENV} is set"
            )
        })?;
    Ok(Some((dir, kind)))
}

/// A guard's kind must equal the kind of the run.
fn check_kind(guard: Kind, run: Kind) -> Result<(), String> {
    if guard == run {
        return Ok(());
    }
    Err(format!(
        "this test is {} but the run is {}: only native tests carry \
         #[ignore = \"native: ...\"], and only they use Kind::Native",
        guard.as_str(),
        run.as_str()
    ))
}

/// Writes `record` to `<run_dir>/records/<id>.json`, atomically.
fn write_record(run_dir: &Path, record: &EvidenceRecord) -> io::Result<()> {
    let records = run_dir.join("records");
    fs::create_dir_all(&records)?;
    write_json_atomic(&records.join(format!("{}.json", record.id)), record)
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;
    use crate::RUST_DEFAULT_ACTUAL;

    const COVERS: &[&str] = &["REQ-051"];

    fn read(dir: &Path, id: Uuid) -> EvidenceRecord {
        let path = dir.join("records").join(format!("{id}.json"));
        serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
    }

    /// Runs `body` with a fresh `with_dir` guard on its own thread, so a
    /// panic there never reaches the calling test, then returns the record
    /// the guard left on disk.
    fn run_guard(body: impl FnOnce(&Evidence) + Send + 'static) -> EvidenceRecord {
        let dir = tempfile::tempdir().unwrap();
        let run_dir = dir.path().to_path_buf();
        let id = thread::spawn(move || {
            let ev = Evidence::with_dir(
                &run_dir,
                test_site!(),
                COVERS,
                Kind::Fixture,
                "a guard test",
            );
            let id = ev.id();
            let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| body(&ev)));
            // Re-raise inside the guard's lifetime so Drop sees the panic.
            if let Err(payload) = outcome {
                std::panic::resume_unwind(payload);
            }
            id
        })
        .join();
        let id = match id {
            Ok(id) => id,
            // The guard was dropped while panicking: find its only record.
            Err(_) => {
                let entry = fs::read_dir(dir.path().join("records"))
                    .unwrap()
                    .next()
                    .unwrap()
                    .unwrap();
                let stem = entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned();
                stem.parse().unwrap()
            }
        };
        read(dir.path(), id)
    }

    #[test]
    fn guard_writes_one_record_per_test_with_its_final_status() {
        let ev = crate::evidence!(
            covers = ["REQ-051"],
            kind = Kind::Fixture,
            expected =
                "every test that proves a PRD ID leaves one record with an honest final status",
        );

        let passed = run_guard(Evidence::pass);
        assert_eq!(passed.status, Status::Pass);
        assert_eq!(passed.actual.as_deref(), Some(RUST_DEFAULT_ACTUAL));
        assert!(passed.validate().is_ok());

        let observed = run_guard(|ev| ev.pass_with("ready after 2 probes"));
        assert_eq!(observed.actual.as_deref(), Some("ready after 2 probes"));

        let blocked = run_guard(|ev| ev.blocked("no managed artifact", "@ojiepermana"));
        assert_eq!(blocked.status, Status::Blocked);
        assert_eq!(blocked.owner.as_deref(), Some("@ojiepermana"));

        let unfinished = run_guard(|_| {});
        assert_eq!(unfinished.status, Status::Fail);
        assert_eq!(unfinished.actual.as_deref(), Some(UNFINISHED_ACTUAL));

        let panicked = run_guard(|ev| {
            ev.pass();
            panic!("an assertion failed after pass()");
        });
        assert_eq!(panicked.status, Status::Fail);
        assert_eq!(panicked.actual.as_deref(), Some(PANIC_ACTUAL));

        ev.pass();
    }

    #[test]
    fn derived_name_is_the_libtest_test_path() {
        let name = derive_name(thread::current().name(), &test_site!());
        assert_eq!(name, "guard::tests::derived_name_is_the_libtest_test_path");
    }

    #[test]
    fn derived_name_falls_back_to_module_path_and_line() {
        let site = test_site!();
        assert_eq!(
            derive_name(Some("main"), &site),
            format!("stackly_evidence::guard::tests:{}", site.line)
        );
    }

    #[test]
    fn noting_a_second_result_panics() {
        let record = run_guard(|ev| {
            ev.pass();
            ev.blocked("too late", "@ojiepermana");
        });
        assert_eq!(record.status, Status::Fail);
    }

    #[test]
    fn a_not_run_record_is_on_disk_before_the_test_ends() {
        let dir = tempfile::tempdir().unwrap();
        let ev = Evidence::with_dir(
            dir.path(),
            test_site!(),
            COVERS,
            Kind::Fixture,
            "a guard test",
        );

        let record = read(dir.path(), ev.id());
        assert_eq!(record.status, Status::NotRun);
        assert_eq!(record.finished_at, None);
        assert_eq!(record.test.package.as_deref(), Some("stackly-evidence"));
        ev.pass();
    }

    #[test]
    fn artifact_dir_in_a_run_becomes_the_evidence_path() {
        let record = run_guard(|ev| {
            let folder = ev.artifact_dir();
            fs::write(folder.join("server.log"), b"ready").unwrap();
            ev.pass();
        });
        assert_eq!(record.evidence_path, format!("artifacts/{}/", record.id));
    }

    #[test]
    fn artifact_dir_without_a_run_is_removed_with_the_guard() {
        let ev = Evidence::create(
            None,
            test_site!(),
            COVERS,
            Kind::Fixture,
            "a guard test",
            false,
        );
        let folder = ev.artifact_dir();
        assert!(folder.is_dir());
        ev.pass();
        drop(ev);
        assert!(!folder.exists());
    }

    #[test]
    fn subjects_are_recorded_with_their_digest() {
        let record = run_guard(|ev| {
            ev.subject(
                "postgresql",
                "18.1",
                Some(&"0".repeat(64)),
                Source::External,
            );
            ev.pass();
        });
        assert_eq!(record.subjects.len(), 1);
        assert_eq!(record.subjects[0].version, "18.1");
    }

    #[test]
    fn a_version_selector_is_rejected_as_a_subject() {
        let record = run_guard(|ev| ev.subject("postgresql", "18", None, Source::Managed));
        assert_eq!(record.status, Status::Fail);
    }

    #[test]
    #[should_panic(expected = "is not a PRD ID")]
    fn covers_must_hold_prd_ids() {
        let _ev = Evidence::create(None, test_site!(), &["OQ-011"], Kind::Fixture, "x", false);
    }

    #[test]
    fn run_target_is_empty_without_a_run_folder() {
        assert_eq!(run_target(None, None), Ok(None));
    }

    #[test]
    fn run_target_rejects_a_relative_folder_a_missing_folder_or_no_kind() {
        let dir = tempfile::tempdir().unwrap();
        let native = Some(OsString::from("native"));
        assert!(run_target(Some("relative/run".into()), native.clone()).is_err());
        assert!(run_target(Some(dir.path().join("missing").into()), native.clone()).is_err());
        assert!(run_target(Some(dir.path().into()), None).is_err());
        assert!(run_target(Some(dir.path().into()), Some("ci".into())).is_err());
        assert_eq!(
            run_target(Some(dir.path().into()), native),
            Ok(Some((dir.path().to_path_buf(), Kind::Native)))
        );
    }

    #[test]
    fn a_kind_mismatch_names_the_native_ignore_rule() {
        let message = check_kind(Kind::Fixture, Kind::Native).unwrap_err();
        assert!(message.contains("#[ignore = \"native: ...\"]"));
        assert!(check_kind(Kind::Native, Kind::Native).is_ok());
    }

    #[test]
    fn one_thread_holds_at_most_one_started_guard() {
        thread::spawn(|| {
            GUARD_HELD.with(|held| held.set(true));
            let second = std::panic::catch_unwind(|| {
                Evidence::start(test_site!(), COVERS, Kind::Fixture, "a second guard")
            });
            assert!(second.is_err());
        })
        .join()
        .unwrap();
    }
}
