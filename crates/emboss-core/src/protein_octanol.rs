//! Sliding-window White-Wimley octanol profiles for plot-producing analytical tools.

use crate::sequence::SequenceRecord;
use crate::Alphabet;

/// Errors for octanol-profile computation.
#[derive(Clone, Debug, PartialEq)]
pub enum ProteinOctanolError {
    /// The input sequence is not protein-like.
    NonProteinSequence,
    /// The sequence contains a residue outside the supported v1 octanol model.
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

impl std::fmt::Display for ProteinOctanolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonProteinSequence => write!(f, "octanol requires a protein sequence input"),
            Self::UnsupportedResidue { residue, position } => write!(
                f,
                "unsupported protein residue '{residue}' at position {} for octanol calculation",
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

impl std::error::Error for ProteinOctanolError {}

/// One sliding-window White-Wimley difference row.
#[derive(Clone, Debug, PartialEq)]
pub struct OctanolWindow {
    /// One-based inclusive window start.
    pub window_start: usize,
    /// One-based inclusive window end.
    pub window_end: usize,
    /// Window length in residues.
    pub window_length: usize,
    /// Windowed White-Wimley interface-minus-octanol free-energy sum.
    pub interface_minus_octanol: f64,
}

/// Full White-Wimley difference profile for one protein sequence.
#[derive(Clone, Debug, PartialEq)]
pub struct ProteinOctanolProfile {
    /// Stable sequence identifier.
    pub identifier: String,
    /// Sequence length.
    pub sequence_length: usize,
    /// Window length.
    pub window: usize,
    /// Step size.
    pub step: usize,
    /// Sliding-window rows.
    pub windows: Vec<OctanolWindow>,
}

/// Computes a deterministic sliding-window White-Wimley interface-minus-octanol profile.
pub fn protein_octanol_profile(
    record: &SequenceRecord,
    window: usize,
    step: usize,
) -> Result<ProteinOctanolProfile, ProteinOctanolError> {
    if !record.molecule().is_protein() || record.alphabet() != Alphabet::Protein {
        return Err(ProteinOctanolError::NonProteinSequence);
    }
    if window == 0 {
        return Err(ProteinOctanolError::InvalidWindow { window });
    }
    if step == 0 {
        return Err(ProteinOctanolError::InvalidStep { step });
    }
    if record.len() < window {
        return Err(ProteinOctanolError::SequenceShorterThanWindow {
            sequence_length: record.len(),
            window,
        });
    }

    let residue_difference = residue_interface_minus_octanol(record.residues())?;
    let mut windows = Vec::new();
    let last_start = record.len() - window;
    let mut start = 0usize;
    while start <= last_start {
        let slice = &residue_difference[start..start + window];
        let interface_minus_octanol = slice.iter().sum::<f64>();
        windows.push(OctanolWindow {
            window_start: start + 1,
            window_end: start + window,
            window_length: window,
            interface_minus_octanol,
        });
        start += step;
    }

    Ok(ProteinOctanolProfile {
        identifier: record.identifier().accession().to_owned(),
        sequence_length: record.len(),
        window,
        step,
        windows,
    })
}

fn residue_interface_minus_octanol(residues: &str) -> Result<Vec<f64>, ProteinOctanolError> {
    residues
        .chars()
        .enumerate()
        .map(|(position, residue)| {
            let score = match residue {
                // White-Wimley interface and octanol values from EMBOSS Ewhite-wimley.dat.
                'A' => 0.17 - 0.50,
                'R' => 0.81 - 1.81,
                'N' => 0.42 - 0.85,
                'D' => 1.23 - 3.64,
                'C' => -0.24 - -0.02,
                'Q' => 0.58 - 0.77,
                'E' => 2.02 - 3.63,
                'G' => 0.01 - 1.15,
                'H' => 0.96 - 2.33,
                'I' => -0.31 - -1.12,
                'L' => -0.56 - -1.25,
                'K' => 0.99 - 2.80,
                'M' => -0.23 - -0.67,
                'F' => -1.13 - -1.71,
                'P' => 0.45 - 0.14,
                'S' => 0.13 - 0.46,
                'T' => 0.14 - 0.25,
                'W' => -1.85 - -2.09,
                'Y' => -0.94 - -0.71,
                'V' => 0.07 - -0.46,
                _ => {
                    return Err(ProteinOctanolError::UnsupportedResidue { residue, position });
                }
            };
            Ok(score)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{MoleculeKind, SequenceIdentifier, SequenceRecord};

    use super::{protein_octanol_profile, ProteinOctanolError};

    #[test]
    fn computes_expected_profile() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("octanol1").expect("valid identifier"),
            MoleculeKind::Protein,
            "ACDEVL",
        )
        .expect("protein sequence should build");

        let profile = protein_octanol_profile(&record, 3, 1).expect("profile should compute");
        assert_eq!(profile.windows.len(), 4);
        assert!((profile.windows[0].interface_minus_octanol - -2.96).abs() < 1e-12);
        assert!((profile.windows[1].interface_minus_octanol - -4.24).abs() < 1e-12);
        assert!((profile.windows[2].interface_minus_octanol - -3.49).abs() < 1e-12);
        assert!((profile.windows[3].interface_minus_octanol - -0.39).abs() < 1e-12);
    }

    #[test]
    fn rejects_invalid_window() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("octanol2").expect("valid identifier"),
            MoleculeKind::Protein,
            "ACDEVL",
        )
        .expect("protein sequence should build");

        assert!(matches!(
            protein_octanol_profile(&record, 0, 1),
            Err(ProteinOctanolError::InvalidWindow { .. })
        ));
    }

    #[test]
    fn rejects_unsupported_residue() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("octanol3").expect("valid identifier"),
            MoleculeKind::Protein,
            "ACXEVL",
        )
        .expect("protein sequence should build");

        let error =
            protein_octanol_profile(&record, 3, 1).expect_err("unsupported residue should fail");
        assert_eq!(
            error,
            ProteinOctanolError::UnsupportedResidue {
                residue: 'X',
                position: 2,
            }
        );
    }

    #[test]
    fn rejects_short_sequences() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("octanol4").expect("valid identifier"),
            MoleculeKind::Protein,
            "ACD",
        )
        .expect("protein sequence should build");

        assert!(matches!(
            protein_octanol_profile(&record, 5, 1),
            Err(ProteinOctanolError::SequenceShorterThanWindow { .. })
        ));
    }
}
