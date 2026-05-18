//! `wordmatch` implementation.

use super::exact_words::{ExactWordRegion, maximal_exact_regions};
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `wordmatch`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordmatchParams {
    /// Singleton left/query input.
    pub query: SequenceInput,
    /// Singleton right/target input.
    pub target: SequenceInput,
    /// Minimum exact shared word length.
    pub word_size: usize,
}

/// One exact identity region.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordmatchHit {
    /// Left/query record identifier.
    pub query_id: String,
    /// Right/target record identifier.
    pub target_id: String,
    /// Zero-based inclusive start in the query sequence.
    pub query_start: usize,
    /// Zero-based half-open end in the query sequence.
    pub query_end: usize,
    /// Zero-based inclusive start in the target sequence.
    pub target_start: usize,
    /// Zero-based half-open end in the target sequence.
    pub target_end: usize,
    /// Exact shared region.
    pub matched: String,
}

/// Structured `wordmatch` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordmatchOutcome {
    /// Source query input.
    pub query: SequenceInput,
    /// Source target input.
    pub target: SequenceInput,
    /// Minimum exact shared word length.
    pub word_size: usize,
    /// Stable ordered exact shared regions.
    pub hits: Vec<WordmatchHit>,
}

/// Returns `wordmatch` help text.
#[must_use]
pub fn wordmatch_help() -> &'static str {
    "Usage: emboss-rs wordmatch <query-input> <target-input> [--word-size <length>]\n\nFind maximal exact shared ungapped regions between exactly one query record and one target record. v1 requires compatible molecule types, reports all maximal exact regions of at least the requested word size, and uses 1-based inclusive coordinates in the rendered table."
}

/// Executes `wordmatch`.
pub fn run_wordmatch(params: WordmatchParams) -> Result<WordmatchOutcome, ToolExecutionError> {
    let query = load_single_record(&params.query, "wordmatch", "query")?;
    let target = load_single_record(&params.target, "wordmatch", "target")?;
    let hits = maximal_exact_regions("wordmatch", &query, &target, params.word_size)?
        .into_iter()
        .map(|region| map_hit(query.identifier().accession(), target.identifier().accession(), region))
        .collect();

    Ok(WordmatchOutcome {
        query: params.query,
        target: params.target,
        word_size: params.word_size,
        hits,
    })
}

fn load_single_record(
    input: &SequenceInput,
    tool: &str,
    role: &str,
) -> Result<emboss_core::SequenceRecord, ToolExecutionError> {
    let records = load_sequence_records(input)?;
    let mut iter = records.into_iter();
    let first = iter.next().ok_or_else(|| {
        emboss_diagnostics::PlatformError::new(
            emboss_diagnostics::ErrorCategory::Validation,
            format!("{tool} requires one {role} record"),
        )
        .with_code(format!("tools.{tool}.input.{role}_missing"))
    })?;
    if iter.next().is_some() {
        return Err(emboss_diagnostics::PlatformError::new(
            emboss_diagnostics::ErrorCategory::Validation,
            format!("{tool} requires exactly one {role} record"),
        )
        .with_code(format!("tools.{tool}.input.{role}_not_singleton")));
    }
    Ok(first)
}

fn map_hit(query_id: &str, target_id: &str, region: ExactWordRegion) -> WordmatchHit {
    WordmatchHit {
        query_id: query_id.to_owned(),
        target_id: target_id.to_owned(),
        query_start: region.left_start,
        query_end: region.left_end,
        target_start: region.right_start,
        target_end: region.right_end,
        matched: region.matched,
    }
}

#[cfg(test)]
mod tests {
    use super::{WordmatchParams, run_wordmatch};
    use crate::sequence_stream::SequenceInput;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-wordmatch-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary fixture should be written");
        path
    }

    #[test]
    fn reports_maximal_exact_shared_regions() {
        let query = write_temp_sequence_file("query", ">query\nTTACGTAA\n");
        let target = write_temp_sequence_file("target", ">target\nGGACGTCC\n");
        let outcome = run_wordmatch(WordmatchParams {
            query: SequenceInput::new(query.clone()),
            target: SequenceInput::new(target.clone()),
            word_size: 4,
        })
        .expect("wordmatch should succeed");
        fs::remove_file(query).ok();
        fs::remove_file(target).ok();

        assert_eq!(outcome.hits.len(), 1);
        assert_eq!(outcome.hits[0].matched, "ACGT");
    }
}
