//! Shared helpers for translation-adjacent tools.

use emboss_core::{
    Alphabet, MoleculeKind, SequenceIdentifier, SequenceMetadata, SequenceRecord,
    TranslationError, translate_dna_frame, translate_dna_strict,
};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::ToolExecutionError;

/// Forward reading-frame selection for translation-adjacent methods.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TranslationFrameSelection {
    /// Translate or render frame 1 only.
    Frame1,
    /// Translate or render frame 2 only.
    Frame2,
    /// Translate or render frame 3 only.
    Frame3,
    /// Translate all forward frames that produce at least one complete codon.
    AllForward,
}

impl TranslationFrameSelection {
    /// Returns the user-facing one-based frame ordinals covered by the selection.
    #[must_use]
    pub const fn ordinals(self) -> &'static [usize] {
        match self {
            Self::Frame1 => &[1],
            Self::Frame2 => &[2],
            Self::Frame3 => &[3],
            Self::AllForward => &[1, 2, 3],
        }
    }

    /// Returns the zero-based frame offsets covered by the selection.
    #[must_use]
    pub const fn offsets(self) -> &'static [usize] {
        match self {
            Self::Frame1 => &[0],
            Self::Frame2 => &[1],
            Self::Frame3 => &[2],
            Self::AllForward => &[0, 1, 2],
        }
    }
}

/// Validates that a sequence record is usable as nucleotide input.
pub fn validate_nucleotide_record(
    tool: &str,
    record: &SequenceRecord,
) -> Result<(), ToolExecutionError> {
    if record.molecule().is_protein()
        || (!record.molecule().is_nucleotide() && !looks_like_nucleotide_record(record))
    {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} expects nucleotide input but '{}' was classified as {}",
                record.identifier().accession(),
                record.molecule()
            ),
        )
        .with_code(format!("tools.{tool}.input.not_nucleotide")));
    }

    Ok(())
}

/// Maps a translation helper error into a governed tool error.
pub fn map_translation_error(tool: &str, error: TranslationError) -> ToolExecutionError {
    let code = match error {
        TranslationError::NonCodingLength { .. } => {
            format!("tools.{tool}.nucleotide.non_coding_length")
        }
        TranslationError::InvalidCodon(_) => format!("tools.{tool}.codon.invalid"),
        TranslationError::InvalidFrameOffset { .. } => {
            format!("tools.{tool}.translation.frame_invalid")
        }
        TranslationError::UnsupportedResidue(_) => {
            format!("tools.{tool}.translation.unsupported")
        }
    };

    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}

/// Translates one validated nucleotide record in a selected forward frame.
pub fn translate_record_frame(
    tool: &str,
    record: &SequenceRecord,
    frame_offset: usize,
) -> Result<String, ToolExecutionError> {
    validate_nucleotide_record(tool, record)?;
    translate_dna_frame(&dna_equivalent_residues(record.residues()), frame_offset)
        .map_err(|error| map_translation_error(tool, error))
}

/// Strictly translates one validated complete coding sequence.
pub fn translate_record_strict(
    tool: &str,
    record: &SequenceRecord,
) -> Result<String, ToolExecutionError> {
    validate_nucleotide_record(tool, record)?;
    translate_dna_strict(&dna_equivalent_residues(record.residues()))
        .map_err(|error| map_translation_error(tool, error))
}

/// Returns a nucleotide string normalized to DNA codon space.
#[must_use]
pub fn dna_equivalent_residues(residues: &str) -> String {
    residues
        .chars()
        .map(|symbol| match symbol {
            'U' => 'T',
            other => other,
        })
        .collect()
}

/// Appends a descriptive suffix to copied metadata.
#[must_use]
pub fn derived_metadata(metadata: &SequenceMetadata, suffix: &str) -> SequenceMetadata {
    let mut derived = metadata.clone();
    derived.description = Some(match metadata.description.as_deref() {
        Some(description) => format!("{description} ({suffix})"),
        None => suffix.to_owned(),
    });
    derived
}

/// Builds a stable derived identifier by suffixing the accession.
pub fn identifier_with_suffix(
    identifier: &SequenceIdentifier,
    suffix: &str,
) -> Result<SequenceIdentifier, ToolExecutionError> {
    let derived = SequenceIdentifier::new(format!("{}.{}", identifier.accession(), suffix))
        .map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.translation.identifier.invalid")
        })?;

    Ok(match identifier.display_name() {
        Some(display_name) => derived.with_display_name(format!("{display_name}.{suffix}")),
        None => derived,
    })
}

/// Removes a single trailing stop symbol for comparison purposes.
#[must_use]
pub fn normalize_terminal_stop(protein: &str) -> String {
    protein.strip_suffix('*').unwrap_or(protein).to_owned()
}

fn looks_like_nucleotide_record(record: &SequenceRecord) -> bool {
    Alphabet::Dna
        .validate(MoleculeKind::Dna, record.residues())
        .is_ok()
        || Alphabet::Rna
            .validate(MoleculeKind::Rna, record.residues())
            .is_ok()
}

#[cfg(test)]
mod tests {
    use super::{TranslationFrameSelection, dna_equivalent_residues, normalize_terminal_stop};

    #[test]
    fn all_forward_reports_three_frames() {
        assert_eq!(TranslationFrameSelection::AllForward.ordinals(), &[1, 2, 3]);
        assert_eq!(TranslationFrameSelection::AllForward.offsets(), &[0, 1, 2]);
    }

    #[test]
    fn converts_uracil_to_thymine_for_translation_space() {
        assert_eq!(dna_equivalent_residues("AUGCUU"), "ATGCTT");
    }

    #[test]
    fn strips_one_terminal_stop() {
        assert_eq!(normalize_terminal_stop("MA*"), "MA");
        assert_eq!(normalize_terminal_stop("MA"), "MA");
    }
}
