//! `compseq` implementation.

use emboss_core::{MoleculeKind, ResidueComposition};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `compseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
}

/// One per-record composition entry.
#[derive(Clone, Debug, PartialEq)]
pub struct CompseqRecord {
    /// Record identifier.
    pub record_id: String,
    /// Molecule kind.
    pub molecule: MoleculeKind,
    /// Raw sequence length.
    pub sequence_length: usize,
    /// Composition summary excluding gaps.
    pub composition: ResidueComposition,
}

/// Structured `compseq` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct CompseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Per-record composition summaries.
    pub records: Vec<CompseqRecord>,
    /// Aggregate composition across all records.
    pub aggregate: ResidueComposition,
}

/// Returns `compseq` help text.
#[must_use]
pub fn compseq_help() -> &'static str {
    "Usage: emboss-rs compseq <input>\n\nReport deterministic residue composition counts and frequencies for nucleotide or protein sequence records. The v1 implementation reports both per-record rows and an aggregate summary across all records. Gap symbols '-' are ignored; all other normalized residue symbols, including ambiguity and stop symbols, are counted."
}

/// Executes `compseq`.
pub fn run_compseq(params: CompseqParams) -> Result<CompseqOutcome, ToolExecutionError> {
    let mut aggregate = ResidueComposition::default();
    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| {
            let composition = ResidueComposition::from_sequence(record.residues());
            aggregate.merge(&composition);
            CompseqRecord {
                record_id: record.identifier().accession().to_owned(),
                molecule: record.molecule(),
                sequence_length: record.len(),
                composition,
            }
        })
        .collect();

    Ok(CompseqOutcome {
        input: params.input,
        records,
        aggregate,
    })
}
