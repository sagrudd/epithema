//! `checktrans` implementation.

use emboss_core::{SequenceRecord, translate_dna_strict};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `checktrans`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChecktransParams {
    /// Local nucleotide coding-sequence input path.
    pub nucleotide_input: SequenceInput,
    /// Local protein input path.
    pub protein_input: SequenceInput,
}

/// One paired translation-check result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChecktransCase {
    /// Nucleotide record identifier.
    pub nucleotide_id: String,
    /// Protein record identifier.
    pub protein_id: String,
    /// Whether translation matched after terminal-stop normalization.
    pub matches: bool,
    /// Translated protein sequence.
    pub translated_protein: String,
    /// Expected protein sequence.
    pub expected_protein: String,
    /// Whether the translated sequence ended with a terminal stop.
    pub translated_terminal_stop: bool,
    /// Whether the expected protein ended with a terminal stop symbol.
    pub expected_terminal_stop: bool,
    /// Stable detail message.
    pub detail: String,
}

/// Structured `checktrans` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChecktransOutcome {
    /// Source nucleotide input.
    pub nucleotide_input: SequenceInput,
    /// Source protein input.
    pub protein_input: SequenceInput,
    /// Paired comparison cases.
    pub cases: Vec<ChecktransCase>,
}

/// Returns `checktrans` help text.
#[must_use]
pub fn checktrans_help() -> &'static str {
    "Usage: emboss-rs checktrans <nucleotide-input> <protein-input>\n\nStrictly translate frame-1 DNA coding sequences with the standard genetic code and compare them against expected protein records. Inputs must contain the same number of records and are paired by order. A single trailing stop is normalized on both sides for comparison."
}

/// Executes `checktrans`.
pub fn run_checktrans(params: ChecktransParams) -> Result<ChecktransOutcome, ToolExecutionError> {
    let nucleotide_records = load_sequence_records(&params.nucleotide_input)?;
    let protein_records = load_sequence_records(&params.protein_input)?;

    if nucleotide_records.len() != protein_records.len() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "checktrans requires nucleotide and protein inputs to contain the same number of records",
        )
        .with_code("tools.checktrans.records.count_mismatch"));
    }

    let cases = nucleotide_records
        .into_iter()
        .zip(protein_records)
        .map(|(nucleotide, protein)| compare_pair(nucleotide, protein))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ChecktransOutcome {
        nucleotide_input: params.nucleotide_input,
        protein_input: params.protein_input,
        cases,
    })
}

fn compare_pair(
    nucleotide: SequenceRecord,
    protein: SequenceRecord,
) -> Result<ChecktransCase, ToolExecutionError> {
    if !nucleotide.molecule().is_nucleotide() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "checktrans expects nucleotide coding input but '{}' was classified as {}",
                nucleotide.identifier().accession(),
                nucleotide.molecule()
            ),
        )
        .with_code("tools.checktrans.nucleotide.not_nucleotide"));
    }
    if protein.molecule().is_nucleotide() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "checktrans expects protein input but '{}' was classified as {}",
                protein.identifier().accession(),
                protein.molecule()
            ),
        )
        .with_code("tools.checktrans.protein.not_protein"));
    }

    let translated = translate_dna_strict(nucleotide.residues()).map_err(|error| {
        let code = match error {
            emboss_core::TranslationError::NonCodingLength { .. } => {
                "tools.checktrans.nucleotide.non_coding_length"
            }
            emboss_core::TranslationError::InvalidCodon(_) => "tools.checktrans.codon.invalid",
            emboss_core::TranslationError::UnsupportedResidue(_) => {
                "tools.checktrans.translation.unsupported"
            }
        };
        PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
    })?;

    let translated_terminal_stop = translated.ends_with('*');
    let expected_terminal_stop = protein.residues().ends_with('*');
    let normalized_translated = normalize_terminal_stop(&translated);
    let normalized_expected = normalize_terminal_stop(protein.residues());
    let matches = normalized_translated == normalized_expected;

    let detail = if matches {
        "translated protein matches expected sequence".to_owned()
    } else {
        format!(
            "translation mismatch: translated '{}' versus expected '{}'",
            normalized_translated, normalized_expected
        )
    };

    Ok(ChecktransCase {
        nucleotide_id: nucleotide.identifier().accession().to_owned(),
        protein_id: protein.identifier().accession().to_owned(),
        matches,
        translated_protein: translated,
        expected_protein: protein.residues().to_owned(),
        translated_terminal_stop,
        expected_terminal_stop,
        detail,
    })
}

fn normalize_terminal_stop(protein: &str) -> String {
    protein.strip_suffix('*').unwrap_or(protein).to_owned()
}
