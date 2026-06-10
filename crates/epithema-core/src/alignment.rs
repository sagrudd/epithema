//! Shared alignment domain types for Epithema.
//!
//! Alignments are represented as ordered rows of equal-length aligned symbols.
//! Coordinates in this module are alignment-column coordinates, not ungapped
//! sequence coordinates.

use crate::alphabet::Alphabet;
use crate::error::DomainError;
use crate::identifier::SequenceIdentifier;
use crate::metadata::SequenceMetadata;
use crate::molecule::MoleculeKind;

/// Canonical gap symbol used in aligned sequence content.
pub const GAP_SYMBOL: char = '-';

/// Single aligned symbol in an alignment row.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum AlignmentSymbol {
    /// Biological residue symbol.
    Residue(char),
    /// Gap placeholder in aligned coordinates.
    Gap,
}

/// One aligned sequence row within a pairwise or multiple alignment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlignmentRow {
    identifier: SequenceIdentifier,
    molecule: MoleculeKind,
    alphabet: Alphabet,
    aligned: String,
    metadata: SequenceMetadata,
}

impl AlignmentRow {
    /// Creates a validated alignment row from aligned residue content.
    pub fn new(
        identifier: SequenceIdentifier,
        molecule: MoleculeKind,
        aligned: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let alphabet = Alphabet::from_molecule(molecule);
        Self::with_alphabet(identifier, molecule, alphabet, aligned)
    }

    /// Creates a validated alignment row with an explicit alphabet.
    pub fn with_alphabet(
        identifier: SequenceIdentifier,
        molecule: MoleculeKind,
        alphabet: Alphabet,
        aligned: impl Into<String>,
    ) -> Result<Self, DomainError> {
        if !alphabet.is_compatible_with(molecule) {
            return Err(DomainError::IncompatibleAlphabet { molecule, alphabet });
        }

        let aligned = normalize_alignment_string(alphabet, molecule, &aligned.into())?;
        if aligned.is_empty() {
            return Err(DomainError::EmptyAlignmentRow);
        }

        Ok(Self {
            identifier,
            molecule,
            alphabet,
            aligned,
            metadata: SequenceMetadata::default(),
        })
    }

    /// Returns the stable row identifier.
    #[must_use]
    pub fn identifier(&self) -> &SequenceIdentifier {
        &self.identifier
    }

    /// Returns the row molecule kind.
    #[must_use]
    pub fn molecule(&self) -> MoleculeKind {
        self.molecule
    }

    /// Returns the row alphabet.
    #[must_use]
    pub fn alphabet(&self) -> Alphabet {
        self.alphabet
    }

    /// Returns the aligned row content.
    #[must_use]
    pub fn aligned(&self) -> &str {
        &self.aligned
    }

    /// Returns the row metadata.
    #[must_use]
    pub fn metadata(&self) -> &SequenceMetadata {
        &self.metadata
    }

    /// Replaces the row metadata.
    #[must_use]
    pub fn with_metadata(mut self, metadata: SequenceMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Returns the aligned column count.
    #[must_use]
    pub fn aligned_len(&self) -> usize {
        self.aligned.len()
    }

    /// Returns the number of gaps in the row.
    #[must_use]
    pub fn gap_count(&self) -> usize {
        self.aligned
            .chars()
            .filter(|symbol| *symbol == GAP_SYMBOL)
            .count()
    }

    /// Returns the number of nongap residues in the row.
    #[must_use]
    pub fn ungapped_len(&self) -> usize {
        self.aligned_len() - self.gap_count()
    }

    /// Returns the symbol at the supplied alignment column.
    #[must_use]
    pub fn symbol_at(&self, column: usize) -> Option<AlignmentSymbol> {
        self.aligned.chars().nth(column).map(|symbol| {
            if symbol == GAP_SYMBOL {
                AlignmentSymbol::Gap
            } else {
                AlignmentSymbol::Residue(symbol)
            }
        })
    }

    /// Returns the ungapped residue string for the row.
    #[must_use]
    pub fn ungapped(&self) -> String {
        self.aligned
            .chars()
            .filter(|symbol| *symbol != GAP_SYMBOL)
            .collect()
    }
}

/// Ordered alignment record containing pairwise or multiple aligned rows.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Alignment {
    identifier: Option<String>,
    rows: Vec<AlignmentRow>,
}

impl Alignment {
    /// Creates a validated alignment from ordered rows.
    pub fn new(rows: Vec<AlignmentRow>) -> Result<Self, DomainError> {
        Self::with_identifier(None::<String>, rows)
    }

    /// Creates a validated alignment with an optional identifier.
    pub fn with_identifier(
        identifier: Option<impl Into<String>>,
        rows: Vec<AlignmentRow>,
    ) -> Result<Self, DomainError> {
        if rows.is_empty() {
            return Err(DomainError::EmptyAlignment);
        }

        let aligned_len = rows[0].aligned_len();
        for row in &rows {
            if row.aligned_len() != aligned_len {
                return Err(DomainError::InconsistentAlignmentRowLength {
                    expected: aligned_len,
                    observed: row.aligned_len(),
                    row_identifier: row.identifier().accession().to_owned(),
                });
            }
        }

        for (index, row) in rows.iter().enumerate() {
            if rows[..index]
                .iter()
                .any(|existing| existing.identifier().accession() == row.identifier().accession())
            {
                return Err(DomainError::DuplicateAlignmentRowIdentifier {
                    identifier: row.identifier().accession().to_owned(),
                });
            }
        }

        let identifier = identifier
            .map(Into::into)
            .map(|value: String| value.trim().to_owned())
            .filter(|value| !value.is_empty());

        Ok(Self { identifier, rows })
    }

    /// Returns the optional alignment identifier.
    #[must_use]
    pub fn identifier(&self) -> Option<&str> {
        self.identifier.as_deref()
    }

    /// Returns the ordered alignment rows.
    #[must_use]
    pub fn rows(&self) -> &[AlignmentRow] {
        &self.rows
    }

    /// Returns the number of rows in the alignment.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns the aligned column count.
    #[must_use]
    pub fn column_count(&self) -> usize {
        self.rows[0].aligned_len()
    }

    /// Returns true when the alignment contains exactly two rows.
    #[must_use]
    pub fn is_pairwise(&self) -> bool {
        self.row_count() == 2
    }

    /// Returns true when the alignment contains more than two rows.
    #[must_use]
    pub fn is_multiple(&self) -> bool {
        self.row_count() > 2
    }

    /// Returns the symbols in a single alignment column.
    #[must_use]
    pub fn column(&self, index: usize) -> Option<Vec<AlignmentSymbol>> {
        if index >= self.column_count() {
            return None;
        }

        Some(
            self.rows
                .iter()
                .map(|row| {
                    row.symbol_at(index)
                        .expect("validated rows have equal length")
                })
                .collect(),
        )
    }

    /// Returns the zero-based row index for the supplied identifier.
    #[must_use]
    pub fn row_index(&self, identifier: &str) -> Option<usize> {
        self.rows
            .iter()
            .position(|row| row.identifier().accession() == identifier)
    }

    /// Returns a row by stable identifier.
    #[must_use]
    pub fn row_by_identifier(&self, identifier: &str) -> Option<&AlignmentRow> {
        self.row_index(identifier)
            .and_then(|index| self.rows.get(index))
    }

    /// Returns a new alignment sliced to the supplied zero-based half-open column interval.
    pub fn slice_columns(&self, start: usize, end: usize) -> Result<Self, DomainError> {
        if start >= end || end > self.column_count() {
            return Err(DomainError::InvalidInterval { start, end });
        }

        let rows = self
            .rows
            .iter()
            .map(|row| {
                AlignmentRow::with_alphabet(
                    row.identifier().clone(),
                    row.molecule(),
                    row.alphabet(),
                    row.aligned()[start..end].to_owned(),
                )
                .map(|rebuilt| rebuilt.with_metadata(row.metadata().clone()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Self::with_identifier(self.identifier.clone(), rows)
    }
}

fn normalize_alignment_string(
    alphabet: Alphabet,
    molecule: MoleculeKind,
    aligned: &str,
) -> Result<String, DomainError> {
    let mut normalized = String::new();

    for (position, symbol) in aligned.chars().enumerate() {
        if symbol.is_whitespace() {
            continue;
        }

        let symbol = symbol.to_ascii_uppercase();
        if matches!(symbol, '-' | '.') {
            normalized.push(GAP_SYMBOL);
            continue;
        }

        if !alphabet.allows(symbol) {
            return Err(DomainError::InvalidResidues {
                molecule,
                alphabet,
                invalid_symbol: symbol,
                position,
            });
        }

        normalized.push(symbol);
    }

    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use crate::{Alignment, AlignmentRow, MoleculeKind, SequenceIdentifier};

    #[test]
    fn normalizes_alignment_rows_and_counts_gaps() {
        let row = AlignmentRow::new(
            SequenceIdentifier::new("seq1").expect("valid identifier"),
            MoleculeKind::Dna,
            "acg-.t",
        )
        .expect("alignment row should parse");

        assert_eq!(row.aligned(), "ACG--T");
        assert_eq!(row.gap_count(), 2);
        assert_eq!(row.ungapped(), "ACGT");
    }

    #[test]
    fn rejects_inconsistent_alignment_lengths() {
        let rows = vec![
            AlignmentRow::new(
                SequenceIdentifier::new("seq1").expect("valid identifier"),
                MoleculeKind::Dna,
                "ACGT",
            )
            .expect("valid row"),
            AlignmentRow::new(
                SequenceIdentifier::new("seq2").expect("valid identifier"),
                MoleculeKind::Dna,
                "A-G",
            )
            .expect("valid row"),
        ];

        let error = Alignment::new(rows).expect_err("alignment should reject unequal lengths");
        assert!(error.to_string().contains("same aligned length"));
    }

    #[test]
    fn builds_pairwise_alignment_and_extracts_columns() {
        let alignment = Alignment::with_identifier(
            Some("example"),
            vec![
                AlignmentRow::new(
                    SequenceIdentifier::new("seq1").expect("valid identifier"),
                    MoleculeKind::Dna,
                    "AC-GT",
                )
                .expect("valid row"),
                AlignmentRow::new(
                    SequenceIdentifier::new("seq2").expect("valid identifier"),
                    MoleculeKind::Dna,
                    "ACTGT",
                )
                .expect("valid row"),
            ],
        )
        .expect("alignment should be valid");

        assert_eq!(alignment.identifier(), Some("example"));
        assert!(alignment.is_pairwise());
        assert_eq!(alignment.column_count(), 5);
        assert_eq!(alignment.column(2).expect("column should exist").len(), 2);
    }

    #[test]
    fn finds_rows_by_identifier() {
        let alignment = Alignment::with_identifier(
            Some("example"),
            vec![
                AlignmentRow::new(
                    SequenceIdentifier::new("seq1").expect("valid identifier"),
                    MoleculeKind::Dna,
                    "AC-GT",
                )
                .expect("valid row"),
                AlignmentRow::new(
                    SequenceIdentifier::new("seq2").expect("valid identifier"),
                    MoleculeKind::Dna,
                    "ACTGT",
                )
                .expect("valid row"),
            ],
        )
        .expect("alignment should be valid");

        assert_eq!(alignment.row_index("seq2"), Some(1));
        assert_eq!(
            alignment
                .row_by_identifier("seq1")
                .expect("row should exist")
                .aligned(),
            "AC-GT"
        );
    }

    #[test]
    fn slices_alignment_columns() {
        let alignment = Alignment::with_identifier(
            Some("example"),
            vec![
                AlignmentRow::new(
                    SequenceIdentifier::new("seq1").expect("valid identifier"),
                    MoleculeKind::Dna,
                    "AC-GT",
                )
                .expect("valid row"),
                AlignmentRow::new(
                    SequenceIdentifier::new("seq2").expect("valid identifier"),
                    MoleculeKind::Dna,
                    "ACTGT",
                )
                .expect("valid row"),
            ],
        )
        .expect("alignment should be valid");

        let sliced = alignment.slice_columns(1, 4).expect("slice should succeed");
        assert_eq!(sliced.column_count(), 3);
        assert_eq!(sliced.rows()[0].aligned(), "C-G");
        assert_eq!(sliced.rows()[1].aligned(), "CTG");
    }
}
