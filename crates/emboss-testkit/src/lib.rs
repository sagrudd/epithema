//! Validation evidence and reporting foundations for EMBOSS-RS.
//!
//! This crate owns the typed evidence model that future tool prompts can attach
//! to from day one. It distinguishes declared evidence, harvested legacy
//! evidence, execution state, and comparison state without pretending that
//! declared examples have already been run.

pub mod cross_surface;
pub mod evidence;
pub mod projection;
pub mod report;

pub use cross_surface::{
    CrossSurfaceExpected, CrossSurfaceFixtureCase, CrossSurfaceFixtureCatalog,
    DEFAULT_NUMERIC_TOLERANCE, write_cross_surface_fixture_catalog_json,
};
pub use evidence::{
    ComparisonStatus, EvidenceDeclarationStatus, EvidenceNote, EvidenceNoteSeverity,
    EvidenceSourceKind, ExecutionStatus, ToolValidationCase,
};
pub use projection::{derive_validation_report, write_validation_report_json};
pub use report::{
    CohortDocumentationRecord, CohortDocumentationStatus, CohortEvidenceLevel, CohortGapCode,
    CohortMethodGap, CohortMethodValidationRecord, CohortValidationReport,
    CohortValidationSummary, ToolValidationReport, ValidationContext,
    ValidationEvidenceSummary, derive_shipped_cohort_validation_report,
    render_cohort_validation_markdown, write_cohort_validation_markdown,
    write_cohort_validation_report_json,
};
