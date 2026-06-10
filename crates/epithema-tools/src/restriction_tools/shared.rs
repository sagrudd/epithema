//! Shared helpers for conservative codon-aware restriction-site design tools.

use std::collections::BTreeSet;

use epithema_core::{
    MoleculeKind, SequenceRecord, sense_codons, summarize_coding_sequence, translate_dna_strict,
};
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::ToolExecutionError;

/// One synonymous single-codon edit that changes restriction-site presence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SynonymousRestrictionEdit {
    /// One-based inclusive site start after the edit is applied.
    pub site_start: usize,
    /// One-based inclusive site end after the edit is applied.
    pub site_end: usize,
    /// One-based codon index mutated by the edit.
    pub codon_index: usize,
    /// One-based inclusive nucleotide start of the mutated codon.
    pub codon_start: usize,
    /// One-based inclusive nucleotide end of the mutated codon.
    pub codon_end: usize,
    /// Amino-acid translation preserved by the edit.
    pub amino_acid: char,
    /// Original codon text.
    pub original_codon: String,
    /// Replacement codon text.
    pub replacement_codon: String,
    /// Full mutated coding sequence.
    pub mutated_sequence: String,
}

/// Validates one coding DNA record and returns a normalized sequence.
pub fn validate_coding_dna_record(
    tool: &str,
    record: &SequenceRecord,
) -> Result<String, ToolExecutionError> {
    if record.molecule() == MoleculeKind::Protein {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} expects coding DNA input but '{}' was classified as protein",
                record.identifier().accession()
            ),
        )
        .with_code(format!("tools.{tool}.input.not_coding_dna")));
    }
    if record.molecule() == MoleculeKind::Rna || record.residues().contains('U') {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} expects coding DNA input but '{}' contains RNA residues",
                record.identifier().accession()
            ),
        )
        .with_code(format!("tools.{tool}.input.not_coding_dna")));
    }

    let sequence = record.residues().to_ascii_uppercase();
    summarize_coding_sequence(&sequence).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} requires strict coding DNA input for '{}' but validation failed: {error}",
                record.identifier().accession()
            ),
        )
        .with_code(format!("tools.{tool}.input.invalid_coding_dna"))
    })?;
    Ok(sequence)
}

/// Normalizes a restriction site pattern for the conservative v1 model.
pub fn normalize_site(site: &str, tool: &str) -> Result<String, ToolExecutionError> {
    let normalized = site.trim().to_ascii_uppercase();
    if normalized.len() < 4 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("{tool} requires a canonical DNA site of length at least four"),
        )
        .with_code(format!("tools.{tool}.site.too_short")));
    }
    if normalized
        .chars()
        .any(|symbol| !matches!(symbol, 'A' | 'C' | 'G' | 'T'))
    {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("{tool} supports only canonical DNA restriction sites in v1"),
        )
        .with_code(format!("tools.{tool}.site.invalid")));
    }
    Ok(normalized)
}

/// Returns all 0-based site starts in stable overlapping order.
pub fn site_positions(sequence: &str, site: &str) -> Vec<usize> {
    if sequence.len() < site.len() {
        return Vec::new();
    }

    (0..=sequence.len() - site.len())
        .filter(|start| &sequence[*start..*start + site.len()] == site)
        .collect()
}

/// Reports all synonymous single-codon edits that remove one existing site occurrence.
pub fn recoder_candidates(
    sequence: &str,
    site: &str,
    occurrence_start: usize,
) -> Result<Vec<SynonymousRestrictionEdit>, ToolExecutionError> {
    let original_positions = site_positions(sequence, site);
    let overlapping_codons = codon_indexes_overlapping(occurrence_start, site.len());
    let mut candidates = Vec::new();

    for codon_index in overlapping_codons {
        let original_codon = codon_at(sequence, codon_index);
        let amino_acid = translate_codon(&original_codon)?;
        for replacement in synonymous_alternatives(&original_codon)? {
            let mutated = replace_codon(sequence, codon_index, &replacement);
            let mutated_positions = site_positions(&mutated, site);
            if !mutated_positions.contains(&occurrence_start)
                && mutated_positions.len() < original_positions.len()
            {
                candidates.push(SynonymousRestrictionEdit {
                    site_start: occurrence_start + 1,
                    site_end: occurrence_start + site.len(),
                    codon_index: codon_index + 1,
                    codon_start: codon_index * 3 + 1,
                    codon_end: codon_index * 3 + 3,
                    amino_acid,
                    original_codon: original_codon.clone(),
                    replacement_codon: replacement,
                    mutated_sequence: mutated,
                });
            }
        }
    }

    Ok(candidates)
}

/// Reports all synonymous single-codon edits that create one new site occurrence.
pub fn silent_candidates(
    sequence: &str,
    site: &str,
) -> Result<Vec<SynonymousRestrictionEdit>, ToolExecutionError> {
    let original_positions: BTreeSet<_> = site_positions(sequence, site).into_iter().collect();
    let mut candidates = Vec::new();

    for codon_index in 0..(sequence.len() / 3) {
        let original_codon = codon_at(sequence, codon_index);
        let amino_acid = translate_codon(&original_codon)?;
        for replacement in synonymous_alternatives(&original_codon)? {
            let mutated = replace_codon(sequence, codon_index, &replacement);
            let mutated_positions = site_positions(&mutated, site);
            for created_start in mutated_positions {
                if !original_positions.contains(&created_start) {
                    candidates.push(SynonymousRestrictionEdit {
                        site_start: created_start + 1,
                        site_end: created_start + site.len(),
                        codon_index: codon_index + 1,
                        codon_start: codon_index * 3 + 1,
                        codon_end: codon_index * 3 + 3,
                        amino_acid,
                        original_codon: original_codon.clone(),
                        replacement_codon: replacement.clone(),
                        mutated_sequence: mutated.clone(),
                    });
                }
            }
        }
    }

    candidates.sort_by(|left, right| {
        left.site_start
            .cmp(&right.site_start)
            .then(left.codon_index.cmp(&right.codon_index))
            .then(left.replacement_codon.cmp(&right.replacement_codon))
    });
    candidates.dedup();
    Ok(candidates)
}

fn codon_indexes_overlapping(
    site_start: usize,
    site_len: usize,
) -> std::ops::RangeInclusive<usize> {
    let first = site_start / 3;
    let last = (site_start + site_len - 1) / 3;
    first..=last
}

fn codon_at(sequence: &str, codon_index: usize) -> String {
    sequence[codon_index * 3..codon_index * 3 + 3].to_owned()
}

fn replace_codon(sequence: &str, codon_index: usize, replacement: &str) -> String {
    let start = codon_index * 3;
    let end = start + 3;
    let mut mutated = String::with_capacity(sequence.len());
    mutated.push_str(&sequence[..start]);
    mutated.push_str(replacement);
    mutated.push_str(&sequence[end..]);
    mutated
}

fn synonymous_alternatives(codon: &str) -> Result<Vec<String>, ToolExecutionError> {
    let amino_acid = translate_codon(codon)?;
    let mut codons: Vec<String> = all_codon_choices()
        .into_iter()
        .filter(|candidate| candidate != codon)
        .filter(|candidate| translate_codon(candidate).ok() == Some(amino_acid))
        .collect();
    codons.sort();
    Ok(codons)
}

fn translate_codon(codon: &str) -> Result<char, ToolExecutionError> {
    let protein = translate_dna_strict(codon).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Validation,
            format!("failed to translate codon '{codon}' during restriction-site design: {error}"),
        )
        .with_code("tools.restriction.codon.invalid")
    })?;
    Ok(protein
        .chars()
        .next()
        .expect("one codon should translate to one residue"))
}

fn all_codon_choices() -> Vec<String> {
    let mut codons: Vec<String> = sense_codons().into_iter().map(str::to_owned).collect();
    codons.extend(["TAA", "TAG", "TGA"].into_iter().map(str::to_owned));
    codons
}

#[cfg(test)]
mod tests {
    use super::{normalize_site, recoder_candidates, silent_candidates, site_positions};

    #[test]
    fn finds_overlapping_sites() {
        assert_eq!(site_positions("AAAAAA", "AAAA"), vec![0, 1, 2]);
    }

    #[test]
    fn reports_synonymous_site_removal_candidates() {
        let candidates = recoder_candidates("ATGGCTGAATTCGAA", "GAATTC", 6).expect("candidates");
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0].original_codon, "GAA");
        assert_eq!(candidates[0].replacement_codon, "GAG");
        assert_eq!(candidates[1].original_codon, "TTC");
        assert_eq!(candidates[1].replacement_codon, "TTT");
    }

    #[test]
    fn reports_synonymous_site_creation_candidates() {
        let candidates = silent_candidates("ATGGCTGAGTTCGAA", "GAATTC").expect("candidates");
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].site_start, 7);
        assert_eq!(candidates[0].original_codon, "GAG");
        assert_eq!(candidates[0].replacement_codon, "GAA");
    }

    #[test]
    fn rejects_noncanonical_site() {
        let error = normalize_site("GANNTC", "recoder").expect_err("site should fail");
        assert!(
            error
                .to_string()
                .contains("canonical DNA restriction sites")
        );
    }
}
