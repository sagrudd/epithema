//! `feattext` implementation.

use emboss_core::{FeatureSelector, select_features};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::feature_tools::render::render_feature_text;
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `feattext`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeattextParams {
    /// Local annotated sequence input path.
    pub input: SequenceInput,
    /// Shared feature selector.
    pub selector: FeatureSelector,
}

/// Structured `feattext` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeattextOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Shared feature selector.
    pub selector: FeatureSelector,
    /// Normalized feature-table text.
    pub body: String,
}

/// Returns `feattext` help text.
#[must_use]
pub fn feattext_help() -> &'static str {
    "Usage: emboss-rs feattext <input> [--kind <kind>] [--name <name>] [--qualifier <key[=value]>] [--strand <forward|reverse|unknown>]\n\nRender selected features from an annotated EMBL or GenBank input as normalized feature-table text. If no selector flags are supplied, all features are rendered. The v1 output is a governed normalized rendering, not a byte-for-byte recovery of the original source flatfile."
}

/// Executes `feattext`.
pub fn run_feattext(params: FeattextParams) -> Result<FeattextOutcome, ToolExecutionError> {
    let mut sections = Vec::new();

    for record in load_sequence_records(&params.input)? {
        let selected = select_features(&record, &params.selector);
        if selected.is_empty() {
            continue;
        }

        let mut lines = vec![
            format!("ID   {}", record.identifier().accession()),
            "FEATURES             Location/Qualifiers".to_owned(),
        ];
        for feature in selected {
            lines.push(render_feature_text(feature));
        }
        lines.push("//".to_owned());
        sections.push(lines.join("\n"));
    }

    if sections.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "feattext did not find any features matching the requested selector",
        )
        .with_code("tools.feattext.feature.no_match"));
    }

    Ok(FeattextOutcome {
        input: params.input,
        selector: params.selector,
        body: sections.join("\n\n"),
    })
}

#[cfg(test)]
mod tests {
    use super::{FeattextParams, run_feattext};
    use crate::sequence_stream::SequenceInput;

    #[test]
    fn renders_normalized_feature_table_text() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/annotated_feature.gbk");
        let outcome = run_feattext(FeattextParams {
            input: SequenceInput::new(&fixture),
            selector: emboss_core::FeatureSelector::Any,
        })
        .expect("feattext should execute");

        assert!(outcome.body.contains("ID   FEAT1"));
        assert!(outcome.body.contains("FEATURES             Location/Qualifiers"));
        assert!(outcome.body.contains("/product=\"short peptide\""));
    }
}
