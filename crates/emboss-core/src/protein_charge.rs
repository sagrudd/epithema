//! Sliding-window protein charge profiles for plot-producing analytical tools.

use crate::Alphabet;
use crate::sequence::SequenceRecord;

/// Errors for charge-profile computation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProteinChargeError {
    /// The input sequence is not protein-like.
    NonProteinSequence,
    /// The sequence contains a residue outside the supported v1 charge model.
    UnsupportedResidue {
        /// Unsupported residue symbol.
        residue: char,
        /// Zero-based residue position.
        position: usize,
    },
    /// The requested window length is invalid.
    InvalidWindow {
        /// Requested window length.
        window: usize,
    },
    /// The requested step size is invalid.
    InvalidStep {
        /// Requested step size.
        step: usize,
    },
    /// The sequence is shorter than the requested window.
    SequenceShorterThanWindow {
        /// Input sequence length.
        sequence_length: usize,
        /// Requested window length.
        window: usize,
    },
}

impl std::fmt::Display for ProteinChargeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonProteinSequence => write!(f, "charge requires a protein sequence input"),
            Self::UnsupportedResidue { residue, position } => write!(
                f,
                "unsupported protein residue '{residue}' at position {} for charge calculation",
                position + 1
            ),
            Self::InvalidWindow { window } => {
                write!(f, "window length must be >= 1, got {window}")
            }
            Self::InvalidStep { step } => write!(f, "step size must be >= 1, got {step}"),
            Self::SequenceShorterThanWindow {
                sequence_length,
                window,
            } => write!(
                f,
                "sequence length {sequence_length} is shorter than requested window {window}"
            ),
        }
    }
}

impl std::error::Error for ProteinChargeError {}

/// One sliding-window mean-charge row.
#[derive(Clone, Debug, PartialEq)]
pub struct ChargeWindow {
    /// One-based inclusive window start.
    pub window_start: usize,
    /// One-based inclusive window end.
    pub window_end: usize,
    /// Window length in residues.
    pub window_length: usize,
    /// Mean charge across the window.
    pub mean_charge: f64,
}

/// Full charge profile for one protein sequence.
#[derive(Clone, Debug, PartialEq)]
pub struct ProteinChargeProfile {
    /// Stable sequence identifier.
    pub identifier: String,
    /// Sequence length.
    pub sequence_length: usize,
    /// Window length.
    pub window: usize,
    /// Step size.
    pub step: usize,
    /// Sliding-window rows.
    pub windows: Vec<ChargeWindow>,
}

/// Computes a deterministic sliding-window protein charge profile.
pub fn protein_charge_profile(
    record: &SequenceRecord,
    window: usize,
    step: usize,
) -> Result<ProteinChargeProfile, ProteinChargeError> {
    if !record.molecule().is_protein() || record.alphabet() != Alphabet::Protein {
        return Err(ProteinChargeError::NonProteinSequence);
    }
    if window == 0 {
        return Err(ProteinChargeError::InvalidWindow { window });
    }
    if step == 0 {
        return Err(ProteinChargeError::InvalidStep { step });
    }
    if record.len() < window {
        return Err(ProteinChargeError::SequenceShorterThanWindow {
            sequence_length: record.len(),
            window,
        });
    }

    let charges = residue_charges(record.residues())?;
    let mut windows = Vec::new();
    let last_start = record.len() - window;
    let mut start = 0usize;
    while start <= last_start {
        let slice = &charges[start..start + window];
        let mean_charge = slice.iter().sum::<f64>() / window as f64;
        windows.push(ChargeWindow {
            window_start: start + 1,
            window_end: start + window,
            window_length: window,
            mean_charge,
        });
        start += step;
    }

    Ok(ProteinChargeProfile {
        identifier: record.identifier().accession().to_owned(),
        sequence_length: record.len(),
        window,
        step,
        windows,
    })
}

fn residue_charges(residues: &str) -> Result<Vec<f64>, ProteinChargeError> {
    residues
        .chars()
        .enumerate()
        .map(|(position, residue)| {
            let charge = match residue {
                'D' | 'E' => -1.0,
                'K' | 'R' => 1.0,
                'H' => 0.5,
                'A' | 'C' | 'F' | 'G' | 'I' | 'L' | 'M' | 'N' | 'P' | 'Q' | 'S' | 'T' | 'V'
                | 'W' | 'Y' => 0.0,
                _ => {
                    return Err(ProteinChargeError::UnsupportedResidue { residue, position });
                }
            };
            Ok(charge)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{MoleculeKind, SequenceIdentifier, SequenceRecord};

    use super::{ProteinChargeError, protein_charge_profile};

    #[test]
    fn computes_expected_profile() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("charge1").expect("valid identifier"),
            MoleculeKind::Protein,
            "AKRHDDE",
        )
        .expect("protein sequence should build");

        let profile = protein_charge_profile(&record, 5, 1).expect("profile should compute");
        assert_eq!(profile.windows.len(), 3);
        assert!((profile.windows[0].mean_charge - 0.3).abs() < 1e-9);
        assert!((profile.windows[1].mean_charge - 0.1).abs() < 1e-9);
        assert!((profile.windows[2].mean_charge + 0.3).abs() < 1e-9);
    }

    #[test]
    fn rejects_unsupported_residue() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("charge2").expect("valid identifier"),
            MoleculeKind::Protein,
            "AKXHD",
        )
        .expect("protein sequence should build");

        let error =
            protein_charge_profile(&record, 3, 1).expect_err("unsupported residue should fail");
        assert_eq!(
            error,
            ProteinChargeError::UnsupportedResidue {
                residue: 'X',
                position: 2,
            }
        );
    }

    #[test]
    fn rejects_short_sequences() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("charge3").expect("valid identifier"),
            MoleculeKind::Protein,
            "AKR",
        )
        .expect("protein sequence should build");

        assert!(matches!(
            protein_charge_profile(&record, 5, 1),
            Err(ProteinChargeError::SequenceShorterThanWindow { .. })
        ));
    }
}
