//! Small deterministic pattern parsing and scanning helpers.
//!
//! This module intentionally supports a narrow v1 scope:
//! - nucleotide patterns with IUPAC ambiguity symbols
//! - protein patterns with exact residues plus `X` wildcard
//! - forward linear scanning with all matches reported

use std::error::Error;
use std::fmt::{Display, Formatter};

/// Pattern parsing or validation failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PatternError {
    /// Empty patterns are not allowed.
    EmptyPattern,
    /// The nucleotide pattern contained an unsupported symbol.
    InvalidNucleotideSymbol(char),
    /// The protein pattern contained an unsupported symbol.
    InvalidProteinSymbol(char),
}

impl Display for PatternError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyPattern => write!(f, "pattern must not be empty"),
            Self::InvalidNucleotideSymbol(symbol) => {
                write!(f, "invalid nucleotide pattern symbol '{symbol}'")
            }
            Self::InvalidProteinSymbol(symbol) => {
                write!(f, "invalid protein pattern symbol '{symbol}'")
            }
        }
    }
}

impl Error for PatternError {}

/// One deterministic pattern hit in a linear sequence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatternMatch {
    start: usize,
    end: usize,
    matched: String,
}

impl PatternMatch {
    /// Creates a pattern match with zero-based half-open coordinates.
    #[must_use]
    pub fn new(start: usize, end: usize, matched: impl Into<String>) -> Self {
        Self {
            start,
            end,
            matched: matched.into(),
        }
    }

    /// Returns the zero-based inclusive start position.
    #[must_use]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the zero-based half-open end position.
    #[must_use]
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns the matched input slice.
    #[must_use]
    pub fn matched(&self) -> &str {
        &self.matched
    }
}

/// Parsed nucleotide pattern with IUPAC ambiguity support.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NucleotidePattern {
    raw: String,
    allowed_sets: Vec<u8>,
}

impl NucleotidePattern {
    /// Parses a nucleotide pattern.
    pub fn parse(pattern: &str) -> Result<Self, PatternError> {
        let raw = pattern.trim().to_ascii_uppercase();
        if raw.is_empty() {
            return Err(PatternError::EmptyPattern);
        }

        let allowed_sets = raw
            .chars()
            .map(iupac_nucleotide_mask)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { raw, allowed_sets })
    }

    /// Returns the original normalized pattern text.
    #[must_use]
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Returns the pattern length in symbols.
    #[must_use]
    pub fn len(&self) -> usize {
        self.allowed_sets.len()
    }

    /// Returns whether the pattern contains zero symbols.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.allowed_sets.is_empty()
    }

    /// Scans a sequence and returns all forward matches.
    #[must_use]
    pub fn scan(&self, sequence: &str) -> Vec<PatternMatch> {
        let normalized = sequence.to_ascii_uppercase();
        let bytes = normalized.as_bytes();
        let pattern_len = self.len();
        if pattern_len == 0 || bytes.len() < pattern_len {
            return Vec::new();
        }

        (0..=bytes.len() - pattern_len)
            .filter_map(|start| {
                let slice = &bytes[start..start + pattern_len];
                let matches = slice
                    .iter()
                    .zip(&self.allowed_sets)
                    .all(|(symbol, allowed)| nucleotide_symbol_matches(*symbol, *allowed));
                matches.then(|| {
                    PatternMatch::new(
                        start,
                        start + pattern_len,
                        std::str::from_utf8(slice)
                            .expect("normalized sequence slices are ASCII")
                            .to_owned(),
                    )
                })
            })
            .collect()
    }
}

/// Parsed protein pattern with exact symbols plus `X` wildcard.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProteinPattern {
    raw: String,
    symbols: Vec<ProteinPatternSymbol>,
}

impl ProteinPattern {
    /// Parses a protein pattern.
    pub fn parse(pattern: &str) -> Result<Self, PatternError> {
        let raw = pattern.trim().to_ascii_uppercase();
        if raw.is_empty() {
            return Err(PatternError::EmptyPattern);
        }

        let symbols = raw
            .chars()
            .map(ProteinPatternSymbol::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { raw, symbols })
    }

    /// Returns the original normalized pattern text.
    #[must_use]
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Returns the pattern length in symbols.
    #[must_use]
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Returns whether the pattern contains zero symbols.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    /// Scans a protein sequence and returns all forward matches.
    #[must_use]
    pub fn scan(&self, sequence: &str) -> Vec<PatternMatch> {
        let normalized = sequence.to_ascii_uppercase();
        let bytes = normalized.as_bytes();
        let pattern_len = self.len();
        if pattern_len == 0 || bytes.len() < pattern_len {
            return Vec::new();
        }

        (0..=bytes.len() - pattern_len)
            .filter_map(|start| {
                let slice = &bytes[start..start + pattern_len];
                let matches = slice
                    .iter()
                    .zip(&self.symbols)
                    .all(|(symbol, token)| token.matches(char::from(*symbol)));
                matches.then(|| {
                    PatternMatch::new(
                        start,
                        start + pattern_len,
                        std::str::from_utf8(slice)
                            .expect("normalized sequence slices are ASCII")
                            .to_owned(),
                    )
                })
            })
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ProteinPatternSymbol {
    Exact(char),
    Wildcard,
}

impl ProteinPatternSymbol {
    fn parse(symbol: char) -> Result<Self, PatternError> {
        let symbol = symbol.to_ascii_uppercase();
        match symbol {
            'X' => Ok(Self::Wildcard),
            'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N'
            | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'Y' | 'Z' | '*' => {
                Ok(Self::Exact(symbol))
            }
            other => Err(PatternError::InvalidProteinSymbol(other)),
        }
    }

    fn matches(&self, symbol: char) -> bool {
        match self {
            Self::Wildcard => true,
            Self::Exact(expected) => symbol.to_ascii_uppercase() == *expected,
        }
    }
}

const A_MASK: u8 = 0b0001;
const C_MASK: u8 = 0b0010;
const G_MASK: u8 = 0b0100;
const T_MASK: u8 = 0b1000;

fn iupac_nucleotide_mask(symbol: char) -> Result<u8, PatternError> {
    let symbol = symbol.to_ascii_uppercase();
    match symbol {
        'A' => Ok(A_MASK),
        'C' => Ok(C_MASK),
        'G' => Ok(G_MASK),
        'T' | 'U' => Ok(T_MASK),
        'R' => Ok(A_MASK | G_MASK),
        'Y' => Ok(C_MASK | T_MASK),
        'S' => Ok(C_MASK | G_MASK),
        'W' => Ok(A_MASK | T_MASK),
        'K' => Ok(G_MASK | T_MASK),
        'M' => Ok(A_MASK | C_MASK),
        'B' => Ok(C_MASK | G_MASK | T_MASK),
        'D' => Ok(A_MASK | G_MASK | T_MASK),
        'H' => Ok(A_MASK | C_MASK | T_MASK),
        'V' => Ok(A_MASK | C_MASK | G_MASK),
        'N' => Ok(A_MASK | C_MASK | G_MASK | T_MASK),
        other => Err(PatternError::InvalidNucleotideSymbol(other)),
    }
}

fn nucleotide_symbol_matches(symbol: u8, allowed_mask: u8) -> bool {
    let symbol_mask = match symbol {
        b'A' => A_MASK,
        b'C' => C_MASK,
        b'G' => G_MASK,
        b'T' | b'U' => T_MASK,
        b'R' => A_MASK | G_MASK,
        b'Y' => C_MASK | T_MASK,
        b'S' => C_MASK | G_MASK,
        b'W' => A_MASK | T_MASK,
        b'K' => G_MASK | T_MASK,
        b'M' => A_MASK | C_MASK,
        b'B' => C_MASK | G_MASK | T_MASK,
        b'D' => A_MASK | G_MASK | T_MASK,
        b'H' => A_MASK | C_MASK | T_MASK,
        b'V' => A_MASK | C_MASK | G_MASK,
        b'N' => A_MASK | C_MASK | G_MASK | T_MASK,
        _ => return false,
    };

    symbol_mask & !allowed_mask == 0
}

#[cfg(test)]
mod tests {
    use super::{NucleotidePattern, PatternError, ProteinPattern};

    #[test]
    fn scans_exact_nucleotide_matches() {
        let pattern = NucleotidePattern::parse("ACGT").expect("pattern should parse");
        let hits = pattern.scan("ACGTACGT");
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].start(), 0);
        assert_eq!(hits[1].matched(), "ACGT");
    }

    #[test]
    fn supports_iupac_nucleotide_ambiguity() {
        let pattern = NucleotidePattern::parse("ATN").expect("pattern should parse");
        let hits = pattern.scan("ATGATN");
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].matched(), "ATG");
        assert_eq!(hits[1].matched(), "ATN");
    }

    #[test]
    fn rejects_invalid_nucleotide_pattern() {
        let error =
            NucleotidePattern::parse("AT?").expect_err("invalid nucleotide pattern should fail");
        assert_eq!(error, PatternError::InvalidNucleotideSymbol('?'));
    }

    #[test]
    fn scans_protein_patterns_with_x_wildcard() {
        let pattern = ProteinPattern::parse("MX").expect("pattern should parse");
        let hits = pattern.scan("MAMQ");
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].matched(), "MA");
        assert_eq!(hits[1].matched(), "MQ");
    }
}
