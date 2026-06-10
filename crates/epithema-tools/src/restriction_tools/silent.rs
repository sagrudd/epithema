//! `silent` implementation.

use crate::restriction_tools::shared::{
    SynonymousRestrictionEdit, normalize_site, silent_candidates, validate_coding_dna_record,
};
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `silent`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SilentParams {
    /// Input coding DNA records.
    pub input: SequenceInput,
    /// Exact canonical DNA site to create.
    pub site: String,
}

/// One synonymous candidate that creates a new site.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SilentCandidate {
    /// Record identifier.
    pub record_id: String,
    /// Candidate edit details.
    pub edit: SynonymousRestrictionEdit,
}

/// Structured `silent` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SilentOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Normalized site string.
    pub site: String,
    /// Candidate synonymous edits.
    pub candidates: Vec<SilentCandidate>,
}

/// Returns `silent` help text.
#[must_use]
pub fn silent_help() -> &'static str {
    "Usage: epithema silent <coding-dna-input> <site>\n\nReport synonymous single-codon edits that create an exact forward-strand canonical DNA restriction site in a strict coding sequence while preserving translation. The v1 implementation accepts only canonical DNA sites of length at least four, requires coding DNA input, and does not use enzyme databases or multi-edit optimization."
}

/// Executes `silent`.
pub fn run_silent(params: SilentParams) -> Result<SilentOutcome, ToolExecutionError> {
    let site = normalize_site(&params.site, "silent")?;
    let mut candidates = Vec::new();

    for record in load_sequence_records(&params.input)? {
        let sequence = validate_coding_dna_record("silent", &record)?;
        let record_id = record.identifier().accession().to_owned();
        candidates.extend(
            silent_candidates(&sequence, &site)?
                .into_iter()
                .map(|edit| SilentCandidate {
                    record_id: record_id.clone(),
                    edit,
                }),
        );
    }

    Ok(SilentOutcome {
        input: params.input,
        site,
        candidates,
    })
}

#[cfg(test)]
mod tests {
    use super::{SilentParams, run_silent};
    use crate::sequence_stream::SequenceInput;

    fn fixture(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/epithema-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn reports_synonymous_candidates_that_create_exact_site() {
        let outcome = run_silent(SilentParams {
            input: SequenceInput::new(fixture("silent_records.fasta")),
            site: "GAATTC".to_owned(),
        })
        .expect("silent should succeed");

        assert_eq!(outcome.candidates.len(), 1);
        assert_eq!(outcome.candidates[0].edit.original_codon, "GAG");
        assert_eq!(outcome.candidates[0].edit.replacement_codon, "GAA");
    }

    #[test]
    fn rejects_noncoding_input() {
        let error = run_silent(SilentParams {
            input: SequenceInput::new(fixture("nucleotide_pattern_records.fasta")),
            site: "GAATTC".to_owned(),
        })
        .expect_err("noncoding input should fail");

        assert!(error.to_string().contains("strict coding DNA input"));
    }
}
