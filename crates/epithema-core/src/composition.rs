//! Small reusable residue-composition and summary-statistics helpers.
//!
//! This module intentionally supports a narrow deterministic v1 scope:
//! - residue counts and frequencies with gap exclusion
//! - GC statistics over canonical nucleotide symbols
//! - protein molecular-weight estimation using average residue masses

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::residue_properties::protein_residue_property;

/// Errors produced by composition/statistics helpers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompositionError {
    /// Protein mass calculation encountered an unsupported residue.
    UnsupportedProteinResidue(char),
}

impl Display for CompositionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedProteinResidue(residue) => {
                write!(
                    f,
                    "protein statistics encountered unsupported residue '{residue}'"
                )
            }
        }
    }
}

impl Error for CompositionError {}

/// Deterministic residue-composition summary.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResidueComposition {
    counts: BTreeMap<char, usize>,
    counted_symbols: usize,
    ignored_gap_symbols: usize,
}

impl ResidueComposition {
    /// Builds a composition summary from sequence content.
    #[must_use]
    pub fn from_sequence(sequence: &str) -> Self {
        let mut summary = Self::default();

        for residue in sequence.chars().map(|residue| residue.to_ascii_uppercase()) {
            if residue == '-' {
                summary.ignored_gap_symbols += 1;
                continue;
            }

            *summary.counts.entry(residue).or_insert(0) += 1;
            summary.counted_symbols += 1;
        }

        summary
    }

    /// Returns counted residues in stable sorted order.
    #[must_use]
    pub fn counts(&self) -> &BTreeMap<char, usize> {
        &self.counts
    }

    /// Returns the number of non-gap symbols counted.
    #[must_use]
    pub fn counted_symbols(&self) -> usize {
        self.counted_symbols
    }

    /// Returns the number of ignored gap symbols.
    #[must_use]
    pub fn ignored_gap_symbols(&self) -> usize {
        self.ignored_gap_symbols
    }

    /// Returns the count for a residue, defaulting to zero.
    #[must_use]
    pub fn count_for(&self, residue: char) -> usize {
        self.counts
            .get(&residue.to_ascii_uppercase())
            .copied()
            .unwrap_or_default()
    }

    /// Returns the frequency for a residue among counted non-gap symbols.
    #[must_use]
    pub fn frequency_for(&self, residue: char) -> f64 {
        if self.counted_symbols == 0 {
            return 0.0;
        }

        self.count_for(residue) as f64 / self.counted_symbols as f64
    }

    /// Merges another composition summary into this one.
    pub fn merge(&mut self, other: &Self) {
        for (residue, count) in &other.counts {
            *self.counts.entry(*residue).or_insert(0) += count;
        }
        self.counted_symbols += other.counted_symbols;
        self.ignored_gap_symbols += other.ignored_gap_symbols;
    }
}

/// Deterministic GC summary over canonical nucleotide residues.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GcSummary {
    /// Total input symbols excluding gaps.
    pub counted_symbols: usize,
    /// Canonical A/C/G/T/U symbols contributing to the denominator.
    pub canonical_symbols: usize,
    /// Canonical G/C symbols contributing to the numerator.
    pub gc_symbols: usize,
    /// Non-gap ambiguous or otherwise non-canonical symbols.
    pub ambiguous_symbols: usize,
    /// Ignored gap symbols.
    pub ignored_gap_symbols: usize,
}

impl GcSummary {
    /// Builds GC statistics from a sequence.
    #[must_use]
    pub fn from_sequence(sequence: &str) -> Self {
        let mut summary = Self::default();

        for residue in sequence.chars().map(|residue| residue.to_ascii_uppercase()) {
            if residue == '-' {
                summary.ignored_gap_symbols += 1;
                continue;
            }

            summary.counted_symbols += 1;
            match residue {
                'G' | 'C' => {
                    summary.canonical_symbols += 1;
                    summary.gc_symbols += 1;
                }
                'A' | 'T' | 'U' => {
                    summary.canonical_symbols += 1;
                }
                _ => {
                    summary.ambiguous_symbols += 1;
                }
            }
        }

        summary
    }

    /// Returns GC percentage over canonical symbols only.
    #[must_use]
    pub fn gc_percent(&self) -> f64 {
        if self.canonical_symbols == 0 {
            return 0.0;
        }

        self.gc_symbols as f64 * 100.0 / self.canonical_symbols as f64
    }

    /// Merges another GC summary into this one.
    pub fn merge(&mut self, other: &Self) {
        self.counted_symbols += other.counted_symbols;
        self.canonical_symbols += other.canonical_symbols;
        self.gc_symbols += other.gc_symbols;
        self.ambiguous_symbols += other.ambiguous_symbols;
        self.ignored_gap_symbols += other.ignored_gap_symbols;
    }
}

/// Estimates protein molecular weight using average residue masses plus water.
pub fn protein_molecular_weight(sequence: &str) -> Result<f64, CompositionError> {
    let mut residue_count = 0usize;
    let mut mass = 18.015_28_f64;

    for residue in sequence.chars().map(|residue| residue.to_ascii_uppercase()) {
        match residue {
            '-' | '*' => {}
            other => {
                let property = protein_residue_property(other)
                    .ok_or(CompositionError::UnsupportedProteinResidue(other))?;
                mass += property.average_mass;
                residue_count += 1;
            }
        }
    }

    if residue_count == 0 {
        Ok(0.0)
    } else {
        Ok(mass)
    }
}

#[cfg(test)]
mod tests {
    use super::{GcSummary, ResidueComposition, protein_molecular_weight};

    #[test]
    fn counts_residues_without_gaps() {
        let summary = ResidueComposition::from_sequence("ACG-TN");
        assert_eq!(summary.counted_symbols(), 5);
        assert_eq!(summary.ignored_gap_symbols(), 1);
        assert_eq!(summary.count_for('A'), 1);
        assert_eq!(summary.count_for('N'), 1);
        assert!((summary.frequency_for('T') - 0.2).abs() < 1e-9);
    }

    #[test]
    fn computes_gc_over_canonical_bases_only() {
        let summary = GcSummary::from_sequence("ACGTN-");
        assert_eq!(summary.counted_symbols, 5);
        assert_eq!(summary.canonical_symbols, 4);
        assert_eq!(summary.gc_symbols, 2);
        assert_eq!(summary.ambiguous_symbols, 1);
        assert_eq!(summary.ignored_gap_symbols, 1);
        assert!((summary.gc_percent() - 50.0).abs() < 1e-9);
    }

    #[test]
    fn estimates_average_protein_mass() {
        let mass = protein_molecular_weight("MA*").expect("mass should compute");
        assert!((mass - 220.286_68).abs() < 1e-6);
    }
}
