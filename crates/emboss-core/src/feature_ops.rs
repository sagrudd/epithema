//! Reusable feature-selection, editing, and extraction operations.
//!
//! Supported extraction scope in this module is intentionally conservative:
//! - selectors operate over any currently modelled feature location
//! - sequence extraction supports only simple single-span feature locations
//! - extracted feature coordinates are rebased onto the new local record span
//! - reverse-complement extraction is supported for DNA and RNA rows only

use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::feature::{Feature, FeatureLocation};
use crate::feature_selector::FeatureSelector;
use crate::identifier::SequenceIdentifier;
use crate::interval::Interval;
use crate::molecule::MoleculeKind;
use crate::sequence::SequenceRecord;
use crate::strand::Strand;

/// Summary of one feature in a stable, tool-friendly shape.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatureSummary {
    /// Feature kind.
    pub kind: crate::FeatureKind,
    /// Optional feature name.
    pub name: Option<String>,
    /// Bounds spanning the feature location.
    pub bounds: Interval,
    /// Shared strand when all spans agree.
    pub strand: Option<Strand>,
    /// Qualifier count.
    pub qualifier_count: usize,
}

/// Result of extracting one feature-linked sequence region.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtractedFeatureRecord {
    /// Source feature summary.
    pub source_feature: FeatureSummary,
    /// Extracted local sequence record.
    pub record: SequenceRecord,
}

/// Errors for in-memory feature editing and extraction operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FeatureOperationError {
    /// No features matched the requested selector.
    NoMatchingFeatures,
    /// More than one feature matched when a single match was required.
    AmbiguousSelection {
        /// Number of matching features found.
        count: usize,
    },
    /// Extraction currently requires a simple single-span location.
    UnsupportedComplexLocation,
    /// Reverse-strand extraction is not supported for the current molecule.
    UnsupportedReverseStrand {
        /// Molecule kind that could not be reverse-complement extracted.
        molecule: MoleculeKind,
    },
    /// Underlying core validation failed.
    Domain(crate::DomainError),
}

impl Display for FeatureOperationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoMatchingFeatures => write!(f, "no features matched the requested selector"),
            Self::AmbiguousSelection { count } => {
                write!(f, "expected a single matching feature, found {count}")
            }
            Self::UnsupportedComplexLocation => write!(
                f,
                "feature extraction currently supports only simple single-span locations"
            ),
            Self::UnsupportedReverseStrand { molecule } => write!(
                f,
                "reverse-strand extraction is not supported for molecule kind {molecule}"
            ),
            Self::Domain(error) => Display::fmt(error, f),
        }
    }
}

impl Error for FeatureOperationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Domain(error) => Some(error),
            _ => None,
        }
    }
}

impl From<crate::DomainError> for FeatureOperationError {
    fn from(value: crate::DomainError) -> Self {
        Self::Domain(value)
    }
}

/// Returns borrowed features matching the selector in stable source order.
#[must_use]
pub fn select_features<'a>(
    record: &'a SequenceRecord,
    selector: &FeatureSelector,
) -> Vec<&'a Feature> {
    record
        .features()
        .iter()
        .filter(|feature| selector.matches(feature))
        .collect()
}

/// Returns cloned summaries for features matching the selector in stable source order.
#[must_use]
pub fn summarize_features(
    record: &SequenceRecord,
    selector: &FeatureSelector,
) -> Vec<FeatureSummary> {
    select_features(record, selector)
        .into_iter()
        .map(summarize_feature)
        .collect()
}

/// Returns a copy of the matching features in stable source order.
#[must_use]
pub fn copy_selected_features(record: &SequenceRecord, selector: &FeatureSelector) -> Vec<Feature> {
    select_features(record, selector)
        .into_iter()
        .cloned()
        .collect()
}

/// Returns a copy of the record with the supplied intervals masked in place.
pub fn mask_intervals(
    record: &SequenceRecord,
    intervals: &[Interval],
    mask_symbol: char,
) -> Result<SequenceRecord, FeatureOperationError> {
    for interval in intervals {
        if !record.span().contains_interval(*interval) {
            return Err(FeatureOperationError::Domain(
                crate::DomainError::SequenceIntervalOutOfBounds {
                    interval_end: interval.end(),
                    sequence_length: record.len(),
                },
            ));
        }
    }

    let mut residues: Vec<char> = record.residues().chars().collect();
    for interval in intervals {
        for residue in residues
            .iter_mut()
            .take(interval.end())
            .skip(interval.start())
        {
            *residue = mask_symbol;
        }
    }

    clone_record_with_features_and_residues(
        record,
        record.features().to_vec(),
        residues.into_iter().collect(),
    )
}

/// Returns a copy of the record with matching simple feature spans masked in place.
pub fn mask_selected_features(
    record: &SequenceRecord,
    selector: &FeatureSelector,
    mask_symbol: char,
) -> Result<SequenceRecord, FeatureOperationError> {
    let matches = select_features(record, selector);
    if matches.is_empty() {
        return Err(FeatureOperationError::NoMatchingFeatures);
    }

    let intervals = matches
        .into_iter()
        .map(|feature| {
            feature
                .location
                .single_span()
                .map(|span| span.interval())
                .ok_or(FeatureOperationError::UnsupportedComplexLocation)
        })
        .collect::<Result<Vec<_>, _>>()?;

    mask_intervals(record, &intervals, mask_symbol)
}

/// Returns a copy of the record retaining only matching features.
pub fn retain_selected_features(
    record: &SequenceRecord,
    selector: &FeatureSelector,
) -> Result<SequenceRecord, FeatureOperationError> {
    clone_record_with_features(record, copy_selected_features(record, selector))
}

/// Returns a copy of the record dropping all matching features.
pub fn drop_selected_features(
    record: &SequenceRecord,
    selector: &FeatureSelector,
) -> Result<SequenceRecord, FeatureOperationError> {
    let features = record
        .features()
        .iter()
        .filter(|feature| !selector.matches(feature))
        .cloned()
        .collect();
    clone_record_with_features(record, features)
}

/// Extracts all matching simple feature-linked regions into independent local records.
pub fn extract_selected_regions(
    record: &SequenceRecord,
    selector: &FeatureSelector,
) -> Result<Vec<ExtractedFeatureRecord>, FeatureOperationError> {
    let matches = select_features(record, selector);
    if matches.is_empty() {
        return Err(FeatureOperationError::NoMatchingFeatures);
    }

    matches
        .into_iter()
        .map(|feature| extract_feature_region(record, feature))
        .collect()
}

/// Extracts exactly one matching feature-linked region.
pub fn extract_single_region(
    record: &SequenceRecord,
    selector: &FeatureSelector,
) -> Result<ExtractedFeatureRecord, FeatureOperationError> {
    let matches = select_features(record, selector);
    match matches.len() {
        0 => Err(FeatureOperationError::NoMatchingFeatures),
        1 => extract_feature_region(record, matches[0]),
        count => Err(FeatureOperationError::AmbiguousSelection { count }),
    }
}

fn clone_record_with_features(
    record: &SequenceRecord,
    features: Vec<Feature>,
) -> Result<SequenceRecord, FeatureOperationError> {
    clone_record_with_features_and_residues(record, features, record.residues().to_owned())
}

fn clone_record_with_features_and_residues(
    record: &SequenceRecord,
    features: Vec<Feature>,
    residues: String,
) -> Result<SequenceRecord, FeatureOperationError> {
    let mut cloned = SequenceRecord::with_alphabet(
        record.identifier().clone(),
        record.molecule(),
        record.alphabet(),
        residues,
    )?
    .with_metadata(record.metadata().clone());

    for feature in features {
        cloned.add_feature(feature)?;
    }

    Ok(cloned)
}

fn extract_feature_region(
    record: &SequenceRecord,
    feature: &Feature,
) -> Result<ExtractedFeatureRecord, FeatureOperationError> {
    let span = feature
        .location
        .single_span()
        .ok_or(FeatureOperationError::UnsupportedComplexLocation)?;
    let interval = span.interval();

    let extracted_residues = match span.strand() {
        Strand::Reverse => reverse_complement(record.molecule(), record.subsequence(interval)?)?,
        _ => record.subsequence(interval)?.to_owned(),
    };

    let extracted_identifier = build_extracted_identifier(record, feature, interval)?;
    let mut extracted = SequenceRecord::with_alphabet(
        extracted_identifier,
        record.molecule(),
        record.alphabet(),
        extracted_residues,
    )?
    .with_metadata(record.metadata().clone());

    let rebased_feature = rebase_feature(feature, interval.start())?;
    extracted.add_feature(rebased_feature)?;

    Ok(ExtractedFeatureRecord {
        source_feature: summarize_feature(feature),
        record: extracted,
    })
}

fn build_extracted_identifier(
    record: &SequenceRecord,
    feature: &Feature,
    interval: Interval,
) -> Result<SequenceIdentifier, FeatureOperationError> {
    let suffix = feature.name.as_deref().unwrap_or(match &feature.kind {
        crate::FeatureKind::Gene => "gene",
        crate::FeatureKind::CodingSequence => "cds",
        crate::FeatureKind::Exon => "exon",
        crate::FeatureKind::Intron => "intron",
        crate::FeatureKind::Region => "region",
        crate::FeatureKind::Motif => "motif",
        crate::FeatureKind::RepeatRegion => "repeat_region",
        crate::FeatureKind::MiscFeature => "misc_feature",
        crate::FeatureKind::Other(label) => label.as_str(),
    });
    let accession = format!(
        "{}:{}-{}:{}",
        record.identifier().accession(),
        interval.start() + 1,
        interval.end(),
        suffix
    );
    Ok(SequenceIdentifier::new(accession)?)
}

fn rebase_feature(feature: &Feature, offset: usize) -> Result<Feature, FeatureOperationError> {
    let span = feature
        .location
        .single_span()
        .ok_or(FeatureOperationError::UnsupportedComplexLocation)?;
    let original = span.interval();
    let rebased = Interval::new(original.start() - offset, original.end() - offset)?;
    let mut cloned = Feature::new(
        feature.kind.clone(),
        FeatureLocation::new(rebased, span.strand()),
    );
    cloned.name = feature.name.clone();
    cloned.note = feature.note.clone();
    cloned.qualifiers = feature.qualifiers.clone();
    Ok(cloned)
}

fn reverse_complement(
    molecule: MoleculeKind,
    residues: &str,
) -> Result<String, FeatureOperationError> {
    match molecule {
        MoleculeKind::Dna => Ok(residues
            .chars()
            .rev()
            .map(complement_dna)
            .collect::<Option<String>>()
            .ok_or(FeatureOperationError::UnsupportedReverseStrand { molecule })?),
        MoleculeKind::Rna => Ok(residues
            .chars()
            .rev()
            .map(complement_rna)
            .collect::<Option<String>>()
            .ok_or(FeatureOperationError::UnsupportedReverseStrand { molecule })?),
        _ => Err(FeatureOperationError::UnsupportedReverseStrand { molecule }),
    }
}

fn complement_dna(symbol: char) -> Option<char> {
    match symbol {
        'A' => Some('T'),
        'T' => Some('A'),
        'G' => Some('C'),
        'C' => Some('G'),
        'N' => Some('N'),
        'R' => Some('Y'),
        'Y' => Some('R'),
        'S' => Some('S'),
        'W' => Some('W'),
        'K' => Some('M'),
        'M' => Some('K'),
        'B' => Some('V'),
        'V' => Some('B'),
        'D' => Some('H'),
        'H' => Some('D'),
        '-' => Some('-'),
        '*' => Some('*'),
        _ => None,
    }
}

fn complement_rna(symbol: char) -> Option<char> {
    match symbol {
        'A' => Some('U'),
        'U' => Some('A'),
        'G' => Some('C'),
        'C' => Some('G'),
        'N' => Some('N'),
        'R' => Some('Y'),
        'Y' => Some('R'),
        'S' => Some('S'),
        'W' => Some('W'),
        'K' => Some('M'),
        'M' => Some('K'),
        'B' => Some('V'),
        'V' => Some('B'),
        'D' => Some('H'),
        'H' => Some('D'),
        '-' => Some('-'),
        '*' => Some('*'),
        _ => None,
    }
}

fn summarize_feature(feature: &Feature) -> FeatureSummary {
    FeatureSummary {
        kind: feature.kind.clone(),
        name: feature.name.clone(),
        bounds: feature.location.bounds(),
        strand: feature.location.strand(),
        qualifier_count: feature.qualifiers.len(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Feature, FeatureKind, FeatureLocation, FeatureSelector, FeatureSpan, Interval,
        MoleculeKind, SequenceIdentifier, SequenceRecord, Strand,
    };

    use super::{
        FeatureOperationError, copy_selected_features, drop_selected_features,
        extract_selected_regions, extract_single_region, mask_intervals, mask_selected_features,
        retain_selected_features, summarize_features,
    };

    fn annotated_record() -> SequenceRecord {
        let mut record = SequenceRecord::new(
            SequenceIdentifier::new("seq1").expect("valid identifier"),
            MoleculeKind::Dna,
            "AACCGGTTAACC",
        )
        .expect("valid sequence");

        record
            .add_feature(
                Feature::new(
                    FeatureKind::Gene,
                    FeatureLocation::new(
                        Interval::new(2, 6).expect("valid interval"),
                        Strand::Forward,
                    ),
                )
                .with_name("geneA")
                .with_qualifier("product", "enzyme"),
            )
            .expect("feature should fit");
        record
            .add_feature(
                Feature::new(
                    FeatureKind::CodingSequence,
                    FeatureLocation::new(
                        Interval::new(6, 10).expect("valid interval"),
                        Strand::Reverse,
                    ),
                )
                .with_name("cdsA")
                .with_qualifier("product", "peptide"),
            )
            .expect("feature should fit");
        record
    }

    #[test]
    fn selects_by_kind_and_qualifier() {
        let record = annotated_record();

        let summaries = summarize_features(&record, &FeatureSelector::Kind(FeatureKind::Gene));
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].name.as_deref(), Some("geneA"));

        let copied = copy_selected_features(
            &record,
            &FeatureSelector::Qualifier {
                key: "product".to_owned(),
                value: Some("peptide".to_owned()),
            },
        );
        assert_eq!(copied.len(), 1);
        assert!(matches!(copied[0].kind, FeatureKind::CodingSequence));
    }

    #[test]
    fn retains_and_drops_features_deterministically() {
        let record = annotated_record();

        let retained = retain_selected_features(&record, &FeatureSelector::Kind(FeatureKind::Gene))
            .expect("retain should succeed");
        assert_eq!(retained.features().len(), 1);
        assert!(matches!(retained.features()[0].kind, FeatureKind::Gene));

        let dropped = drop_selected_features(&record, &FeatureSelector::Kind(FeatureKind::Gene))
            .expect("drop should succeed");
        assert_eq!(dropped.features().len(), 1);
        assert!(matches!(
            dropped.features()[0].kind,
            FeatureKind::CodingSequence
        ));
    }

    #[test]
    fn extracts_simple_feature_region_and_rebases_coordinates() {
        let record = annotated_record();

        let extracted = extract_single_region(&record, &FeatureSelector::Name("geneA".to_owned()))
            .expect("extraction should succeed");
        assert_eq!(extracted.record.residues(), "CCGG");
        assert_eq!(
            extracted.record.features()[0].location.bounds(),
            Interval::new(0, 4).expect("valid interval")
        );
    }

    #[test]
    fn reverse_complements_reverse_strand_nucleotide_extraction() {
        let record = annotated_record();

        let extracted = extract_single_region(&record, &FeatureSelector::Name("cdsA".to_owned()))
            .expect("reverse-strand extraction should succeed");
        assert_eq!(extracted.record.residues(), "TTAA");
        assert_eq!(
            extracted.record.features()[0].location.strand(),
            Some(Strand::Reverse)
        );
    }

    #[test]
    fn rejects_no_match_and_ambiguous_match() {
        let record = annotated_record();

        let no_match = extract_single_region(&record, &FeatureSelector::Name("missing".to_owned()))
            .expect_err("missing selector should fail");
        assert_eq!(no_match, FeatureOperationError::NoMatchingFeatures);

        let ambiguous = extract_single_region(
            &record,
            &FeatureSelector::AnyOf(vec![
                FeatureSelector::Kind(FeatureKind::Gene),
                FeatureSelector::Kind(FeatureKind::CodingSequence),
            ]),
        )
        .expect_err("multiple matches should fail");
        assert!(matches!(
            ambiguous,
            FeatureOperationError::AmbiguousSelection { count: 2 }
        ));
    }

    #[test]
    fn rejects_complex_locations_for_extraction() {
        let mut record = SequenceRecord::new(
            SequenceIdentifier::new("seq2").expect("valid identifier"),
            MoleculeKind::Dna,
            "AACCGGTTAACC",
        )
        .expect("valid sequence");
        record
            .add_feature(
                Feature::new(
                    FeatureKind::Gene,
                    FeatureLocation::from_spans(vec![
                        FeatureSpan::new(
                            Interval::new(1, 3).expect("valid interval"),
                            Strand::Forward,
                        ),
                        FeatureSpan::new(
                            Interval::new(5, 7).expect("valid interval"),
                            Strand::Forward,
                        ),
                    ])
                    .expect("valid location"),
                )
                .with_name("joined"),
            )
            .expect("feature should fit");

        let error = extract_selected_regions(&record, &FeatureSelector::Any)
            .expect_err("joined locations should be unsupported");
        assert_eq!(error, FeatureOperationError::UnsupportedComplexLocation);
    }

    #[test]
    fn masks_intervals_and_preserves_features() {
        let record = annotated_record();
        let masked = mask_intervals(
            &record,
            &[Interval::new(1, 4).expect("valid interval")],
            'N',
        )
        .expect("interval masking should succeed");

        assert_eq!(masked.residues(), "ANNNGGTTAACC");
        assert_eq!(masked.features(), record.features());
    }

    #[test]
    fn masks_selected_simple_feature_spans() {
        let record = annotated_record();
        let masked =
            mask_selected_features(&record, &FeatureSelector::Kind(FeatureKind::Gene), 'N')
                .expect("feature masking should succeed");

        assert_eq!(masked.residues(), "AANNNNTTAACC");
        assert_eq!(masked.features(), record.features());
    }
}
