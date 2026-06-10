//! `makeprotseq` implementation.

use epithema_core::{MoleculeKind, SequenceRecord};
use epithema_diagnostics::{ErrorCategory, PlatformError};

use super::ToolExecutionError;
use super::generation::{
    generate_protein_residues, generated_record_identifier, generated_sequence_record,
};

/// Typed parameters for `makeprotseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MakeprotseqParams {
    /// Identifier or identifier prefix for generated records.
    pub identifier_prefix: String,
    /// Residue length for every generated record.
    pub length: usize,
    /// Number of records to generate.
    pub count: usize,
    /// Deterministic seed.
    pub seed: u64,
    /// Optional shared description.
    pub description: Option<String>,
}

/// Structured `makeprotseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MakeprotseqOutcome {
    /// Prefix used for generated records.
    pub identifier_prefix: String,
    /// Residue length for every generated record.
    pub length: usize,
    /// Number of records generated.
    pub count: usize,
    /// Deterministic seed used.
    pub seed: u64,
    /// Generated records in stable order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `makeprotseq` help text.
#[must_use]
pub fn makeprotseq_help() -> &'static str {
    "Usage: epithema makeprotseq <identifier-prefix> <length> [--count <value>] [--seed <value>] [--description <text>]\n\nCreate one or more deterministic random protein sequence records. v1 generates exact-length canonical amino-acid sequences from a documented seed."
}

/// Executes `makeprotseq`.
pub fn run_makeprotseq(
    params: MakeprotseqParams,
) -> Result<MakeprotseqOutcome, ToolExecutionError> {
    if params.length == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "makeprotseq requires length >= 1",
        )
        .with_code("tools.makeprotseq.length.invalid"));
    }
    if params.count == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "makeprotseq requires count >= 1",
        )
        .with_code("tools.makeprotseq.count.invalid"));
    }

    let mut records = Vec::with_capacity(params.count);
    for ordinal in 0..params.count {
        let identifier =
            generated_record_identifier(&params.identifier_prefix, params.count, ordinal)?;
        let seed = params.seed ^ ((ordinal as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let residues = generate_protein_residues(params.length, seed);
        records.push(generated_sequence_record(
            identifier,
            MoleculeKind::Protein,
            residues,
            params.description.as_deref(),
        )?);
    }

    Ok(MakeprotseqOutcome {
        identifier_prefix: params.identifier_prefix,
        length: params.length,
        count: params.count,
        seed: params.seed,
        records,
    })
}

#[cfg(test)]
mod tests {
    use epithema_core::MoleculeKind;

    use super::{MakeprotseqParams, run_makeprotseq};

    #[test]
    fn generates_deterministic_protein_records() {
        let first = run_makeprotseq(MakeprotseqParams {
            identifier_prefix: "prot".to_owned(),
            length: 8,
            count: 2,
            seed: 7,
            description: None,
        })
        .expect("makeprotseq should succeed");
        let second = run_makeprotseq(MakeprotseqParams {
            identifier_prefix: "prot".to_owned(),
            length: 8,
            count: 2,
            seed: 7,
            description: None,
        })
        .expect("makeprotseq should succeed");

        assert_eq!(first.records, second.records);
        assert_eq!(first.records[0].molecule(), MoleculeKind::Protein);
        assert_eq!(first.records[1].identifier().accession(), "prot_2");
    }
}
