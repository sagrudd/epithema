//! Output helpers for the `emboss-rs` command surface.

use std::path::Path;

use emboss_docgen::{AutodocProcessingSummary, GeneratedDocsReport};
use emboss_service::{EmbossService, InvocationResponse};

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

/// Prints the current placeholder response for a known but unimplemented tool.
pub fn print_unimplemented_tool(response: &InvocationResponse, service: &EmbossService) {
    println!(
        "Tool '{}' is governed but not implemented yet.",
        response.tool
    );
    println!("{}", response.descriptor.summary);
    println!("Run ID: {}", response.report.metadata.run_id);
    println!("{}", service.status_line());
}

/// Renders a stable human-readable autodoc summary.
#[must_use]
pub fn format_autodoc_summary(summary: &AutodocProcessingSummary, path: &Path) -> String {
    format!(
        "Autodoc contract loaded successfully\nInput: {}\nSchema version: {}\nDocument ID: {}\nTool: {}\nSections: {}\nArtifacts: {}\nExamples: {}\nSource mode: {}\nValidation: passed\nDiagnostics: {}",
        path.display(),
        summary.schema_version,
        summary.document_id,
        summary.tool_name,
        summary.section_count,
        summary.artifact_count,
        summary.example_count,
        summary.source_mode,
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use emboss_docgen::{AutodocProcessingSummary, GeneratedDocsReport};

    use super::{format_autodoc_summary, format_generated_docs_report};

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
            valid: true,
            diagnostics: Vec::new(),
        };

        let rendered = format_autodoc_summary(&summary, Path::new("example.json"));
        assert!(rendered.contains("Autodoc contract loaded successfully"));
        assert!(rendered.contains("Tool: needle"));
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
}
