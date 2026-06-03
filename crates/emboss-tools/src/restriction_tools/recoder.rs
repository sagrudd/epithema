//! `recoder` implementation.

use crate::restriction_tools::shared::{
    SynonymousRestrictionEdit, normalize_site, recoder_candidates, site_positions,
    validate_coding_dna_record,
};
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `recoder`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecoderParams {
    /// Input coding DNA records.
    pub input: SequenceInput,
    /// Exact canonical DNA site to remove.
    pub site: String,
}

/// One synonymous candidate that removes an existing site.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecoderCandidate {
    /// Record identifier.
    pub record_id: String,
    /// One-based occurrence ordinal within the record.
    pub occurrence_index: usize,
    /// Candidate edit details.
    pub edit: SynonymousRestrictionEdit,
}

/// Structured `recoder` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecoderOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Normalized site string.
    pub site: String,
    /// Candidate synonymous edits.
    pub candidates: Vec<RecoderCandidate>,
}

/// Returns `recoder` help text.
#[must_use]
pub fn recoder_help() -> &'static str {
    "Usage: emboss-rs recoder <coding-dna-input> <site>\n\nReport synonymous single-codon edits that remove an exact forward-strand canonical DNA restriction site from a strict coding sequence while preserving translation. The v1 implementation accepts only canonical DNA sites of length at least four, requires coding DNA input, and does not use enzyme databases or multi-edit optimization."
}

/// Executes `recoder`.
pub fn run_recoder(params: RecoderParams) -> Result<RecoderOutcome, ToolExecutionError> {
    let site = normalize_site(&params.site, "recoder")?;
    let mut candidates = Vec::new();

    for record in load_sequence_records(&params.input)? {
        let sequence = validate_coding_dna_record("recoder", &record)?;
        for (occurrence_index, occurrence_start) in
            site_positions(&sequence, &site).into_iter().enumerate()
        {
            let record_id = record.identifier().accession().to_owned();
            candidates.extend(
                recoder_candidates(&sequence, &site, occurrence_start)?
                    .into_iter()
                    .map(|edit| RecoderCandidate {
                        record_id: record_id.clone(),
                        occurrence_index: occurrence_index + 1,
                        edit,
                    }),
            );
        }
    }

    Ok(RecoderOutcome {
        input: params.input,
        site,
        candidates,
    })
}

#[cfg(test)]
mod tests {
    use super::{RecoderParams, run_recoder};
    use crate::sequence_stream::SequenceInput;

    fn fixture(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/emboss-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn reports_synonymous_candidates_that_remove_exact_site() {
        let outcome = run_recoder(RecoderParams {
            input: SequenceInput::new(fixture("recoder_records.fasta")),
            site: "GAATTC".to_owned(),
        })
        .expect("recoder should succeed");

        assert_eq!(outcome.candidates.len(), 2);
        assert_eq!(outcome.candidates[0].occurrence_index, 1);
        assert_eq!(outcome.candidates[0].edit.original_codon, "GAA");
        assert_eq!(outcome.candidates[1].edit.original_codon, "TTC");
    }

    #[test]
    fn rejects_noncoding_input() {
        let error = run_recoder(RecoderParams {
            input: SequenceInput::new(fixture("nucleotide_pattern_records.fasta")),
            site: "GAATTC".to_owned(),
        })
        .expect_err("noncoding input should fail");

        assert!(error.to_string().contains("strict coding DNA input"));
    }
}
