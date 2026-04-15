//! `complex` implementation.

use emboss_core::{
    ComplexityError, ComplexityParameters, SequenceComplexity, WindowComplexity,
    sequence_complexity, sliding_window_complexity,
};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `complex`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComplexParams {
    /// Local nucleotide sequence input path.
    pub input: SequenceInput,
    /// Inclusive minimum k-mer size.
    pub k_min: usize,
    /// Inclusive maximum k-mer size.
    pub k_max: usize,
    /// Optional sliding-window length.
    pub window: Option<usize>,
    /// Optional sliding-window step size.
    pub step: Option<usize>,
}

/// Structured `complex` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct ComplexOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Whole-sequence summaries for all records.
    pub sequences: Vec<SequenceComplexity>,
    /// Sliding-window rows, when requested.
    pub windows: Vec<WindowComplexity>,
    /// Requested k-min.
    pub k_min: usize,
    /// Requested k-max.
    pub k_max: usize,
    /// Requested window size.
    pub window: Option<usize>,
    /// Requested step size.
    pub step: Option<usize>,
}

/// Returns `complex` help text.
#[must_use]
pub fn complex_help() -> &'static str {
    "Usage: emboss-rs complex <input> --k-min <k> --k-max <k> [--window <length> --step <length>]\n\nCompute nucleotide linguistic complexity using sum(observed distinct k-mers) / sum(min(4^k, L-k+1)) over an inclusive k-mer range. v1 is strict A/C/G/T only, rejects non-canonical symbols, always reports whole-sequence complexity for each record, and adds sliding-window rows when both --window and --step are supplied."
}

/// Executes `complex`.
pub fn run_complex(params: ComplexParams) -> Result<ComplexOutcome, ToolExecutionError> {
    if params.window.is_some() ^ params.step.is_some() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "complex requires --window and --step together for sliding-window analysis",
        )
        .with_code("tools.complex.window_step.required_together"));
    }

    let parameters = ComplexityParameters {
        k_min: params.k_min,
        k_max: params.k_max,
    };
    let mut sequences = Vec::new();
    let mut windows = Vec::new();
    for record in load_sequence_records(&params.input)? {
        sequences.push(sequence_complexity(&record, parameters).map_err(map_complexity_error)?);

        if let (Some(window), Some(step)) = (params.window, params.step) {
            windows.extend(
                sliding_window_complexity(&record, window, step, parameters)
                    .map_err(map_complexity_error)?,
            );
        }
    }

    Ok(ComplexOutcome {
        input: params.input,
        sequences,
        windows,
        k_min: params.k_min,
        k_max: params.k_max,
        window: params.window,
        step: params.step,
    })
}

fn map_complexity_error(error: ComplexityError) -> ToolExecutionError {
    let code = match error {
        ComplexityError::NonCanonicalNucleotideInput { .. } => "tools.complex.input.non_nucleotide",
        ComplexityError::UnsupportedSymbol { .. } => "tools.complex.input.unsupported_symbol",
        ComplexityError::InvalidKRange { .. } => "tools.complex.k_range.invalid",
        ComplexityError::InvalidWindow { .. } => "tools.complex.window.invalid",
        ComplexityError::WindowLongerThanSequence { .. } => "tools.complex.window.too_long",
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}
