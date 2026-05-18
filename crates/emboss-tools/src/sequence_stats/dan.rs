//! `dan` implementation.

use emboss_core::{Alphabet, GcSummary, MoleculeKind, SequenceRecord};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `dan`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DanParams {
    /// Local nucleotide input path.
    pub input: SequenceInput,
    /// Optional sliding-window width. When absent, use the whole record as one window.
    pub window: Option<usize>,
    /// Sliding-window step size.
    pub step: usize,
}

/// Stable one-window melting summary.
#[derive(Clone, Debug, PartialEq)]
pub struct DanWindow {
    /// Stable record identifier.
    pub record_id: String,
    /// 1-based window ordinal within the record.
    pub window_index: usize,
    /// Zero-based inclusive start.
    pub start: usize,
    /// Zero-based exclusive end.
    pub end: usize,
    /// GC summary for the window.
    pub gc: GcSummary,
    /// Conservative melting estimate in Celsius.
    pub tm_celsius: f64,
}

/// Structured `dan` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct DanOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Configured window width for this invocation.
    pub window: Option<usize>,
    /// Configured step.
    pub step: usize,
    /// Stable window rows in record order and scan order.
    pub windows: Vec<DanWindow>,
}

/// Returns `dan` help text.
#[must_use]
pub fn dan_help() -> &'static str {
    "Usage: emboss-rs dan <nucleotide-input> [--window <count>] [--step <count>]\n\nReport conservative nucleic-acid melting estimates for one or more nucleotide records. v1 accepts DNA and RNA inputs, emits one row per whole record by default, optionally scans sliding windows, requires canonical A/C/G/T/U residues only, and estimates melting temperature with a simple Wallace/GC-length hybrid rule."
}

/// Executes `dan`.
pub fn run_dan(params: DanParams) -> Result<DanOutcome, ToolExecutionError> {
    let mut windows = Vec::new();

    for record in load_sequence_records(&params.input)? {
        validate_dan_record(&record)?;
        let window_width = params.window.unwrap_or(record.len());
        if window_width > record.len() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!(
                    "dan window size {} exceeds sequence length {} for '{}'",
                    window_width,
                    record.len(),
                    record.identifier().accession()
                ),
            )
            .with_code("tools.dan.window.out_of_range"));
        }

        for (window_index, start) in window_starts(record.len(), window_width, params.step)
            .into_iter()
            .enumerate()
        {
            let end = start + window_width;
            let sequence = &record.residues()[start..end];
            let gc = GcSummary::from_sequence(sequence);
            windows.push(DanWindow {
                record_id: record.identifier().accession().to_owned(),
                window_index: window_index + 1,
                start,
                end,
                gc,
                tm_celsius: estimate_tm_celsius(sequence),
            });
        }
    }

    Ok(DanOutcome {
        input: params.input,
        window: params.window,
        step: params.step,
        windows,
    })
}

fn validate_dan_record(record: &SequenceRecord) -> Result<(), ToolExecutionError> {
    if record.molecule().is_protein()
        || (!record.molecule().is_nucleotide() && !looks_like_nucleotide_record(record))
    {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "dan expects nucleotide input but '{}' was classified as {}",
                record.identifier().accession(),
                record.molecule()
            ),
        )
        .with_code("tools.dan.input.not_nucleotide"));
    }

    if record
        .residues()
        .chars()
        .any(|symbol| !matches!(symbol, 'A' | 'C' | 'G' | 'T' | 'U'))
    {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "dan requires canonical A/C/G/T/U residues only for '{}'",
                record.identifier().accession()
            ),
        )
        .with_code("tools.dan.residue.non_canonical"));
    }

    Ok(())
}

fn looks_like_nucleotide_record(record: &SequenceRecord) -> bool {
    Alphabet::Dna
        .validate(MoleculeKind::Dna, record.residues())
        .is_ok()
        || Alphabet::Rna
            .validate(MoleculeKind::Rna, record.residues())
            .is_ok()
}

fn window_starts(length: usize, window: usize, step: usize) -> Vec<usize> {
    if window >= length {
        return vec![0];
    }

    let mut starts = Vec::new();
    let mut start = 0usize;
    while start + window <= length {
        starts.push(start);
        start += step;
    }
    starts
}

fn estimate_tm_celsius(sequence: &str) -> f64 {
    let gc = sequence
        .chars()
        .filter(|symbol| matches!(symbol, 'G' | 'C'))
        .count() as f64;
    let at_or_u = sequence.len() as f64 - gc;
    if sequence.len() < 14 {
        4.0 * gc + 2.0 * at_or_u
    } else {
        64.9 + 41.0 * (gc - 16.4) / sequence.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{DanParams, estimate_tm_celsius, run_dan};
    use crate::sequence_stream::SequenceInput;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-dan-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary sequence fixture should be written");
        path
    }

    #[test]
    fn reports_whole_sequence_tm_by_default() {
        let outcome = run_dan(DanParams {
            input: SequenceInput::new(
                "/Users/stephen/Projects/emboss-rs/crates/emboss-tools/tests/fixtures/three_records.fasta",
            ),
            window: None,
            step: 1,
        })
        .expect("dan should execute");

        assert_eq!(outcome.windows.len(), 3);
        assert_eq!(outcome.windows[0].record_id, "alpha");
        assert_eq!(outcome.windows[0].start, 0);
        assert_eq!(outcome.windows[0].end, 4);
        assert!((outcome.windows[0].tm_celsius - 12.0).abs() < 1e-9);
    }

    #[test]
    fn reports_sliding_windows() {
        let outcome = run_dan(DanParams {
            input: SequenceInput::new(
                "/Users/stephen/Projects/emboss-rs/crates/emboss-tools/tests/fixtures/three_records.fasta",
            ),
            window: Some(2),
            step: 1,
        })
        .expect("dan should execute");

        assert_eq!(outcome.windows.len(), 9);
        assert_eq!(outcome.windows[0].window_index, 1);
        assert_eq!(outcome.windows[1].window_index, 2);
        assert!((outcome.windows[0].tm_celsius - 6.0).abs() < 1e-9);
    }

    #[test]
    fn accepts_rna_input() {
        let path = write_temp_sequence_file("rna", ">rna\nACGU\n");
        let outcome = run_dan(DanParams {
            input: SequenceInput::new(path.clone()),
            window: None,
            step: 1,
        })
        .expect("dan should execute");
        fs::remove_file(path).ok();

        assert_eq!(outcome.windows.len(), 1);
        assert_eq!(outcome.windows[0].gc.gc_symbols, 2);
        assert!((outcome.windows[0].tm_celsius - 12.0).abs() < 1e-9);
    }

    #[test]
    fn rejects_ambiguous_input() {
        let error = run_dan(DanParams {
            input: SequenceInput::new(
                "/Users/stephen/Projects/emboss-rs/crates/emboss-tools/tests/fixtures/nucleotide_pattern_records.fasta",
            ),
            window: None,
            step: 1,
        })
        .expect_err("ambiguous nucleotide input should fail");

        assert!(error.to_string().contains("canonical A/C/G/T/U"));
    }

    #[test]
    fn uses_hybrid_tm_rule() {
        assert!((estimate_tm_celsius("ACGT") - 12.0).abs() < 1e-9);
        assert!((estimate_tm_celsius("ACGTACGTACGTAC") - 37.371_428_571_428_58).abs() < 1e-9);
    }
}
