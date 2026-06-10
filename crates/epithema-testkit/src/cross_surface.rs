//! Canonical fixture-driven cross-surface validation support.
//!
//! Rust remains the computational source of truth for the first-class R client.
//! This module owns canonical, typed fixtures for a curated subset of
//! R-exposed methods so that `epithemaR` can compare its public-returned objects
//! against normalized Rust-native expectations without relying on CLI text.

use std::fs;
use std::path::Path;

use epithema_diagnostics::{ErrorCategory, PlatformError};
use epithema_r_bridge::{
    BridgeChargeProfile, BridgeChargeWindow, BridgeCompositionRow, BridgeDistanceMatrix,
    BridgeGcRow, BridgeMatcherSummary, BridgePepstatsSummaryRow, BridgeRequest,
    BridgeSequenceInput, BridgeSequenceRecord, charge_profile, composition_summary,
    consensus_ambiguous, consensus_simple, count_gc_content, degap_sequences,
    direct_match_sequences, extract_sequences, new_sequence, not_sequence, nth_sequence,
    p_distance_for_sequences, pepstats_summary, sequence_count, skip_sequences, trim_sequences,
    update_descriptions,
};
use serde::{Deserialize, Serialize};

/// Default numeric tolerance for cross-surface floating-point comparisons.
pub const DEFAULT_NUMERIC_TOLERANCE: f64 = 1.0e-9;

/// Canonical Rust-owned fixture catalogue for cross-surface validation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CrossSurfaceFixtureCatalog {
    /// Contract version for the fixture shape.
    pub version: u32,
    /// Which repository owns the canonical expected outputs.
    pub canonical_owner: String,
    /// Default floating-point tolerance for semantic comparison.
    pub numeric_tolerance: f64,
    /// Ordered curated validation cases.
    pub cases: Vec<CrossSurfaceFixtureCase>,
}

/// One curated cross-surface validation case.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CrossSurfaceFixtureCase {
    /// Stable case identifier.
    pub id: String,
    /// Public bridge-backed method identifier.
    pub method: String,
    /// Result family label used in docs and tests.
    pub family: String,
    /// Typed canonical request payload.
    pub request: BridgeRequest,
    /// Semantic expected output for comparison.
    pub expected: CrossSurfaceExpected,
}

/// Semantic expected output families used for cross-surface comparison.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CrossSurfaceExpected {
    /// Scalar count.
    Count { count: usize },
    /// One sequence record.
    SequenceRecord { record: BridgeSequenceRecord },
    /// Ordered sequence records.
    SequenceSet { records: Vec<BridgeSequenceRecord> },
    /// Long-form composition rows.
    CompositionRows { rows: Vec<BridgeCompositionRow> },
    /// Long-form GC summary rows.
    GcRows { rows: Vec<BridgeGcRow> },
    /// Pepstats summary and composition rows.
    Pepstats {
        summary_rows: Vec<BridgePepstatsSummaryRow>,
        composition_rows: Vec<BridgeCompositionRow>,
    },
    /// Matcher summary row.
    MatcherSummary { summary: BridgeMatcherSummary },
    /// Equal-length distance matrix.
    DistanceMatrix { matrix: BridgeDistanceMatrix },
    /// Charge-profile analytical rows only.
    ChargeProfile {
        identifier: String,
        sequence_length: usize,
        window: usize,
        step: usize,
        windows: Vec<BridgeChargeWindow>,
    },
}

impl CrossSurfaceFixtureCatalog {
    /// Builds the curated canonical cross-surface fixture catalogue.
    pub fn curated() -> Result<Self, PlatformError> {
        Ok(Self {
            version: 1,
            canonical_owner: "epithema".to_owned(),
            numeric_tolerance: DEFAULT_NUMERIC_TOLERANCE,
            cases: vec![
                new_sequence_case()?,
                sequence_count_case()?,
                nth_sequence_case()?,
                skip_sequences_case()?,
                not_sequence_case()?,
                extract_sequences_case()?,
                degap_sequences_case()?,
                trim_sequences_case()?,
                update_descriptions_case()?,
                composition_summary_case()?,
                gc_content_case()?,
                pepstats_case()?,
                matcher_case()?,
                distance_matrix_case()?,
                consensus_simple_case()?,
                consensus_ambiguous_case()?,
                charge_profile_case()?,
            ],
        })
    }
}

/// Writes a cross-surface fixture catalogue as pretty JSON.
pub fn write_cross_surface_fixture_catalog_json(
    catalogue: &CrossSurfaceFixtureCatalog,
    path: impl AsRef<Path>,
) -> Result<(), PlatformError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to create cross-surface fixture directory",
            )
            .with_code("testkit.cross_surface.create_dir_failed")
            .with_detail(format!("{}: {error}", parent.display()))
        })?;
    }

    let json = serde_json::to_string_pretty(catalogue).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Internal,
            "failed to serialize cross-surface fixture catalogue",
        )
        .with_code("testkit.cross_surface.serialize_failed")
        .with_source(error)
    })?;

    fs::write(path, json).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write cross-surface fixture catalogue",
        )
        .with_code("testkit.cross_surface.write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

fn new_sequence_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let request = BridgeRequest::NewSequence {
        record: dna_input("seq.alpha", "ACGT", Some("alpha description")),
    };
    let record = new_sequence(dna_input("seq.alpha", "ACGT", Some("alpha description")))?;
    Ok(CrossSurfaceFixtureCase {
        id: "newseq_basic".to_owned(),
        method: "newseq".to_owned(),
        family: "sequence_record".to_owned(),
        request,
        expected: CrossSurfaceExpected::SequenceRecord { record },
    })
}

fn sequence_count_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let records = basic_dna_records();
    let request = BridgeRequest::SequenceCount {
        records: records.clone(),
    };
    let count = sequence_count(&records)?;
    Ok(CrossSurfaceFixtureCase {
        id: "seqcount_basic".to_owned(),
        method: "seqcount".to_owned(),
        family: "count".to_owned(),
        request,
        expected: CrossSurfaceExpected::Count { count },
    })
}

fn nth_sequence_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let records = basic_dna_records();
    let request = BridgeRequest::NthSequence {
        records: records.clone(),
        index: 2,
    };
    let record = nth_sequence(&records, 2)?;
    Ok(CrossSurfaceFixtureCase {
        id: "nthseq_basic".to_owned(),
        method: "nthseq".to_owned(),
        family: "sequence_record".to_owned(),
        request,
        expected: CrossSurfaceExpected::SequenceRecord { record },
    })
}

fn skip_sequences_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let records = basic_dna_records();
    let request = BridgeRequest::SkipSequences {
        records: records.clone(),
        count: 1,
    };
    let records = skip_sequences(&records, 1)?;
    Ok(CrossSurfaceFixtureCase {
        id: "skipseq_basic".to_owned(),
        method: "skipseq".to_owned(),
        family: "sequence_set".to_owned(),
        request,
        expected: CrossSurfaceExpected::SequenceSet { records },
    })
}

fn not_sequence_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let records = basic_dna_records();
    let request = BridgeRequest::NotSequence {
        records: records.clone(),
        index: 1,
    };
    let records = not_sequence(&records, 1)?;
    Ok(CrossSurfaceFixtureCase {
        id: "notseq_basic".to_owned(),
        method: "notseq".to_owned(),
        family: "sequence_set".to_owned(),
        request,
        expected: CrossSurfaceExpected::SequenceSet { records },
    })
}

fn extract_sequences_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let records = basic_dna_records();
    let request = BridgeRequest::ExtractSequences {
        records: records.clone(),
        start: 2,
        end: 4,
    };
    let records = extract_sequences(&records, 2, 4)?;
    Ok(CrossSurfaceFixtureCase {
        id: "extractseq_basic".to_owned(),
        method: "extractseq".to_owned(),
        family: "sequence_set".to_owned(),
        request,
        expected: CrossSurfaceExpected::SequenceSet { records },
    })
}

fn degap_sequences_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let records = vec![
        dna_input("gap.a", "A-C-GT", None),
        dna_input("gap.b", "AC-GT-", Some("with gaps")),
    ];
    let request = BridgeRequest::DegapSequences {
        records: records.clone(),
    };
    let records = degap_sequences(&records)?;
    Ok(CrossSurfaceFixtureCase {
        id: "degapseq_basic".to_owned(),
        method: "degapseq".to_owned(),
        family: "sequence_set".to_owned(),
        request,
        expected: CrossSurfaceExpected::SequenceSet { records },
    })
}

fn trim_sequences_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let records = vec![dna_input("trim.a", "ACGTAC", None)];
    let request = BridgeRequest::TrimSequences {
        records: records.clone(),
        left_trim: 1,
        right_trim: 1,
    };
    let records = trim_sequences(&records, 1, 1)?;
    Ok(CrossSurfaceFixtureCase {
        id: "trimseq_basic".to_owned(),
        method: "trimseq".to_owned(),
        family: "sequence_set".to_owned(),
        request,
        expected: CrossSurfaceExpected::SequenceSet { records },
    })
}

fn update_descriptions_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let records = vec![
        dna_input("desc.a", "ACGT", Some("old one")),
        dna_input("desc.b", "AAGT", None),
    ];
    let request = BridgeRequest::UpdateDescriptions {
        records: records.clone(),
        description: Some("updated description".to_owned()),
        clear: false,
    };
    let records = update_descriptions(&records, Some("updated description".to_owned()), false)?;
    Ok(CrossSurfaceFixtureCase {
        id: "descseq_basic".to_owned(),
        method: "descseq".to_owned(),
        family: "sequence_set".to_owned(),
        request,
        expected: CrossSurfaceExpected::SequenceSet { records },
    })
}

fn composition_summary_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let records = basic_dna_records();
    let request = BridgeRequest::CompositionSummary {
        records: records.clone(),
    };
    let rows = composition_summary(&records)?;
    Ok(CrossSurfaceFixtureCase {
        id: "compseq_basic".to_owned(),
        method: "compseq".to_owned(),
        family: "table".to_owned(),
        request,
        expected: CrossSurfaceExpected::CompositionRows { rows },
    })
}

fn gc_content_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let records = vec![
        dna_input("gc.a", "ACGT", None),
        dna_input("gc.b", "GGCC", None),
    ];
    let request = BridgeRequest::CountGcContent {
        records: records.clone(),
    };
    let rows = count_gc_content(&records)?;
    Ok(CrossSurfaceFixtureCase {
        id: "geecee_basic".to_owned(),
        method: "geecee".to_owned(),
        family: "table".to_owned(),
        request,
        expected: CrossSurfaceExpected::GcRows { rows },
    })
}

fn pepstats_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let records = vec![protein_input("pep.a", "MSTN", Some("protein one"))];
    let request = BridgeRequest::PepstatsSummary {
        records: records.clone(),
    };
    let result = pepstats_summary(&records)?;
    Ok(CrossSurfaceFixtureCase {
        id: "pepstats_basic".to_owned(),
        method: "pepstats".to_owned(),
        family: "table".to_owned(),
        request,
        expected: CrossSurfaceExpected::Pepstats {
            summary_rows: result.summary_rows,
            composition_rows: result.composition_rows,
        },
    })
}

fn matcher_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let query = dna_input("query", "ACGT", None);
    let target = dna_input("target", "ACGA", None);
    let request = BridgeRequest::DirectMatchSequences {
        query: query.clone(),
        target: target.clone(),
    };
    let summary = direct_match_sequences(query, target)?;
    Ok(CrossSurfaceFixtureCase {
        id: "matcher_basic".to_owned(),
        method: "matcher".to_owned(),
        family: "alignment_summary".to_owned(),
        request,
        expected: CrossSurfaceExpected::MatcherSummary { summary },
    })
}

fn distance_matrix_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let records = vec![
        dna_input("a", "ACGT", None),
        dna_input("b", "ACGA", None),
        dna_input("c", "TCGT", None),
    ];
    let request = BridgeRequest::PDistanceForSequences {
        records: records.clone(),
    };
    let matrix = p_distance_for_sequences(&records)?;
    Ok(CrossSurfaceFixtureCase {
        id: "distmat_basic".to_owned(),
        method: "distmat".to_owned(),
        family: "alignment_summary".to_owned(),
        request,
        expected: CrossSurfaceExpected::DistanceMatrix { matrix },
    })
}

fn consensus_simple_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let alignment = simple_alignment();
    let request = BridgeRequest::ConsensusSimple {
        alignment: alignment.clone(),
    };
    let record = consensus_simple(alignment)?;
    Ok(CrossSurfaceFixtureCase {
        id: "cons_basic".to_owned(),
        method: "cons".to_owned(),
        family: "sequence_record".to_owned(),
        request,
        expected: CrossSurfaceExpected::SequenceRecord { record },
    })
}

fn consensus_ambiguous_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let alignment = simple_alignment();
    let request = BridgeRequest::ConsensusAmbiguous {
        alignment: alignment.clone(),
    };
    let record = consensus_ambiguous(alignment)?;
    Ok(CrossSurfaceFixtureCase {
        id: "consambig_basic".to_owned(),
        method: "consambig".to_owned(),
        family: "sequence_record".to_owned(),
        request,
        expected: CrossSurfaceExpected::SequenceRecord { record },
    })
}

fn charge_profile_case() -> Result<CrossSurfaceFixtureCase, PlatformError> {
    let record = protein_input("charge1", "AKRHDDE", None);
    let request = BridgeRequest::ChargeProfile {
        record: record.clone(),
        window: 5,
        step: 1,
    };
    let profile = charge_profile(record, 5, 1)?;
    Ok(CrossSurfaceFixtureCase {
        id: "charge_profile_basic".to_owned(),
        method: "charge_profile".to_owned(),
        family: "numeric_profile".to_owned(),
        request,
        expected: expected_charge_profile(profile),
    })
}

fn expected_charge_profile(profile: BridgeChargeProfile) -> CrossSurfaceExpected {
    CrossSurfaceExpected::ChargeProfile {
        identifier: profile.identifier,
        sequence_length: profile.sequence_length,
        window: profile.window,
        step: profile.step,
        windows: profile.windows,
    }
}

fn basic_dna_records() -> Vec<BridgeSequenceInput> {
    vec![
        dna_input("dna.a", "ACGT", Some("first dna")),
        dna_input("dna.b", "AAGT", Some("second dna")),
    ]
}

fn simple_alignment() -> epithema_r_bridge::BridgeAlignmentInput {
    epithema_r_bridge::BridgeAlignmentInput {
        identifier: Some("aln1".to_owned()),
        molecule: Some("dna".to_owned()),
        rows: vec![
            epithema_r_bridge::BridgeAlignmentRowInput {
                identifier: "a".to_owned(),
                aligned: "AC-GT".to_owned(),
                description: None,
            },
            epithema_r_bridge::BridgeAlignmentRowInput {
                identifier: "b".to_owned(),
                aligned: "ACTGT".to_owned(),
                description: None,
            },
            epithema_r_bridge::BridgeAlignmentRowInput {
                identifier: "c".to_owned(),
                aligned: "ACCGT".to_owned(),
                description: None,
            },
        ],
    }
}

fn dna_input(identifier: &str, sequence: &str, description: Option<&str>) -> BridgeSequenceInput {
    BridgeSequenceInput {
        identifier: Some(identifier.to_owned()),
        sequence: sequence.to_owned(),
        description: description.map(ToOwned::to_owned),
        molecule: Some("dna".to_owned()),
    }
}

fn protein_input(
    identifier: &str,
    sequence: &str,
    description: Option<&str>,
) -> BridgeSequenceInput {
    BridgeSequenceInput {
        identifier: Some(identifier.to_owned()),
        sequence: sequence.to_owned(),
        description: description.map(ToOwned::to_owned),
        molecule: Some("protein".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{CrossSurfaceFixtureCatalog, DEFAULT_NUMERIC_TOLERANCE};

    fn fixture(path: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }

    #[test]
    fn curated_catalog_matches_checked_in_fixture() {
        let expected = CrossSurfaceFixtureCatalog::curated().expect("catalog should build");
        let fixture_text =
            std::fs::read_to_string(fixture("tests/fixtures/cross_surface/curated_methods.json"))
                .expect("fixture should exist");
        let actual: CrossSurfaceFixtureCatalog =
            serde_json::from_str(&fixture_text).expect("fixture should parse");
        assert_eq!(actual, expected);
    }

    #[test]
    fn curated_catalog_covers_curated_method_subset() {
        let catalog = CrossSurfaceFixtureCatalog::curated().expect("catalog should build");
        let methods = catalog
            .cases
            .iter()
            .map(|case| case.method.as_str())
            .collect::<Vec<_>>();
        assert_eq!(catalog.canonical_owner, "epithema");
        assert_eq!(catalog.numeric_tolerance, DEFAULT_NUMERIC_TOLERANCE);
        assert_eq!(
            methods,
            vec![
                "newseq",
                "seqcount",
                "nthseq",
                "skipseq",
                "notseq",
                "extractseq",
                "degapseq",
                "trimseq",
                "descseq",
                "compseq",
                "geecee",
                "pepstats",
                "matcher",
                "distmat",
                "cons",
                "consambig",
                "charge_profile",
            ]
        );
    }
}
