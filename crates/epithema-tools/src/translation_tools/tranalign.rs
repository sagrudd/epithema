//! `tranalign` implementation.

use epithema_core::{Alignment, AlignmentRow, SequenceRecord};
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::alignment_tools::{AlignmentInput, load_alignment};
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

use super::shared::{normalize_terminal_stop, translate_record_strict};

/// Typed parameters for `tranalign`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranalignParams {
    /// Local protein alignment input path.
    pub protein_alignment: AlignmentInput,
    /// Local nucleotide coding-sequence input path.
    pub nucleotide_input: SequenceInput,
}

/// Structured `tranalign` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranalignOutcome {
    /// Source protein alignment.
    pub protein_alignment: AlignmentInput,
    /// Source nucleotide input.
    pub nucleotide_input: SequenceInput,
    /// Derived codon alignment.
    pub alignment: Alignment,
}

/// Returns `tranalign` help text.
#[must_use]
pub fn tranalign_help() -> &'static str {
    "Usage: epithema tranalign <protein-alignment> <nucleotide-input>\n\nProject an aligned protein set onto matching coding-sequence codon alignments. v1 accepts one aligned FASTA or Stockholm protein alignment plus one nucleotide sequence file, pairs rows to coding sequences by exact identifier, requires strict frame-1 translation compatibility after terminal-stop normalization, preserves protein-alignment row order, and emits Stockholm."
}

/// Executes `tranalign`.
pub fn run_tranalign(params: TranalignParams) -> Result<TranalignOutcome, ToolExecutionError> {
    let protein_alignment = load_alignment(&params.protein_alignment)?;
    let nucleotide_records = load_sequence_records(&params.nucleotide_input)?;

    let rows = protein_alignment
        .rows()
        .iter()
        .map(|row| project_row(row, &nucleotide_records))
        .collect::<Result<Vec<_>, _>>()?;

    let alignment =
        Alignment::with_identifier(protein_alignment.identifier().map(str::to_owned), rows)
            .map_err(|error| {
                PlatformError::new(ErrorCategory::Validation, error.to_string())
                    .with_code("tools.tranalign.alignment.invalid")
            })?;

    Ok(TranalignOutcome {
        protein_alignment: params.protein_alignment,
        nucleotide_input: params.nucleotide_input,
        alignment,
    })
}

fn project_row(
    protein_row: &AlignmentRow,
    nucleotide_records: &[SequenceRecord],
) -> Result<AlignmentRow, ToolExecutionError> {
    if protein_row.molecule().is_nucleotide() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "tranalign expects a protein alignment row but '{}' was classified as {}",
                protein_row.identifier().accession(),
                protein_row.molecule()
            ),
        )
        .with_code("tools.tranalign.alignment.not_protein"));
    }

    let nucleotide = nucleotide_records
        .iter()
        .find(|record| record.identifier().accession() == protein_row.identifier().accession())
        .ok_or_else(|| {
            PlatformError::new(
                ErrorCategory::Validation,
                format!(
                    "tranalign could not find a matching coding sequence for alignment row '{}'",
                    protein_row.identifier().accession()
                ),
            )
            .with_code("tools.tranalign.sequence.identifier_not_found")
        })?;

    let translated = translate_record_strict("tranalign", nucleotide)?;
    let ungapped_protein = protein_row.ungapped();
    if normalize_terminal_stop(&translated) != normalize_terminal_stop(&ungapped_protein) {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "tranalign translation mismatch for '{}': translated '{}' versus aligned '{}'",
                protein_row.identifier().accession(),
                normalize_terminal_stop(&translated),
                normalize_terminal_stop(&ungapped_protein)
            ),
        )
        .with_code("tools.tranalign.translation.mismatch"));
    }

    let mut codons = nucleotide
        .residues()
        .as_bytes()
        .chunks(3)
        .map(|chunk| {
            std::str::from_utf8(chunk)
                .expect("sequence records are normalized ASCII residues")
                .to_owned()
        })
        .collect::<Vec<_>>();
    if translated.ends_with('*') && !ungapped_protein.ends_with('*') {
        let _ = codons.pop();
    }

    let mut codon_index = 0usize;
    let mut aligned = String::new();
    for symbol in protein_row.aligned().chars() {
        if symbol == '-' {
            aligned.push_str("---");
            continue;
        }

        let codon = codons.get(codon_index).ok_or_else(|| {
            PlatformError::new(
                ErrorCategory::Validation,
                format!(
                    "tranalign ran out of codons while projecting row '{}'",
                    protein_row.identifier().accession()
                ),
            )
            .with_code("tools.tranalign.codons.exhausted")
        })?;
        aligned.push_str(codon);
        codon_index += 1;
    }

    if codon_index != codons.len() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "tranalign left unused codons for '{}'",
                protein_row.identifier().accession()
            ),
        )
        .with_code("tools.tranalign.codons.unused"));
    }

    AlignmentRow::new(
        protein_row.identifier().clone(),
        nucleotide.molecule(),
        aligned,
    )
    .map(|row| row.with_metadata(nucleotide.metadata().clone()))
    .map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.tranalign.row.invalid")
    })
}

#[cfg(test)]
mod tests {
    use super::{TranalignParams, run_tranalign};
    use crate::alignment_tools::AlignmentInput;
    use crate::sequence_stream::SequenceInput;

    fn fixture(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/epithema-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn projects_a_protein_alignment_onto_codon_rows() {
        let outcome = run_tranalign(TranalignParams {
            protein_alignment: AlignmentInput::new(fixture("tranalign_protein_alignment.sto")),
            nucleotide_input: SequenceInput::new(fixture("checktrans_nucleotide.fasta")),
        })
        .expect("tranalign should execute");

        assert_eq!(outcome.alignment.row_count(), 2);
        assert_eq!(outcome.alignment.rows()[0].aligned(), "ATGGCT---");
        assert_eq!(outcome.alignment.rows()[1].aligned(), "---CTTTCT");
    }
}
