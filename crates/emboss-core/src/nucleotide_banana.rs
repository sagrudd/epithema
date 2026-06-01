//! Bounded per-base B-DNA bendability and curvature profiles for `banana`.

use std::f64::consts::FRAC_PI_2;

use crate::sequence::SequenceRecord;

const TWIST_RADIANS: f64 = 34.3_f64.to_radians();
const LOCAL_BEND_BRACKET: usize = 1;
const CURVATURE_BRACKET: usize = 15;
const AVERAGING_RADIUS: usize = 5;
const LOCAL_BEND_SCALE: f64 = 2.1;

const ROLL: [[[f64; 4]; 4]; 4] = [
    [
        [0.0, 0.6, 4.2, 3.0],
        [2.3, 0.6, 5.4, 4.3],
        [2.7, 4.7, 4.4, 6.1],
        [4.2, 4.7, 4.4, 4.4],
    ],
    [
        [1.6, 2.3, 1.8, 3.0],
        [1.6, 0.0, 2.7, 2.4],
        [4.4, 4.2, 4.4, 4.9],
        [4.4, 2.7, 6.7, 3.1],
    ],
    [
        [2.4, 4.3, 4.4, 4.4],
        [3.0, 3.0, 5.3, 4.4],
        [3.1, 4.4, 4.9, 8.1],
        [4.9, 6.1, 6.1, 8.1],
    ],
    [
        [2.7, 5.4, 3.4, 5.3],
        [1.8, 4.2, 3.4, 4.4],
        [6.7, 4.4, 3.8, 6.1],
        [4.4, 4.4, 3.8, 4.9],
    ],
];

/// Errors for `banana` profile computation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NucleotideBananaError {
    /// The input sequence is not nucleotide-like.
    NonNucleotideSequence,
    /// The sequence is too short to form even one scored trimer.
    SequenceTooShort {
        /// Input sequence length.
        sequence_length: usize,
        /// Minimum supported sequence length.
        minimum_length: usize,
    },
    /// The bounded v1 model only supports canonical A/C/G/T residues, with U
    /// treated as T.
    UnsupportedResidue {
        /// One-based residue position.
        position: usize,
        /// Unsupported residue symbol.
        residue: char,
    },
}

impl std::fmt::Display for NucleotideBananaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonNucleotideSequence => {
                write!(f, "banana requires a nucleotide sequence input")
            }
            Self::SequenceTooShort {
                sequence_length,
                minimum_length,
            } => write!(
                f,
                "sequence length {sequence_length} is shorter than the minimum supported length {minimum_length}"
            ),
            Self::UnsupportedResidue { position, residue } => write!(
                f,
                "banana requires canonical DNA-like residues in v1: unsupported residue '{residue}' at position {position}"
            ),
        }
    }
}

impl std::error::Error for NucleotideBananaError {}

/// One bounded analytical `banana` row.
#[derive(Clone, Debug, PartialEq)]
pub struct BananaPoint {
    /// One-based sequence position.
    pub position: usize,
    /// Residue at the profiled position.
    pub residue: char,
    /// Local bend magnitude at the position, when defined by the bounded model.
    pub local_bend: Option<f64>,
    /// Macroscopic curvature at the position, when defined by the bounded model.
    pub curvature: Option<f64>,
}

/// Full bounded `banana` profile for one nucleotide sequence.
#[derive(Clone, Debug, PartialEq)]
pub struct NucleotideBananaProfile {
    /// Stable sequence identifier.
    pub identifier: String,
    /// Sequence length.
    pub sequence_length: usize,
    /// Bend bracket in residues.
    pub local_bend_bracket: usize,
    /// Curvature bracket in residues.
    pub curvature_bracket: usize,
    /// Smoothing radius used before curvature calculation.
    pub smoothing_radius: usize,
    /// Per-base analytical rows.
    pub points: Vec<BananaPoint>,
}

/// Computes a deterministic bounded `banana` profile for one nucleotide
/// sequence using the default EMBOSS trimer-angle model.
pub fn nucleotide_banana_profile(
    record: &SequenceRecord,
) -> Result<NucleotideBananaProfile, NucleotideBananaError> {
    if !record.molecule().is_nucleotide() || !record.alphabet().is_nucleotide() {
        return Err(NucleotideBananaError::NonNucleotideSequence);
    }
    if record.len() < 3 {
        return Err(NucleotideBananaError::SequenceTooShort {
            sequence_length: record.len(),
            minimum_length: 3,
        });
    }

    let indexed_residues = normalize_residues(record)?;
    let sequence_length = indexed_residues.len();

    let mut x = vec![0.0; sequence_length + 1];
    let mut y = vec![0.0; sequence_length + 1];
    let mut xave = vec![0.0; sequence_length + 1];
    let mut yave = vec![0.0; sequence_length + 1];
    let mut bend = vec![None; sequence_length + 1];
    let mut curve = vec![None; sequence_length + 1];

    let mut twist_sum = 0.0;
    for position in 1..=(sequence_length - 2) {
        let left = indexed_residues[position - 1].1;
        let center = indexed_residues[position].1;
        let right = indexed_residues[position + 1].1;
        twist_sum += TWIST_RADIANS;
        let roll = ROLL[left][center][right];
        let dx = roll * twist_sum.sin();
        let dy = roll * (twist_sum - FRAC_PI_2).sin();
        x[position + 1] = x[position] + dx;
        y[position + 1] = y[position] + dy;
    }

    if sequence_length > (2 * AVERAGING_RADIUS + 1) {
        for position in (AVERAGING_RADIUS + 1)..=(sequence_length - AVERAGING_RADIUS) {
            let mut xsum = 0.0;
            let mut ysum = 0.0;
            for offset in -4isize..=4 {
                let index = (position as isize + offset) as usize;
                xsum += x[index];
                ysum += y[index];
            }
            xsum += 0.5 * (x[position + 5] + x[position - 5]);
            ysum += 0.5 * (y[position + 5] + y[position - 5]);
            xave[position] = xsum * 0.1;
            yave[position] = ysum * 0.1;
        }
    }

    if sequence_length > (2 * LOCAL_BEND_BRACKET + 1) {
        for position in (LOCAL_BEND_BRACKET + 1)..=(sequence_length - LOCAL_BEND_BRACKET - 1) {
            let delta_x = x[position + LOCAL_BEND_BRACKET] - x[position - LOCAL_BEND_BRACKET];
            let delta_y = y[position + LOCAL_BEND_BRACKET] - y[position - LOCAL_BEND_BRACKET];
            bend[position] = Some(delta_x.hypot(delta_y) * LOCAL_BEND_SCALE);
        }
    }

    let curvature_margin = CURVATURE_BRACKET + AVERAGING_RADIUS + 1;
    if sequence_length >= curvature_margin * 2 {
        for position in curvature_margin..=(sequence_length - curvature_margin + 1) {
            let delta_x =
                xave[position + CURVATURE_BRACKET] - xave[position - CURVATURE_BRACKET];
            let delta_y =
                yave[position + CURVATURE_BRACKET] - yave[position - CURVATURE_BRACKET];
            curve[position] = Some(delta_x.hypot(delta_y));
        }
    }

    Ok(NucleotideBananaProfile {
        identifier: record.identifier().accession().to_owned(),
        sequence_length,
        local_bend_bracket: LOCAL_BEND_BRACKET,
        curvature_bracket: CURVATURE_BRACKET,
        smoothing_radius: AVERAGING_RADIUS,
        points: indexed_residues
            .into_iter()
            .enumerate()
            .map(|(index, (residue, _))| {
                let position = index + 1;
                BananaPoint {
                    position,
                    residue,
                    local_bend: bend[position],
                    curvature: curve[position],
                }
            })
            .collect(),
    })
}

fn normalize_residues(
    record: &SequenceRecord,
) -> Result<Vec<(char, usize)>, NucleotideBananaError> {
    record
        .residues()
        .chars()
        .enumerate()
        .map(|(index, residue)| {
            let mapped = match residue {
                'A' => 0,
                'T' | 'U' => 1,
                'G' => 2,
                'C' => 3,
                _ => {
                    return Err(NucleotideBananaError::UnsupportedResidue {
                        position: index + 1,
                        residue,
                    });
                }
            };
            Ok((residue, mapped))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{MoleculeKind, SequenceIdentifier, SequenceRecord};

    use super::{NucleotideBananaError, nucleotide_banana_profile};

    #[test]
    fn computes_expected_banana_profile_for_canonical_dna() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("banana1").expect("valid identifier"),
            MoleculeKind::Dna,
            "AAAAAGGGGGCCCCCTTTTTAAAAAGGGGGCCCCCTTTTTAAAAA",
        )
        .expect("dna sequence should build");

        let profile = nucleotide_banana_profile(&record).expect("profile should compute");
        assert_eq!(profile.points.len(), 45);
        assert_eq!(profile.local_bend_bracket, 1);
        assert_eq!(profile.curvature_bracket, 15);
        assert_eq!(profile.smoothing_radius, 5);

        assert_eq!(profile.points[0].position, 1);
        assert_eq!(profile.points[0].residue, 'A');
        assert_eq!(profile.points[0].local_bend, None);
        assert_eq!(profile.points[0].curvature, None);

        let tenth = &profile.points[9];
        assert_eq!(tenth.position, 10);
        assert!((tenth.local_bend.expect("bend should be defined") - 32.507336499296).abs() < 1e-9);
        assert_eq!(tenth.curvature, None);

        let twenty_first = &profile.points[20];
        assert!((twenty_first.local_bend.expect("bend should be defined") - 3.36).abs() < 1e-9);
        assert!((twenty_first.curvature.expect("curvature should be defined") - 10.48310492061).abs() < 1e-9);
    }

    #[test]
    fn treats_uracil_as_thymine_in_bounded_model() {
        let dna = SequenceRecord::new(
            SequenceIdentifier::new("banana2_dna").expect("valid identifier"),
            MoleculeKind::Dna,
            "ATGTTTATGTTTATGTTTATGTTTATGTTTATGTTTATGTTT",
        )
        .expect("dna sequence should build");
        let rna = SequenceRecord::new(
            SequenceIdentifier::new("banana2_rna").expect("valid identifier"),
            MoleculeKind::Rna,
            "AUGUUUAUGUUUAUGUUUAUGUUUAUGUUUAUGUUUAUGUUU",
        )
        .expect("rna sequence should build");

        let dna_profile = nucleotide_banana_profile(&dna).expect("dna profile should compute");
        let rna_profile = nucleotide_banana_profile(&rna).expect("rna profile should compute");

        assert_eq!(dna_profile.points.len(), rna_profile.points.len());
        for (dna_point, rna_point) in dna_profile.points.iter().zip(&rna_profile.points) {
            assert_eq!(dna_point.position, rna_point.position);
            assert_eq!(dna_point.local_bend, rna_point.local_bend);
            assert_eq!(dna_point.curvature, rna_point.curvature);
        }
    }

    #[test]
    fn rejects_non_nucleotide_records() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("banana3").expect("valid identifier"),
            MoleculeKind::Protein,
            "MSTNPKPQR",
        )
        .expect("protein sequence should build");

        assert_eq!(
            nucleotide_banana_profile(&record),
            Err(NucleotideBananaError::NonNucleotideSequence)
        );
    }

    #[test]
    fn rejects_ambiguous_residues_in_v1_model() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("banana4").expect("valid identifier"),
            MoleculeKind::Dna,
            "AAANGGG",
        )
        .expect("ambiguous nucleotide sequence should still build");

        assert_eq!(
            nucleotide_banana_profile(&record),
            Err(NucleotideBananaError::UnsupportedResidue {
                position: 4,
                residue: 'N',
            })
        );
    }

    #[test]
    fn rejects_sequences_shorter_than_one_scored_trimer() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("banana5").expect("valid identifier"),
            MoleculeKind::Dna,
            "AT",
        )
        .expect("dna sequence should build");

        assert_eq!(
            nucleotide_banana_profile(&record),
            Err(NucleotideBananaError::SequenceTooShort {
                sequence_length: 2,
                minimum_length: 3,
            })
        );
    }
}
