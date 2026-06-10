//! `eprimer3` implementation.

use epithema_core::{Eprimer3Parameters, eprimer3_profile};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `eprimer3`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Eprimer3Params {
    /// Local nucleotide input path.
    pub input: SequenceInput,
}

/// Stable summary row for one bounded `eprimer3` candidate.
#[derive(Clone, Debug, PartialEq)]
pub struct Eprimer3Row {
    /// Stable target record identifier.
    pub record_id: String,
    /// Stable candidate identifier.
    pub candidate_id: String,
    /// Deterministic strand/orientation label.
    pub strand: String,
    /// One-based inclusive candidate start.
    pub oligo_start: usize,
    /// One-based inclusive candidate end.
    pub oligo_end: usize,
    /// Candidate length in residues.
    pub oligo_length: usize,
    /// Normalized oligo sequence.
    pub oligo_sequence: String,
    /// Canonical-symbol count in the underlying genomic window.
    pub canonical_symbols: usize,
    /// Ambiguous-symbol count in the underlying genomic window.
    pub ambiguous_symbols: usize,
    /// GC fraction across canonical symbols.
    pub gc_fraction: f64,
    /// Conservative melting estimate in Celsius.
    pub tm_celsius: f64,
    /// 3'-terminal GC count over the trailing triplet or shorter suffix.
    pub three_prime_gc_count: usize,
}

/// Structured `eprimer3` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct Eprimer3Outcome {
    /// Source nucleotide input.
    pub input: SequenceInput,
    /// Stable bounded design parameters.
    pub parameters: Eprimer3Parameters,
    /// Count of target records analyzed.
    pub record_count: usize,
    /// Stable ordered candidate rows.
    pub rows: Vec<Eprimer3Row>,
}

/// Returns the bounded `eprimer3` help text.
#[must_use]
pub fn eprimer3_help() -> &'static str {
    "Usage: epithema eprimer3 <nucleotide-input>\n\nReport deterministic bounded primer-and-oligo design candidates against one local nucleotide input. The bounded v1 seam uses local default design parameters only and emits table-first candidate rows rather than a generalized assay-ranking or thermodynamic optimization workflow."
}

/// Executes bounded `eprimer3`.
pub fn run_eprimer3(params: Eprimer3Params) -> Result<Eprimer3Outcome, ToolExecutionError> {
    let parameters = Eprimer3Parameters::default();
    let mut rows = Vec::new();
    let mut record_count = 0usize;

    for record in load_sequence_records(&params.input)? {
        record_count += 1;
        let profile = eprimer3_profile(&record, parameters).map_err(|error| {
            epithema_diagnostics::PlatformError::new(
                epithema_diagnostics::ErrorCategory::Validation,
                error.to_string(),
            )
            .with_code("tools.eprimer3.profile.invalid")
        })?;

        rows.extend(profile.candidates.into_iter().map(|candidate| Eprimer3Row {
            record_id: profile.identifier.clone(),
            candidate_id: candidate.candidate_id,
            strand: strand_label(candidate.strand),
            oligo_start: candidate.oligo_start,
            oligo_end: candidate.oligo_end,
            oligo_length: candidate.oligo_length,
            oligo_sequence: candidate.oligo_sequence,
            canonical_symbols: candidate.canonical_symbols,
            ambiguous_symbols: candidate.ambiguous_symbols,
            gc_fraction: candidate.gc_fraction,
            tm_celsius: candidate.tm_celsius,
            three_prime_gc_count: candidate.three_prime_gc_count,
        }));
    }

    Ok(Eprimer3Outcome {
        input: params.input,
        parameters,
        record_count,
        rows,
    })
}

fn strand_label(strand: epithema_core::Strand) -> String {
    match strand {
        epithema_core::Strand::Forward => "forward".to_owned(),
        epithema_core::Strand::Reverse => "reverse".to_owned(),
        epithema_core::Strand::Unstranded => "unstranded".to_owned(),
        epithema_core::Strand::Unknown => "unknown".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::{Eprimer3Params, run_eprimer3};
    use crate::sequence_stream::SequenceInput;

    fn fixture(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name)
    }

    #[test]
    fn reports_expected_candidate_rows() {
        let outcome = run_eprimer3(Eprimer3Params {
            input: SequenceInput::new(fixture("eprimer3_targets.fasta")),
        })
        .expect("eprimer3 should execute");

        assert_eq!(outcome.record_count, 2);
        assert_eq!(outcome.parameters.min_oligo_length, 18);
        assert_eq!(outcome.parameters.max_oligo_length, 22);
        assert_eq!(outcome.rows.len(), 24);

        assert_eq!(outcome.rows[0].record_id, "ep3targetA");
        assert_eq!(outcome.rows[0].candidate_id, "ep3targetA:forward:8-26");
        assert_eq!(outcome.rows[0].strand, "forward");
        assert_eq!(outcome.rows[1].candidate_id, "ep3targetA:reverse:8-26");
        assert_eq!(outcome.rows[1].strand, "reverse");
        assert!(outcome.rows.iter().all(|row| row.ambiguous_symbols == 0));
    }

    #[test]
    fn rejects_protein_input() {
        let error = run_eprimer3(Eprimer3Params {
            input: SequenceInput::new(fixture("protein_records.fasta")),
        })
        .expect_err("protein input should fail");

        assert!(
            error
                .to_string()
                .contains("requires a nucleotide sequence input")
        );
    }
}
