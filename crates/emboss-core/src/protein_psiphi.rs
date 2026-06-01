//! Bounded phi/psi torsion-angle profiles for `psiphi`.

use std::collections::HashMap;

/// Errors for bounded `psiphi` profile computation.
#[derive(Clone, Debug, PartialEq)]
pub enum ProteinPsiphiError {
    /// No parsable protein backbone atoms were found in the input text.
    NoBackboneAtoms,
    /// A required numeric coordinate field could not be parsed.
    InvalidCoordinateField {
        /// One-based input line number.
        line_number: usize,
        /// Stable field label.
        field: &'static str,
    },
    /// A required residue sequence field could not be parsed.
    InvalidResidueNumber {
        /// One-based input line number.
        line_number: usize,
    },
}

impl std::fmt::Display for ProteinPsiphiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoBackboneAtoms => write!(
                f,
                "psiphi requires PDB ATOM input with at least one protein backbone atom"
            ),
            Self::InvalidCoordinateField { line_number, field } => write!(
                f,
                "invalid {field} coordinate field in PDB ATOM line {line_number}"
            ),
            Self::InvalidResidueNumber { line_number } => {
                write!(f, "invalid residue number in PDB ATOM line {line_number}")
            }
        }
    }
}

impl std::error::Error for ProteinPsiphiError {}

/// One bounded `psiphi` analytical row for one residue.
#[derive(Clone, Debug, PartialEq)]
pub struct PsiPhiResidue {
    /// One-based residue ordinal in the retained chain/residue order.
    pub ordinal: usize,
    /// Optional single-character chain identifier.
    pub chain_id: Option<char>,
    /// Three-letter residue name as observed in the coordinate input.
    pub residue_name: String,
    /// PDB residue sequence number.
    pub residue_number: i32,
    /// Optional insertion code.
    pub insertion_code: Option<char>,
    /// Whether a backbone `N` atom was retained for the residue.
    pub has_backbone_n: bool,
    /// Whether a backbone `CA` atom was retained for the residue.
    pub has_backbone_ca: bool,
    /// Whether a backbone `C` atom was retained for the residue.
    pub has_backbone_c: bool,
    /// Whether the previous residue is treated as chain-contiguous for bounded v1 torsion use.
    pub previous_contiguous: bool,
    /// Whether the next residue is treated as chain-contiguous for bounded v1 torsion use.
    pub next_contiguous: bool,
    /// Deterministic phi torsion angle in degrees when computable.
    pub phi_degrees: Option<f64>,
    /// Deterministic psi torsion angle in degrees when computable.
    pub psi_degrees: Option<f64>,
}

/// Full bounded `psiphi` profile for one coordinate input.
#[derive(Clone, Debug, PartialEq)]
pub struct ProteinPsiphiProfile {
    /// Count of residues retained into the analytical surface.
    pub residue_count: usize,
    /// Count of residues with computable phi angles.
    pub phi_count: usize,
    /// Count of residues with computable psi angles.
    pub psi_count: usize,
    /// Per-residue analytical rows in stable chain/residue order.
    pub residues: Vec<PsiPhiResidue>,
}

/// Computes a deterministic bounded `psiphi` profile from PDB `ATOM` coordinate text.
///
/// The bounded v1 core:
/// - retains only backbone `N`, `CA`, and `C` atoms from `ATOM` records
/// - treats only blank or `A` alternate-location atoms as eligible
/// - computes continuity only across same-chain, sequential, insertion-code-free residues
/// - leaves torsions as `None` when continuity or backbone atoms are missing
pub fn protein_psiphi_profile(
    pdb_atom_text: &str,
) -> Result<ProteinPsiphiProfile, ProteinPsiphiError> {
    let residues = parse_backbone_residues(pdb_atom_text)?;
    if residues.is_empty() {
        return Err(ProteinPsiphiError::NoBackboneAtoms);
    }

    let mut rows = Vec::with_capacity(residues.len());
    let mut phi_count = 0usize;
    let mut psi_count = 0usize;

    for (index, residue) in residues.iter().enumerate() {
        let previous = index.checked_sub(1).and_then(|prev| residues.get(prev));
        let next = residues.get(index + 1);
        let previous_contiguous = previous.is_some_and(|prev| residues_are_contiguous(prev, residue));
        let next_contiguous = next.is_some_and(|next| residues_are_contiguous(residue, next));

        let phi_degrees = if previous_contiguous {
            previous.and_then(|prev| {
                torsion_angle_degrees(prev.c?, residue.n?, residue.ca?, residue.c?)
            })
        } else {
            None
        };
        let psi_degrees = if next_contiguous {
            next.and_then(|next| {
                torsion_angle_degrees(residue.n?, residue.ca?, residue.c?, next.n?)
            })
        } else {
            None
        };

        if phi_degrees.is_some() {
            phi_count += 1;
        }
        if psi_degrees.is_some() {
            psi_count += 1;
        }

        rows.push(PsiPhiResidue {
            ordinal: index + 1,
            chain_id: residue.chain_id,
            residue_name: residue.residue_name.clone(),
            residue_number: residue.residue_number,
            insertion_code: residue.insertion_code,
            has_backbone_n: residue.n.is_some(),
            has_backbone_ca: residue.ca.is_some(),
            has_backbone_c: residue.c.is_some(),
            previous_contiguous,
            next_contiguous,
            phi_degrees,
            psi_degrees,
        });
    }

    Ok(ProteinPsiphiProfile {
        residue_count: rows.len(),
        phi_count,
        psi_count,
        residues: rows,
    })
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct ResidueKey {
    chain_id: Option<char>,
    residue_number: i32,
    insertion_code: Option<char>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, Debug)]
struct ResidueBackbone {
    residue_name: String,
    chain_id: Option<char>,
    residue_number: i32,
    insertion_code: Option<char>,
    n: Option<Point3>,
    ca: Option<Point3>,
    c: Option<Point3>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum AltLocationPriority {
    Blank,
    A,
}

impl AltLocationPriority {
    fn from_char(alt: char) -> Option<Self> {
        match alt {
            ' ' => Some(Self::Blank),
            'A' => Some(Self::A),
            _ => None,
        }
    }
}

fn parse_backbone_residues(input: &str) -> Result<Vec<ResidueBackbone>, ProteinPsiphiError> {
    let mut residues = Vec::<ResidueBackbone>::new();
    let mut indexes = HashMap::<ResidueKey, usize>::new();
    let mut atom_priorities =
        HashMap::<(ResidueKey, &'static str), AltLocationPriority>::new();

    for (line_index, line) in input.lines().enumerate() {
        let line_number = line_index + 1;
        if !line.starts_with("ATOM") {
            continue;
        }
        if line.len() < 54 {
            continue;
        }

        let atom_name = slice(line, 12, 16).trim();
        let atom_label = match atom_name {
            "N" => "N",
            "CA" => "CA",
            "C" => "C",
            _ => continue,
        };

        let alt_loc = line.as_bytes()[16] as char;
        let Some(priority) = AltLocationPriority::from_char(alt_loc) else {
            continue;
        };

        let residue_name = slice(line, 17, 20).trim().to_owned();
        let chain_id = normalize_char(line.as_bytes()[21] as char);
        let residue_number = slice(line, 22, 26)
            .trim()
            .parse::<i32>()
            .map_err(|_| ProteinPsiphiError::InvalidResidueNumber { line_number })?;
        let insertion_code = normalize_char(line.as_bytes()[26] as char);
        let key = ResidueKey {
            chain_id,
            residue_number,
            insertion_code,
        };

        let point = Point3 {
            x: parse_coordinate(slice(line, 30, 38), line_number, "x")?,
            y: parse_coordinate(slice(line, 38, 46), line_number, "y")?,
            z: parse_coordinate(slice(line, 46, 54), line_number, "z")?,
        };

        let residue_index = *indexes.entry(key).or_insert_with(|| {
            residues.push(ResidueBackbone {
                residue_name,
                chain_id,
                residue_number,
                insertion_code,
                n: None,
                ca: None,
                c: None,
            });
            residues.len() - 1
        });

        let priority_key = (key, atom_label);
        if atom_priorities
            .get(&priority_key)
            .is_some_and(|existing| *existing <= priority)
        {
            continue;
        }
        atom_priorities.insert(priority_key, priority);

        let residue = &mut residues[residue_index];
        match atom_label {
            "N" => residue.n = Some(point),
            "CA" => residue.ca = Some(point),
            "C" => residue.c = Some(point),
            _ => unreachable!(),
        }
    }

    residues.retain(|residue| residue.n.is_some() || residue.ca.is_some() || residue.c.is_some());
    residues.sort_by_key(|residue| {
        (
            residue.chain_id.unwrap_or(' '),
            residue.residue_number,
            residue.insertion_code.unwrap_or(' '),
        )
    });
    Ok(residues)
}

fn residues_are_contiguous(previous: &ResidueBackbone, current: &ResidueBackbone) -> bool {
    previous.chain_id == current.chain_id
        && previous.insertion_code.is_none()
        && current.insertion_code.is_none()
        && current.residue_number == previous.residue_number + 1
}

fn torsion_angle_degrees(a: Point3, b: Point3, c: Point3, d: Point3) -> Option<f64> {
    let b1 = b.sub(a);
    let b2 = c.sub(b);
    let b3 = d.sub(c);
    let n1 = b1.cross(b2);
    let n2 = b2.cross(b3);
    let b2_norm = b2.norm();
    let n1_norm = n1.norm();
    let n2_norm = n2.norm();
    if b2_norm <= 1e-12 || n1_norm <= 1e-12 || n2_norm <= 1e-12 {
        return None;
    }

    let m1 = n1.cross(b2.scale(1.0 / b2_norm));
    let x = n1.dot(n2);
    let y = m1.dot(n2);
    Some(y.atan2(x).to_degrees())
}

fn slice(line: &str, start: usize, end: usize) -> &str {
    line.get(start..end).unwrap_or("")
}

fn normalize_char(ch: char) -> Option<char> {
    match ch {
        ' ' => None,
        other => Some(other),
    }
}

fn parse_coordinate(
    value: &str,
    line_number: usize,
    field: &'static str,
) -> Result<f64, ProteinPsiphiError> {
    value
        .trim()
        .parse::<f64>()
        .map_err(|_| ProteinPsiphiError::InvalidCoordinateField { line_number, field })
}

impl Point3 {
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn scale(self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn norm(self) -> f64 {
        self.dot(self).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::{ProteinPsiphiError, protein_psiphi_profile};

    #[test]
    fn computes_expected_phi_psi_angles_for_contiguous_backbone() {
        let pdb = concat!(
            "ATOM      1  N   GLY A   1       0.000   0.000   0.000\n",
            "ATOM      2  CA  GLY A   1       0.700   1.000   0.200\n",
            "ATOM      3  C   GLY A   1       1.500   1.200   0.800\n",
            "ATOM      4  N   ALA A   2       2.400   1.900   1.100\n",
            "ATOM      5  CA  ALA A   2       3.600   1.400   1.700\n",
            "ATOM      6  C   ALA A   2       4.300   2.300   2.800\n",
            "ATOM      7  N   SER A   3       5.600   1.900   3.100\n",
            "ATOM      8  CA  SER A   3       6.200   2.700   4.200\n",
            "ATOM      9  C   SER A   3       7.500   2.100   4.600\n",
        );

        let profile = protein_psiphi_profile(pdb).expect("profile should compute cleanly");
        assert_eq!(profile.residue_count, 3);
        assert_eq!(profile.phi_count, 2);
        assert_eq!(profile.psi_count, 2);

        let first = &profile.residues[0];
        assert_eq!(first.residue_name, "GLY");
        assert_eq!(first.previous_contiguous, false);
        assert_eq!(first.next_contiguous, true);
        assert_eq!(first.phi_degrees, None);
        assert!(first.psi_degrees.is_some());

        let second = &profile.residues[1];
        assert_eq!(second.residue_name, "ALA");
        assert!(second.previous_contiguous);
        assert!(second.next_contiguous);
        assert!((second.phi_degrees.expect("phi should exist") - 143.70145061638846).abs() < 1e-9);
        assert!((second.psi_degrees.expect("psi should exist") - 165.84642141919093).abs() < 1e-9);

        let third = &profile.residues[2];
        assert_eq!(third.residue_name, "SER");
        assert!(third.previous_contiguous);
        assert!(!third.next_contiguous);
        assert!((third.phi_degrees.expect("phi should exist") + 171.89498089918422).abs() < 1e-9);
        assert_eq!(third.psi_degrees, None);
    }

    #[test]
    fn leaves_angles_absent_when_backbone_atoms_are_missing() {
        let pdb = concat!(
            "ATOM      1  N   GLY A   1       0.000   0.000   0.000\n",
            "ATOM      2  CA  GLY A   1       0.700   1.000   0.200\n",
            "ATOM      3  C   GLY A   1       1.500   1.200   0.800\n",
            "ATOM      4  N   ALA A   2       2.400   1.900   1.100\n",
            "ATOM      5  C   ALA A   2       4.300   2.300   2.800\n",
            "ATOM      6  N   SER A   3       5.600   1.900   3.100\n",
            "ATOM      7  CA  SER A   3       6.200   2.700   4.200\n",
            "ATOM      8  C   SER A   3       7.500   2.100   4.600\n",
        );

        let profile = protein_psiphi_profile(pdb).expect("profile should compute cleanly");
        let second = &profile.residues[1];
        assert!(second.has_backbone_n);
        assert!(!second.has_backbone_ca);
        assert!(second.has_backbone_c);
        assert_eq!(second.phi_degrees, None);
        assert_eq!(second.psi_degrees, None);
    }

    #[test]
    fn marks_numbering_gaps_as_non_contiguous() {
        let pdb = concat!(
            "ATOM      1  N   GLY A   1       0.000   0.000   0.000\n",
            "ATOM      2  CA  GLY A   1       0.700   1.000   0.200\n",
            "ATOM      3  C   GLY A   1       1.500   1.200   0.800\n",
            "ATOM      4  N   ALA A   3       2.400   1.900   1.100\n",
            "ATOM      5  CA  ALA A   3       3.600   1.400   1.700\n",
            "ATOM      6  C   ALA A   3       4.300   2.300   2.800\n",
        );

        let profile = protein_psiphi_profile(pdb).expect("profile should compute cleanly");
        assert_eq!(profile.residue_count, 2);
        assert_eq!(profile.phi_count, 0);
        assert_eq!(profile.psi_count, 0);
        assert!(!profile.residues[1].previous_contiguous);
        assert_eq!(profile.residues[1].phi_degrees, None);
    }

    #[test]
    fn rejects_coordinate_text_without_backbone_atoms() {
        let error =
            protein_psiphi_profile("HEADER    TEST").expect_err("no backbone atoms should fail");
        assert_eq!(error, ProteinPsiphiError::NoBackboneAtoms);
    }

    #[test]
    fn rejects_invalid_numeric_coordinates() {
        let pdb = "ATOM      1  N   GLY A   1       X.000   0.000   0.000\n";
        let error = protein_psiphi_profile(pdb).expect_err("invalid x should fail");
        assert_eq!(
            error,
            ProteinPsiphiError::InvalidCoordinateField {
                line_number: 1,
                field: "x",
            }
        );
    }
}
