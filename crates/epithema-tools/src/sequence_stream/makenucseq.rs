//! `makenucseq` implementation.

use epithema_core::{MoleculeKind, SequenceRecord};
use epithema_diagnostics::{ErrorCategory, PlatformError};

use super::ToolExecutionError;
use super::generation::{
    generate_dna_residues, generate_rna_residues, generated_record_identifier,
    generated_sequence_record,
};

/// Typed parameters for `makenucseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MakenucseqParams {
    /// Identifier or identifier prefix for generated records.
    pub identifier_prefix: String,
    /// Residue length for every generated record.
    pub length: usize,
    /// Number of records to generate.
    pub count: usize,
    /// Deterministic seed.
    pub seed: u64,
    /// Nucleotide molecule kind.
    pub molecule: MoleculeKind,
    /// Optional shared description.
    pub description: Option<String>,
}
/// Structured `makenucseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MakenucseqOutcome {
    /// Prefix used for generated records.
    pub identifier_prefix: String,
    /// Residue length for every generated record.
    pub length: usize,
    /// Number of records generated.
    pub count: usize,
    /// Deterministic seed used.
    pub seed: u64,
    /// Chosen nucleotide molecule kind.
    pub molecule: MoleculeKind,
    /// Generated records in stable order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `makenucseq` help text.
#[must_use]
pub fn makenucseq_help() -> &'static str {
    "Usage: epithema makenucseq <identifier-prefix> <length> [--count <value>] [--seed <value>] [--molecule <dna|rna>] [--description <text>]\n\nCreate one or more deterministic random nucleotide sequence records. v1 generates exact-length canonical DNA or RNA records from a documented seed."
}

/// Executes `makenucseq`.
pub fn run_makenucseq(params: MakenucseqParams) -> Result<MakenucseqOutcome, ToolExecutionError> {
    if params.length == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "makenucseq requires length >= 1",
        )
        .with_code("tools.makenucseq.length.invalid"));
    }
    if params.count == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "makenucseq requires count >= 1",
        )
        .with_code("tools.makenucseq.count.invalid"));
    }
    if !matches!(params.molecule, MoleculeKind::Dna | MoleculeKind::Rna) {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "makenucseq requires molecule dna or rna",
        )
        .with_code("tools.makenucseq.molecule.invalid"));
    }

    let mut records = Vec::with_capacity(params.count);
    for ordinal in 0..params.count {
        let identifier =
            generated_record_identifier(&params.identifier_prefix, params.count, ordinal)?;
        let seed = params.seed ^ ((ordinal as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let residues = match params.molecule {
            MoleculeKind::Dna => generate_dna_residues(params.length, seed),
            MoleculeKind::Rna => generate_rna_residues(params.length, seed),
            _ => unreachable!("validated above"),
        };
        records.push(generated_sequence_record(
            identifier,
            params.molecule,
            residues,
            params.description.as_deref(),
        )?);
    }

    Ok(MakenucseqOutcome {
        identifier_prefix: params.identifier_prefix,
        length: params.length,
        count: params.count,
        seed: params.seed,
        molecule: params.molecule,
        records,
    })
}

#[cfg(test)]
mod tests {
    use epithema_core::MoleculeKind;

    use super::{MakenucseqParams, run_makenucseq};

    #[test]
    fn generates_deterministic_dna_records() {
        let first = run_makenucseq(MakenucseqParams {
            identifier_prefix: "dna".to_owned(),
            length: 8,
            count: 2,
            seed: 7,
            molecule: MoleculeKind::Dna,
            description: None,
        })
        .expect("makenucseq should succeed");
        let second = run_makenucseq(MakenucseqParams {
            identifier_prefix: "dna".to_owned(),
            length: 8,
            count: 2,
            seed: 7,
            molecule: MoleculeKind::Dna,
            description: None,
        })
        .expect("makenucseq should succeed");

        assert_eq!(first.records, second.records);
        assert_eq!(first.records[0].identifier().accession(), "dna_1");
        assert_eq!(first.records[1].identifier().accession(), "dna_2");
    }

    #[test]
    fn generates_single_rna_record_with_exact_identifier() {
        let outcome = run_makenucseq(MakenucseqParams {
            identifier_prefix: "rna-made".to_owned(),
            length: 6,
            count: 1,
            seed: 3,
            molecule: MoleculeKind::Rna,
            description: Some("generated".to_owned()),
        })
        .expect("makenucseq should succeed");

        assert_eq!(outcome.records[0].identifier().accession(), "rna-made");
        assert_eq!(outcome.records[0].molecule(), MoleculeKind::Rna);
        assert_eq!(
            outcome.records[0].metadata().description.as_deref(),
            Some("generated")
        );
    }
}
