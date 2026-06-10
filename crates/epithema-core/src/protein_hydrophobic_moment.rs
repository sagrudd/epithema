//! Sliding-window protein hydrophobic-moment profiles for plot-producing analytical tools.

use crate::Alphabet;
use crate::sequence::SequenceRecord;

/// Default per-residue turn angle used for the bounded v1 hydrophobic-moment model.
pub const DEFAULT_HYDROPHOBIC_MOMENT_ANGLE_DEGREES: f64 = 100.0;

/// Errors for hydrophobic-moment profile computation.
#[derive(Clone, Debug, PartialEq)]
pub enum ProteinHydrophobicMomentError {
    /// The input sequence is not protein-like.
    NonProteinSequence,
    /// The sequence contains a residue outside the supported v1 hydrophobic-moment model.
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
    /// The requested angle is invalid.
    InvalidAngle {
        /// Requested angle in degrees.
        angle_degrees: f64,
    },
    /// The sequence is shorter than the requested window.
    SequenceShorterThanWindow {
        /// Input sequence length.
        sequence_length: usize,
        /// Requested window length.
        window: usize,
    },
}

impl std::fmt::Display for ProteinHydrophobicMomentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonProteinSequence => {
                write!(f, "hmoment requires a protein sequence input")
            }
            Self::UnsupportedResidue { residue, position } => write!(
                f,
                "unsupported protein residue '{residue}' at position {} for hydrophobic-moment calculation",
                position + 1
            ),
            Self::InvalidWindow { window } => {
                write!(f, "window length must be >= 1, got {window}")
            }
            Self::InvalidStep { step } => write!(f, "step size must be >= 1, got {step}"),
            Self::InvalidAngle { angle_degrees } => write!(
                f,
                "hydrophobic-moment angle must be finite and > 0, got {angle_degrees}"
            ),
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

impl std::error::Error for ProteinHydrophobicMomentError {}

/// One sliding-window hydrophobic-moment row.
#[derive(Clone, Debug, PartialEq)]
pub struct HydrophobicMomentWindow {
    /// One-based inclusive window start.
    pub window_start: usize,
    /// One-based inclusive window end.
    pub window_end: usize,
    /// Window length in residues.
    pub window_length: usize,
    /// Hydrophobic moment across the window.
    pub hydrophobic_moment: f64,
}

/// Full hydrophobic-moment profile for one protein sequence.
#[derive(Clone, Debug, PartialEq)]
pub struct ProteinHydrophobicMomentProfile {
    /// Stable sequence identifier.
    pub identifier: String,
    /// Sequence length.
    pub sequence_length: usize,
    /// Window length.
    pub window: usize,
    /// Step size.
    pub step: usize,
    /// Per-residue turn angle in degrees.
    pub angle_degrees: f64,
    /// Sliding-window rows.
    pub windows: Vec<HydrophobicMomentWindow>,
}

/// Computes a deterministic sliding-window protein hydrophobic-moment profile.
pub fn protein_hydrophobic_moment_profile(
    record: &SequenceRecord,
    window: usize,
    step: usize,
    angle_degrees: f64,
) -> Result<ProteinHydrophobicMomentProfile, ProteinHydrophobicMomentError> {
    if !record.molecule().is_protein() || record.alphabet() != Alphabet::Protein {
        return Err(ProteinHydrophobicMomentError::NonProteinSequence);
    }
    if window == 0 {
        return Err(ProteinHydrophobicMomentError::InvalidWindow { window });
    }
    if step == 0 {
        return Err(ProteinHydrophobicMomentError::InvalidStep { step });
    }
    if !angle_degrees.is_finite() || angle_degrees <= 0.0 {
        return Err(ProteinHydrophobicMomentError::InvalidAngle { angle_degrees });
    }
    if record.len() < window {
        return Err(ProteinHydrophobicMomentError::SequenceShorterThanWindow {
            sequence_length: record.len(),
            window,
        });
    }

    let hydrophobicity = residue_hydrophobicity(record.residues())?;
    let angle_radians = angle_degrees.to_radians();
    let mut windows = Vec::new();
    let last_start = record.len() - window;
    let mut start = 0usize;
    while start <= last_start {
        let slice = &hydrophobicity[start..start + window];
        let mut x = 0.0;
        let mut y = 0.0;
        for (offset, score) in slice.iter().enumerate() {
            let angle = angle_radians * offset as f64;
            x += score * angle.cos();
            y += score * angle.sin();
        }
        let hydrophobic_moment = (x * x + y * y).sqrt() / window as f64;
        windows.push(HydrophobicMomentWindow {
            window_start: start + 1,
            window_end: start + window,
            window_length: window,
            hydrophobic_moment,
        });
        start += step;
    }

    Ok(ProteinHydrophobicMomentProfile {
        identifier: record.identifier().accession().to_owned(),
        sequence_length: record.len(),
        window,
        step,
        angle_degrees,
        windows,
    })
}

fn residue_hydrophobicity(residues: &str) -> Result<Vec<f64>, ProteinHydrophobicMomentError> {
    residues
        .chars()
        .enumerate()
        .map(|(position, residue)| {
            let score = match residue {
                'A' => 0.62,
                'R' => -2.53,
                'N' => -0.78,
                'D' => -0.90,
                'C' => 0.29,
                'Q' => -0.85,
                'E' => -0.74,
                'G' => 0.48,
                'H' => -0.40,
                'I' => 1.38,
                'L' => 1.06,
                'K' => -1.50,
                'M' => 0.64,
                'F' => 1.19,
                'P' => 0.12,
                'S' => -0.18,
                'T' => -0.05,
                'W' => 0.81,
                'Y' => 0.26,
                'V' => 1.08,
                _ => {
                    return Err(ProteinHydrophobicMomentError::UnsupportedResidue {
                        residue,
                        position,
                    });
                }
            };
            Ok(score)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{MoleculeKind, SequenceIdentifier, SequenceRecord};

    use super::{
        DEFAULT_HYDROPHOBIC_MOMENT_ANGLE_DEGREES, ProteinHydrophobicMomentError,
        protein_hydrophobic_moment_profile,
    };

    #[test]
    fn computes_expected_profile() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("hmoment1").expect("valid identifier"),
            MoleculeKind::Protein,
            "AKLVR",
        )
        .expect("protein sequence should build");

        let profile = protein_hydrophobic_moment_profile(
            &record,
            4,
            1,
            DEFAULT_HYDROPHOBIC_MOMENT_ANGLE_DEGREES,
        )
        .expect("profile should compute");
        assert_eq!(profile.windows.len(), 2);
        assert!((profile.windows[0].hydrophobic_moment - 0.701_831_267_935_911_7).abs() < 1e-12);
        assert!((profile.windows[1].hydrophobic_moment - 1.222_809_478_488_536_8).abs() < 1e-12);
    }

    #[test]
    fn rejects_invalid_angle() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("hmoment2").expect("valid identifier"),
            MoleculeKind::Protein,
            "AKLVR",
        )
        .expect("protein sequence should build");

        assert!(matches!(
            protein_hydrophobic_moment_profile(&record, 4, 1, 0.0),
            Err(ProteinHydrophobicMomentError::InvalidAngle { .. })
        ));
    }

    #[test]
    fn rejects_unsupported_residue() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("hmoment3").expect("valid identifier"),
            MoleculeKind::Protein,
            "AKXVR",
        )
        .expect("protein sequence should build");

        let error = protein_hydrophobic_moment_profile(
            &record,
            4,
            1,
            DEFAULT_HYDROPHOBIC_MOMENT_ANGLE_DEGREES,
        )
        .expect_err("unsupported residue should fail");
        assert_eq!(
            error,
            ProteinHydrophobicMomentError::UnsupportedResidue {
                residue: 'X',
                position: 2,
            }
        );
    }

    #[test]
    fn rejects_non_protein_sequences() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("hmoment4").expect("valid identifier"),
            MoleculeKind::Dna,
            "ACGT",
        )
        .expect("dna sequence should build");

        assert_eq!(
            protein_hydrophobic_moment_profile(
                &record,
                3,
                1,
                DEFAULT_HYDROPHOBIC_MOMENT_ANGLE_DEGREES
            ),
            Err(ProteinHydrophobicMomentError::NonProteinSequence)
        );
    }
}
