//! `newseq` implementation.

use emboss_core::{MoleculeKind, SequenceIdentifier, SequenceMetadata, SequenceRecord};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use super::ToolExecutionError;

/// Typed parameters for `newseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewseqParams {
    /// Stable sequence identifier.
    pub identifier: String,
    /// Residue content to normalize and validate.
    pub residues: String,
    /// Optional free-text description.
    pub description: Option<String>,
    /// Optional explicit molecule hint.
    pub molecule: Option<MoleculeKind>,
}

/// Structured `newseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewseqOutcome {
    /// Created sequence record.
    pub record: SequenceRecord,
}

/// Returns the `newseq` help text.
#[must_use]
pub fn newseq_help() -> &'static str {
    "Usage: emboss-rs newseq <identifier> <sequence> [--description <text>] [--molecule <dna|rna|protein|unknown>]\n\nCreate a new sequence record from inline residue content and emit it as FASTA."
}

/// Executes `newseq`.
pub fn run_newseq(params: NewseqParams) -> Result<NewseqOutcome, ToolExecutionError> {
    let identifier = SequenceIdentifier::new(params.identifier).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.newseq.identifier.invalid")
    })?;

    let molecule = params
        .molecule
        .unwrap_or_else(|| infer_molecule_kind(&params.residues));
    let mut record =
        SequenceRecord::new(identifier, molecule, params.residues).map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.newseq.sequence.invalid")
        })?;

    if let Some(description) = params.description {
        record = record.with_metadata(SequenceMetadata::new().with_description(description));
    }

    Ok(NewseqOutcome { record })
}

fn infer_molecule_kind(residues: &str) -> MoleculeKind {
    let uppercase: String = residues
        .chars()
        .filter(|symbol| !symbol.is_whitespace())
        .map(|symbol| symbol.to_ascii_uppercase())
        .collect();

    let has_u = uppercase.contains('U');
    let has_t = uppercase.contains('T');
    if has_u && !has_t {
        return MoleculeKind::Rna;
    }
    if has_t && !has_u {
        return MoleculeKind::Dna;
    }

    if uppercase
        .chars()
        .all(|symbol| matches!(symbol, 'A' | 'C' | 'G' | 'T' | 'N' | '-' | '*'))
    {
        return MoleculeKind::Dna;
    }
    if uppercase
        .chars()
        .all(|symbol| matches!(symbol, 'A' | 'C' | 'G' | 'U' | 'N' | '-' | '*'))
    {
        return MoleculeKind::Rna;
    }

    MoleculeKind::Protein
}

#[cfg(test)]
mod tests {
    use emboss_core::MoleculeKind;

    use super::{NewseqParams, run_newseq};

    #[test]
    fn infers_dna_for_simple_residues() {
        let outcome = run_newseq(NewseqParams {
            identifier: "created".to_owned(),
            residues: "acgt".to_owned(),
            description: None,
            molecule: None,
        })
        .expect("sequence should build");

        assert_eq!(outcome.record.molecule(), MoleculeKind::Dna);
        assert_eq!(outcome.record.residues(), "ACGT");
    }
}
