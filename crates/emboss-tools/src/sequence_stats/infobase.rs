//! `infobase` implementation.

use emboss_core::{NucleotideBaseInfo, nucleotide_base_info};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::ToolExecutionError;

/// Typed parameters for `infobase`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InfobaseParams {
    /// Queried nucleotide symbol.
    pub symbol: char,
}

/// Structured `infobase` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InfobaseOutcome {
    /// Queried nucleotide symbol.
    pub symbol: char,
    /// Stable metadata row.
    pub info: NucleotideBaseInfo,
}

/// Returns `infobase` help text.
#[must_use]
pub fn infobase_help() -> &'static str {
    "Usage: emboss-rs infobase <base>\n\nReport deterministic metadata for one nucleotide base or IUPAC ambiguity symbol. The v1 implementation reports a single stable row with symbol class, supported molecule space, canonical expansion, and DNA/RNA complements."
}

/// Executes `infobase`.
pub fn run_infobase(params: InfobaseParams) -> Result<InfobaseOutcome, ToolExecutionError> {
    let info = nucleotide_base_info(params.symbol).ok_or_else(|| {
        ToolExecutionError::from(
            PlatformError::new(
                ErrorCategory::Validation,
                format!("unsupported nucleotide symbol '{}'", params.symbol),
            )
            .with_code("tools.infobase.symbol.unsupported"),
        )
    })?;

    Ok(InfobaseOutcome {
        symbol: params.symbol.to_ascii_uppercase(),
        info,
    })
}

#[cfg(test)]
mod tests {
    use super::{InfobaseParams, run_infobase};

    #[test]
    fn reports_ambiguity_symbol_info() {
        let outcome =
            run_infobase(InfobaseParams { symbol: 'n' }).expect("infobase should execute");
        assert_eq!(outcome.info.symbol, 'N');
        assert_eq!(outcome.info.canonical_expansion, "ACGTU");
        assert_eq!(outcome.info.dna_complement, "N");
    }

    #[test]
    fn rejects_unknown_symbols() {
        let error =
            run_infobase(InfobaseParams { symbol: 'Z' }).expect_err("unknown base should fail");
        assert!(error.to_string().contains("unsupported nucleotide symbol"));
    }
}
