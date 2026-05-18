use emboss_core::{FeatureOperationError, Interval, MoleculeKind, SequenceRecord};
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

pub fn contiguous_matching_intervals(
    record: &SequenceRecord,
    predicate: impl Fn(char) -> bool,
) -> Vec<Interval> {
    let mut intervals = Vec::new();
    let mut run_start = None;

    for (index, symbol) in record.residues().chars().enumerate() {
        if predicate(symbol) {
            run_start.get_or_insert(index);
            continue;
        }

        if let Some(start) = run_start.take() {
            intervals.push(
                Interval::new(start, index)
                    .expect("contiguous matching runs should always yield valid intervals"),
            );
        }
    }

    if let Some(start) = run_start {
        intervals.push(
            Interval::new(start, record.len())
                .expect("terminal matching run should always yield a valid interval"),
        );
    }

    intervals
}
