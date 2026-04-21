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
    "Usage: emboss-rs newseq <identifier> <sequence> [--description <text>] [--molecule <dna|rna|protein|unknown>]\n\nCreate a new sequence record from inline residue content and emit it as FASTA. v1 requires an explicit identifier, accepts optional description text, and infers molecule kind conservatively when --molecule is omitted."
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

    if uppercase.is_empty() {
        return MoleculeKind::Unknown;
    }

    let has_u = uppercase.contains('U');
    let has_t = uppercase.contains('T');
    if has_u && !has_t && uppercase.chars().all(is_rna_inference_symbol) {
        return MoleculeKind::Rna;
    }
    if has_t && !has_u && uppercase.chars().all(is_dna_inference_symbol) {
        return MoleculeKind::Dna;
    }
    MoleculeKind::Unknown
}

fn is_dna_inference_symbol(symbol: char) -> bool {
    matches!(symbol, 'A' | 'C' | 'G' | 'T' | 'N' | '-' | '*')
}

fn is_rna_inference_symbol(symbol: char) -> bool {
    matches!(symbol, 'A' | 'C' | 'G' | 'U' | 'N' | '-' | '*')
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

    #[test]
    fn infers_rna_for_simple_uracil_sequence() {
        let outcome = run_newseq(NewseqParams {
            identifier: "rna-created".to_owned(),
            residues: "acgu".to_owned(),
            description: None,
            molecule: None,
        })
        .expect("sequence should build");

        assert_eq!(outcome.record.molecule(), MoleculeKind::Rna);
        assert_eq!(outcome.record.residues(), "ACGU");
    }

    #[test]
    fn falls_back_to_unknown_when_inference_is_not_cleanly_nucleotide() {
        let outcome = run_newseq(NewseqParams {
            identifier: "protein-like".to_owned(),
            residues: "MSTN".to_owned(),
            description: None,
            molecule: None,
        })
        .expect("sequence should build");

        assert_eq!(outcome.record.molecule(), MoleculeKind::Unknown);
        assert_eq!(outcome.record.alphabet().to_string(), "text alphabet");
        assert_eq!(outcome.record.residues(), "MSTN");
    }

    #[test]
    fn accepts_explicit_protein_molecule_and_description() {
        let outcome = run_newseq(NewseqParams {
            identifier: "prot1".to_owned(),
            residues: "m s t n".to_owned(),
            description: Some("created protein".to_owned()),
            molecule: Some(MoleculeKind::Protein),
        })
        .expect("protein should build");

        assert_eq!(outcome.record.molecule(), MoleculeKind::Protein);
        assert_eq!(outcome.record.residues(), "MSTN");
        assert_eq!(
            outcome.record.metadata().description.as_deref(),
            Some("created protein")
        );
    }

    #[test]
    fn rejects_invalid_residue_for_explicit_molecule() {
        let error = run_newseq(NewseqParams {
            identifier: "bad-dna".to_owned(),
            residues: "ACGTZ".to_owned(),
            description: None,
            molecule: Some(MoleculeKind::Dna),
        })
        .expect_err("dna sequence should reject invalid residue");

        assert!(error.to_string().contains("invalid residue"));
    }
}
