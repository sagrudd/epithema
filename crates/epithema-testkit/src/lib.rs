//! Validation evidence and reporting foundations for Epithema.
//!
//! This crate owns the typed evidence model that future tool prompts can attach
//! to from day one. It distinguishes declared evidence, harvested legacy
//! evidence, execution state, and comparison state without pretending that
//! declared examples have already been run.

pub mod anchor;
pub mod cohort_health;
pub mod comparison_coverage;
pub mod cross_surface;
pub mod evidence;
pub mod full_compared_cohort;
pub mod governance;
pub mod harvest_coverage;
pub mod projection;
pub mod report;
pub mod retained_backlog;

pub use anchor::{
    AcceptanceAnchorSpec, acceptance_anchor_specs, derive_acceptance_anchor_report,
    write_acceptance_anchor_reports,
};
pub use cohort_health::{
    CohortHealthRecommendation, CohortHealthReport, CohortHealthSignal, CohortHealthSignalCode,
    CohortHealthSignalSeverity, CohortHealthSummary, derive_cohort_health_report,
    render_cohort_health_markdown, write_cohort_health_markdown, write_cohort_health_report_json,
};
pub use comparison_coverage::{
    ComparisonCoverageFamilyRecord, ComparisonCoverageReport, ComparisonCoverageSummary,
    derive_comparison_coverage_report, render_comparison_coverage_markdown,
    write_comparison_coverage_markdown, write_comparison_coverage_report_json,
};
pub use cross_surface::{
    CrossSurfaceExpected, CrossSurfaceFixtureCase, CrossSurfaceFixtureCatalog,
    DEFAULT_NUMERIC_TOLERANCE, write_cross_surface_fixture_catalog_json,
};
pub use evidence::{
    ComparisonStatus, EvidenceDeclarationStatus, EvidenceNote, EvidenceNoteSeverity,
    EvidenceSourceKind, ExecutionStatus, ToolValidationCase,
};
pub use full_compared_cohort::{
    BelowComparedMethodRecord, FullComparedCohortReport, FullComparedCohortSummary,
    derive_full_compared_cohort_report, render_full_compared_cohort_markdown,
    write_full_compared_cohort_markdown, write_full_compared_cohort_report_json,
};
pub use governance::{
    GovernanceAlignmentFamilyRecord, GovernanceAlignmentMethodRecord, GovernanceAlignmentReport,
    GovernanceAlignmentSummary, GovernanceDecision, GovernanceMappingEntry,
    derive_governance_alignment_report, parse_governance_mapping_reference,
    render_governance_alignment_markdown, write_governance_alignment_markdown,
    write_governance_alignment_report_json,
};
pub use harvest_coverage::{
    HarvestCoverageExceptionRecord, HarvestCoverageReport, HarvestCoverageSummary,
    derive_harvest_coverage_report, render_harvest_coverage_markdown,
    write_harvest_coverage_markdown, write_harvest_coverage_report_json,
};
pub use projection::{derive_validation_report, write_validation_report_json};
pub use report::{
    CohortDocumentationRecord, CohortDocumentationStatus, CohortEvidenceLevel, CohortGapCode,
    CohortMethodGap, CohortMethodValidationRecord, CohortValidationReport, CohortValidationSummary,
    ToolValidationReport, ValidationContext, ValidationEvidenceSummary,
    derive_shipped_cohort_validation_report, render_cohort_validation_markdown,
    write_cohort_validation_markdown, write_cohort_validation_report_json,
};
pub use retained_backlog::{
    RetainedBacklogBlocker, RetainedBacklogRecord, RetainedBacklogReport, RetainedBacklogSummary,
    derive_retained_backlog_report, render_retained_backlog_markdown,
    write_retained_backlog_markdown, write_retained_backlog_report_json,
};
