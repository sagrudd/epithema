//! Small shared codon and translation helpers for translation-adjacent tools.
//!
//! This module intentionally supports a narrow, deterministic v1 scope:
//! - standard genetic code only
//! - representative codon back-translation with one stable codon per amino acid
//! - ambiguous codon back-translation using IUPAC nucleotide ambiguity
//! - strict frame-1 translation of complete codon triplets only

use std::error::Error;
use std::fmt::{Display, Formatter};

/// Errors produced by strict translation and back-translation helpers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TranslationError {
    /// Protein residue is not supported by the current mapping.
    UnsupportedResidue(char),
    /// Codon contains unsupported nucleotide symbols.
    InvalidCodon(String),
    /// Nucleotide sequence length is not divisible by three.
    NonCodingLength {
        /// Observed nucleotide sequence length.
        length: usize,
    },
    /// Translation frame offset was outside the supported range.
    InvalidFrameOffset {
        /// Requested zero-based frame offset.
        offset: usize,
    },
}

impl Display for TranslationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedResidue(residue) => {
                write!(f, "unsupported residue '{residue}' for translation helper")
            }
            Self::InvalidCodon(codon) => write!(f, "invalid codon '{codon}'"),
            Self::NonCodingLength { length } => {
                write!(
                    f,
                    "nucleotide sequence length {length} is not divisible by three"
                )
            }
            Self::InvalidFrameOffset { offset } => {
                write!(f, "translation frame offset {offset} must be 0, 1, or 2")
            }
        }
    }
}

impl Error for TranslationError {}

/// Deterministically back-translates a protein sequence into representative DNA codons.
pub fn backtranslate_representative(protein: &str) -> Result<String, TranslationError> {
    protein
        .chars()
        .map(representative_codon)
        .collect::<Result<Vec<_>, _>>()
        .map(|codons| codons.concat())
}

/// Deterministically back-translates a protein sequence into ambiguous DNA codons.
pub fn backtranslate_ambiguous(protein: &str) -> Result<String, TranslationError> {
    protein
        .chars()
        .map(ambiguous_codon)
        .collect::<Result<Vec<_>, _>>()
        .map(|codons| codons.concat())
}

/// Strictly translates a frame-1 DNA coding sequence using the standard genetic code.
pub fn translate_dna_strict(coding_sequence: &str) -> Result<String, TranslationError> {
    if coding_sequence.len() % 3 != 0 {
        return Err(TranslationError::NonCodingLength {
            length: coding_sequence.len(),
        });
    }

    coding_sequence
        .as_bytes()
        .chunks(3)
        .map(|chunk| {
            let codon = std::str::from_utf8(chunk)
                .expect("sequence records are normalized ASCII residues")
                .to_ascii_uppercase();
            amino_acid_for_codon(&codon)
        })
        .collect()
}

/// Strictly translates a forward DNA frame, ignoring any trailing partial codon.
pub fn translate_dna_frame(
    nucleotide_sequence: &str,
    frame_offset: usize,
) -> Result<String, TranslationError> {
    if frame_offset > 2 {
        return Err(TranslationError::InvalidFrameOffset {
            offset: frame_offset,
        });
    }

    let sequence = nucleotide_sequence.to_ascii_uppercase();
    let bytes = sequence.as_bytes();
    if bytes.len() <= frame_offset {
        return Ok(String::new());
    }
    let usable_len = bytes.len().saturating_sub(frame_offset);
    let coding_len = usable_len - (usable_len % 3);

    bytes[frame_offset..frame_offset + coding_len]
        .chunks(3)
        .map(|chunk| {
            let codon = std::str::from_utf8(chunk)
                .expect("sequence records are normalized ASCII residues")
                .to_ascii_uppercase();
            amino_acid_for_codon(&codon)
        })
        .collect()
}

/// Returns a deterministic representative DNA codon for an amino-acid symbol.
pub fn representative_codon(residue: char) -> Result<&'static str, TranslationError> {
    match residue.to_ascii_uppercase() {
        'A' => Ok("GCT"),
        'B' => Err(TranslationError::UnsupportedResidue('B')),
        'C' => Ok("TGT"),
        'D' => Ok("GAT"),
        'E' => Ok("GAA"),
        'F' => Ok("TTT"),
        'G' => Ok("GGT"),
        'H' => Ok("CAT"),
        'I' => Ok("ATT"),
        'J' => Err(TranslationError::UnsupportedResidue('J')),
        'K' => Ok("AAA"),
        'L' => Ok("CTT"),
        'M' => Ok("ATG"),
        'N' => Ok("AAT"),
        'O' => Err(TranslationError::UnsupportedResidue('O')),
        'P' => Ok("CCT"),
        'Q' => Ok("CAA"),
        'R' => Ok("CGT"),
        'S' => Ok("TCT"),
        'T' => Ok("ACT"),
        'U' => Err(TranslationError::UnsupportedResidue('U')),
        'V' => Ok("GTT"),
        'W' => Ok("TGG"),
        'X' => Err(TranslationError::UnsupportedResidue('X')),
        'Y' => Ok("TAT"),
        'Z' => Err(TranslationError::UnsupportedResidue('Z')),
        '*' => Ok("TAA"),
        '-' => Err(TranslationError::UnsupportedResidue('-')),
        other => Err(TranslationError::UnsupportedResidue(other)),
    }
}

/// Returns an ambiguous DNA codon for an amino-acid symbol.
pub fn ambiguous_codon(residue: char) -> Result<&'static str, TranslationError> {
    match residue.to_ascii_uppercase() {
        'A' => Ok("GCN"),
        'B' => Ok("RAY"),
        'C' => Ok("TGY"),
        'D' => Ok("GAY"),
        'E' => Ok("GAR"),
        'F' => Ok("TTY"),
        'G' => Ok("GGN"),
        'H' => Ok("CAY"),
        'I' => Ok("ATH"),
        'J' => Ok("YTN"),
        'K' => Ok("AAR"),
        'L' => Ok("YTN"),
        'M' => Ok("ATG"),
        'N' => Ok("AAY"),
        'O' => Err(TranslationError::UnsupportedResidue('O')),
        'P' => Ok("CCN"),
        'Q' => Ok("CAR"),
        'R' => Ok("MGN"),
        'S' => Ok("WSN"),
        'T' => Ok("ACN"),
        'U' => Err(TranslationError::UnsupportedResidue('U')),
        'V' => Ok("GTN"),
        'W' => Ok("TGG"),
        'X' => Ok("NNN"),
        'Y' => Ok("TAY"),
        'Z' => Ok("SAR"),
        '*' => Ok("TAR"),
        '-' => Err(TranslationError::UnsupportedResidue('-')),
        other => Err(TranslationError::UnsupportedResidue(other)),
    }
}

fn amino_acid_for_codon(codon: &str) -> Result<char, TranslationError> {
    match codon {
        "TTT" | "TTC" => Ok('F'),
        "TTA" | "TTG" | "CTT" | "CTC" | "CTA" | "CTG" => Ok('L'),
        "ATT" | "ATC" | "ATA" => Ok('I'),
        "ATG" => Ok('M'),
        "GTT" | "GTC" | "GTA" | "GTG" => Ok('V'),
        "TCT" | "TCC" | "TCA" | "TCG" | "AGT" | "AGC" => Ok('S'),
        "CCT" | "CCC" | "CCA" | "CCG" => Ok('P'),
        "ACT" | "ACC" | "ACA" | "ACG" => Ok('T'),
        "GCT" | "GCC" | "GCA" | "GCG" => Ok('A'),
        "TAT" | "TAC" => Ok('Y'),
        "CAT" | "CAC" => Ok('H'),
        "CAA" | "CAG" => Ok('Q'),
        "AAT" | "AAC" => Ok('N'),
        "AAA" | "AAG" => Ok('K'),
        "GAT" | "GAC" => Ok('D'),
        "GAA" | "GAG" => Ok('E'),
        "TGT" | "TGC" => Ok('C'),
        "TGG" => Ok('W'),
        "CGT" | "CGC" | "CGA" | "CGG" | "AGA" | "AGG" => Ok('R'),
        "GGT" | "GGC" | "GGA" | "GGG" => Ok('G'),
        "TAA" | "TAG" | "TGA" => Ok('*'),
        other => Err(TranslationError::InvalidCodon(other.to_owned())),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        backtranslate_ambiguous, backtranslate_representative, translate_dna_frame,
        translate_dna_strict, TranslationError,
    };

    #[test]
    fn backtranslates_representative_codons_deterministically() {
        let translated =
            backtranslate_representative("MA*").expect("representative backtranslation");
        assert_eq!(translated, "ATGGCTTAA");
    }

    #[test]
    fn backtranslates_ambiguous_codons_deterministically() {
        let translated = backtranslate_ambiguous("LS*").expect("ambiguous backtranslation");
        assert_eq!(translated, "YTNWSNTAR");
    }

    #[test]
    fn translates_strict_frame_one_dna() {
        let protein = translate_dna_strict("ATGGCTTAA").expect("strict translation");
        assert_eq!(protein, "MA*");
    }

    #[test]
    fn rejects_invalid_length() {
        let error = translate_dna_strict("ATGG").expect_err("length should fail");
        assert_eq!(error, TranslationError::NonCodingLength { length: 4 });
    }

    #[test]
    fn translates_forward_frames_with_trailing_remainder_ignored() {
        let protein = translate_dna_frame("AATGGCTTAA", 1).expect("frame translation");
        assert_eq!(protein, "MA*");
    }
}
