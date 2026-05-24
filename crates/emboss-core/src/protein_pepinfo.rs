//! Sliding-window bounded multi-property protein profiles for `pepinfo`.

use crate::residue_properties::protein_residue_property;
use crate::sequence::SequenceRecord;
use crate::Alphabet;

/// Errors for `pepinfo` profile computation.
#[derive(Clone, Debug, PartialEq)]
pub enum ProteinPepinfoError {
    /// The input sequence is not protein-like.
    NonProteinSequence,
    /// The sequence contains a residue outside the supported v1 `pepinfo` model.
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

impl std::fmt::Display for ProteinPepinfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonProteinSequence => write!(f, "pepinfo requires a protein sequence input"),
            Self::UnsupportedResidue { residue, position } => write!(
                f,
                "unsupported protein residue '{residue}' at position {} for pepinfo calculation",
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

impl std::error::Error for ProteinPepinfoError {}

/// One sliding-window bounded multi-property row for `pepinfo`.
#[derive(Clone, Debug, PartialEq)]
pub struct PepinfoWindow {
    /// One-based inclusive window start.
    pub window_start: usize,
    /// One-based inclusive window end.
    pub window_end: usize,
    /// Window length in residues.
    pub window_length: usize,
    /// Mean Kyte-Doolittle hydropathy across the window.
    pub mean_hydropathy: f64,
    /// Mean residue mass across the window.
    pub mean_residue_mass: f64,
    /// Fraction of charged residues across the window.
    pub charged_fraction: f64,
    /// Fraction of polar/basic/acidic residues across the window.
    pub polar_fraction: f64,
}

/// Full bounded `pepinfo` profile for one protein sequence.
#[derive(Clone, Debug, PartialEq)]
pub struct ProteinPepinfoProfile {
    /// Stable sequence identifier.
    pub identifier: String,
    /// Sequence length.
    pub sequence_length: usize,
    /// Window length.
    pub window: usize,
    /// Step size.
    pub step: usize,
    /// Sliding-window rows.
    pub windows: Vec<PepinfoWindow>,
}

/// Computes a deterministic sliding-window bounded multi-property `pepinfo` profile.
pub fn protein_pepinfo_profile(
    record: &SequenceRecord,
    window: usize,
    step: usize,
) -> Result<ProteinPepinfoProfile, ProteinPepinfoError> {
    if !record.molecule().is_protein() || record.alphabet() != Alphabet::Protein {
        return Err(ProteinPepinfoError::NonProteinSequence);
    }
    if window == 0 {
        return Err(ProteinPepinfoError::InvalidWindow { window });
    }
    if step == 0 {
        return Err(ProteinPepinfoError::InvalidStep { step });
    }
    if record.len() < window {
        return Err(ProteinPepinfoError::SequenceShorterThanWindow {
            sequence_length: record.len(),
            window,
        });
    }

    let residue_metrics = residue_metrics(record.residues())?;
    let mut windows = Vec::new();
    let last_start = record.len() - window;
    let mut start = 0usize;
    while start <= last_start {
        let slice = &residue_metrics[start..start + window];
        let mean_hydropathy =
            slice.iter().map(|metric| metric.hydropathy).sum::<f64>() / window as f64;
        let mean_residue_mass =
            slice.iter().map(|metric| metric.average_mass).sum::<f64>() / window as f64;
        let charged_fraction = slice
            .iter()
            .map(|metric| f64::from(metric.is_charged))
            .sum::<f64>()
            / window as f64;
        let polar_fraction = slice
            .iter()
            .map(|metric| f64::from(metric.is_polar))
            .sum::<f64>()
            / window as f64;
        windows.push(PepinfoWindow {
            window_start: start + 1,
            window_end: start + window,
            window_length: window,
            mean_hydropathy,
            mean_residue_mass,
            charged_fraction,
            polar_fraction,
        });
        start += step;
    }

    Ok(ProteinPepinfoProfile {
        identifier: record.identifier().accession().to_owned(),
        sequence_length: record.len(),
        window,
        step,
        windows,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ResiduePepinfoMetrics {
    hydropathy: f64,
    average_mass: f64,
    is_charged: bool,
    is_polar: bool,
}

fn residue_metrics(residues: &str) -> Result<Vec<ResiduePepinfoMetrics>, ProteinPepinfoError> {
    residues
        .chars()
        .enumerate()
        .map(|(position, residue)| {
            let property = protein_residue_property(residue)
                .ok_or(ProteinPepinfoError::UnsupportedResidue { residue, position })?;
            Ok(ResiduePepinfoMetrics {
                hydropathy: property.hydropathy,
                average_mass: property.average_mass,
                is_charged: property.charge_class != "neutral",
                is_polar: matches!(property.polarity_class, "polar" | "basic" | "acidic"),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{MoleculeKind, SequenceIdentifier, SequenceRecord};

    use super::{protein_pepinfo_profile, ProteinPepinfoError};

    #[test]
    fn computes_expected_multi_property_profile() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("pepinfo1").expect("valid identifier"),
            MoleculeKind::Protein,
            "AKDE",
        )
        .expect("protein sequence should build");

        let profile = protein_pepinfo_profile(&record, 2, 1).expect("profile should compute");
        assert_eq!(profile.windows.len(), 3);

        let first = &profile.windows[0];
        assert_eq!(first.window_start, 1);
        assert_eq!(first.window_end, 2);
        assert!((first.mean_hydropathy - -1.05).abs() < 1e-9);
        assert!((first.mean_residue_mass - 99.626_45).abs() < 1e-9);
        assert!((first.charged_fraction - 0.5).abs() < 1e-9);
        assert!((first.polar_fraction - 0.5).abs() < 1e-9);

        let second = &profile.windows[1];
        assert!((second.mean_hydropathy - -3.7).abs() < 1e-9);
        assert!((second.mean_residue_mass - 121.631_35).abs() < 1e-9);
        assert!((second.charged_fraction - 1.0).abs() < 1e-9);
        assert!((second.polar_fraction - 1.0).abs() < 1e-9);

        let third = &profile.windows[2];
        assert!((third.mean_hydropathy - -3.5).abs() < 1e-9);
        assert!((third.mean_residue_mass - 122.102_05).abs() < 1e-9);
        assert!((third.charged_fraction - 1.0).abs() < 1e-9);
        assert!((third.polar_fraction - 1.0).abs() < 1e-9);
    }

    #[test]
    fn rejects_unsupported_residue() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("pepinfo2").expect("valid identifier"),
            MoleculeKind::Protein,
            "AKXE",
        )
        .expect("protein sequence should build");

        let error =
            protein_pepinfo_profile(&record, 2, 1).expect_err("unsupported residue should fail");
        assert_eq!(
            error,
            ProteinPepinfoError::UnsupportedResidue {
                residue: 'X',
                position: 2,
            }
        );
    }

    #[test]
    fn rejects_short_sequences() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("pepinfo3").expect("valid identifier"),
            MoleculeKind::Protein,
            "AK",
        )
        .expect("protein sequence should build");

        assert!(matches!(
            protein_pepinfo_profile(&record, 3, 1),
            Err(ProteinPepinfoError::SequenceShorterThanWindow { .. })
        ));
    }
}
