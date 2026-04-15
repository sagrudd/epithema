//! `extractalign` implementation.

use emboss_core::{Alignment, AlignmentRow};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use super::shared::{AlignmentInput, AlignmentToolError, load_alignment};

/// Typed parameters for `extractalign`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtractalignParams {
    /// Local alignment input path.
    pub input: AlignmentInput,
    /// Optional 1-based row ordinals to retain.
    pub row_ordinals: Vec<usize>,
    /// Optional row identifiers to retain.
    pub row_identifiers: Vec<String>,
    /// Optional 1-based inclusive column start.
    pub start: Option<usize>,
    /// Optional 1-based inclusive column end.
    pub end: Option<usize>,
}

/// Structured `extractalign` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtractalignOutcome {
    /// Source input.
    pub input: AlignmentInput,
    /// Selected 1-based row ordinals.
    pub row_ordinals: Vec<usize>,
    /// Selected row identifiers.
    pub row_identifiers: Vec<String>,
    /// Optional 1-based inclusive column start.
    pub start: Option<usize>,
    /// Optional 1-based inclusive column end.
    pub end: Option<usize>,
    /// Extracted alignment payload.
    pub alignment: Alignment,
}

/// Returns `extractalign` help text.
#[must_use]
pub fn extractalign_help() -> &'static str {
    "Usage: emboss-rs extractalign <input> [--row <ordinal>]... [--row-id <identifier>]... [--start <column>] [--end <column>]\n\nExtract rows by 1-based ordinal and/or row identifier plus an optional 1-based inclusive column range from a single aligned FASTA or Stockholm alignment. If no row selectors are supplied, all rows are retained. If no column range is supplied, all columns are retained."
}

/// Executes `extractalign`.
pub fn run_extractalign(
    params: ExtractalignParams,
) -> Result<ExtractalignOutcome, AlignmentToolError> {
    if params.start.is_some() ^ params.end.is_some() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "extractalign requires both --start and --end when slicing columns",
        )
        .with_code("tools.extractalign.columns.partial"));
    }

    let alignment = load_alignment(&params.input)?;
    let alignment = select_rows(&alignment, &params.row_ordinals, &params.row_identifiers)?;
    let alignment = if let (Some(start), Some(end)) = (params.start, params.end) {
        if start == 0 || end == 0 || start > end {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "extractalign column coordinates are 1-based inclusive with start <= end",
            )
            .with_code("tools.extractalign.columns.invalid"));
        }
        alignment.slice_columns(start - 1, end).map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.extractalign.columns.out_of_range")
        })?
    } else {
        alignment
    };

    Ok(ExtractalignOutcome {
        input: params.input,
        row_ordinals: params.row_ordinals,
        row_identifiers: params.row_identifiers,
        start: params.start,
        end: params.end,
        alignment,
    })
}

fn select_rows(
    alignment: &Alignment,
    row_ordinals: &[usize],
    row_identifiers: &[String],
) -> Result<Alignment, AlignmentToolError> {
    if row_ordinals.is_empty() && row_identifiers.is_empty() {
        return Ok(alignment.clone());
    }

    let mut selected = Vec::<AlignmentRow>::new();
    for (index, row) in alignment.rows().iter().enumerate() {
        let ordinal_selected = row_ordinals.iter().any(|ordinal| *ordinal == index + 1);
        let identifier_selected = row_identifiers
            .iter()
            .any(|identifier| identifier == row.identifier().accession());
        if ordinal_selected || identifier_selected {
            selected.push(row.clone());
        }
    }

    for ordinal in row_ordinals {
        if *ordinal == 0 || *ordinal > alignment.row_count() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("requested row ordinal {ordinal} is out of range"),
            )
            .with_code("tools.extractalign.rows.ordinal_out_of_range"));
        }
    }

    for identifier in row_identifiers {
        if alignment.row_by_identifier(identifier).is_none() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("requested row identifier '{identifier}' was not found"),
            )
            .with_code("tools.extractalign.rows.identifier_not_found"));
        }
    }

    Alignment::with_identifier(alignment.identifier().map(str::to_owned), selected).map_err(
        |error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.extractalign.rows.invalid")
        },
    )
}
