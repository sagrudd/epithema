//! Small shared residue- and base-property lookup tables.
//!
//! The v1 scope is intentionally narrow and deterministic:
//! - canonical amino-acid property rows used by statistics tools
//! - standard IUPAC nucleotide-base and ambiguity-symbol metadata

/// Stable metadata for one amino-acid residue.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProteinResidueProperty {
    /// One-letter amino-acid code.
    pub residue: char,
    /// Three-letter amino-acid code.
    pub three_letter: &'static str,
    /// Full residue name.
    pub name: &'static str,
    /// Average residue mass in Daltons.
    pub average_mass: f64,
    /// Kyte-Doolittle hydropathy score.
    pub hydropathy: f64,
    /// Coarse charge class used by v1 reporting.
    pub charge_class: &'static str,
    /// Coarse polarity class used by v1 reporting.
    pub polarity_class: &'static str,
}

/// Stable metadata for one nucleotide base or ambiguity symbol.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NucleotideBaseInfo {
    /// One-letter symbol.
    pub symbol: char,
    /// Human-readable symbol name.
    pub name: &'static str,
    /// Stable class label.
    pub base_class: &'static str,
    /// Supported molecule space.
    pub supported_molecules: &'static str,
    /// Canonical expansion expressed as A/C/G/T/U symbols.
    pub canonical_expansion: &'static str,
    /// DNA complement symbol.
    pub dna_complement: &'static str,
    /// RNA complement symbol.
    pub rna_complement: &'static str,
}

const PROTEIN_PROPERTIES: [ProteinResidueProperty; 20] = [
    ProteinResidueProperty {
        residue: 'A',
        three_letter: "Ala",
        name: "Alanine",
        average_mass: 71.078_8,
        hydropathy: 1.8,
        charge_class: "neutral",
        polarity_class: "nonpolar",
    },
    ProteinResidueProperty {
        residue: 'R',
        three_letter: "Arg",
        name: "Arginine",
        average_mass: 156.187_5,
        hydropathy: -4.5,
        charge_class: "positive",
        polarity_class: "basic",
    },
    ProteinResidueProperty {
        residue: 'N',
        three_letter: "Asn",
        name: "Asparagine",
        average_mass: 114.103_8,
        hydropathy: -3.5,
        charge_class: "neutral",
        polarity_class: "polar",
    },
    ProteinResidueProperty {
        residue: 'D',
        three_letter: "Asp",
        name: "Aspartic acid",
        average_mass: 115.088_6,
        hydropathy: -3.5,
        charge_class: "negative",
        polarity_class: "acidic",
    },
    ProteinResidueProperty {
        residue: 'C',
        three_letter: "Cys",
        name: "Cysteine",
        average_mass: 103.138_8,
        hydropathy: 2.5,
        charge_class: "neutral",
        polarity_class: "polar",
    },
    ProteinResidueProperty {
        residue: 'E',
        three_letter: "Glu",
        name: "Glutamic acid",
        average_mass: 129.115_5,
        hydropathy: -3.5,
        charge_class: "negative",
        polarity_class: "acidic",
    },
    ProteinResidueProperty {
        residue: 'Q',
        three_letter: "Gln",
        name: "Glutamine",
        average_mass: 128.130_7,
        hydropathy: -3.5,
        charge_class: "neutral",
        polarity_class: "polar",
    },
    ProteinResidueProperty {
        residue: 'G',
        three_letter: "Gly",
        name: "Glycine",
        average_mass: 57.051_9,
        hydropathy: -0.4,
        charge_class: "neutral",
        polarity_class: "nonpolar",
    },
    ProteinResidueProperty {
        residue: 'H',
        three_letter: "His",
        name: "Histidine",
        average_mass: 137.141_1,
        hydropathy: -3.2,
        charge_class: "positive",
        polarity_class: "basic",
    },
    ProteinResidueProperty {
        residue: 'I',
        three_letter: "Ile",
        name: "Isoleucine",
        average_mass: 113.159_4,
        hydropathy: 4.5,
        charge_class: "neutral",
        polarity_class: "nonpolar",
    },
    ProteinResidueProperty {
        residue: 'L',
        three_letter: "Leu",
        name: "Leucine",
        average_mass: 113.159_4,
        hydropathy: 3.8,
        charge_class: "neutral",
        polarity_class: "nonpolar",
    },
    ProteinResidueProperty {
        residue: 'K',
        three_letter: "Lys",
        name: "Lysine",
        average_mass: 128.174_1,
        hydropathy: -3.9,
        charge_class: "positive",
        polarity_class: "basic",
    },
    ProteinResidueProperty {
        residue: 'M',
        three_letter: "Met",
        name: "Methionine",
        average_mass: 131.192_6,
        hydropathy: 1.9,
        charge_class: "neutral",
        polarity_class: "nonpolar",
    },
    ProteinResidueProperty {
        residue: 'F',
        three_letter: "Phe",
        name: "Phenylalanine",
        average_mass: 147.176_6,
        hydropathy: 2.8,
        charge_class: "neutral",
        polarity_class: "aromatic",
    },
    ProteinResidueProperty {
        residue: 'P',
        three_letter: "Pro",
        name: "Proline",
        average_mass: 97.116_7,
        hydropathy: -1.6,
        charge_class: "neutral",
        polarity_class: "nonpolar",
    },
    ProteinResidueProperty {
        residue: 'S',
        three_letter: "Ser",
        name: "Serine",
        average_mass: 87.078_2,
        hydropathy: -0.8,
        charge_class: "neutral",
        polarity_class: "polar",
    },
    ProteinResidueProperty {
        residue: 'T',
        three_letter: "Thr",
        name: "Threonine",
        average_mass: 101.105_1,
        hydropathy: -0.7,
        charge_class: "neutral",
        polarity_class: "polar",
    },
    ProteinResidueProperty {
        residue: 'W',
        three_letter: "Trp",
        name: "Tryptophan",
        average_mass: 186.213_2,
        hydropathy: -0.9,
        charge_class: "neutral",
        polarity_class: "aromatic",
    },
    ProteinResidueProperty {
        residue: 'Y',
        three_letter: "Tyr",
        name: "Tyrosine",
        average_mass: 163.176_0,
        hydropathy: -1.3,
        charge_class: "neutral",
        polarity_class: "aromatic",
    },
    ProteinResidueProperty {
        residue: 'V',
        three_letter: "Val",
        name: "Valine",
        average_mass: 99.132_6,
        hydropathy: 4.2,
        charge_class: "neutral",
        polarity_class: "nonpolar",
    },
];

const NUCLEOTIDE_BASES: [NucleotideBaseInfo; 15] = [
    NucleotideBaseInfo {
        symbol: 'A',
        name: "Adenine",
        base_class: "canonical",
        supported_molecules: "dna,rna",
        canonical_expansion: "A",
        dna_complement: "T",
        rna_complement: "U",
    },
    NucleotideBaseInfo {
        symbol: 'C',
        name: "Cytosine",
        base_class: "canonical",
        supported_molecules: "dna,rna",
        canonical_expansion: "C",
        dna_complement: "G",
        rna_complement: "G",
    },
    NucleotideBaseInfo {
        symbol: 'G',
        name: "Guanine",
        base_class: "canonical",
        supported_molecules: "dna,rna",
        canonical_expansion: "G",
        dna_complement: "C",
        rna_complement: "C",
    },
    NucleotideBaseInfo {
        symbol: 'T',
        name: "Thymine",
        base_class: "canonical",
        supported_molecules: "dna",
        canonical_expansion: "T",
        dna_complement: "A",
        rna_complement: "A",
    },
    NucleotideBaseInfo {
        symbol: 'U',
        name: "Uracil",
        base_class: "canonical",
        supported_molecules: "rna",
        canonical_expansion: "U",
        dna_complement: "A",
        rna_complement: "A",
    },
    NucleotideBaseInfo {
        symbol: 'R',
        name: "Purine",
        base_class: "ambiguity",
        supported_molecules: "dna,rna",
        canonical_expansion: "AG",
        dna_complement: "Y",
        rna_complement: "Y",
    },
    NucleotideBaseInfo {
        symbol: 'Y',
        name: "Pyrimidine",
        base_class: "ambiguity",
        supported_molecules: "dna,rna",
        canonical_expansion: "CTU",
        dna_complement: "R",
        rna_complement: "R",
    },
    NucleotideBaseInfo {
        symbol: 'S',
        name: "Strong interaction",
        base_class: "ambiguity",
        supported_molecules: "dna,rna",
        canonical_expansion: "CG",
        dna_complement: "S",
        rna_complement: "S",
    },
    NucleotideBaseInfo {
        symbol: 'W',
        name: "Weak interaction",
        base_class: "ambiguity",
        supported_molecules: "dna,rna",
        canonical_expansion: "ATU",
        dna_complement: "W",
        rna_complement: "W",
    },
    NucleotideBaseInfo {
        symbol: 'K',
        name: "Keto",
        base_class: "ambiguity",
        supported_molecules: "dna,rna",
        canonical_expansion: "GTU",
        dna_complement: "M",
        rna_complement: "M",
    },
    NucleotideBaseInfo {
        symbol: 'M',
        name: "Amino",
        base_class: "ambiguity",
        supported_molecules: "dna,rna",
        canonical_expansion: "AC",
        dna_complement: "K",
        rna_complement: "K",
    },
    NucleotideBaseInfo {
        symbol: 'B',
        name: "Not A",
        base_class: "ambiguity",
        supported_molecules: "dna,rna",
        canonical_expansion: "CGTU",
        dna_complement: "V",
        rna_complement: "V",
    },
    NucleotideBaseInfo {
        symbol: 'D',
        name: "Not C",
        base_class: "ambiguity",
        supported_molecules: "dna,rna",
        canonical_expansion: "AGTU",
        dna_complement: "H",
        rna_complement: "H",
    },
    NucleotideBaseInfo {
        symbol: 'H',
        name: "Not G",
        base_class: "ambiguity",
        supported_molecules: "dna,rna",
        canonical_expansion: "ACTU",
        dna_complement: "D",
        rna_complement: "D",
    },
    NucleotideBaseInfo {
        symbol: 'V',
        name: "Not T or U",
        base_class: "ambiguity",
        supported_molecules: "dna,rna",
        canonical_expansion: "ACG",
        dna_complement: "B",
        rna_complement: "B",
    },
];

const NUCLEOTIDE_UNKNOWN: NucleotideBaseInfo = NucleotideBaseInfo {
    symbol: 'N',
    name: "Any base",
    base_class: "ambiguity",
    supported_molecules: "dna,rna",
    canonical_expansion: "ACGTU",
    dna_complement: "N",
    rna_complement: "N",
};

/// Returns all governed protein residue property rows in stable residue order.
#[must_use]
pub fn protein_residue_properties() -> &'static [ProteinResidueProperty] {
    &PROTEIN_PROPERTIES
}

/// Returns the governed property row for one protein residue.
#[must_use]
pub fn protein_residue_property(residue: char) -> Option<ProteinResidueProperty> {
    let residue = residue.to_ascii_uppercase();
    PROTEIN_PROPERTIES
        .iter()
        .find(|property| property.residue == residue)
        .copied()
}

/// Returns all governed nucleotide base rows in stable symbol order.
#[must_use]
pub fn nucleotide_base_infos() -> Vec<NucleotideBaseInfo> {
    let mut infos = NUCLEOTIDE_BASES.to_vec();
    infos.push(NUCLEOTIDE_UNKNOWN);
    infos
}

/// Returns governed metadata for one nucleotide symbol.
#[must_use]
pub fn nucleotide_base_info(symbol: char) -> Option<NucleotideBaseInfo> {
    let symbol = symbol.to_ascii_uppercase();
    if symbol == 'N' {
        return Some(NUCLEOTIDE_UNKNOWN);
    }
    NUCLEOTIDE_BASES
        .iter()
        .find(|info| info.symbol == symbol)
        .copied()
}

#[cfg(test)]
mod tests {
    use super::{nucleotide_base_info, protein_residue_properties, protein_residue_property};

    #[test]
    fn exposes_expected_residue_properties() {
        let alanine = protein_residue_property('a').expect("alanine should exist");
        assert_eq!(alanine.three_letter, "Ala");
        assert_eq!(alanine.charge_class, "neutral");
        assert!((alanine.average_mass - 71.078_8).abs() < 1e-9);
        assert_eq!(protein_residue_properties().len(), 20);
    }

    #[test]
    fn exposes_expected_nucleotide_ambiguity_info() {
        let any_base = nucleotide_base_info('n').expect("n should exist");
        assert_eq!(any_base.canonical_expansion, "ACGTU");
        assert_eq!(any_base.dna_complement, "N");

        let thymine = nucleotide_base_info('T').expect("thymine should exist");
        assert_eq!(thymine.supported_molecules, "dna");
        assert_eq!(thymine.rna_complement, "A");
    }
}
