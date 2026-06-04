//! `sirna` implementation.

use emboss_core::{SirnaParameters, sirna_profile};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `sirna`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SirnaParams {
    /// Local nucleotide input path.
    pub input: SequenceInput,
}

/// Stable summary row for one bounded `sirna` candidate.
#[derive(Clone, Debug, PartialEq)]
pub struct SirnaRow {
    /// Stable target record identifier.
    pub record_id: String,
    /// Stable candidate identifier.
    pub candidate_id: String,
    /// Deterministic strand/orientation label.
    pub strand: String,
    /// One-based inclusive target start.
    pub target_start: usize,
    /// One-based inclusive target end.
    pub target_end: usize,
    /// Duplex length in residues.
    pub duplex_length: usize,
    /// Sense-strand sequence in target orientation.
    pub sense_sequence: String,
    /// Guide-strand sequence paired against the target.
    pub guide_sequence: String,
    /// Canonical-symbol count in the target window.
    pub canonical_symbols: usize,
    /// Ambiguous-symbol count in the target window.
    pub ambiguous_symbols: usize,
    /// GC fraction across canonical symbols.
    pub gc_fraction: f64,
    /// Guide-strand 5' terminal base.
    pub guide_five_prime_base: String,
    /// A/U-like count in guide positions 2-8.
    pub guide_seed_au_count: usize,
    /// Longest homopolymer run in the target window.
    pub max_homopolymer_run: usize,
}

/// Structured `sirna` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct SirnaOutcome {
    /// Source nucleotide input.
    pub input: SequenceInput,
    /// Stable bounded design parameters.
    pub parameters: SirnaParameters,
    /// Count of target records analyzed.
    pub record_count: usize,
    /// Stable ordered candidate rows.
    pub rows: Vec<SirnaRow>,
}

/// Returns the bounded `sirna` help text.
#[must_use]
pub fn sirna_help() -> &'static str {
    "Usage: emboss-rs sirna <nucleotide-input>\n\nReport deterministic bounded siRNA-candidate rows against one local nucleotide input. The bounded v1 seam uses local default candidate-selection parameters only and emits table-first rows rather than a generalized RNAi-efficacy or off-target ranking workflow."
}

/// Executes bounded `sirna`.
pub fn run_sirna(params: SirnaParams) -> Result<SirnaOutcome, ToolExecutionError> {
    let parameters = SirnaParameters::default();
    let mut rows = Vec::new();
    let mut record_count = 0usize;

    for record in load_sequence_records(&params.input)? {
        record_count += 1;
        let profile = sirna_profile(&record, parameters).map_err(|error| {
            emboss_diagnostics::PlatformError::new(
                emboss_diagnostics::ErrorCategory::Validation,
                error.to_string(),
            )
            .with_code("tools.sirna.profile.invalid")
        })?;

        rows.extend(profile.candidates.into_iter().map(|candidate| SirnaRow {
            record_id: profile.identifier.clone(),
            candidate_id: candidate.candidate_id,
            strand: strand_label(candidate.strand),
            target_start: candidate.target_start,
            target_end: candidate.target_end,
            duplex_length: candidate.duplex_length,
            sense_sequence: candidate.sense_sequence,
            guide_sequence: candidate.guide_sequence,
            canonical_symbols: candidate.canonical_symbols,
            ambiguous_symbols: candidate.ambiguous_symbols,
            gc_fraction: candidate.gc_fraction,
            guide_five_prime_base: candidate.guide_five_prime_base.to_string(),
            guide_seed_au_count: candidate.guide_seed_au_count,
            max_homopolymer_run: candidate.max_homopolymer_run,
        }));
    }

    Ok(SirnaOutcome {
        input: params.input,
        parameters,
        record_count,
        rows,
    })
}

fn strand_label(strand: emboss_core::Strand) -> String {
    match strand {
        emboss_core::Strand::Forward => "forward".to_owned(),
        emboss_core::Strand::Reverse => "reverse".to_owned(),
        emboss_core::Strand::Unstranded => "unstranded".to_owned(),
        emboss_core::Strand::Unknown => "unknown".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::{SirnaParams, run_sirna};
    use crate::sequence_stream::SequenceInput;

    fn fixture(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name)
    }

    #[test]
    fn reports_expected_candidate_rows() {
        let outcome = run_sirna(SirnaParams {
            input: SequenceInput::new(fixture("sirna_targets.fasta")),
        })
        .expect("sirna should execute");

        assert_eq!(outcome.record_count, 2);
        assert_eq!(outcome.parameters.duplex_length, 21);
        assert_eq!(outcome.rows.len(), 1);

        let row = &outcome.rows[0];
        assert_eq!(row.record_id, "sirnatargetA");
        assert_eq!(row.candidate_id, "sirna-00001");
        assert_eq!(row.strand, "forward");
        assert_eq!(row.target_start, 1);
        assert_eq!(row.target_end, 21);
        assert_eq!(row.duplex_length, 21);
        assert_eq!(row.sense_sequence, "AATATCGCCATGCGATATATT");
        assert_eq!(row.guide_five_prime_base, "A");
        assert_eq!(row.guide_seed_au_count, 6);
        assert_eq!(row.max_homopolymer_run, 2);
    }

    #[test]
    fn rejects_protein_input() {
        let error = run_sirna(SirnaParams {
            input: SequenceInput::new(fixture("protein_records.fasta")),
        })
        .expect_err("protein input should fail");

        assert!(error.to_string().contains("requires a nucleotide sequence input"));
    }
}
