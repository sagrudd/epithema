//! `infoalign` implementation.

use emboss_core::Alignment;

use super::shared::{AlignmentInput, AlignmentToolError, load_alignment};

/// Typed parameters for `infoalign`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InfoalignParams {
    /// Local alignment input path.
    pub input: AlignmentInput,
}

/// Row-level summary for `infoalign`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InfoalignRow {
    /// Stable row identifier.
    pub identifier: String,
    /// Row ordinal in 1-based user-facing coordinates.
    pub ordinal: usize,
    /// Ungapped residue count.
    pub ungapped_length: usize,
    /// Gap count.
    pub gap_count: usize,
}

/// Structured `infoalign` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InfoalignOutcome {
    /// Source input.
    pub input: AlignmentInput,
    /// Alignment identifier when available.
    pub alignment_identifier: Option<String>,
    /// Row count.
    pub row_count: usize,
    /// Alignment column count.
    pub column_count: usize,
    /// Whether the alignment is pairwise or multiple.
    pub classification: String,
    /// Row-level summaries in input order.
    pub rows: Vec<InfoalignRow>,
}

/// Returns `infoalign` help text.
#[must_use]
pub fn infoalign_help() -> &'static str {
    "Usage: emboss-rs infoalign <input>\n\nReport summary statistics for a single aligned FASTA or Stockholm alignment, including row count, column count, pairwise-versus-multiple classification, and per-row ungapped and gap counts."
}

/// Executes `infoalign`.
pub fn run_infoalign(params: InfoalignParams) -> Result<InfoalignOutcome, AlignmentToolError> {
    let alignment = load_alignment(&params.input)?;
    Ok(build_infoalign_outcome(params.input, &alignment))
}

fn build_infoalign_outcome(input: AlignmentInput, alignment: &Alignment) -> InfoalignOutcome {
    InfoalignOutcome {
        input,
        alignment_identifier: alignment.identifier().map(ToOwned::to_owned),
        row_count: alignment.row_count(),
        column_count: alignment.column_count(),
        classification: if alignment.is_pairwise() {
            "pairwise".to_owned()
        } else {
            "multiple".to_owned()
        },
        rows: alignment
            .rows()
            .iter()
            .enumerate()
            .map(|(index, row)| InfoalignRow {
                identifier: row.identifier().accession().to_owned(),
                ordinal: index + 1,
                ungapped_length: row.ungapped_len(),
                gap_count: row.gap_count(),
            })
            .collect(),
    }
}
