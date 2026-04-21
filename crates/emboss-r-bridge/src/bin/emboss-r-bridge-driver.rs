//! Developer-facing JSON bridge driver for the first `emboss-r` method surface.

use std::io::{self, Read};

use emboss_r_bridge::protocol::{BridgeRequest, BridgeResponse};
use emboss_r_bridge::{
    backtranslate_ambiguous_sequences, backtranslate_representative_sequences, charge_profile,
    compare_translation_sets, complexity_profile, composition_summary, consensus_ambiguous,
    consensus_simple, copy_features, count_gc_content, cut_sequences, degap_sequences,
    describe_sequence_file, describe_sequences, direct_match_sequences, extract_features,
    extract_sequences, fuzz_nucleotide, fuzz_protein, fuzz_translated_frames, mask_features,
    mask_sequences, new_sequence, not_sequence, nth_sequence, p_distance_for_sequences,
    pepstats_summary, reverse_sequences, sequence_count, skip_sequences,
    split_sequence_partitions, trim_sequences, union_sequence_collections, update_descriptions,
};
use emboss_r_bridge::list_tools;
use emboss_service::{EmbossService, ServiceRegistry};
use emboss_tools::governed_tool_descriptors;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let request: BridgeRequest = serde_json::from_str(&input)?;

    let response = match request {
        BridgeRequest::ListTools => BridgeResponse::ListTools {
            tools: list_tools(&implemented_service()),
        },
        BridgeRequest::NewSequence { record } => BridgeResponse::NewSequence {
            record: new_sequence(record)?,
        },
        BridgeRequest::SequenceCount { records } => BridgeResponse::SequenceCount {
            count: sequence_count(&records)?,
        },
        BridgeRequest::NthSequence { records, index } => BridgeResponse::NthSequence {
            record: nth_sequence(&records, index)?,
        },
        BridgeRequest::SkipSequences { records, count } => BridgeResponse::SkipSequences {
            records: skip_sequences(&records, count)?,
        },
        BridgeRequest::NotSequence { records, index } => BridgeResponse::NotSequence {
            records: not_sequence(&records, index)?,
        },
        BridgeRequest::ExtractSequences {
            records,
            start,
            end,
        } => BridgeResponse::ExtractSequences {
            records: extract_sequences(&records, start, end)?,
        },
        BridgeRequest::CutSequences {
            records,
            cut_position,
        } => BridgeResponse::CutSequences {
            records: cut_sequences(&records, cut_position)?,
        },
        BridgeRequest::UnionSequenceCollections { collections } => {
            BridgeResponse::UnionSequenceCollections {
                records: union_sequence_collections(&collections)?,
            }
        }
        BridgeRequest::SplitSequencePartitions {
            records,
            chunk_size,
        } => BridgeResponse::SplitSequencePartitions {
            partitions: split_sequence_partitions(&records, chunk_size)?,
        },
        BridgeRequest::DegapSequences { records } => BridgeResponse::DegapSequences {
            records: degap_sequences(&records)?,
        },
        BridgeRequest::ReverseSequences { records, mode } => BridgeResponse::ReverseSequences {
            records: reverse_sequences(&records, mode.as_deref())?,
        },
        BridgeRequest::MaskSequences {
            records,
            intervals,
            mask_char,
        } => BridgeResponse::MaskSequences {
            records: mask_sequences(&records, &intervals, mask_char.as_deref())?,
        },
        BridgeRequest::TrimSequences {
            records,
            left_trim,
            right_trim,
        } => BridgeResponse::TrimSequences {
            records: trim_sequences(&records, left_trim, right_trim)?,
        },
        BridgeRequest::UpdateDescriptions {
            records,
            description,
            clear,
        } => BridgeResponse::UpdateDescriptions {
            records: update_descriptions(&records, description, clear)?,
        },
        BridgeRequest::BacktranslateRepresentative { records } => {
            BridgeResponse::BacktranslateRepresentative {
                records: backtranslate_representative_sequences(&records)?,
            }
        }
        BridgeRequest::BacktranslateAmbiguous { records } => {
            BridgeResponse::BacktranslateAmbiguous {
                records: backtranslate_ambiguous_sequences(&records)?,
            }
        }
        BridgeRequest::CompareTranslationSets {
            nucleotide_records,
            protein_records,
        } => BridgeResponse::CompareTranslationSets {
            cases: compare_translation_sets(&nucleotide_records, &protein_records)?,
        },
        BridgeRequest::FuzzNucleotide { records, pattern } => BridgeResponse::FuzzNucleotide {
            hits: fuzz_nucleotide(&records, &pattern)?,
        },
        BridgeRequest::FuzzProtein { records, pattern } => BridgeResponse::FuzzProtein {
            hits: fuzz_protein(&records, &pattern)?,
        },
        BridgeRequest::FuzzTranslatedFrames { records, pattern } => {
            BridgeResponse::FuzzTranslatedFrames {
                hits: fuzz_translated_frames(&records, &pattern)?,
            }
        }
        BridgeRequest::CompositionSummary { records } => BridgeResponse::CompositionSummary {
            rows: composition_summary(&records)?,
        },
        BridgeRequest::CountGcContent { records } => BridgeResponse::CountGcContent {
            rows: count_gc_content(&records)?,
        },
        BridgeRequest::PepstatsSummary { records } => BridgeResponse::PepstatsSummary {
            result: pepstats_summary(&records)?,
        },
        BridgeRequest::DescribeSequences { records } => BridgeResponse::DescribeSequences {
            rows: describe_sequences(&records)?,
        },
        BridgeRequest::DescribeSequenceFile { input } => BridgeResponse::DescribeSequenceFile {
            rows: describe_sequence_file(&input)?,
        },
        BridgeRequest::ExtractFeatures {
            input,
            kind,
            name,
            qualifier,
            strand,
        } => BridgeResponse::ExtractFeatures {
            records: extract_features(
                &input,
                kind.as_deref(),
                name.as_deref(),
                qualifier.as_deref(),
                strand.as_deref(),
            )?,
        },
        BridgeRequest::MaskFeatures {
            input,
            kind,
            name,
            qualifier,
            strand,
            mask_char,
        } => BridgeResponse::MaskFeatures {
            records: mask_features(
                &input,
                kind.as_deref(),
                name.as_deref(),
                qualifier.as_deref(),
                strand.as_deref(),
                mask_char.as_deref(),
            )?,
        },
        BridgeRequest::CopyFeatures {
            source,
            target,
            kind,
            name,
            qualifier,
            strand,
        } => BridgeResponse::CopyFeatures {
            records: copy_features(
                &source,
                &target,
                kind.as_deref(),
                name.as_deref(),
                qualifier.as_deref(),
                strand.as_deref(),
            )?,
        },
        BridgeRequest::ComplexityProfile {
            record,
            k_min,
            k_max,
            window,
            step,
        } => BridgeResponse::ComplexityProfile {
            result: complexity_profile(record, k_min, k_max, window, step)?,
        },
        BridgeRequest::DirectMatchSequences { query, target } => {
            BridgeResponse::DirectMatchSequences {
                summary: direct_match_sequences(query, target)?,
            }
        }
        BridgeRequest::PDistanceForSequences { records } => BridgeResponse::PDistanceForSequences {
            matrix: p_distance_for_sequences(&records)?,
        },
        BridgeRequest::ConsensusSimple { alignment } => BridgeResponse::ConsensusSimple {
            record: consensus_simple(alignment)?,
        },
        BridgeRequest::ConsensusAmbiguous { alignment } => BridgeResponse::ConsensusAmbiguous {
            record: consensus_ambiguous(alignment)?,
        },
        BridgeRequest::ChargeProfile {
            record,
            window,
            step,
        } => BridgeResponse::ChargeProfile {
            profile: charge_profile(record, window, step)?,
        },
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}

fn implemented_service() -> EmbossService {
    let mut registry = ServiceRegistry::new();
    for descriptor in governed_tool_descriptors() {
        registry
            .register(*descriptor)
            .expect("governed descriptors should register without duplication");
    }
    EmbossService::new(registry)
}
