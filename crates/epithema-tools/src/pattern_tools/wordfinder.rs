//! `wordfinder` implementation.

use super::exact_words::maximal_exact_regions;
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `wordfinder`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordfinderParams {
    /// Singleton query input.
    pub query: SequenceInput,
    /// One-or-more target records.
    pub targets: SequenceInput,
    /// Minimum exact shared word length.
    pub word_size: usize,
}

/// One exact shared region between the query and one target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordfinderHit {
    /// Query record identifier.
    pub query_id: String,
    /// Target record identifier.
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

/// Structured `wordfinder` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordfinderOutcome {
    /// Source query input.
    pub query: SequenceInput,
    /// Source target input.
    pub targets: SequenceInput,
    /// Minimum exact shared word length.
    pub word_size: usize,
    /// Stable ordered exact shared regions across targets.
    pub hits: Vec<WordfinderHit>,
}

/// Returns `wordfinder` help text.
#[must_use]
pub fn wordfinder_help() -> &'static str {
    "Usage: epithema wordfinder <query-input> <target-input> [--word-size <length>]\n\nFind maximal exact shared ungapped regions between exactly one query record and one or more target records. v1 reports all exact regions of at least the requested word size, preserves target-record order, and uses 1-based inclusive coordinates in the rendered table."
}

/// Executes `wordfinder`.
pub fn run_wordfinder(params: WordfinderParams) -> Result<WordfinderOutcome, ToolExecutionError> {
    let query = load_single_record(&params.query, "wordfinder", "query")?;
    let targets = load_sequence_records(&params.targets)?;
    let mut hits = Vec::new();

    for target in targets {
        hits.extend(
            maximal_exact_regions("wordfinder", &query, &target, params.word_size)?
                .into_iter()
                .map(|region| WordfinderHit {
                    query_id: query.identifier().accession().to_owned(),
                    target_id: target.identifier().accession().to_owned(),
                    query_start: region.left_start,
                    query_end: region.left_end,
                    target_start: region.right_start,
                    target_end: region.right_end,
                    matched: region.matched,
                }),
        );
    }

    Ok(WordfinderOutcome {
        query: params.query,
        targets: params.targets,
        word_size: params.word_size,
        hits,
    })
}

fn load_single_record(
    input: &SequenceInput,
    tool: &str,
    role: &str,
) -> Result<epithema_core::SequenceRecord, ToolExecutionError> {
    let records = load_sequence_records(input)?;
    let mut iter = records.into_iter();
    let first = iter.next().ok_or_else(|| {
        epithema_diagnostics::PlatformError::new(
            epithema_diagnostics::ErrorCategory::Validation,
            format!("{tool} requires one {role} record"),
        )
        .with_code(format!("tools.{tool}.input.{role}_missing"))
    })?;
    if iter.next().is_some() {
        return Err(epithema_diagnostics::PlatformError::new(
            epithema_diagnostics::ErrorCategory::Validation,
            format!("{tool} requires exactly one {role} record"),
        )
        .with_code(format!("tools.{tool}.input.{role}_not_singleton")));
    }
    Ok(first)
}

#[cfg(test)]
mod tests {
    use super::{WordfinderParams, run_wordfinder};
    use crate::sequence_stream::SequenceInput;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "epithema-wordfinder-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary fixture should be written");
        path
    }

    #[test]
    fn reports_hits_across_target_set_in_stable_order() {
        let query = write_temp_sequence_file("query", ">query\nTTACGTAA\n");
        let targets = write_temp_sequence_file("targets", ">a\nGGACGTCC\n>b\nTTTTAAAA\n");
        let outcome = run_wordfinder(WordfinderParams {
            query: SequenceInput::new(query.clone()),
            targets: SequenceInput::new(targets.clone()),
            word_size: 4,
        })
        .expect("wordfinder should succeed");
        fs::remove_file(query).ok();
        fs::remove_file(targets).ok();

        assert_eq!(outcome.hits.len(), 1);
        assert_eq!(outcome.hits[0].target_id, "a");
        assert_eq!(outcome.hits[0].matched, "ACGT");
    }
}
