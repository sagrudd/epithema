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

use crate::evidence::{
    ComparisonStatus, EvidenceDeclarationStatus, EvidenceSourceKind, ExecutionStatus,
};
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
        tool_name: "backtranseq",
        autodoc_contract: "docs/autodoc/tools/backtranseq.json",
        example_id: "representative_backtranslation",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/backtranseq_representative_backtranslation.fasta",
        legacy_source: "EMBOSS backtranseq application",
        legacy_locator:
            "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/backtranseq.acd",
        legacy_invocation: "backtranseq -sequence protein_stats_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "backtranambig",
        autodoc_contract: "docs/autodoc/tools/backtranambig.json",
        example_id: "ambiguous_backtranslation",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/backtranambig_ambiguous_backtranslation.fasta",
        legacy_source: "EMBOSS backtranambig application",
        legacy_locator:
            "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/backtranambig.acd",
        legacy_invocation: "backtranambig -sequence protein_stats_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "checktrans",
        autodoc_contract: "docs/autodoc/tools/checktrans.json",
        example_id: "compare_matching_translation_pair",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/checktrans_compare_matching_translation_pair.tsv",
        legacy_source: "EMBOSS checktrans application",
        legacy_locator:
            "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/checktrans.acd",
        legacy_invocation:
            "checktrans -sequence checktrans_nucleotide.fasta -translation checktrans_protein.fasta -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "transeq",
        autodoc_contract: "docs/autodoc/tools/transeq.json",
        example_id: "forward_frame_one_translation",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/transeq_forward_frame_one_translation.fasta",
        legacy_source: "EMBOSS transeq application",
        legacy_locator:
            "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/transeq.acd",
        legacy_invocation:
            "transeq -sequence checktrans_nucleotide.fasta -frame 1 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "getorf",
        autodoc_contract: "docs/autodoc/tools/getorf.json",
        example_id: "extract_stop_bounded_forward_orfs",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/getorf_extract_stop_bounded_forward_orfs.fasta",
        legacy_source: "EMBOSS getorf application",
        legacy_locator:
            "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/getorf.acd",
        legacy_invocation: "getorf -sequence getorf_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "prettyseq",
        autodoc_contract: "docs/autodoc/tools/prettyseq.json",
        example_id: "render_forward_frame_report",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/prettyseq_render_forward_frame_report.txt",
        legacy_source: "EMBOSS prettyseq application",
        legacy_locator:
            "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/prettyseq.acd",
        legacy_invocation:
            "prettyseq -sequence checktrans_nucleotide.fasta -frame 1 -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "tranalign",
        autodoc_contract: "docs/autodoc/tools/tranalign.json",
        example_id: "project_protein_alignment_to_codons",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/tranalign_project_protein_alignment_to_codons.sto",
        legacy_source: "EMBOSS tranalign application",
        legacy_locator:
            "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/tranalign.acd",
        legacy_invocation:
            "tranalign -asequence tranalign_protein_alignment.sto -bsequence checktrans_nucleotide.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "water",
        autodoc_contract: "docs/autodoc/tools/water.json",
        example_id: "basic_local_alignment",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/water_basic_local_alignment.sto",
        legacy_source: "EMBOSS water application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/water.acd",
        legacy_invocation:
            "water -asequence water_query.fasta -bsequence water_target.fasta -gapopen 5 -gapextend 1",
    },
    AcceptanceAnchorSpec {
        tool_name: "descseq",
        autodoc_contract: "docs/autodoc/tools/descseq.json",
        example_id: "summarize_plain_fasta_records",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/descseq_summarize_plain_fasta_records.tsv",
        legacy_source: "EMBOSS descseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/descseq.acd",
        legacy_invocation: "descseq -sequence annotated_feature.gbk -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "geecee",
        autodoc_contract: "docs/autodoc/tools/geecee.json",
        example_id: "per_record_and_aggregate_gc",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/geecee_per_record_and_aggregate_gc.tsv",
        legacy_source: "EMBOSS geecee application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/geecee.acd",
        legacy_invocation:
            "geecee -sequence nucleotide_pattern_records.fasta -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "infoseq",
        autodoc_contract: "docs/autodoc/tools/infoseq.json",
        example_id: "report_basic_sequence_information",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/infoseq_report_basic_sequence_information.tsv",
        legacy_source: "EMBOSS infoseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/infoseq.acd",
        legacy_invocation: "infoseq -sequence three_records.fasta -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "cusp",
        autodoc_contract: "docs/autodoc/tools/cusp.json",
        example_id: "report_complete_codon_usage_table",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/cusp_report_complete_codon_usage_table.tsv",
        legacy_source: "EMBOSS cusp application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/cusp.acd",
        legacy_invocation: "cusp -sequence codon_reference.fasta -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "fuzznuc",
        autodoc_contract: "docs/autodoc/tools/fuzznuc.json",
        example_id: "iupac_forward_search",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/fuzznuc_iupac_forward_search.tsv",
        legacy_source: "EMBOSS fuzznuc application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/fuzznuc.acd",
        legacy_invocation:
            "fuzznuc -sequence nucleotide_pattern_records.fasta -pattern ACGN -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "fuzzpro",
        autodoc_contract: "docs/autodoc/tools/fuzzpro.json",
        example_id: "wildcard_forward_search",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/fuzzpro_wildcard_forward_search.tsv",
        legacy_source: "EMBOSS fuzzpro application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/fuzzpro.acd",
        legacy_invocation: "fuzzpro -sequence protein_records.fasta -pattern MX -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "fuzztran",
        autodoc_contract: "docs/autodoc/tools/fuzztran.json",
        example_id: "forward_frame_search",
        expected_output:
            "crates/emboss-testkit/tests/fixtures/acceptance_anchors/fuzztran_forward_frame_search.tsv",
        legacy_source: "EMBOSS fuzztran application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/fuzztran.acd",
        legacy_invocation:
            "fuzztran -sequence checktrans_nucleotide.fasta -pattern MA -stdout yes",
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
    case.declaration_status = EvidenceDeclarationStatus::Harvested;
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
        "backtranseq" | "backtranambig" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/protein_stats_records.fasta")
                .display()
                .to_string(),
        ],
        "checktrans" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/checktrans_nucleotide.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/checktrans_protein.fasta")
                .display()
                .to_string(),
        ],
        "transeq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/checktrans_nucleotide.fasta")
                .display()
                .to_string(),
            "--frame".to_owned(),
            "1".to_owned(),
        ],
        "getorf" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/getorf_records.fasta")
                .display()
                .to_string(),
        ],
        "prettyseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/checktrans_nucleotide.fasta")
                .display()
                .to_string(),
            "--frame".to_owned(),
            "1".to_owned(),
        ],
        "tranalign" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/tranalign_protein_alignment.sto")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/checktrans_nucleotide.fasta")
                .display()
                .to_string(),
        ],
        "water" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/water_query.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/water_target.fasta")
                .display()
                .to_string(),
        ],
        "descseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/annotated_feature.gbk")
                .display()
                .to_string(),
        ],
        "geecee" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/nucleotide_pattern_records.fasta")
                .display()
                .to_string(),
        ],
        "infoseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
        ],
        "cusp" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/codon_reference.fasta")
                .display()
                .to_string(),
        ],
        "fuzznuc" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/nucleotide_pattern_records.fasta")
                .display()
                .to_string(),
            "ACGN".to_owned(),
        ],
        "fuzzpro" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/protein_records.fasta")
                .display()
                .to_string(),
            "MX".to_owned(),
        ],
        "fuzztran" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/checktrans_nucleotide.fasta")
                .display()
                .to_string(),
            "MA".to_owned(),
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
        ResultPayload::TextReport(report) => Ok(report.body.clone()),
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
