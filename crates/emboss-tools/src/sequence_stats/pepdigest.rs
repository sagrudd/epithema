//! `pepdigest` implementation.

use std::str::FromStr;

use emboss_core::{DigestProtease, digest_protein_sequence};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stats::protein_support::validate_protein_record;
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Supported proteases or reagents for `pepdigest` v1.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PepdigestProtease {
    /// Trypsin cleavage after `K` or `R` unless followed by `P`.
    Trypsin,
    /// Lys-C cleavage after `K`.
    LysC,
    /// Arg-C cleavage after `R`.
    ArgC,
    /// Cyanogen bromide cleavage after `M`.
    CnBr,
}

impl PepdigestProtease {
    /// Stable lower-case label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Trypsin => "trypsin",
            Self::LysC => "lys-c",
            Self::ArgC => "arg-c",
            Self::CnBr => "cnbr",
        }
    }

    #[must_use]
    const fn to_core(self) -> DigestProtease {
        match self {
            Self::Trypsin => DigestProtease::Trypsin,
            Self::LysC => DigestProtease::LysC,
            Self::ArgC => DigestProtease::ArgC,
            Self::CnBr => DigestProtease::CnBr,
        }
    }
}

impl FromStr for PepdigestProtease {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "trypsin" => Ok(Self::Trypsin),
            "lys-c" | "lysc" => Ok(Self::LysC),
            "arg-c" | "argc" => Ok(Self::ArgC),
            "cnbr" => Ok(Self::CnBr),
            _ => Err(()),
        }
    }
}

/// Typed parameters for `pepdigest`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PepdigestParams {
    /// Local protein input path.
    pub input: SequenceInput,
    /// Selected v1 protease or reagent.
    pub protease: PepdigestProtease,
}

/// One peptide row emitted by `pepdigest`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PepdigestPeptide {
    /// Source record identifier.
    pub record_id: String,
    /// Stable protease label.
    pub protease: PepdigestProtease,
    /// One-based peptide ordinal within the digested record.
    pub peptide_index: usize,
    /// One-based inclusive peptide start coordinate.
    pub start: usize,
    /// One-based inclusive peptide end coordinate.
    pub end: usize,
    /// One-based cleavage coordinate terminating the peptide, if present.
    pub cleavage_after: Option<usize>,
    /// Digested peptide sequence.
    pub sequence: String,
}

/// Structured `pepdigest` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PepdigestOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Selected v1 protease.
    pub protease: PepdigestProtease,
    /// Digested peptide rows in stable record then peptide order.
    pub peptides: Vec<PepdigestPeptide>,
}

/// Returns `pepdigest` help text.
#[must_use]
pub fn pepdigest_help() -> &'static str {
    "Usage: emboss-rs pepdigest <protein-input> [--protease <trypsin|lys-c|arg-c|cnbr>]\n\nReport deterministic full-digest peptide fragments for each protein record. The v1 implementation supports a small typed protease set, reports one row per resulting peptide with source coordinates, applies exact cleavage rules with trypsin proline blocking, and rejects unsupported ambiguous residues."
}

/// Executes `pepdigest`.
pub fn run_pepdigest(params: PepdigestParams) -> Result<PepdigestOutcome, ToolExecutionError> {
    let peptides = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| -> Result<Vec<PepdigestPeptide>, ToolExecutionError> {
            validate_protein_record("pepdigest", &record)?;
            let digested =
                digest_protein_sequence(record.residues(), params.protease.to_core()).map_err(
                    |error| {
                        PlatformError::new(ErrorCategory::Validation, error.to_string())
                            .with_code("tools.pepdigest.residue.unsupported")
                    },
                )?;

            Ok(digested
                .into_iter()
                .map(|peptide| PepdigestPeptide {
                    record_id: record.identifier().accession().to_owned(),
                    protease: params.protease,
                    peptide_index: peptide.ordinal,
                    start: peptide.start,
                    end: peptide.end,
                    cleavage_after: peptide.cleavage_after,
                    sequence: peptide.sequence,
                })
                .collect())
        })
        .collect::<Result<Vec<Vec<_>>, _>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(PepdigestOutcome {
        input: params.input,
        protease: params.protease,
        peptides,
    })
}

#[cfg(test)]
mod tests {
    use super::{PepdigestParams, PepdigestProtease, run_pepdigest};
    use crate::sequence_stream::SequenceInput;

    fn fixture(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/emboss-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn reports_tryptic_peptides_in_source_order() {
        let outcome = run_pepdigest(PepdigestParams {
            input: SequenceInput::new(fixture("pepdigest_records.fasta")),
            protease: PepdigestProtease::Trypsin,
        })
        .expect("pepdigest should succeed");

        assert_eq!(outcome.peptides.len(), 3);
        assert_eq!(outcome.peptides[0].sequence, "AK");
        assert_eq!(outcome.peptides[1].sequence, "RPQMK");
        assert_eq!(outcome.peptides[2].sequence, "MAMK");
    }

    #[test]
    fn supports_cnbr_reagent_mode() {
        let outcome = run_pepdigest(PepdigestParams {
            input: SequenceInput::new(fixture("pepdigest_records.fasta")),
            protease: PepdigestProtease::CnBr,
        })
        .expect("pepdigest should succeed");

        assert_eq!(outcome.peptides.len(), 5);
        assert_eq!(outcome.peptides[0].sequence, "AKRPQM");
        assert_eq!(outcome.peptides[1].sequence, "K");
        assert_eq!(outcome.peptides[2].sequence, "M");
        assert_eq!(outcome.peptides[3].sequence, "AM");
        assert_eq!(outcome.peptides[4].sequence, "K");
    }

    #[test]
    fn rejects_nucleotide_input() {
        let error = run_pepdigest(PepdigestParams {
            input: SequenceInput::new(fixture("nucleotide_pattern_records.fasta")),
            protease: PepdigestProtease::Trypsin,
        })
        .expect_err("nucleotide input should fail");

        assert!(error.to_string().contains("expects protein input"));
    }
}
