//! Executable acceptance anchors for a small cross-family validation cohort.
//!
//! These anchors turn declared autodoc examples into real executed and compared
//! evidence without widening the scope to a full historical-harvest framework.

use std::fs;
use std::path::{Path, PathBuf};

use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_docgen::{LegacyReference, load_document_from_path};
use emboss_io::{write_fasta_string, write_stockholm_string};
use emboss_service::{
    EmbossService, ExecutionContext, InvocationRequest, ResultPayload, ServiceRegistry, ToolName,
};
use emboss_tools::governed_tool_descriptors;

use crate::evidence::{ComparisonStatus, EvidenceSourceKind, ExecutionStatus};
use crate::projection::{derive_validation_report, write_validation_report_json};
use crate::report::ToolValidationReport;

/// Stable anchor specification used to execute and compare one accepted case.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AcceptanceAnchorSpec {
    /// Tool name in the governed registry.
    pub tool_name: &'static str,
    /// Relative path to the committed autodoc contract.
    pub autodoc_contract: &'static str,
    /// Example ID inside the autodoc contract.
    pub example_id: &'static str,
    /// Relative path to the checked-in expected output fixture.
    pub expected_output: &'static str,
    /// Human-readable historical source label.
    pub legacy_source: &'static str,
    /// Historical source locator.
    pub legacy_locator: &'static str,
    /// Historical invocation form associated with the example.
    pub legacy_invocation: &'static str,
}

const ACCEPTANCE_ANCHORS: &[AcceptanceAnchorSpec] = &[
    AcceptanceAnchorSpec {
        tool_name: "needle",
        autodoc_contract: "docs/autodoc/tools/needle.json",
        example_id: "basic_alignment",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/needle_basic_alignment.sto",
        legacy_source: "EMBOSS needle application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/needle.acd",
        legacy_invocation:
            "needle -asequence needle_query.fasta -bsequence needle_target.fasta -gapopen 10 -gapextend 0.5",
    },
    AcceptanceAnchorSpec {
        tool_name: "seqret",
        autodoc_contract: "docs/autodoc/tools/seqret.json",
        example_id: "normalize_local_fasta_records",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/seqret_normalize_local_fasta_records.fasta",
        legacy_source: "EMBOSS seqret application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/seqret.acd",
        legacy_invocation: "seqret -sequence three_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "extractfeat",
        autodoc_contract: "docs/autodoc/tools/extractfeat.json",
        example_id: "extract_selected_gene_feature",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/extractfeat_extract_selected_gene_feature.fasta",
        legacy_source: "EMBOSS extractfeat application",
        legacy_locator:
            "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/extractfeat.acd",
        legacy_invocation:
            "extractfeat -sequence annotated_feature.gbk -type gene -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "maskseq",
        autodoc_contract: "docs/autodoc/tools/maskseq.json",
        example_id: "mask_positions_two_to_three",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/maskseq_mask_positions_two_to_three.fasta",
        legacy_source: "EMBOSS maskseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/maskseq.acd",
        legacy_invocation: "maskseq -sequence three_records.fasta -regions 2:3 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "compseq",
        autodoc_contract: "docs/autodoc/tools/compseq.json",
        example_id: "per_record_and_aggregate_composition",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/compseq_per_record_and_aggregate_composition.tsv",
        legacy_source: "EMBOSS compseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/compseq.acd",
        legacy_invocation: "compseq -sequence nucleotide_pattern_records.fasta -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "pepstats",
        autodoc_contract: "docs/autodoc/tools/pepstats.json",
        example_id: "protein_summary_statistics",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/pepstats_protein_summary_statistics.tsv",
        legacy_source: "EMBOSS pepstats application",
        legacy_locator:
            "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/pepstats.acd",
        legacy_invocation: "pepstats -sequence protein_stats_records.fasta -stdout yes",
    },
];

/// Returns the committed acceptance-anchor cohort.
#[must_use]
pub fn acceptance_anchor_specs() -> &'static [AcceptanceAnchorSpec] {
    ACCEPTANCE_ANCHORS
}

/// Derives an executed-and-compared validation report for one acceptance anchor.
pub fn derive_acceptance_anchor_report(
    repo_root: impl AsRef<Path>,
    tool_name: &str,
) -> Result<ToolValidationReport, PlatformError> {
    let repo_root = repo_root.as_ref();
    let spec = acceptance_anchor_specs()
        .iter()
        .find(|spec| spec.tool_name == tool_name)
        .ok_or_else(|| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "requested tool is not part of the acceptance-anchor cohort",
            )
            .with_code("testkit.anchor.unknown_tool")
            .with_detail(tool_name.to_owned())
        })?;

    let document =
        load_document_from_path(repo_root.join(spec.autodoc_contract)).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Validation,
                "failed to load acceptance-anchor autodoc contract",
            )
            .with_code("testkit.anchor.autodoc.load_failed")
            .with_detail(format!(
                "{}: {error}",
                repo_root.join(spec.autodoc_contract).display()
            ))
        })?;
    let mut report = derive_validation_report(&document)?;
    let actual = execute_anchor_payload(repo_root, spec)?;
    let expected = fs::read_to_string(repo_root.join(spec.expected_output)).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to read committed acceptance-anchor expected output",
        )
        .with_code("testkit.anchor.expected_output.read_failed")
        .with_detail(format!("{}: {error}", repo_root.join(spec.expected_output).display()))
    })?;

    let actual_normalized = normalize_text(&actual);
    let expected_normalized = normalize_text(&expected);
    if actual_normalized != expected_normalized {
        return Err(
            PlatformError::new(
                ErrorCategory::Validation,
                "acceptance-anchor output differed from the committed expected output",
            )
            .with_code("testkit.anchor.comparison.failed")
            .with_detail(format!(
                "tool '{}' anchor case '{}' did not match '{}'",
                spec.tool_name, spec.example_id, spec.expected_output
            )),
        );
    }

    let legacy_reference = LegacyReference {
        source: spec.legacy_source.to_owned(),
        locator: Some(spec.legacy_locator.to_owned()),
        invocation: Some(spec.legacy_invocation.to_owned()),
    };

    let case = report
        .cases
        .iter_mut()
        .find(|case| case.id == spec.example_id)
        .ok_or_else(|| {
            PlatformError::new(
                ErrorCategory::Validation,
                "acceptance-anchor contract did not contain its declared example id",
            )
            .with_code("testkit.anchor.example.missing")
            .with_detail(format!("{} in {}", spec.example_id, spec.autodoc_contract))
        })?;

    case.evidence_source = EvidenceSourceKind::ExecutedRun;
    case.execution_status = ExecutionStatus::Executed;
    case.comparison_status = ComparisonStatus::Passed;
    if !case.provenance.contains(&legacy_reference) {
        case.provenance.push(legacy_reference.clone());
    }

    if !report.provenance.contains(&legacy_reference) {
        report.provenance.push(legacy_reference);
    }
    report.evidence_source = EvidenceSourceKind::ExecutedRun;

    Ok(ToolValidationReport::new(
        report.tool_name,
        report.document_id,
        report.source_mode,
        report.evidence_source,
        report.cases,
        report.unresolved_gaps,
        report.diagnostics,
        report.provenance,
    ))
}

/// Writes executed-and-compared reports for the acceptance-anchor cohort.
pub fn write_acceptance_anchor_reports(
    repo_root: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
) -> Result<Vec<PathBuf>, PlatformError> {
    let repo_root = repo_root.as_ref();
    let output_dir = output_dir.as_ref();
    fs::create_dir_all(output_dir).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to create acceptance-anchor validation output directory",
        )
        .with_code("testkit.anchor.output_dir.create_failed")
        .with_detail(format!("{}: {error}", output_dir.display()))
    })?;

    let mut written = Vec::new();
    for spec in acceptance_anchor_specs() {
        let report = derive_acceptance_anchor_report(repo_root, spec.tool_name)?;
        let path = output_dir.join(format!("{}.validation.json", spec.tool_name));
        write_validation_report_json(&report, &path)?;
        written.push(path);
    }

    Ok(written)
}

fn execute_anchor_payload(
    repo_root: &Path,
    spec: &AcceptanceAnchorSpec,
) -> Result<String, PlatformError> {
    let service = implemented_service()?;
    let request = InvocationRequest::new(
        ExecutionContext::default(),
        ToolName::new(spec.tool_name).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "invalid acceptance-anchor tool name",
            )
            .with_code("testkit.anchor.tool.invalid")
            .with_source(error)
        })?,
    )
    .with_arguments(anchor_arguments(repo_root, spec.tool_name));

    let response = service.invoke(request).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Invocation,
            "acceptance-anchor invocation failed",
        )
        .with_code("testkit.anchor.invoke_failed")
        .with_source(error)
        .with_detail(spec.tool_name.to_owned())
    })?;

    render_payload(&response.result.payload)
}

fn implemented_service() -> Result<EmbossService, PlatformError> {
    let mut registry = ServiceRegistry::new();
    for descriptor in governed_tool_descriptors() {
        registry.register(*descriptor).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to register governed tool in acceptance-anchor service",
            )
            .with_code("testkit.anchor.registry.register_failed")
            .with_source(error)
            .with_detail(descriptor.name.to_owned())
        })?;
    }
    Ok(EmbossService::new(registry))
}

fn anchor_arguments(repo_root: &Path, tool_name: &str) -> Vec<String> {
    match tool_name {
        "needle" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/needle_query.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/needle_target.fasta")
                .display()
                .to_string(),
        ],
        "seqret" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
        ],
        "extractfeat" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/annotated_feature.gbk")
                .display()
                .to_string(),
            "--kind".to_owned(),
            "gene".to_owned(),
        ],
        "maskseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
            "2:3".to_owned(),
        ],
        "compseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/nucleotide_pattern_records.fasta")
                .display()
                .to_string(),
        ],
        "pepstats" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/protein_stats_records.fasta")
                .display()
                .to_string(),
        ],
        _ => Vec::new(),
    }
}

fn render_payload(payload: &ResultPayload) -> Result<String, PlatformError> {
    match payload {
        ResultPayload::Alignment(alignment) => write_stockholm_string(alignment).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Internal,
                "failed to render acceptance-anchor alignment payload",
            )
            .with_code("testkit.anchor.render_alignment_failed")
            .with_source(error)
        }),
        ResultPayload::Sequence(record) => {
            write_fasta_string(std::slice::from_ref(record)).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Internal,
                    "failed to render acceptance-anchor sequence payload",
                )
                .with_code("testkit.anchor.render_sequence_failed")
                .with_source(error)
            })
        }
        ResultPayload::SequenceCollection(records) => {
            write_fasta_string(records).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Internal,
                    "failed to render acceptance-anchor sequence collection",
                )
                .with_code("testkit.anchor.render_sequence_collection_failed")
                .with_source(error)
            })
        }
        ResultPayload::TableReport(table) => Ok(render_table(table)),
        other => Err(PlatformError::new(
            ErrorCategory::Validation,
            "acceptance-anchor payload kind is not currently comparable",
        )
        .with_code("testkit.anchor.payload.unsupported")
        .with_detail(other.kind_label().to_owned())),
    }
}

fn render_table(table: &emboss_service::TableReport) -> String {
    let mut rendered = String::new();
    if !table.columns.is_empty() {
        rendered.push_str(&table.columns.join("\t"));
        rendered.push('\n');
    }
    for row in &table.rows {
        rendered.push_str(&row.join("\t"));
        rendered.push('\n');
    }
    rendered
}

fn normalize_text(text: &str) -> String {
    text.replace("\r\n", "\n").trim_end().to_owned()
}

#[cfg(test)]
mod tests {
    use super::{acceptance_anchor_specs, derive_acceptance_anchor_report};

    fn repo_root() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repo root should resolve")
    }

    #[test]
    fn derives_passed_reports_for_every_acceptance_anchor() {
        let repo_root = repo_root();
        for spec in acceptance_anchor_specs() {
            let report = derive_acceptance_anchor_report(&repo_root, spec.tool_name)
                .expect("acceptance anchor should derive");
            assert_eq!(report.summary.total_case_count, 1);
            assert_eq!(report.summary.executed_case_count, 1);
            assert_eq!(report.summary.compared_case_count, 1);
            assert_eq!(report.summary.passed_case_count, 1);
            assert_eq!(report.cases[0].id, spec.example_id);
        }
    }
}
