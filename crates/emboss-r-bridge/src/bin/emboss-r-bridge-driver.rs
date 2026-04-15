//! Developer-facing JSON bridge driver for the first `emboss-r` method surface.

use std::io::{self, Read};

use emboss_r_bridge::protocol::{BridgeRequest, BridgeResponse};
use emboss_r_bridge::{
    backtranslate_ambiguous_sequences, backtranslate_representative_sequences, charge_profile,
    compare_translation_sets, complexity_profile, composition_summary, consensus_ambiguous,
    consensus_simple, count_gc_content, cut_sequences, degap_sequences, direct_match_sequences,
    extract_sequences, fuzz_nucleotide, fuzz_protein, fuzz_translated_frames, new_sequence,
    not_sequence, nth_sequence, p_distance_for_sequences, pepstats_summary, reverse_sequences,
    sequence_count, skip_sequences, split_sequence_partitions, trim_sequences,
    union_sequence_collections, update_descriptions,
};

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
        BridgeRequest::ReverseSequences { records } => BridgeResponse::ReverseSequences {
            records: reverse_sequences(&records)?,
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
