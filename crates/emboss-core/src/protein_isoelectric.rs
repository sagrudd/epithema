//! Deterministic protein isoelectric-point estimation for retained protein-property tools.

use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::residue_properties::protein_residue_property;

const N_TERM_PKA: f64 = 9.69;
const C_TERM_PKA: f64 = 2.34;
const D_PKA: f64 = 3.86;
const E_PKA: f64 = 4.25;
const C_PKA: f64 = 8.33;
const Y_PKA: f64 = 10.07;
const H_PKA: f64 = 6.00;
const K_PKA: f64 = 10.53;
const R_PKA: f64 = 12.48;

/// Titratable residue counts used by the v1 pI estimator.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TitratableResidueCounts {
    /// Aspartate count.
    pub aspartate: usize,
    /// Glutamate count.
    pub glutamate: usize,
    /// Cysteine count.
    pub cysteine: usize,
    /// Tyrosine count.
    pub tyrosine: usize,
    /// Histidine count.
    pub histidine: usize,
    /// Lysine count.
    pub lysine: usize,
    /// Arginine count.
    pub arginine: usize,
}

impl TitratableResidueCounts {
    /// Total titratable side chains counted in the sequence.
    #[must_use]
    pub fn total_side_chains(&self) -> usize {
        self.aspartate
            + self.glutamate
            + self.cysteine
            + self.tyrosine
            + self.histidine
            + self.lysine
            + self.arginine
    }
}

/// Stable per-sequence pI estimate output.
#[derive(Clone, Debug, PartialEq)]
pub struct ProteinIsoelectricEstimate {
    /// Estimated isoelectric point from the fixed v1 pKa model.
    pub isoelectric_point: f64,
    /// Net charge at pH 7.0 from the same fixed model.
    pub net_charge_at_ph7: f64,
    /// Count of titratable residues used during estimation.
    pub titratable_counts: TitratableResidueCounts,
    /// Number of residues contributing to the estimate.
    pub residue_length: usize,
}

/// Errors raised by deterministic pI estimation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProteinIsoelectricError {
    /// The sequence contains an unsupported residue.
    UnsupportedResidue {
        /// The unsupported residue symbol after normalization.
        residue: char,
        /// The 1-based residue position in the normalized ungapped sequence.
        position: usize,
    },
}

impl Display for ProteinIsoelectricError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedResidue { residue, position } => write!(
                f,
                "unsupported protein residue '{residue}' at position {position} for pI estimation"
            ),
        }
    }
}

impl Error for ProteinIsoelectricError {}

/// Estimates pI and net charge from a fixed, explicit v1 pKa model.
pub fn estimate_protein_isoelectric_point(
    sequence: &str,
) -> Result<ProteinIsoelectricEstimate, ProteinIsoelectricError> {
    let normalized = normalized_protein_residues(sequence)?;
    let counts = titratable_counts(&normalized);
    let isoelectric_point = solve_isoelectric_point(&counts);
    let net_charge_at_ph7 = net_charge(&counts, 7.0);

    Ok(ProteinIsoelectricEstimate {
        isoelectric_point,
        net_charge_at_ph7,
        titratable_counts: counts,
        residue_length: normalized.len(),
    })
}

fn normalized_protein_residues(sequence: &str) -> Result<Vec<char>, ProteinIsoelectricError> {
    let mut residues = Vec::new();
    for (index, residue) in sequence.chars().enumerate() {
        let residue = residue.to_ascii_uppercase();
        match residue {
            '-' | '*' => continue,
            other => {
                if protein_residue_property(other).is_none() {
                    return Err(ProteinIsoelectricError::UnsupportedResidue {
                        residue: other,
                        position: index + 1,
                    });
                }
                residues.push(other);
            }
        }
    }
    Ok(residues)
}

fn titratable_counts(residues: &[char]) -> TitratableResidueCounts {
    let mut counts = TitratableResidueCounts::default();
    for residue in residues {
        match residue {
            'D' => counts.aspartate += 1,
            'E' => counts.glutamate += 1,
            'C' => counts.cysteine += 1,
            'Y' => counts.tyrosine += 1,
            'H' => counts.histidine += 1,
            'K' => counts.lysine += 1,
            'R' => counts.arginine += 1,
            _ => {}
        }
    }
    counts
}

fn solve_isoelectric_point(counts: &TitratableResidueCounts) -> f64 {
    let mut low = 0.0_f64;
    let mut high = 14.0_f64;
    for _ in 0..128 {
        let mid = (low + high) / 2.0;
        let charge = net_charge(counts, mid);
        if charge > 0.0 {
            low = mid;
        } else {
            high = mid;
        }
    }
    (low + high) / 2.0
}

fn net_charge(counts: &TitratableResidueCounts, ph: f64) -> f64 {
    let positive = positive_fraction(ph, N_TERM_PKA)
        + counts.histidine as f64 * positive_fraction(ph, H_PKA)
        + counts.lysine as f64 * positive_fraction(ph, K_PKA)
        + counts.arginine as f64 * positive_fraction(ph, R_PKA);

    let negative = negative_fraction(ph, C_TERM_PKA)
        + counts.aspartate as f64 * negative_fraction(ph, D_PKA)
        + counts.glutamate as f64 * negative_fraction(ph, E_PKA)
        + counts.cysteine as f64 * negative_fraction(ph, C_PKA)
        + counts.tyrosine as f64 * negative_fraction(ph, Y_PKA);

    positive - negative
}

fn positive_fraction(ph: f64, pka: f64) -> f64 {
    1.0 / (1.0 + 10_f64.powf(ph - pka))
}

fn negative_fraction(ph: f64, pka: f64) -> f64 {
    1.0 / (1.0 + 10_f64.powf(pka - ph))
}

#[cfg(test)]
mod tests {
    use super::{ProteinIsoelectricError, estimate_protein_isoelectric_point};

    #[test]
    fn estimates_basic_and_acidic_pi_values() {
        let basic = estimate_protein_isoelectric_point("KKKK").expect("estimate ok");
        let acidic = estimate_protein_isoelectric_point("DEDE").expect("estimate ok");

        assert!(basic.isoelectric_point > 9.0);
        assert!(basic.net_charge_at_ph7 > 0.0);
        assert!(acidic.isoelectric_point < 4.5);
        assert!(acidic.net_charge_at_ph7 < 0.0);
    }

    #[test]
    fn ignores_stop_symbols_when_estimating_pi() {
        let estimate = estimate_protein_isoelectric_point("MA*").expect("estimate ok");

        assert_eq!(estimate.residue_length, 2);
        assert!(estimate.isoelectric_point > 5.0);
    }

    #[test]
    fn rejects_unsupported_residues() {
        assert_eq!(
            estimate_protein_isoelectric_point("MZX"),
            Err(ProteinIsoelectricError::UnsupportedResidue {
                residue: 'Z',
                position: 2,
            })
        );
    }
}
