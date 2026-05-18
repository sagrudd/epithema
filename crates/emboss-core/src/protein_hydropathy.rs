//! Sliding-window protein hydropathy profiles for plot-producing analytical tools.

use crate::Alphabet;
use crate::sequence::SequenceRecord;

/// Errors for hydropathy-profile computation.
#[derive(Clone, Debug, PartialEq)]
pub enum ProteinHydropathyError {
    /// The input sequence is not protein-like.
    NonProteinSequence,
    /// The sequence contains a residue outside the supported v1 hydropathy model.
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

impl std::fmt::Display for ProteinHydropathyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonProteinSequence => write!(f, "pepwindow requires a protein sequence input"),
            Self::UnsupportedResidue { residue, position } => write!(
                f,
                "unsupported protein residue '{residue}' at position {} for hydropathy calculation",
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

impl std::error::Error for ProteinHydropathyError {}

/// One sliding-window mean-hydropathy row.
#[derive(Clone, Debug, PartialEq)]
pub struct HydropathyWindow {
    /// One-based inclusive window start.
    pub window_start: usize,
    /// One-based inclusive window end.
    pub window_end: usize,
    /// Window length in residues.
    pub window_length: usize,
    /// Mean hydropathy across the window.
    pub mean_hydropathy: f64,
}

/// Full hydropathy profile for one protein sequence.
#[derive(Clone, Debug, PartialEq)]
pub struct ProteinHydropathyProfile {
    /// Stable sequence identifier.
    pub identifier: String,
    /// Sequence length.
    pub sequence_length: usize,
    /// Window length.
    pub window: usize,
    /// Step size.
    pub step: usize,
    /// Sliding-window rows.
    pub windows: Vec<HydropathyWindow>,
}

/// Computes a deterministic sliding-window Kyte-Doolittle hydropathy profile.
pub fn protein_hydropathy_profile(
    record: &SequenceRecord,
    window: usize,
    step: usize,
) -> Result<ProteinHydropathyProfile, ProteinHydropathyError> {
    if !record.molecule().is_protein() || record.alphabet() != Alphabet::Protein {
        return Err(ProteinHydropathyError::NonProteinSequence);
    }
    if window == 0 {
        return Err(ProteinHydropathyError::InvalidWindow { window });
    }
    if step == 0 {
        return Err(ProteinHydropathyError::InvalidStep { step });
    }
    if record.len() < window {
        return Err(ProteinHydropathyError::SequenceShorterThanWindow {
            sequence_length: record.len(),
            window,
        });
    }

    let hydropathy = residue_hydropathy(record.residues())?;
    let mut windows = Vec::new();
    let last_start = record.len() - window;
    let mut start = 0usize;
    while start <= last_start {
        let slice = &hydropathy[start..start + window];
        let mean_hydropathy = slice.iter().sum::<f64>() / window as f64;
        windows.push(HydropathyWindow {
            window_start: start + 1,
            window_end: start + window,
            window_length: window,
            mean_hydropathy,
        });
        start += step;
    }

    Ok(ProteinHydropathyProfile {
        identifier: record.identifier().accession().to_owned(),
        sequence_length: record.len(),
        window,
        step,
        windows,
    })
}

fn residue_hydropathy(residues: &str) -> Result<Vec<f64>, ProteinHydropathyError> {
    residues
        .chars()
        .enumerate()
        .map(|(position, residue)| {
            let score = match residue {
                'I' => 4.5,
                'V' => 4.2,
                'L' => 3.8,
                'F' => 2.8,
                'C' => 2.5,
                'M' => 1.9,
                'A' => 1.8,
                'G' => -0.4,
                'T' => -0.7,
                'S' => -0.8,
                'W' => -0.9,
                'Y' => -1.3,
                'P' => -1.6,
                'H' => -3.2,
                'E' | 'Q' | 'D' | 'N' => -3.5,
                'K' => -3.9,
                'R' => -4.5,
                _ => {
                    return Err(ProteinHydropathyError::UnsupportedResidue { residue, position });
                }
            };
            Ok(score)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{MoleculeKind, SequenceIdentifier, SequenceRecord};

    use super::{ProteinHydropathyError, protein_hydropathy_profile};

    #[test]
    fn computes_expected_profile() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("pepwindow1").expect("valid identifier"),
            MoleculeKind::Protein,
            "IVLF",
        )
        .expect("protein sequence should build");

        let profile =
            protein_hydropathy_profile(&record, 2, 1).expect("profile should compute");
        assert_eq!(profile.windows.len(), 3);
        assert!((profile.windows[0].mean_hydropathy - 4.35).abs() < 1e-9);
        assert!((profile.windows[1].mean_hydropathy - 4.0).abs() < 1e-9);
        assert!((profile.windows[2].mean_hydropathy - 3.3).abs() < 1e-9);
    }

    #[test]
    fn rejects_unsupported_residue() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("pepwindow2").expect("valid identifier"),
            MoleculeKind::Protein,
            "IVXLF",
        )
        .expect("protein sequence should build");

        let error = protein_hydropathy_profile(&record, 3, 1)
            .expect_err("unsupported residue should fail");
        assert_eq!(
            error,
            ProteinHydropathyError::UnsupportedResidue {
                residue: 'X',
                position: 2,
            }
        );
    }

    #[test]
    fn rejects_short_sequences() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("pepwindow3").expect("valid identifier"),
            MoleculeKind::Protein,
            "IVL",
        )
        .expect("protein sequence should build");

        assert!(matches!(
            protein_hydropathy_profile(&record, 5, 1),
            Err(ProteinHydropathyError::SequenceShorterThanWindow { .. })
        ));
    }
}
