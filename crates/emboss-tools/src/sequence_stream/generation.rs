//! Shared deterministic sequence-generation helpers.

use emboss_core::{MoleculeKind, SequenceIdentifier, SequenceMetadata, SequenceRecord};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use super::ToolExecutionError;

const DNA_SYMBOLS: &[u8] = b"ACGT";
const RNA_SYMBOLS: &[u8] = b"ACGU";
const PROTEIN_SYMBOLS: &[u8] = b"ACDEFGHIKLMNPQRSTVWY";

pub(crate) fn generated_record_identifier(
    prefix: &str,
    count: usize,
    ordinal: usize,
) -> Result<SequenceIdentifier, ToolExecutionError> {
    let raw = if count == 1 {
        prefix.to_owned()
    } else {
        format!("{prefix}_{}", ordinal + 1)
    };
    SequenceIdentifier::new(raw).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.make.identifier.invalid")
    })
}
pub(crate) fn generated_sequence_record(
    identifier: SequenceIdentifier,
    molecule: MoleculeKind,
    residues: String,
    description: Option<&str>,
) -> Result<SequenceRecord, ToolExecutionError> {
    let mut record = SequenceRecord::new(identifier, molecule, residues).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.make.sequence.invalid")
    })?;
    if let Some(description) = description {
        record = record.with_metadata(SequenceMetadata::new().with_description(description));
    }
    Ok(record)
}

pub(crate) fn generate_dna_residues(length: usize, seed: u64) -> String {
    generate_symbols(length, seed, DNA_SYMBOLS)
}

pub(crate) fn generate_rna_residues(length: usize, seed: u64) -> String {
    generate_symbols(length, seed, RNA_SYMBOLS)
}

pub(crate) fn generate_protein_residues(length: usize, seed: u64) -> String {
    generate_symbols(length, seed, PROTEIN_SYMBOLS)
}

fn generate_symbols(length: usize, seed: u64, symbols: &[u8]) -> String {
    let mut rng = DeterministicRng::new(seed);
    let mut residues = String::with_capacity(length);
    for _ in 0..length {
        let index = rng.next_bounded(symbols.len());
        residues.push(symbols[index] as char);
    }
    residues
}

#[derive(Clone, Debug)]
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        let state = if seed == 0 {
            0xA5A5_A5A5_A5A5_A5A5
        } else {
            seed
        };
        Self { state }
    }

    fn next_u64(&mut self) -> u64 {
        let mut state = self.state;
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        self.state = state;
        state.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn next_bounded(&mut self, upper: usize) -> usize {
        if upper <= 1 {
            return 0;
        }
        (self.next_u64() % upper as u64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::{generate_dna_residues, generate_protein_residues, generate_rna_residues};

    #[test]
    fn generates_deterministic_nucleotides() {
        assert_eq!(generate_dna_residues(6, 7), generate_dna_residues(6, 7));
        assert_ne!(generate_dna_residues(6, 7), generate_dna_residues(6, 8));
    }

    #[test]
    fn generates_rna_from_rna_alphabet() {
        let residues = generate_rna_residues(32, 11);
        assert!(
            residues
                .chars()
                .all(|symbol| matches!(symbol, 'A' | 'C' | 'G' | 'U'))
        );
    }

    #[test]
    fn generates_protein_from_canonical_alphabet() {
        let residues = generate_protein_residues(32, 11);
        assert!(
            residues
                .chars()
                .all(|symbol| "ACDEFGHIKLMNPQRSTVWY".contains(symbol))
        );
    }
}
