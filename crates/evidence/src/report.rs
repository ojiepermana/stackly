//! The run report `stackly.evidence-report/v1`.

use std::collections::BTreeMap;

use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{EvidenceRecord, Kind, RunContext, Status};

/// What `cargo xtask evidence run` writes to `report.json` after a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RunReport {
    /// Always [`crate::REPORT_SCHEMA`].
    pub schema: String,
    /// Equal to `context.runId`.
    pub run_id: Uuid,
    /// Equal to `context.runKind`.
    pub kind: Kind,
    /// `CARGO_PKG_VERSION` of `xtask`.
    pub wrapper_version: String,
    /// The context of this run.
    pub context: RunContext,
    /// When the wrapper started.
    pub started_at: Timestamp,
    /// When the wrapper finished.
    pub finished_at: Timestamp,
    /// The exit code of the command, or null when a signal killed it.
    #[serde(default)]
    pub command_exit_code: Option<i32>,
    /// The signal that killed the command, or null.
    #[serde(default)]
    pub command_signal: Option<i32>,
    /// True when the wrapper received SIGINT or SIGTERM.
    pub interrupted: bool,
    /// Record counts.
    pub summary: Summary,
    /// Every problem that fails the run.
    pub problems: Vec<Problem>,
    /// Every record, each with its own `context`, so a reader can take one
    /// record without its report.
    pub records: Vec<EvidenceRecord>,
}

impl RunReport {
    /// Creates the report of a run that has no records, problems, or
    /// command result yet; set those fields once the run ends.
    #[must_use]
    pub fn new(
        run_id: Uuid,
        kind: Kind,
        wrapper_version: impl Into<String>,
        context: RunContext,
        started_at: Timestamp,
        finished_at: Timestamp,
    ) -> Self {
        Self {
            schema: crate::REPORT_SCHEMA.to_owned(),
            run_id,
            kind,
            wrapper_version: wrapper_version.into(),
            context,
            started_at,
            finished_at,
            command_exit_code: None,
            command_signal: None,
            interrupted: false,
            summary: Summary::of(&[]),
            problems: Vec::new(),
            records: Vec::new(),
        }
    }

    /// Sets the records and problems, and counts the records again.
    pub fn set_records(&mut self, records: Vec<EvidenceRecord>, problems: Vec<Problem>) {
        self.summary = Summary::of(&records);
        self.records = records;
        self.problems = problems;
    }
}

/// Record counts of one run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    /// The number of records.
    pub total: u32,
    /// Records per status, with every status present.
    pub by_status: BTreeMap<Status, u32>,
    /// Records per kind, with every kind present.
    pub by_kind: BTreeMap<Kind, u32>,
}

impl Summary {
    /// Counts `records` by status and by kind.
    #[must_use]
    pub fn of(records: &[EvidenceRecord]) -> Self {
        let mut by_status: BTreeMap<Status, u32> = Status::ALL.iter().map(|s| (*s, 0)).collect();
        let mut by_kind: BTreeMap<Kind, u32> = [Kind::Fixture, Kind::Native]
            .iter()
            .map(|k| (*k, 0))
            .collect();
        for record in records {
            *by_status.entry(record.status).or_default() += 1;
            *by_kind.entry(record.kind).or_default() += 1;
        }
        Self {
            total: u32::try_from(records.len()).unwrap_or(u32::MAX),
            by_status,
            by_kind,
        }
    }
}

/// One problem that fails a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Problem {
    /// What went wrong.
    pub code: ProblemCode,
    /// The record it concerns, or null for a run level problem.
    #[serde(default)]
    pub record_id: Option<Uuid>,
    /// A lowercase sentence naming the rule.
    pub message: String,
}

/// The kinds of problem that fail a run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ProblemCode {
    /// A raw record could not be decoded, or breaks a rule.
    InvalidRecord,
    /// A record's kind differs from the run's kind.
    KindMismatch,
    /// The same test identity appears twice.
    DuplicateTest,
    /// A record is still `NOT_RUN` although the command exited 0.
    LeftNotRun,
    /// The fixture credential prefix showed up in the run output.
    CredentialLeak,
}

impl ProblemCode {
    /// The value used on the wire, for example `kind_mismatch`.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InvalidRecord => "invalid_record",
            Self::KindMismatch => "kind_mismatch",
            Self::DuplicateTest => "duplicate_test",
            Self::LeftNotRun => "left_not_run",
            Self::CredentialLeak => "credential_leak",
        }
    }
}
