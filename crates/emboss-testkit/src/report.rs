//! Aggregate validation reports.

use emboss_docgen::{AutodocSourceMode, LegacyReference};
use emboss_fixtures::FixtureCatalog;
use serde::{Deserialize, Serialize};

use crate::evidence::{
    ComparisonStatus, EvidenceNote, EvidenceSourceKind, ExecutionStatus, ToolValidationCase,
};

/// Minimal validation context for future harness expansion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationContext {
    /// Fixture source used by the current validation run.
    pub fixtures: FixtureCatalog,
}

impl ValidationContext {
    /// Creates a validation context for the workspace fixture catalogue.
    #[must_use]
    pub fn new() -> Self {
        Self {
            fixtures: FixtureCatalog::workspace(),
        }
    }
}

impl Default for ValidationContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Aggregate counts that summarize a tool validation record.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ValidationEvidenceSummary {
    /// Total validation case count.
    pub total_case_count: usize,
    /// Cases declared directly from curated inputs.
    pub declared_case_count: usize,
    /// Cases backed wholly or partly by harvested legacy material.
    pub harvested_case_count: usize,
    /// Cases that appear runnable later.
    pub runnable_case_count: usize,
    /// Cases already marked as executed.
    pub executed_case_count: usize,
    /// Cases with a comparison already completed.
    pub compared_case_count: usize,
    /// Cases with passing comparison status.
    pub passed_case_count: usize,
    /// Cases with failing comparison status.
    pub failed_case_count: usize,
    /// Cases with partial comparison status.
    pub partial_case_count: usize,
    /// Cases marked unsupported.
    pub unsupported_case_count: usize,
}

/// Tool-level validation evidence report.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ToolValidationReport {
    /// Stable tool name.
    pub tool_name: String,
    /// Source autodoc document identifier.
    pub document_id: String,
    /// High-level source mode inherited from autodoc.
    pub source_mode: AutodocSourceMode,
    /// Primary evidence source classification.
    pub evidence_source: EvidenceSourceKind,
    /// Aggregate case summary.
    pub summary: ValidationEvidenceSummary,
    /// Case-level validation records.
    pub cases: Vec<ToolValidationCase>,
    /// Unresolved gaps that future execution or comparison work must address.
    pub unresolved_gaps: Vec<String>,
    /// Report-level diagnostics.
    pub diagnostics: Vec<EvidenceNote>,
    /// Report-level provenance references.
    pub provenance: Vec<LegacyReference>,
}

impl ToolValidationReport {
    /// Builds a report and derives its summary counts.
    #[must_use]
    pub fn new(
        tool_name: impl Into<String>,
        document_id: impl Into<String>,
        source_mode: AutodocSourceMode,
        evidence_source: EvidenceSourceKind,
        cases: Vec<ToolValidationCase>,
        unresolved_gaps: Vec<String>,
        diagnostics: Vec<EvidenceNote>,
        provenance: Vec<LegacyReference>,
    ) -> Self {
        let summary = ValidationEvidenceSummary::from_cases(&cases);
        Self {
            tool_name: tool_name.into(),
            document_id: document_id.into(),
            source_mode,
            evidence_source,
            summary,
            cases,
            unresolved_gaps,
            diagnostics,
            provenance,
        }
    }
}

impl ValidationEvidenceSummary {
    /// Builds aggregate counts from case records.
    #[must_use]
    pub fn from_cases(cases: &[ToolValidationCase]) -> Self {
        let mut summary = Self {
            total_case_count: cases.len(),
            ..Self::default()
        };

        for case in cases {
            match case.declaration_status {
                crate::EvidenceDeclarationStatus::Declared => summary.declared_case_count += 1,
                crate::EvidenceDeclarationStatus::Harvested => summary.harvested_case_count += 1,
            }

            match case.execution_status {
                ExecutionStatus::Runnable => summary.runnable_case_count += 1,
                ExecutionStatus::Executed => summary.executed_case_count += 1,
                ExecutionStatus::Unsupported => summary.unsupported_case_count += 1,
                ExecutionStatus::Pending => {}
            }

            match case.comparison_status {
                ComparisonStatus::Compared => summary.compared_case_count += 1,
                ComparisonStatus::Passed => {
                    summary.compared_case_count += 1;
                    summary.passed_case_count += 1;
                }
                ComparisonStatus::Failed => {
                    summary.compared_case_count += 1;
                    summary.failed_case_count += 1;
                }
                ComparisonStatus::Partial => {
                    summary.compared_case_count += 1;
                    summary.partial_case_count += 1;
                }
                ComparisonStatus::Unsupported => summary.unsupported_case_count += 1,
                ComparisonStatus::NotRequested | ComparisonStatus::Pending => {}
            }
        }

        summary
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ComparisonStatus, EvidenceDeclarationStatus, EvidenceSourceKind, ExecutionStatus,
        ToolValidationCase, ValidationContext, ValidationEvidenceSummary,
    };

    #[test]
    fn binds_fixture_catalogue() {
        assert_eq!(
            ValidationContext::new().fixtures.source,
            "workspace fixture catalogue"
        );
    }

    #[test]
    fn aggregates_case_counts_by_status() {
        let cases = vec![
            ToolValidationCase {
                id: "declared".to_owned(),
                title: "Declared".to_owned(),
                evidence_source: EvidenceSourceKind::CuratedAutodoc,
                declaration_status: EvidenceDeclarationStatus::Declared,
                execution_status: ExecutionStatus::Runnable,
                comparison_status: ComparisonStatus::Pending,
                required: true,
                artifact_ids: vec!["input".to_owned()],
                expected_output_ids: vec!["output".to_owned()],
                provenance: Vec::new(),
                diagnostics: Vec::new(),
            },
            ToolValidationCase {
                id: "harvested".to_owned(),
                title: "Harvested".to_owned(),
                evidence_source: EvidenceSourceKind::LegacyHarvestedAutodoc,
                declaration_status: EvidenceDeclarationStatus::Harvested,
                execution_status: ExecutionStatus::Executed,
                comparison_status: ComparisonStatus::Passed,
                required: false,
                artifact_ids: Vec::new(),
                expected_output_ids: Vec::new(),
                provenance: Vec::new(),
                diagnostics: Vec::new(),
            },
        ];

        let summary = ValidationEvidenceSummary::from_cases(&cases);
        assert_eq!(summary.total_case_count, 2);
        assert_eq!(summary.declared_case_count, 1);
        assert_eq!(summary.harvested_case_count, 1);
        assert_eq!(summary.runnable_case_count, 1);
        assert_eq!(summary.executed_case_count, 1);
        assert_eq!(summary.passed_case_count, 1);
        assert_eq!(summary.compared_case_count, 1);
    }
}
