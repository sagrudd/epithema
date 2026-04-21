use emboss_core::{FeatureOperationError, MoleculeKind, SequenceRecord};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::ToolExecutionError;

pub fn effective_mask_symbol(
    tool: &str,
    record: &SequenceRecord,
    explicit: Option<char>,
) -> Result<char, ToolExecutionError> {
    let symbol = explicit.unwrap_or(default_mask_symbol(record.molecule()));
    if !record.alphabet().allows(symbol) {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} mask character '{symbol}' is not valid for {} sequences",
                record.molecule()
            ),
        )
        .with_code(format!("tools.{tool}.mask_char.invalid_for_molecule")));
    }

    Ok(symbol.to_ascii_uppercase())
}

pub fn default_mask_symbol(molecule: MoleculeKind) -> char {
    match molecule {
        MoleculeKind::Protein => 'X',
        _ => 'N',
    }
}

pub fn map_feature_error(tool: &str, error: FeatureOperationError) -> ToolExecutionError {
    let code = match error {
        FeatureOperationError::NoMatchingFeatures => {
            format!("tools.{tool}.feature.no_match")
        }
        FeatureOperationError::AmbiguousSelection { .. } => {
            format!("tools.{tool}.feature.ambiguous")
        }
        FeatureOperationError::UnsupportedComplexLocation => {
            format!("tools.{tool}.feature.unsupported_complex_location")
        }
        FeatureOperationError::UnsupportedReverseStrand { .. } => {
            format!("tools.{tool}.feature.unsupported_reverse_strand")
        }
        FeatureOperationError::Domain(_) => format!("tools.{tool}.feature.domain"),
    };

    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}
