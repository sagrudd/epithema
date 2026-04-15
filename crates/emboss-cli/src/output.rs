//! Output helpers for the `emboss-rs` command surface.

use std::path::Path;

use emboss_docgen::{AutodocProcessingSummary, GeneratedDocsReport};
use emboss_io::write_fasta_string;
use emboss_service::{EmbossService, InvocationResponse, MethodResult, ResultPayload};
use emboss_testkit::ToolValidationReport;

/// Prints the current governed tool catalogue.
pub fn print_tool_list(service: &EmbossService) {
    println!("EMBOSS-RS governed tool catalogue");
    println!("{}", service.status_line());

    if service.descriptors().is_empty() {
        println!("No governed tools are registered yet.");
        return;
    }

    for descriptor in service.descriptors() {
        println!("{:<16} {}", descriptor.name, descriptor.summary);
    }
}

/// Prints a tool response using shared result payload rendering.
pub fn print_tool_response(response: &InvocationResponse, service: &EmbossService) {
    println!("{}", format_method_result_summary(&response.result));
    println!("Run ID: {}", response.report.metadata.run_id);
    println!("{}", service.status_line());
}

/// Renders a stable human-readable method-result summary.
#[must_use]
pub fn format_method_result_summary(result: &MethodResult) -> String {
    let mut rendered = String::new();
    rendered.push_str(&result.summary.title);
    rendered.push('\n');

    for line in &result.summary.lines {
        rendered.push_str(line);
        rendered.push('\n');
    }

    match &result.payload {
        ResultPayload::Empty => {
            rendered.push_str(&format!("Payload kind: {}", result.payload.kind_label()));
            rendered.push('\n');
            rendered.push_str(&format!("Artifacts: {}", result.artifacts.len()));
        }
        ResultPayload::Sequence(record) => {
            rendered.push_str(&format!("Payload kind: {}", result.payload.kind_label()));
            rendered.push('\n');
            rendered.push_str(&format!("Artifacts: {}", result.artifacts.len()));
            rendered.push_str("\n\n");
            match write_fasta_string(std::slice::from_ref(record)) {
                Ok(fasta) => rendered.push_str(fasta.trim_end()),
                Err(error) => rendered.push_str(&format!(
                    "failed to render sequence payload as FASTA: {error}"
                )),
            }
        }
        ResultPayload::SequenceCollection(records) => {
            rendered.push_str(&format!("Payload kind: {}", result.payload.kind_label()));
            rendered.push('\n');
            rendered.push_str(&format!("Artifacts: {}", result.artifacts.len()));
            rendered.push_str("\n\n");
            match write_fasta_string(records) {
                Ok(fasta) => rendered.push_str(fasta.trim_end()),
                Err(error) => rendered.push_str(&format!(
                    "failed to render sequence payload as FASTA: {error}"
                )),
            }
        }
        ResultPayload::TextReport(report) => {
            rendered.push_str(&format!("Payload kind: {}", result.payload.kind_label()));
            rendered.push('\n');
            rendered.push_str(&format!("Artifacts: {}", result.artifacts.len()));
            rendered.push_str("\n\n");
            if let Some(title) = &report.title {
                rendered.push_str(title);
                rendered.push('\n');
            }
            rendered.push_str(&report.body);
        }
        ResultPayload::TableReport(table) => {
            rendered.push_str(&format!("Payload kind: {}", result.payload.kind_label()));
            rendered.push('\n');
            rendered.push_str(&format!("Artifacts: {}", result.artifacts.len()));
            if !table.columns.is_empty() {
                rendered.push_str("\n\n");
                rendered.push_str(&table.columns.join("\t"));
                for row in &table.rows {
                    rendered.push('\n');
                    rendered.push_str(&row.join("\t"));
                }
            }
        }
        ResultPayload::Alignment(_) | ResultPayload::Features(_) => {
            rendered.push_str(&format!("Payload kind: {}", result.payload.kind_label()));
            rendered.push('\n');
            rendered.push_str(&format!("Artifacts: {}", result.artifacts.len()));
        }
    }

    if !result.report.diagnostics().is_empty() {
        rendered.push('\n');
        rendered.push_str(&format!(
            "\nDiagnostics: {}",
            result.report.diagnostics().len()
        ));
    }

    rendered
}

/// Renders a stable human-readable autodoc summary.
#[must_use]
pub fn format_autodoc_summary(summary: &AutodocProcessingSummary, path: &Path) -> String {
    format!(
        "Autodoc contract loaded successfully\nInput: {}\nSchema version: {}\nDocument ID: {}\nTool: {}\nSections: {}\nArtifacts: {}\nExamples: {}\nSource mode: {}\nGoverned acquisition records: {}\nValidation: passed\nDiagnostics: {}",
        path.display(),
        summary.schema_version,
        summary.document_id,
        summary.tool_name,
        summary.section_count,
        summary.artifact_count,
        summary.example_count,
        summary.source_mode,
        summary.acquisition_record_count,
        summary.diagnostics.len(),
    )
}

/// Prints a stable human-readable autodoc summary.
pub fn print_autodoc_summary(summary: &AutodocProcessingSummary, path: &Path) {
    println!("{}", format_autodoc_summary(summary, path));
}

/// Renders a stable human-readable generated-docs emission summary.
#[must_use]
pub fn format_generated_docs_report(report: &GeneratedDocsReport) -> String {
    format!(
        "Generated documentation pages emitted successfully\nOutput root: {}\nTool page: {}\nIndex page: {}\nTool slug: {}\nSections rendered: {}\nArtifacts rendered: {}\nExamples rendered: {}\nDiagnostics included: {}",
        report.output_root.display(),
        report.tool_page.display(),
        report.index_page.display(),
        report.tool_slug,
        report.section_count,
        report.artifact_count,
        report.example_count,
        report.diagnostic_count,
    )
}

/// Prints a stable generated-docs emission summary.
pub fn print_generated_docs_report(report: &GeneratedDocsReport) {
    println!("{}", format_generated_docs_report(report));
}

/// Renders a stable human-readable validation-stub emission summary.
#[must_use]
pub fn format_validation_report_summary(report: &ToolValidationReport, path: &Path) -> String {
    format!(
        "Validation evidence stub emitted successfully\nOutput: {}\nTool: {}\nDocument ID: {}\nCases: {}\nDeclared: {}\nHarvested: {}\nRunnable: {}\nExecuted: {}\nCompared: {}\nPending gaps: {}\nDiagnostics: {}",
        path.display(),
        report.tool_name,
        report.document_id,
        report.summary.total_case_count,
        report.summary.declared_case_count,
        report.summary.harvested_case_count,
        report.summary.runnable_case_count,
        report.summary.executed_case_count,
        report.summary.compared_case_count,
        report.unresolved_gaps.len(),
        report.diagnostics.len(),
    )
}

/// Prints a stable validation-stub emission summary.
pub fn print_validation_report_summary(report: &ToolValidationReport, path: &Path) {
    println!("{}", format_validation_report_summary(report, path));
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use emboss_docgen::{AutodocProcessingSummary, GeneratedDocsReport};
    use emboss_docgen::{AutodocSourceMode, LegacyReference};
    use emboss_service::{
        ExecutionContext, ExecutionOutcome, ExecutionReport, InvocationOrigin, MethodResult,
        OutcomeStatus, ResultPayload, ResultSummary, ToolName,
    };
    use emboss_testkit::{
        ComparisonStatus, EvidenceDeclarationStatus, EvidenceSourceKind, ExecutionStatus,
        ToolValidationCase, ToolValidationReport, ValidationEvidenceSummary,
    };

    use super::{
        format_autodoc_summary, format_generated_docs_report, format_method_result_summary,
        format_validation_report_summary,
    };

    #[test]
    fn formats_autodoc_summary() {
        let summary = AutodocProcessingSummary {
            schema_version: "emboss-rs.autodoc/v1".to_owned(),
            document_id: "needle-minimal".to_owned(),
            tool_name: "needle".to_owned(),
            section_count: 1,
            artifact_count: 1,
            example_count: 1,
            source_mode: "curated".to_owned(),
            acquisition_record_count: 1,
            valid: true,
            diagnostics: Vec::new(),
        };

        let rendered = format_autodoc_summary(&summary, Path::new("example.json"));
        assert!(rendered.contains("Autodoc contract loaded successfully"));
        assert!(rendered.contains("Tool: needle"));
        assert!(rendered.contains("Governed acquisition records: 1"));
        assert!(rendered.contains("Validation: passed"));
    }

    #[test]
    fn formats_generated_docs_report() {
        let report = GeneratedDocsReport {
            output_root: Path::new("docs/generated").to_path_buf(),
            tool_page: Path::new("docs/generated/tools/needle.md").to_path_buf(),
            index_page: Path::new("docs/generated/index.md").to_path_buf(),
            tool_slug: "needle".to_owned(),
            section_count: 1,
            artifact_count: 1,
            example_count: 1,
            diagnostic_count: 0,
        };

        let rendered = format_generated_docs_report(&report);
        assert!(rendered.contains("Generated documentation pages emitted successfully"));
        assert!(rendered.contains("Tool page: docs/generated/tools/needle.md"));
    }

    #[test]
    fn formats_validation_report_summary() {
        let report = ToolValidationReport {
            tool_name: "needle".to_owned(),
            document_id: "needle-minimal".to_owned(),
            source_mode: AutodocSourceMode::Curated,
            evidence_source: EvidenceSourceKind::CuratedAutodoc,
            summary: ValidationEvidenceSummary {
                total_case_count: 1,
                declared_case_count: 1,
                harvested_case_count: 0,
                runnable_case_count: 1,
                executed_case_count: 0,
                compared_case_count: 0,
                passed_case_count: 0,
                failed_case_count: 0,
                partial_case_count: 0,
                unsupported_case_count: 0,
            },
            cases: vec![ToolValidationCase {
                id: "basic_alignment".to_owned(),
                title: "Basic alignment example".to_owned(),
                evidence_source: EvidenceSourceKind::CuratedAutodoc,
                declaration_status: EvidenceDeclarationStatus::Declared,
                execution_status: ExecutionStatus::Runnable,
                comparison_status: ComparisonStatus::NotRequested,
                required: true,
                artifact_ids: vec!["example_fasta".to_owned()],
                expected_output_ids: vec!["report".to_owned()],
                provenance: vec![LegacyReference {
                    source: "curated".to_owned(),
                    locator: None,
                    invocation: None,
                }],
                diagnostics: Vec::new(),
            }],
            unresolved_gaps: Vec::new(),
            diagnostics: Vec::new(),
            provenance: Vec::new(),
        };

        let rendered = format_validation_report_summary(
            &report,
            Path::new("docs/generated/validation/needle.validation.json"),
        );
        assert!(rendered.contains("Validation evidence stub emitted successfully"));
        assert!(rendered.contains("Runnable: 1"));
    }

    #[test]
    fn formats_method_result_summary() {
        let context = ExecutionContext::for_origin(InvocationOrigin::Cli);
        let report = ExecutionReport::from_context(
            &context,
            "emboss-rs",
            "0.1.0",
            ExecutionOutcome::new(OutcomeStatus::NotImplemented).with_summary("pending"),
        );
        let result = MethodResult::new(
            ToolName::new("needle").expect("tool name should build"),
            ResultPayload::Empty,
            ResultSummary::new("needle not implemented")
                .with_line("global alignment")
                .with_line("Execution path is registered but implementation is pending."),
            report,
        );

        let rendered = format_method_result_summary(&result);
        assert!(rendered.contains("needle not implemented"));
        assert!(rendered.contains("Payload kind: empty"));
    }

    #[test]
    fn renders_sequence_payload_as_fasta() {
        let context = ExecutionContext::for_origin(InvocationOrigin::Cli);
        let report = ExecutionReport::from_context(
            &context,
            "emboss-rs",
            "0.1.0",
            ExecutionOutcome::new(OutcomeStatus::Succeeded).with_summary("ok"),
        );
        let sequence = emboss_core::SequenceRecord::new(
            emboss_core::SequenceIdentifier::new("seq1").expect("identifier should build"),
            emboss_core::MoleculeKind::Dna,
            "ACGT",
        )
        .expect("sequence should build");
        let result = MethodResult::new(
            ToolName::new("newseq").expect("tool name should build"),
            ResultPayload::Sequence(sequence),
            ResultSummary::new("Sequence record created"),
            report,
        );

        let rendered = format_method_result_summary(&result);
        assert!(rendered.contains(">seq1"));
        assert!(rendered.contains("ACGT"));
    }
}
