//! Validation evidence model.

use emboss_docgen::LegacyReference;
use serde::{Deserialize, Serialize};

/// Where a validation case's evidence currently comes from.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceSourceKind {
    /// Human-curated autodoc content.
    #[serde(alias = "curated")]
    CuratedAutodoc,
    /// Legacy-derived autodoc content.
    #[serde(alias = "legacy_harvested")]
    LegacyHarvestedAutodoc,
    /// Mixed curated and legacy-derived content.
    #[serde(alias = "mixed")]
    MixedAutodoc,
    /// Future executed EMBOSS-RS run evidence.
    #[serde(alias = "executed")]
    ExecutedRun,
}

/// Whether a case is only declared or backed by harvested legacy evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceDeclarationStatus {
    /// Declared in autodoc but not harvested from legacy material.
    Declared,
    /// Derived wholly or partly from harvested legacy material.
    Harvested,
}

/// Current execution-oriented state for a validation case.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    /// Case exists but cannot be run yet with current information.
    Pending,
    /// Case has enough declared structure to be runnable later.
    Runnable,
    /// Case has been executed.
    Executed,
    /// Case is known to be unsupported.
    Unsupported,
}

/// Current comparison-oriented state for a validation case.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonStatus {
    /// No comparison has been requested.
    #[serde(alias = "not_compared")]
    NotRequested,
    /// Comparison is intended but has not yet been run.
    Pending,
    /// Comparison completed.
    Compared,
    /// Comparison passed.
    Passed,
    /// Comparison failed.
    Failed,
    /// Comparison is only partially satisfied.
    Partial,
    /// Comparison is not currently supported.
    Unsupported,
}

/// Serializable note preserved in a validation record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvidenceNote {
    /// Relative severity of the note.
    pub severity: EvidenceNoteSeverity,
    /// Stable machine-oriented code when available.
    pub code: Option<String>,
    /// Human-readable note text.
    pub message: String,
    /// Optional contextual detail.
    pub context: Option<String>,
}

impl EvidenceNote {
    /// Creates a new evidence note.
    #[must_use]
    pub fn new(severity: EvidenceNoteSeverity, message: impl Into<String>) -> Self {
        Self {
            severity,
            code: None,
            message: message.into(),
            context: None,
        }
    }

    /// Adds a stable note code.
    #[must_use]
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Adds optional contextual detail.
    #[must_use]
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

/// Relative severity of a validation note.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceNoteSeverity {
    /// Advisory note.
    Notice,
    /// Warning that should be surfaced to contributors and reviewers.
    Warning,
    /// Error-like note indicating a broken validation record.
    Error,
}

/// Per-example validation case derived from autodoc or future execution evidence.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ToolValidationCase {
    /// Stable case identifier.
    pub id: String,
    /// Human-readable case title.
    #[serde(default)]
    pub title: String,
    /// Declaration source for this case.
    #[serde(default = "default_evidence_source_kind", alias = "source_kind")]
    pub evidence_source: EvidenceSourceKind,
    /// Whether the case is declared or harvested.
    pub declaration_status: EvidenceDeclarationStatus,
    /// Current execution status.
    pub execution_status: ExecutionStatus,
    /// Current comparison status.
    pub comparison_status: ComparisonStatus,
    /// Whether this case is required by the document-level validation block.
    #[serde(default = "default_required_case")]
    pub required: bool,
    /// Referenced artefacts from the autodoc declaration.
    #[serde(default)]
    pub artifact_ids: Vec<String>,
    /// Declared expected outputs from the autodoc declaration.
    #[serde(default)]
    pub expected_output_ids: Vec<String>,
    /// Legacy or source references relevant to the case.
    #[serde(default)]
    pub provenance: Vec<LegacyReference>,
    /// Non-fatal notes about the case.
    #[serde(default)]
    pub diagnostics: Vec<EvidenceNote>,
}

fn default_evidence_source_kind() -> EvidenceSourceKind {
    EvidenceSourceKind::CuratedAutodoc
}

fn default_required_case() -> bool {
    true
}
