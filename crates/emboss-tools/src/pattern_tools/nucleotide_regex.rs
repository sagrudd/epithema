//! Bounded nucleotide regular-expression support for `dreg`.

use emboss_diagnostics::{ErrorCategory, PlatformError};
use regex::Regex;

use crate::sequence_stream::ToolExecutionError;

/// Compiled bounded nucleotide regular expression.
#[derive(Clone, Debug)]
pub struct CompiledNucleotideRegex {
    raw: String,
    regex: Regex,
}

impl CompiledNucleotideRegex {
    /// Parses a bounded nucleotide regular expression.
    pub fn parse(pattern: &str) -> Result<Self, ToolExecutionError> {
        let raw = pattern.trim().to_owned();
        if raw.is_empty() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "nucleotide regular expression must not be empty",
            )
            .with_code("tools.dreg.pattern.empty"));
        }

        let regex = Regex::new(&format!("(?i:{raw})")).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Validation,
                format!("invalid nucleotide regular expression: {error}"),
            )
            .with_code("tools.dreg.pattern.invalid")
            .with_detail(raw.clone())
        })?;

        if regex.is_match("") {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "nucleotide regular expression must consume at least one residue",
            )
            .with_code("tools.dreg.pattern.empty_match")
            .with_detail(raw.clone()));
        }

        Ok(Self { raw, regex })
    }

    /// Returns the normalized original pattern text.
    #[must_use]
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Returns all overlapping forward matches in a nucleotide sequence.
    #[must_use]
    pub fn scan(&self, sequence: &str) -> Vec<NucleotideRegexMatch> {
        let normalized = sequence.to_ascii_uppercase();
        let mut hits = Vec::new();
        let mut search_start = 0usize;

        while search_start < normalized.len() {
            let Some(found) = self.regex.find_at(&normalized, search_start) else {
                break;
            };

            hits.push(NucleotideRegexMatch {
                start: found.start(),
                end: found.end(),
                matched: found.as_str().to_owned(),
            });
            search_start = found.start() + 1;
        }

        hits
    }
}

/// One regex match in a nucleotide sequence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NucleotideRegexMatch {
    start: usize,
    end: usize,
    matched: String,
}

impl NucleotideRegexMatch {
    /// Zero-based inclusive start coordinate.
    #[must_use]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Zero-based half-open end coordinate.
    #[must_use]
    pub fn end(&self) -> usize {
        self.end
    }

    /// Matched normalized sequence slice.
    #[must_use]
    pub fn matched(&self) -> &str {
        &self.matched
    }
}
