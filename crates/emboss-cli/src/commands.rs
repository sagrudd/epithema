//! Command handlers for the `emboss-rs` CLI.

use std::path::Path;

use emboss_docgen::{
    AutodocProcessingSummary, DEFAULT_GENERATED_DOCS_ROOT, GeneratedDocsReport,
    emit_generated_docs_from_path, load_summary_from_path,
};
use emboss_service::EmbossService;

use crate::output;

/// Runs the `list` command against the shared service registry.
pub fn run_list(service: &EmbossService) {
    output::print_tool_list(service);
}

/// Loads, validates, and reports an autodoc contract from a JSON file path.
pub fn run_autodoc(
    path: &Path,
    emit_docs: bool,
    docs_output_dir: Option<&Path>,
) -> Result<AutodocProcessingSummary, emboss_docgen::AutodocContractError> {
    let summary = load_summary_from_path(path)?;
    output::print_autodoc_summary(&summary, path);
    if emit_docs {
        let report = run_autodoc_emit_docs(path, docs_output_dir)?;
        output::print_generated_docs_report(&report);
    }
    Ok(summary)
}

/// Loads an autodoc document and emits generated Markdown pages.
pub fn run_autodoc_emit_docs(
    path: &Path,
    docs_output_dir: Option<&Path>,
) -> Result<GeneratedDocsReport, emboss_docgen::AutodocContractError> {
    let output_root = docs_output_dir.unwrap_or_else(|| Path::new(DEFAULT_GENERATED_DOCS_ROOT));
    emit_generated_docs_from_path(path, output_root)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{run_autodoc, run_autodoc_emit_docs};

    fn fixture(path: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }

    #[test]
    fn autodoc_command_loads_valid_fixture() {
        let summary = run_autodoc(
            &fixture("../emboss-docgen/tests/fixtures/minimal_autodoc.json"),
            false,
            None,
        )
        .expect("fixture should load");
        assert_eq!(summary.tool_name, "needle");
        assert!(summary.valid);
    }

    #[test]
    fn autodoc_command_rejects_missing_fixture() {
        assert!(run_autodoc(&fixture("tests/fixtures/missing.json"), false, None).is_err());
    }

    #[test]
    fn autodoc_command_can_emit_generated_docs() {
        let output_root = std::env::temp_dir().join(format!(
            "emboss-cli-autodoc-generated-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should advance")
                .as_nanos()
        ));

        let report = run_autodoc_emit_docs(
            &fixture("../emboss-docgen/tests/fixtures/minimal_autodoc.json"),
            Some(&output_root),
        )
        .expect("generated docs emission should succeed");

        assert!(report.tool_page.exists());
        let _ = std::fs::remove_dir_all(&output_root);
    }
}
