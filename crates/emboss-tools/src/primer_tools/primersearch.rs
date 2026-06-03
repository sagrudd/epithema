//! `primersearch` implementation.

use std::fs;
use std::path::PathBuf;

use emboss_core::{PrimerPair, primersearch_profile};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Local primer-pair file input handled by the bounded `primersearch` seam.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimersearchPairInput {
    /// Canonical local path to the primer-pair file.
    pub path: PathBuf,
}

impl PrimersearchPairInput {
    /// Creates a new local primer-pair input wrapper.
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

/// Typed parameters for `primersearch`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimersearchParams {
    /// Local nucleotide target input path.
    pub input: SequenceInput,
    /// Local tab-delimited primer-pair input path.
    pub primer_pairs: PrimersearchPairInput,
}

/// Stable summary row for one complete primer-pair hit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimersearchRow {
    /// Stable target record identifier.
    pub record_id: String,
    /// Stable primer-pair name.
    pub primer_pair_name: String,
    /// Deterministic strand/orientation label.
    pub strand: String,
    /// One-based inclusive left-primer start.
    pub left_primer_start: usize,
    /// One-based inclusive left-primer end.
    pub left_primer_end: usize,
    /// One-based inclusive right-primer start.
    pub right_primer_start: usize,
    /// One-based inclusive right-primer end.
    pub right_primer_end: usize,
    /// One-based inclusive amplicon start.
    pub amplicon_start: usize,
    /// One-based inclusive amplicon end.
    pub amplicon_end: usize,
    /// Amplicon span length in residues.
    pub amplicon_length: usize,
    /// Matched left-primer slice.
    pub left_matched: String,
    /// Matched right-primer slice.
    pub right_matched: String,
}

/// Structured `primersearch` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimersearchOutcome {
    /// Source target input.
    pub input: SequenceInput,
    /// Source primer-pair input.
    pub primer_pairs: PrimersearchPairInput,
    /// Count of parsed primer pairs.
    pub primer_pair_count: usize,
    /// Count of target records searched.
    pub record_count: usize,
    /// Stable ordered complete-pair hit rows.
    pub rows: Vec<PrimersearchRow>,
}

/// Returns the `primersearch` help text.
#[must_use]
pub fn primersearch_help() -> &'static str {
    "Usage: emboss-rs primersearch <nucleotide-input> <primer-pairs.tsv>\n\nReport deterministic complete primer-pair hits against one local nucleotide sequence input and one local tab-delimited primer-pair file. The bounded v1 seam accepts three tab-separated fields per non-empty row: pair_name, forward_primer, reverse_primer."
}

/// Executes `primersearch`.
pub fn run_primersearch(
    params: PrimersearchParams,
) -> Result<PrimersearchOutcome, ToolExecutionError> {
    let primer_pairs = load_primer_pairs(&params.primer_pairs)?;
    let primer_pair_count = primer_pairs.len();
    let mut rows = Vec::new();
    let mut record_count = 0usize;

    for record in load_sequence_records(&params.input)? {
        record_count += 1;
        let profile = primersearch_profile(&record, &primer_pairs).map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.primersearch.profile.invalid")
        })?;
        rows.extend(profile.hits.into_iter().map(|hit| PrimersearchRow {
            record_id: profile.identifier.clone(),
            primer_pair_name: hit.primer_pair_name,
            strand: strand_label(hit.strand),
            left_primer_start: hit.left_primer_start,
            left_primer_end: hit.left_primer_end,
            right_primer_start: hit.right_primer_start,
            right_primer_end: hit.right_primer_end,
            amplicon_start: hit.amplicon_start,
            amplicon_end: hit.amplicon_end,
            amplicon_length: hit.amplicon_length,
            left_matched: hit.left_matched,
            right_matched: hit.right_matched,
        }));
    }

    Ok(PrimersearchOutcome {
        input: params.input,
        primer_pairs: params.primer_pairs,
        primer_pair_count,
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

fn load_primer_pairs(input: &PrimersearchPairInput) -> Result<Vec<PrimerPair>, ToolExecutionError> {
    let contents = fs::read_to_string(&input.path).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Validation,
            "failed to read primer-pair input",
        )
        .with_code("tools.primersearch.pairs.read_failed")
        .with_detail(format!("{}: {error}", input.path.display()))
    })?;

    let mut pairs = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let fields: Vec<_> = trimmed.split('\t').collect();
        if fields.len() != 3 {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!(
                    "primer-pair input line {} must contain exactly 3 tab-delimited fields",
                    index + 1
                ),
            )
            .with_code("tools.primersearch.pairs.invalid_row"));
        }

        let pair = PrimerPair::new(fields[0], fields[1], fields[2]).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Validation,
                format!("invalid primer pair on line {}: {error}", index + 1),
            )
            .with_code("tools.primersearch.pairs.invalid_row")
        })?;
        pairs.push(pair);
    }

    if pairs.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "primersearch requires at least one primer pair row",
        )
        .with_code("tools.primersearch.pairs.empty"));
    }

    Ok(pairs)
}

#[cfg(test)]
mod tests {
    use super::{PrimersearchPairInput, PrimersearchParams, run_primersearch};
    use crate::sequence_stream::SequenceInput;

    fn fixture(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name)
    }

    #[test]
    fn reports_expected_complete_pair_rows() {
        let outcome = run_primersearch(PrimersearchParams {
            input: SequenceInput::new(fixture("primersearch_targets.fasta")),
            primer_pairs: PrimersearchPairInput::new(fixture("primersearch_pairs.tsv")),
        })
        .expect("primersearch should execute");

        assert_eq!(outcome.primer_pair_count, 2);
        assert_eq!(outcome.record_count, 2);
        assert_eq!(outcome.rows.len(), 2);

        assert_eq!(outcome.rows[0].record_id, "targetA");
        assert_eq!(outcome.rows[0].primer_pair_name, "pair1");
        assert_eq!(outcome.rows[0].strand, "forward");
        assert_eq!(outcome.rows[0].amplicon_start, 4);
        assert_eq!(outcome.rows[0].amplicon_end, 14);

        assert_eq!(outcome.rows[1].record_id, "targetB");
        assert_eq!(outcome.rows[1].primer_pair_name, "pair2");
        assert_eq!(outcome.rows[1].strand, "reverse");
        assert_eq!(outcome.rows[1].left_matched, "GCAT");
        assert_eq!(outcome.rows[1].right_matched, "TTAA");
    }

    #[test]
    fn rejects_invalid_primer_pair_rows() {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-primersearch-invalid-{}-{}.tsv",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        std::fs::write(&path, "pair1\tATGC\n").expect("fixture should write");

        let error = run_primersearch(PrimersearchParams {
            input: SequenceInput::new(fixture("primersearch_targets.fasta")),
            primer_pairs: PrimersearchPairInput::new(&path),
        })
        .expect_err("invalid primer-pair row should fail");
        std::fs::remove_file(path).ok();

        assert!(
            error
                .to_string()
                .contains("must contain exactly 3 tab-delimited fields")
        );
    }

    #[test]
    fn rejects_protein_input() {
        let error = run_primersearch(PrimersearchParams {
            input: SequenceInput::new(fixture("protein_records.fasta")),
            primer_pairs: PrimersearchPairInput::new(fixture("primersearch_pairs.tsv")),
        })
        .expect_err("protein input should fail");

        assert!(
            error
                .to_string()
                .contains("requires a nucleotide sequence input")
        );
    }
}
