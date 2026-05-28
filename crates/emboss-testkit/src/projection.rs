//! Projection from autodoc declarations into validation evidence stubs.

use std::fs;
use std::path::Path;

use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_docgen::{AutodocDocument, AutodocSourceMode, LegacyReference};

use crate::evidence::{
    ComparisonStatus, EvidenceDeclarationStatus, EvidenceNote, EvidenceNoteSeverity,
    EvidenceSourceKind, ExecutionStatus, ToolValidationCase,
};
use crate::report::ToolValidationReport;

/// Derives a structured validation report from a validated autodoc document.
pub fn derive_validation_report(
    document: &AutodocDocument,
) -> Result<ToolValidationReport, PlatformError> {
    document.validate()?;

    let evidence_source = source_kind(document.provenance.source_mode);
    let required_ids = document
        .validation
        .as_ref()
        .map(|validation| validation.required_example_ids.clone())
        .unwrap_or_default();
    let compare_against_legacy = document
        .validation
        .as_ref()
        .map(|validation| validation.compare_against_legacy)
        .unwrap_or(false);

    let mut cases = Vec::new();
    let mut diagnostics = Vec::new();
    let mut unresolved_gaps = Vec::new();

    for example in &document.examples {
        let declaration_status = declaration_status(
            document.provenance.source_mode,
            example.legacy_reference.as_ref(),
        );
        let execution_status = if example.artifact_ids.is_empty() {
            ExecutionStatus::Pending
        } else {
            ExecutionStatus::Runnable
        };
        let comparison_status = if compare_against_legacy || example.legacy_reference.is_some() {
            ComparisonStatus::Pending
        } else {
            ComparisonStatus::NotRequested
        };

        let mut case_diagnostics = Vec::new();
        if example.artifact_ids.is_empty() {
            case_diagnostics.push(
                EvidenceNote::new(
                    EvidenceNoteSeverity::Warning,
                    "validation case has no declared input artefacts",
                )
                .with_code("testkit.case.no_artifacts")
                .with_context(example.id.clone()),
            );
            unresolved_gaps.push(format!(
                "example '{}' needs declared input artefacts before it can become runnable",
                example.id
            ));
        }
        if example.expected_outputs.is_empty() {
            case_diagnostics.push(
                EvidenceNote::new(
                    EvidenceNoteSeverity::Notice,
                    "validation case has no declared expected outputs yet",
                )
                .with_code("testkit.case.no_expected_outputs")
                .with_context(example.id.clone()),
            );
            unresolved_gaps.push(format!(
                "example '{}' needs expected outputs or comparison targets for stronger evidence",
                example.id
            ));
        }
        if compare_against_legacy && example.legacy_reference.is_none() {
            case_diagnostics.push(
                EvidenceNote::new(
                    EvidenceNoteSeverity::Warning,
                    "legacy comparison is requested but this example has no explicit legacy reference",
                )
                .with_code("testkit.case.missing_legacy_reference")
                .with_context(example.id.clone()),
            );
            unresolved_gaps.push(format!(
                "example '{}' needs a legacy reference before comparison can run",
                example.id
            ));
        }

        diagnostics.extend(case_diagnostics.iter().cloned());

        cases.push(ToolValidationCase {
            id: example.id.clone(),
            title: example.title.clone(),
            evidence_source,
            declaration_status,
            execution_status,
            comparison_status,
            required: required_ids.contains(&example.id),
            artifact_ids: example.artifact_ids.clone(),
            expected_output_ids: example
                .expected_outputs
                .iter()
                .map(|output| output.id.clone())
                .collect(),
            provenance: unique_references(
                example
                    .legacy_reference
                    .iter()
                    .cloned()
                    .chain(document.provenance.source_references.iter().cloned())
                    .collect(),
            ),
            diagnostics: case_diagnostics,
        });
    }

    if cases.is_empty() {
        diagnostics.push(
            EvidenceNote::new(
                EvidenceNoteSeverity::Warning,
                "autodoc document does not declare any validation cases",
            )
            .with_code("testkit.report.no_cases")
            .with_context(document.tool.name.clone()),
        );
        unresolved_gaps.push(format!(
            "tool '{}' needs at least one declared example to establish validation evidence",
            document.tool.name
        ));
    }

    Ok(ToolValidationReport::new(
        document.tool.name.clone(),
        document.document_id.clone(),
        document.provenance.source_mode,
        evidence_source,
        cases,
        unresolved_gaps,
        diagnostics,
        report_provenance(document),
    ))
}

/// Writes a validation report as pretty-printed JSON.
pub fn write_validation_report_json(
    report: &ToolValidationReport,
    path: impl AsRef<Path>,
) -> Result<(), PlatformError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to create validation report output directory",
            )
            .with_code("testkit.report.create_dir_failed")
            .with_detail(format!("{}: {error}", parent.display()))
        })?;
    }

    let json = serde_json::to_string_pretty(report).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Internal,
            "failed to serialize validation report",
        )
        .with_code("testkit.report.serialize_failed")
        .with_source(error)
    })?;

    fs::write(path, json).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write validation report",
        )
        .with_code("testkit.report.write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

fn source_kind(source_mode: AutodocSourceMode) -> EvidenceSourceKind {
    match source_mode {
        AutodocSourceMode::RegistryStub => EvidenceSourceKind::RegistryStubAutodoc,
        AutodocSourceMode::Curated => EvidenceSourceKind::CuratedAutodoc,
        AutodocSourceMode::LegacyDerived => EvidenceSourceKind::LegacyHarvestedAutodoc,
        AutodocSourceMode::Mixed => EvidenceSourceKind::MixedAutodoc,
    }
}

fn declaration_status(
    source_mode: AutodocSourceMode,
    legacy_reference: Option<&LegacyReference>,
) -> EvidenceDeclarationStatus {
    match source_mode {
        AutodocSourceMode::RegistryStub => EvidenceDeclarationStatus::Declared,
        AutodocSourceMode::LegacyDerived => EvidenceDeclarationStatus::Harvested,
        AutodocSourceMode::Mixed if legacy_reference.is_some() => {
            EvidenceDeclarationStatus::Harvested
        }
        AutodocSourceMode::Curated | AutodocSourceMode::Mixed => {
            EvidenceDeclarationStatus::Declared
        }
    }
}

fn report_provenance(document: &AutodocDocument) -> Vec<LegacyReference> {
    unique_references(document.provenance.source_references.clone())
}

fn unique_references(references: Vec<LegacyReference>) -> Vec<LegacyReference> {
    let mut unique = Vec::new();
    for reference in references {
        if !unique.contains(&reference) {
            unique.push(reference);
        }
    }
    unique
}

#[cfg(test)]
mod tests {
    use emboss_docgen::load_document_from_path;

    use super::{derive_validation_report, write_validation_report_json};

    fn fixture(path: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }

    #[test]
    fn derives_validation_report_from_minimal_autodoc() {
        let document = load_document_from_path(fixture(
            "../emboss-docgen/tests/fixtures/minimal_autodoc.json",
        ))
        .expect("fixture should load");

        let report = derive_validation_report(&document).expect("report should derive");
        assert_eq!(report.tool_name, "needle");
        assert_eq!(report.summary.total_case_count, 1);
        assert_eq!(report.summary.declared_case_count, 1);
        assert_eq!(report.summary.runnable_case_count, 1);
        assert!(report.diagnostics.is_empty());
    }

    #[test]
    fn derives_harvested_and_pending_comparison_status_from_rich_autodoc() {
        let document =
            load_document_from_path(fixture("../emboss-docgen/tests/fixtures/rich_autodoc.json"))
                .expect("fixture should load");

        let report = derive_validation_report(&document).expect("report should derive");
        assert_eq!(report.summary.total_case_count, 2);
        assert_eq!(report.summary.harvested_case_count, 2);
        assert_eq!(report.summary.declared_case_count, 0);
        assert!(
            report
                .cases
                .iter()
                .all(|case| case.comparison_status == crate::ComparisonStatus::Pending)
        );
    }

    #[test]
    fn registry_stub_document_projects_as_declared_stub_evidence() {
        let document = emboss_docgen::build_stub_document(
            emboss_tools::governed_tool_descriptors()
                .iter()
                .copied()
                .find(|descriptor| descriptor.name == "aligncopy")
                .expect("aligncopy descriptor should exist"),
        );

        let report = derive_validation_report(&document).expect("report should derive");
        assert_eq!(
            report.source_mode,
            emboss_docgen::AutodocSourceMode::RegistryStub
        );
        assert_eq!(
            report.evidence_source,
            crate::EvidenceSourceKind::RegistryStubAutodoc
        );
        assert_eq!(report.summary.total_case_count, 0);
        assert_eq!(report.summary.declared_case_count, 0);
        assert_eq!(report.summary.harvested_case_count, 0);
    }

    #[test]
    fn serializes_validation_report_to_json() {
        let document = load_document_from_path(fixture(
            "../emboss-docgen/tests/fixtures/minimal_autodoc.json",
        ))
        .expect("fixture should load");
        let report = derive_validation_report(&document).expect("report should derive");
        let output_path = std::env::temp_dir().join(format!(
            "emboss-testkit-validation-{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should advance")
                .as_nanos()
        ));

        write_validation_report_json(&report, &output_path).expect("write should succeed");
        let json = std::fs::read_to_string(&output_path).expect("report should be readable");
        let reparsed: crate::ToolValidationReport =
            serde_json::from_str(&json).expect("report should round-trip");

        assert_eq!(reparsed.tool_name, "needle");
        let _ = std::fs::remove_file(&output_path);
    }
}
