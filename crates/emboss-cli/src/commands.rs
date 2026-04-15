//! Command handlers for the `emboss-rs` CLI.

use std::path::Path;

use emboss_docgen::{AutodocProcessingSummary, load_summary_from_path};
use emboss_service::EmbossService;

use crate::output;

/// Runs the `list` command against the shared service registry.
pub fn run_list(service: &EmbossService) {
    output::print_tool_list(service);
}

/// Loads, validates, and reports an autodoc contract from a JSON file path.
pub fn run_autodoc(
    path: &Path,
) -> Result<AutodocProcessingSummary, emboss_docgen::AutodocContractError> {
    let summary = load_summary_from_path(path)?;
    output::print_autodoc_summary(&summary, path);
    Ok(summary)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::run_autodoc;

    fn fixture(path: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }

    #[test]
    fn autodoc_command_loads_valid_fixture() {
        let summary = run_autodoc(&fixture(
            "../emboss-docgen/tests/fixtures/minimal_autodoc.json",
        ))
        .expect("fixture should load");
        assert_eq!(summary.tool_name, "needle");
        assert!(summary.valid);
    }

    #[test]
    fn autodoc_command_rejects_missing_fixture() {
        assert!(run_autodoc(&fixture("tests/fixtures/missing.json")).is_err());
    }
}
