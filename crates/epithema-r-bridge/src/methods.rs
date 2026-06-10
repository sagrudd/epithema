//! Typed analytical bridge methods for the first idiomatic R surface.

use std::str::FromStr;

use epithema_core::{
    Alignment, AlignmentAnalysisError, AlignmentMode, AlignmentRow, Alphabet, ComplexityError,
    ComplexityParameters, ConsensusStrategy, DirectMatchSummary, DistanceMatrix, FeatureKind,
    FeatureSelector, GcSummary, Interval, MoleculeKind, NucleotidePattern, PatternError,
    PatternMatch, ProteinChargeError, ProteinPattern, ResidueComposition, RevseqMode,
    SequenceComplexity, SequenceIdentifier, SequenceMetadata, SequenceRecord, SequenceTopology,
    Strand, TranslationError, WindowComplexity, backtranslate_ambiguous,
    backtranslate_representative, consensus_sequence, direct_match_summary, mask_intervals,
    p_distance_matrix, protein_charge_profile, protein_molecular_weight, sequence_complexity,
    sliding_window_complexity, transform_sequence_record, translate_dna_frame,
    translate_dna_strict,
};
use epithema_diagnostics::{ErrorCategory, PlatformError};
use epithema_plot_contract::{
    AxisScaleHint, DataVector, GeometryHint, PlotAxis, PlotKind, PlotMetadata, PlotProvenance,
    PlotSeries, PlotSpec, SeriesStyle,
};
use epithema_tools::feature_tools::{
    ExtractfeatParams, FeatcopyParams, MaskfeatParams, run_extractfeat, run_featcopy, run_maskfeat,
};
use epithema_tools::sequence_edit::{DescseqParams, run_descseq};
use epithema_tools::sequence_stream::SequenceInput;

use crate::conversion::project_plot_contract;
use crate::types::{
    BridgeAlignmentInput, BridgeAlignmentRowInput, BridgeChargeProfile, BridgeChargeWindow,
    BridgeComplexityResult, BridgeComplexitySummary, BridgeComplexityWindow, BridgeCompositionRow,
    BridgeDescseqRow, BridgeDistanceMatrix, BridgeFeatureSummary, BridgeGcRow, BridgeIntervalInput,
    BridgeMatcherSummary, BridgePatternHit, BridgePepstatsResult, BridgePepstatsSummaryRow,
    BridgeSequenceInput, BridgeSequenceRecord, BridgeTranslationCheck,
};

/// Creates a validated bridge-safe sequence record from in-memory input.
pub fn new_sequence(input: BridgeSequenceInput) -> Result<BridgeSequenceRecord, PlatformError> {
    let record = build_sequence_record(input, 1)?;
    Ok(project_sequence_record(&record))
}

/// Counts bridge-safe sequence inputs.
pub fn sequence_count(inputs: &[BridgeSequenceInput]) -> Result<usize, PlatformError> {
    let records = build_sequence_records(inputs)?;
    Ok(records.len())
}

/// Selects the 1-based Nth bridge-safe sequence record.
pub fn nth_sequence(
    inputs: &[BridgeSequenceInput],
    index: usize,
) -> Result<BridgeSequenceRecord, PlatformError> {
    if index == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "sequence index must be 1 or greater",
        )
        .with_code("bridge.nth_sequence.index.invalid"));
    }

    let mut records = build_sequence_records(inputs)?;
    let total = records.len();
    if index > total {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("requested sequence index {index} is out of range for {total} records"),
        )
        .with_code("bridge.nth_sequence.index.out_of_range"));
    }

    Ok(project_sequence_record(&records.remove(index - 1)))
}

/// Skips the first `count` sequence records.
pub fn skip_sequences(
    inputs: &[BridgeSequenceInput],
    count: usize,
) -> Result<Vec<BridgeSequenceRecord>, PlatformError> {
    let mut records = build_sequence_records(inputs)?;
    let skipped = count.min(records.len());
    let remaining = records.split_off(skipped);
    Ok(remaining
        .iter()
        .map(project_sequence_record)
        .collect::<Vec<_>>())
}

/// Returns all sequence records except the 1-based excluded index.
pub fn not_sequence(
    inputs: &[BridgeSequenceInput],
    index: usize,
) -> Result<Vec<BridgeSequenceRecord>, PlatformError> {
    if index == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "excluded sequence index must be 1 or greater",
        )
        .with_code("bridge.not_sequence.index.invalid"));
    }

    let mut records = build_sequence_records(inputs)?;
    let total = records.len();
    if index > total {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("excluded sequence index {index} is out of range for {total} records"),
        )
        .with_code("bridge.not_sequence.index.out_of_range"));
    }

    records.remove(index - 1);
    Ok(records
        .iter()
        .map(project_sequence_record)
        .collect::<Vec<_>>())
}

/// Computes a bridge-safe protein charge profile with an attached plot contract.
pub fn charge_profile(
    input: BridgeSequenceInput,
    window: usize,
    step: usize,
) -> Result<BridgeChargeProfile, PlatformError> {
    let record = build_sequence_record(input, 1)?;
    let profile = protein_charge_profile(&record, window, step).map_err(map_charge_error)?;
    let plot = build_charge_plot(&profile)?;
    let plot_contract = project_plot_contract(&plot)?;

    Ok(BridgeChargeProfile {
        identifier: profile.identifier.clone(),
        sequence_length: profile.sequence_length,
        window: profile.window,
        step: profile.step,
        windows: profile
            .windows
            .iter()
            .map(|window| BridgeChargeWindow {
                identifier: profile.identifier.clone(),
                window_start: window.window_start,
                window_end: window.window_end,
                window_length: window.window_length,
                mean_charge: window.mean_charge,
            })
            .collect(),
        plot_contract_json: plot_contract.json,
    })
}

/// Extracts the same 1-based inclusive interval from each sequence.
pub fn extract_sequences(
    inputs: &[BridgeSequenceInput],
    start: usize,
    end: usize,
) -> Result<Vec<BridgeSequenceRecord>, PlatformError> {
    if start == 0 || end == 0 || start > end {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "extractseq requires 1-based inclusive coordinates with start <= end",
        )
        .with_code("bridge.extract_sequences.coordinates.invalid"));
    }

    build_sequence_records(inputs)?
        .into_iter()
        .map(|record| {
            if end > record.len() {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    format!(
                        "requested region {start}..{end} is out of range for sequence '{}' of length {}",
                        record.identifier().accession(),
                        record.len()
                    ),
                )
                .with_code("bridge.extract_sequences.coordinates.out_of_range"));
            }

            let interval = epithema_core::Interval::from_one_based_inclusive(start, end).map_err(
                |error| {
                    PlatformError::new(ErrorCategory::Validation, error.to_string())
                        .with_code("bridge.extract_sequences.interval.invalid")
                },
            )?;
            let subsequence = record.subsequence(interval).map_err(|error| {
                PlatformError::new(ErrorCategory::Validation, error.to_string())
                    .with_code("bridge.extract_sequences.interval.invalid")
            })?;
            let extracted =
                SequenceRecord::new(record.identifier().clone(), record.molecule(), subsequence)
                    .map_err(|error| {
                        PlatformError::new(ErrorCategory::Validation, error.to_string())
                            .with_code("bridge.extract_sequences.record.invalid")
                    })?
                    .with_metadata(record.metadata().clone());
            Ok(project_sequence_record(&extracted))
        })
        .collect()
}

/// Cuts each sequence after the supplied 1-based interior position.
pub fn cut_sequences(
    inputs: &[BridgeSequenceInput],
    cut_position: usize,
) -> Result<Vec<BridgeSequenceRecord>, PlatformError> {
    if cut_position == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "cut position must be 1 or greater",
        )
        .with_code("bridge.cut_sequences.position.invalid"));
    }

    let mut output = Vec::new();
    for record in build_sequence_records(inputs)? {
        if cut_position >= record.len() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!(
                    "cut position {cut_position} must be an interior position for sequence '{}' of length {}",
                    record.identifier().accession(),
                    record.len()
                ),
            )
            .with_code("bridge.cut_sequences.position.out_of_range"));
        }
        let left = build_fragment_record(&record, "left", &record.residues()[..cut_position])?;
        let right = build_fragment_record(&record, "right", &record.residues()[cut_position..])?;
        output.push(project_sequence_record(&left));
        output.push(project_sequence_record(&right));
    }
    Ok(output)
}

/// Concatenates ordered input collections into one sequence collection.
pub fn union_sequence_collections(
    collections: &[Vec<BridgeSequenceInput>],
) -> Result<Vec<BridgeSequenceRecord>, PlatformError> {
    if collections.len() < 2 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "union requires at least two input collections",
        )
        .with_code("bridge.union_sequence_collections.inputs.too_few"));
    }

    let mut output = Vec::new();
    for collection in collections {
        output.extend(
            build_sequence_records(collection)?
                .iter()
                .map(project_sequence_record)
                .collect::<Vec<_>>(),
        );
    }
    Ok(output)
}

/// Partitions a sequence collection into fixed-size chunks.
pub fn split_sequence_partitions(
    inputs: &[BridgeSequenceInput],
    chunk_size: usize,
) -> Result<Vec<Vec<BridgeSequenceRecord>>, PlatformError> {
    if chunk_size == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "splitter requires chunk size >= 1",
        )
        .with_code("bridge.split_sequence_partitions.chunk_size.invalid"));
    }

    let records = build_sequence_records(inputs)?;
    Ok(records
        .chunks(chunk_size)
        .map(|chunk| {
            chunk
                .iter()
                .map(project_sequence_record)
                .collect::<Vec<_>>()
        })
        .collect())
}

/// Removes gap characters from sequences.
pub fn degap_sequences(
    inputs: &[BridgeSequenceInput],
) -> Result<Vec<BridgeSequenceRecord>, PlatformError> {
    build_sequence_records(inputs)?
        .into_iter()
        .map(|record| {
            let cleaned: String = record
                .residues()
                .chars()
                .filter(|symbol| !matches!(symbol, '-' | '.'))
                .collect();
            if cleaned.is_empty() {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    format!(
                        "degapping removed all residues from sequence '{}'",
                        record.identifier().accession()
                    ),
                )
                .with_code("bridge.degap_sequences.sequence.empty"));
            }
            let updated =
                SequenceRecord::new(record.identifier().clone(), record.molecule(), cleaned)
                    .map_err(|error| {
                        PlatformError::new(ErrorCategory::Validation, error.to_string())
                            .with_code("bridge.degap_sequences.sequence.invalid")
                    })?
                    .with_metadata(record.metadata().clone());
            Ok(project_sequence_record(&updated))
        })
        .collect()
}

/// Reverses sequence content using `revseq`-style modes.
pub fn reverse_sequences(
    inputs: &[BridgeSequenceInput],
    mode: Option<&str>,
) -> Result<Vec<BridgeSequenceRecord>, PlatformError> {
    let mode = parse_revseq_mode(mode)?;
    build_sequence_records(inputs)?
        .into_iter()
        .map(|record| {
            transform_sequence_record(&record, mode)
                .map(|updated| project_sequence_record(&updated))
                .map_err(map_revseq_error)
        })
        .collect()
}

/// Masks one or more explicit intervals in each sequence.
pub fn mask_sequences(
    inputs: &[BridgeSequenceInput],
    intervals: &[BridgeIntervalInput],
    mask_char: Option<&str>,
) -> Result<Vec<BridgeSequenceRecord>, PlatformError> {
    if intervals.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "maskseq requires at least one interval",
        )
        .with_code("bridge.mask_sequences.interval.missing"));
    }

    let intervals = intervals
        .iter()
        .map(|interval| {
            Interval::from_one_based_inclusive(interval.start, interval.end).map_err(|error| {
                PlatformError::new(ErrorCategory::Validation, error.to_string())
                    .with_code("bridge.mask_sequences.interval.invalid")
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    build_sequence_records(inputs)?
        .into_iter()
        .map(|record| {
            let mask_symbol = effective_mask_symbol_for_record("maskseq", &record, mask_char)?;
            let updated = mask_intervals(&record, &intervals, mask_symbol).map_err(|error| {
                PlatformError::new(ErrorCategory::Validation, error.to_string())
                    .with_code("bridge.mask_sequences.interval.invalid")
            })?;
            Ok(project_sequence_record(&updated))
        })
        .collect()
}

/// Trims explicit residue counts from each sequence.
pub fn trim_sequences(
    inputs: &[BridgeSequenceInput],
    left_trim: usize,
    right_trim: usize,
) -> Result<Vec<BridgeSequenceRecord>, PlatformError> {
    build_sequence_records(inputs)?
        .into_iter()
        .map(|record| {
            let total_trim = left_trim.saturating_add(right_trim);
            if total_trim >= record.len() {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    format!(
                        "trimming {} residues would exhaust sequence '{}' of length {}",
                        total_trim,
                        record.identifier().accession(),
                        record.len()
                    ),
                )
                .with_code("bridge.trim_sequences.trim.exhausted"));
            }
            let end = record.len() - right_trim;
            let trimmed = &record.residues()[left_trim..end];
            let updated =
                SequenceRecord::new(record.identifier().clone(), record.molecule(), trimmed)
                    .map_err(|error| {
                        PlatformError::new(ErrorCategory::Validation, error.to_string())
                            .with_code("bridge.trim_sequences.sequence.invalid")
                    })?
                    .with_metadata(record.metadata().clone());
            Ok(project_sequence_record(&updated))
        })
        .collect()
}

/// Replaces or clears sequence descriptions.
pub fn update_descriptions(
    inputs: &[BridgeSequenceInput],
    description: Option<String>,
    clear: bool,
) -> Result<Vec<BridgeSequenceRecord>, PlatformError> {
    if clear == description.is_some() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "description update requires exactly one of description or clear",
        )
        .with_code("bridge.update_descriptions.arguments.invalid"));
    }

    Ok(build_sequence_records(inputs)?
        .into_iter()
        .map(|record| {
            let mut metadata = record.metadata().clone();
            metadata.description = description.clone();
            project_sequence_record(&record.with_metadata(metadata))
        })
        .collect())
}

/// Back-translates protein sequences to representative DNA codons.
pub fn backtranslate_representative_sequences(
    inputs: &[BridgeSequenceInput],
) -> Result<Vec<BridgeSequenceRecord>, PlatformError> {
    backtranslate_sequences(inputs, false)
}

/// Back-translates protein sequences to ambiguous DNA codons.
pub fn backtranslate_ambiguous_sequences(
    inputs: &[BridgeSequenceInput],
) -> Result<Vec<BridgeSequenceRecord>, PlatformError> {
    backtranslate_sequences(inputs, true)
}

/// Compares nucleotide coding sequences to expected proteins.
pub fn compare_translation_sets(
    nucleotide_inputs: &[BridgeSequenceInput],
    protein_inputs: &[BridgeSequenceInput],
) -> Result<Vec<BridgeTranslationCheck>, PlatformError> {
    let nucleotides = build_sequence_records(nucleotide_inputs)?;
    let proteins = build_sequence_records(protein_inputs)?;
    if nucleotides.len() != proteins.len() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "checktrans requires nucleotide and protein collections to contain the same number of records",
        )
        .with_code("bridge.compare_translation_sets.records.count_mismatch"));
    }

    nucleotides
        .into_iter()
        .zip(proteins)
        .map(|(nucleotide, protein)| {
            if !nucleotide.molecule().is_nucleotide() {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    format!(
                        "expected nucleotide coding input but '{}' was classified as {}",
                        nucleotide.identifier().accession(),
                        nucleotide.molecule()
                    ),
                )
                .with_code("bridge.compare_translation_sets.nucleotide.not_nucleotide"));
            }
            if protein.molecule().is_nucleotide() {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    format!(
                        "expected protein input but '{}' was classified as {}",
                        protein.identifier().accession(),
                        protein.molecule()
                    ),
                )
                .with_code("bridge.compare_translation_sets.protein.not_protein"));
            }

            let translated =
                translate_dna_strict(nucleotide.residues()).map_err(map_translation_error)?;
            let translated_terminal_stop = translated.ends_with('*');
            let expected_terminal_stop = protein.residues().ends_with('*');
            let normalized_translated = normalize_terminal_stop(&translated);
            let normalized_expected = normalize_terminal_stop(protein.residues());
            let matches = normalized_translated == normalized_expected;
            let detail = if matches {
                "translated protein matches expected sequence".to_owned()
            } else {
                format!(
                    "translation mismatch: translated '{}' versus expected '{}'",
                    normalized_translated, normalized_expected
                )
            };

            Ok(BridgeTranslationCheck {
                nucleotide_id: nucleotide.identifier().accession().to_owned(),
                protein_id: protein.identifier().accession().to_owned(),
                matches,
                translated_protein: translated,
                expected_protein: protein.residues().to_owned(),
                translated_terminal_stop,
                expected_terminal_stop,
                detail,
            })
        })
        .collect()
}

/// Scans nucleotide sequences with an IUPAC-aware pattern.
pub fn fuzz_nucleotide(
    inputs: &[BridgeSequenceInput],
    pattern: &str,
) -> Result<Vec<BridgePatternHit>, PlatformError> {
    let pattern = NucleotidePattern::parse(pattern).map_err(map_pattern_error)?;
    let pattern_text = pattern.raw().to_owned();
    let mut hits = Vec::new();
    for record in build_sequence_records(inputs)? {
        ensure_nucleotide_record("fuzznuc", &record)?;
        hits.extend(pattern.scan(record.residues()).into_iter().map(|hit| {
            project_pattern_hit(
                record.identifier().accession(),
                &pattern_text,
                hit,
                Some("forward".to_owned()),
                None,
                None,
                None,
                None,
            )
        }));
    }
    Ok(hits)
}

/// Scans protein sequences with an exact or X-wildcard pattern.
pub fn fuzz_protein(
    inputs: &[BridgeSequenceInput],
    pattern: &str,
) -> Result<Vec<BridgePatternHit>, PlatformError> {
    let pattern = ProteinPattern::parse(pattern).map_err(map_pattern_error)?;
    let pattern_text = pattern.raw().to_owned();
    let mut hits = Vec::new();
    for record in build_sequence_records(inputs)? {
        ensure_protein_record("fuzzpro", &record)?;
        hits.extend(pattern.scan(record.residues()).into_iter().map(|hit| {
            project_pattern_hit(
                record.identifier().accession(),
                &pattern_text,
                hit,
                None,
                None,
                None,
                None,
                None,
            )
        }));
    }
    Ok(hits)
}

/// Translates nucleotide sequences in all three forward frames and scans for a protein pattern.
pub fn fuzz_translated_frames(
    inputs: &[BridgeSequenceInput],
    pattern: &str,
) -> Result<Vec<BridgePatternHit>, PlatformError> {
    let pattern = ProteinPattern::parse(pattern).map_err(map_pattern_error)?;
    let pattern_text = pattern.raw().to_owned();
    let mut hits = Vec::new();
    for record in build_sequence_records(inputs)? {
        ensure_nucleotide_record("fuzztran", &record)?;
        for frame_offset in 0..3 {
            let translated = translate_dna_frame(record.residues(), frame_offset)
                .map_err(map_translation_error)?;
            hits.extend(
                pattern
                    .scan(&translated)
                    .into_iter()
                    .map(|hit| BridgePatternHit {
                        identifier: record.identifier().accession().to_owned(),
                        pattern: pattern_text.clone(),
                        strand: None,
                        frame: Some(frame_offset + 1),
                        start: hit.start(),
                        end: hit.end(),
                        amino_start: Some(hit.start()),
                        amino_end: Some(hit.end()),
                        nucleotide_start: Some(frame_offset + hit.start() * 3),
                        nucleotide_end: Some(frame_offset + hit.end() * 3),
                        matched: hit.matched().to_owned(),
                    }),
            );
        }
    }
    Ok(hits)
}

/// Computes per-record and aggregate residue composition summaries.
pub fn composition_summary(
    inputs: &[BridgeSequenceInput],
) -> Result<Vec<BridgeCompositionRow>, PlatformError> {
    let records = build_sequence_records(inputs)?;
    let mut rows = Vec::new();
    let mut aggregate = ResidueComposition::default();
    for record in records {
        let composition = ResidueComposition::from_sequence(record.residues());
        aggregate.merge(&composition);
        rows.extend(project_composition_rows(
            "record",
            Some(record.identifier().accession().to_owned()),
            Some(record.molecule().as_str().to_owned()),
            Some(record.len()),
            &composition,
        ));
    }
    rows.extend(project_composition_rows(
        "aggregate",
        None,
        None,
        None,
        &aggregate,
    ));
    Ok(rows)
}

/// Computes per-record and aggregate GC summaries.
pub fn count_gc_content(inputs: &[BridgeSequenceInput]) -> Result<Vec<BridgeGcRow>, PlatformError> {
    let records = build_sequence_records(inputs)?;
    let mut rows = Vec::new();
    let mut aggregate = GcSummary::default();
    let mut total_length = 0usize;
    for record in records {
        ensure_nucleotide_record("geecee", &record)?;
        let gc = GcSummary::from_sequence(record.residues());
        aggregate.merge(&gc);
        total_length += record.len();
        rows.push(project_gc_row(
            "record",
            Some(record.identifier().accession().to_owned()),
            record.len(),
            &gc,
        ));
    }
    rows.push(project_gc_row("aggregate", None, total_length, &aggregate));
    Ok(rows)
}

/// Computes deterministic pepstats summaries and composition rows.
pub fn pepstats_summary(
    inputs: &[BridgeSequenceInput],
) -> Result<BridgePepstatsResult, PlatformError> {
    let records = build_sequence_records(inputs)?;
    let mut summary_rows = Vec::new();
    let mut composition_rows = Vec::new();
    for record in records {
        ensure_protein_record("pepstats", &record)?;
        let composition = ResidueComposition::from_sequence(record.residues());
        let stop_count = composition.count_for('*');
        let residue_length = composition.counted_symbols().saturating_sub(stop_count);
        let molecular_weight =
            protein_molecular_weight(record.residues()).map_err(map_composition_error)?;
        summary_rows.push(BridgePepstatsSummaryRow {
            identifier: record.identifier().accession().to_owned(),
            sequence_length: record.len(),
            residue_length,
            stop_count,
            molecular_weight,
        });
        composition_rows.extend(project_composition_rows(
            "record",
            Some(record.identifier().accession().to_owned()),
            Some(record.molecule().as_str().to_owned()),
            Some(record.len()),
            &composition,
        ));
    }

    Ok(BridgePepstatsResult {
        summary_rows,
        composition_rows,
    })
}

/// Reports `descseq`-style summaries for in-memory records.
pub fn describe_sequences(
    inputs: &[BridgeSequenceInput],
) -> Result<Vec<BridgeDescseqRow>, PlatformError> {
    build_sequence_records(inputs)?
        .into_iter()
        .enumerate()
        .map(|(index, record)| Ok(project_descseq_row(index + 1, &record)))
        .collect()
}

/// Executes file-backed `descseq` and projects stable rows.
pub fn describe_sequence_file(input: &str) -> Result<Vec<BridgeDescseqRow>, PlatformError> {
    let outcome = run_descseq(DescseqParams {
        input: SequenceInput::new(input),
    })
    .map_err(map_tool_error)?;

    Ok(outcome
        .rows
        .into_iter()
        .map(|row| BridgeDescseqRow {
            ordinal: row.ordinal,
            identifier: row.identifier,
            display_name: row.display_name,
            description: row.description,
            length: row.length,
            molecule: row.molecule,
            alphabet: row.alphabet,
            feature_count: row.feature_count,
            source: row.source,
            organism: row.organism,
            topology: row.topology,
        })
        .collect())
}

/// Executes file-backed `extractfeat`.
pub fn extract_features(
    input: &str,
    kind: Option<&str>,
    name: Option<&str>,
    qualifier: Option<&str>,
    strand: Option<&str>,
) -> Result<Vec<BridgeSequenceRecord>, PlatformError> {
    let selector = build_feature_selector(kind, name, qualifier, strand)?;
    let outcome = run_extractfeat(ExtractfeatParams {
        input: SequenceInput::new(input),
        selector,
    })
    .map_err(map_tool_error)?;
    Ok(outcome
        .records
        .iter()
        .map(|record| project_sequence_record(&record.record))
        .collect())
}

/// Executes file-backed `maskfeat`.
pub fn mask_features(
    input: &str,
    kind: Option<&str>,
    name: Option<&str>,
    qualifier: Option<&str>,
    strand: Option<&str>,
    mask_char: Option<&str>,
) -> Result<Vec<BridgeSequenceRecord>, PlatformError> {
    let selector = build_feature_selector(kind, name, qualifier, strand)?;
    let outcome = run_maskfeat(MaskfeatParams {
        input: SequenceInput::new(input),
        selector,
        mask_char: parse_mask_char(mask_char)?,
    })
    .map_err(map_tool_error)?;
    Ok(outcome
        .records
        .iter()
        .map(project_sequence_record)
        .collect())
}

/// Executes file-backed `featcopy`.
pub fn copy_features(
    source: &str,
    target: &str,
    kind: Option<&str>,
    name: Option<&str>,
    qualifier: Option<&str>,
    strand: Option<&str>,
) -> Result<Vec<BridgeSequenceRecord>, PlatformError> {
    let selector = build_feature_selector(kind, name, qualifier, strand)?;
    let outcome = run_featcopy(FeatcopyParams {
        source: SequenceInput::new(source),
        target: SequenceInput::new(target),
        selector,
    })
    .map_err(map_tool_error)?;
    Ok(outcome
        .records
        .iter()
        .map(project_sequence_record)
        .collect())
}

/// Computes whole-sequence and optional sliding-window linguistic complexity.
pub fn complexity_profile(
    input: BridgeSequenceInput,
    k_min: usize,
    k_max: usize,
    window: Option<usize>,
    step: Option<usize>,
) -> Result<BridgeComplexityResult, PlatformError> {
    let record = build_sequence_record(input, 1)?;
    let parameters = ComplexityParameters { k_min, k_max };
    let summary = sequence_complexity(&record, parameters).map_err(map_complexity_error)?;
    let windows = match (window, step) {
        (Some(window), Some(step)) => sliding_window_complexity(&record, window, step, parameters)
            .map_err(map_complexity_error)?,
        (None, None) => Vec::new(),
        _ => {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "complexity windowed analysis requires both window and step when either is supplied",
            )
            .with_code("bridge.complexity.window.arguments.invalid"));
        }
    };

    Ok(BridgeComplexityResult {
        summary: project_complexity_summary(&summary),
        windows: windows.iter().map(project_complexity_window).collect(),
    })
}

/// Computes an ungapped direct-match summary for two singleton sequences.
pub fn direct_match_sequences(
    query: BridgeSequenceInput,
    target: BridgeSequenceInput,
) -> Result<BridgeMatcherSummary, PlatformError> {
    let query = build_sequence_record(query, 1)?;
    let target = build_sequence_record(target, 1)?;
    let summary = direct_match_summary(&query, &target).map_err(map_alignment_analysis_error)?;
    Ok(project_match_summary(&summary))
}

/// Computes a deterministic equal-length p-distance matrix.
pub fn p_distance_for_sequences(
    inputs: &[BridgeSequenceInput],
) -> Result<BridgeDistanceMatrix, PlatformError> {
    let records = build_sequence_records(inputs)?;
    let matrix = p_distance_matrix(&records).map_err(map_alignment_analysis_error)?;
    Ok(project_distance_matrix(&matrix))
}

/// Derives a simple consensus sequence from an alignment input.
pub fn consensus_simple(
    input: BridgeAlignmentInput,
) -> Result<BridgeSequenceRecord, PlatformError> {
    consensus_from_alignment(input, ConsensusStrategy::Simple, "consensus")
}

/// Derives an ambiguity-aware consensus sequence from an alignment input.
pub fn consensus_ambiguous(
    input: BridgeAlignmentInput,
) -> Result<BridgeSequenceRecord, PlatformError> {
    consensus_from_alignment(input, ConsensusStrategy::Ambiguous, "consambig")
}

fn build_sequence_records(
    inputs: &[BridgeSequenceInput],
) -> Result<Vec<SequenceRecord>, PlatformError> {
    if inputs.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "sequence collection must contain at least one record",
        )
        .with_code("bridge.sequence_collection.empty"));
    }

    inputs
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, input)| build_sequence_record(input, index + 1))
        .collect()
}

fn build_sequence_record(
    input: BridgeSequenceInput,
    position: usize,
) -> Result<SequenceRecord, PlatformError> {
    let identifier = SequenceIdentifier::new(
        input
            .identifier
            .unwrap_or_else(|| format!("sequence{position}")),
    )
    .map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("bridge.sequence.identifier.invalid")
    })?;

    let molecule = match input.molecule {
        Some(label) => MoleculeKind::from_str(&label).map_err(|_| {
            PlatformError::new(
                ErrorCategory::Validation,
                format!("unsupported molecule label '{label}'"),
            )
            .with_code("bridge.sequence.molecule.invalid")
        })?,
        None => infer_molecule_kind(&input.sequence),
    };

    let mut record =
        SequenceRecord::new(identifier, molecule, input.sequence).map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("bridge.sequence.invalid")
        })?;

    if let Some(description) = input.description {
        record = record.with_metadata(SequenceMetadata::new().with_description(description));
    }

    Ok(record)
}

fn infer_molecule_kind(residues: &str) -> MoleculeKind {
    let uppercase: String = residues
        .chars()
        .filter(|symbol| !symbol.is_whitespace())
        .map(|symbol| symbol.to_ascii_uppercase())
        .collect();

    let has_u = uppercase.contains('U');
    let has_t = uppercase.contains('T');
    if has_u && !has_t {
        return MoleculeKind::Rna;
    }
    if has_t && !has_u {
        return MoleculeKind::Dna;
    }

    if uppercase
        .chars()
        .all(|symbol| matches!(symbol, 'A' | 'C' | 'G' | 'T' | 'N' | '-' | '*'))
    {
        return MoleculeKind::Dna;
    }
    if uppercase
        .chars()
        .all(|symbol| matches!(symbol, 'A' | 'C' | 'G' | 'U' | 'N' | '-' | '*'))
    {
        return MoleculeKind::Rna;
    }

    MoleculeKind::Protein
}

fn parse_revseq_mode(mode: Option<&str>) -> Result<RevseqMode, PlatformError> {
    match mode.unwrap_or("auto") {
        "auto" => Ok(RevseqMode::Auto),
        "reverse_only" | "reverse-only" => Ok(RevseqMode::ReverseOnly),
        "complement" | "reverse_complement" | "reverse-complement" => {
            Ok(RevseqMode::ReverseComplement)
        }
        other => Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "unsupported revseq mode '{other}'; expected auto, reverse_only, or complement"
            ),
        )
        .with_code("bridge.reverse_sequences.mode.invalid")),
    }
}

fn parse_mask_char(mask_char: Option<&str>) -> Result<Option<char>, PlatformError> {
    match mask_char {
        None => Ok(None),
        Some(value) => {
            let mut chars = value.chars();
            let symbol = chars.next().ok_or_else(|| {
                PlatformError::new(
                    ErrorCategory::Validation,
                    "mask character must be a single non-empty character",
                )
                .with_code("bridge.mask_char.invalid")
            })?;
            if chars.next().is_some() {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    "mask character must be a single character",
                )
                .with_code("bridge.mask_char.invalid"));
            }
            Ok(Some(symbol.to_ascii_uppercase()))
        }
    }
}

fn effective_mask_symbol_for_record(
    tool: &str,
    record: &SequenceRecord,
    mask_char: Option<&str>,
) -> Result<char, PlatformError> {
    let explicit = parse_mask_char(mask_char)?;
    let symbol = explicit.unwrap_or_else(|| {
        if record.molecule().is_protein() {
            'X'
        } else {
            'N'
        }
    });

    record
        .alphabet()
        .validate(record.molecule(), &symbol.to_string())
        .map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code(format!("bridge.{tool}.mask_char.invalid_for_molecule"))
        })?;
    Ok(symbol)
}

fn build_feature_selector(
    kind: Option<&str>,
    name: Option<&str>,
    qualifier: Option<&str>,
    strand: Option<&str>,
) -> Result<FeatureSelector, PlatformError> {
    let mut selectors = Vec::new();

    if let Some(kind) = kind {
        selectors.push(FeatureSelector::Kind(parse_feature_kind(kind)));
    }
    if let Some(name) = name {
        selectors.push(FeatureSelector::Name(name.to_owned()));
    }
    if let Some(qualifier) = qualifier {
        selectors.push(parse_feature_qualifier_selector(qualifier)?);
    }
    if let Some(strand) = strand {
        selectors.push(FeatureSelector::Strand(parse_feature_strand(strand)?));
    }

    Ok(match selectors.len() {
        0 => FeatureSelector::Any,
        1 => selectors.into_iter().next().expect("selector exists"),
        _ => FeatureSelector::All(selectors),
    })
}

fn parse_feature_kind(value: &str) -> FeatureKind {
    match value.trim().to_ascii_lowercase().as_str() {
        "gene" => FeatureKind::Gene,
        "cds" | "coding_sequence" => FeatureKind::CodingSequence,
        "exon" => FeatureKind::Exon,
        "intron" => FeatureKind::Intron,
        "region" => FeatureKind::Region,
        "motif" => FeatureKind::Motif,
        "repeat_region" | "repeat" => FeatureKind::RepeatRegion,
        "misc_feature" | "misc" => FeatureKind::MiscFeature,
        other => FeatureKind::Other(other.to_owned()),
    }
}

fn parse_feature_qualifier_selector(value: &str) -> Result<FeatureSelector, PlatformError> {
    let (key, qualifier_value) = match value.split_once('=') {
        Some((key, qualifier_value)) => (key.trim(), Some(qualifier_value.trim().to_owned())),
        None => (value.trim(), None),
    };

    if key.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "feature qualifier selectors require a non-empty key",
        )
        .with_code("bridge.feature.qualifier.invalid"));
    }

    Ok(FeatureSelector::Qualifier {
        key: key.to_owned(),
        value: qualifier_value.filter(|value| !value.is_empty()),
    })
}

fn parse_feature_strand(value: &str) -> Result<Strand, PlatformError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "forward" | "+" | "plus" => Ok(Strand::Forward),
        "reverse" | "-" | "minus" => Ok(Strand::Reverse),
        "unknown" | "." | "?" => Ok(Strand::Unknown),
        _ => Err(PlatformError::new(
            ErrorCategory::Validation,
            "strand must be one of forward, reverse, or unknown",
        )
        .with_code("bridge.feature.strand.invalid")),
    }
}

fn project_sequence_record(record: &SequenceRecord) -> BridgeSequenceRecord {
    BridgeSequenceRecord {
        identifier: record.identifier().accession().to_owned(),
        sequence: record.residues().to_owned(),
        description: record.metadata().description.clone(),
        molecule: record.molecule().as_str().to_owned(),
        alphabet: match record.alphabet() {
            Alphabet::Dna => "dna",
            Alphabet::Rna => "rna",
            Alphabet::Protein => "protein",
            Alphabet::Text => "text",
        }
        .to_owned(),
        length: record.len(),
        feature_count: record.features().len(),
        features: record
            .features()
            .iter()
            .map(project_feature_summary)
            .collect(),
    }
}

fn project_feature_summary(feature: &epithema_core::Feature) -> BridgeFeatureSummary {
    let bounds = feature.location.bounds();
    BridgeFeatureSummary {
        kind: feature_kind_label(&feature.kind),
        name: feature.name.clone(),
        start: bounds.start(),
        end: bounds.end(),
        strand: feature.location.strand().map(|strand| strand.to_string()),
        span_count: feature.location.spans().len(),
        qualifier_count: feature.qualifiers.len(),
    }
}

fn feature_kind_label(kind: &FeatureKind) -> String {
    match kind {
        FeatureKind::Gene => "gene".to_owned(),
        FeatureKind::CodingSequence => "cds".to_owned(),
        FeatureKind::Exon => "exon".to_owned(),
        FeatureKind::Intron => "intron".to_owned(),
        FeatureKind::Region => "region".to_owned(),
        FeatureKind::Motif => "motif".to_owned(),
        FeatureKind::RepeatRegion => "repeat_region".to_owned(),
        FeatureKind::MiscFeature => "misc_feature".to_owned(),
        FeatureKind::Other(label) => label.clone(),
    }
}

fn project_descseq_row(ordinal: usize, record: &SequenceRecord) -> BridgeDescseqRow {
    BridgeDescseqRow {
        ordinal,
        identifier: record.identifier().accession().to_owned(),
        display_name: record.identifier().display_name().map(ToOwned::to_owned),
        description: record.metadata().description.clone(),
        length: record.len(),
        molecule: record.molecule().to_string(),
        alphabet: record.alphabet().to_string(),
        feature_count: record.features().len(),
        source: record.metadata().source.clone(),
        organism: record.metadata().organism.clone(),
        topology: record.metadata().topology.map(|topology| match topology {
            SequenceTopology::Linear => "linear".to_owned(),
            SequenceTopology::Circular => "circular".to_owned(),
        }),
    }
}

fn build_charge_plot(
    profile: &epithema_core::ProteinChargeProfile,
) -> Result<epithema_plot_contract::PlotPayload, PlatformError> {
    let plot = PlotSpec::new(
        PlotKind::Line,
        PlotMetadata::new(
            format!("charge_{}", profile.identifier),
            format!("Charge profile for {}", profile.identifier),
        )
        .with_subtitle(format!("Window {} step {}", profile.window, profile.step))
        .with_provenance(PlotProvenance {
            tool: Some("charge".to_owned()),
            method: Some("protein_charge_profile".to_owned()),
            source_artifact_ids: vec!["table:charge-profile".to_owned()],
        }),
        PlotAxis::new("Window start").with_scale_hint(AxisScaleHint::Linear),
        PlotAxis::new("Mean charge").with_scale_hint(AxisScaleHint::Linear),
        vec![
            PlotSeries::new(
                "charge_profile",
                "Charge profile",
                DataVector::Numeric(
                    profile
                        .windows
                        .iter()
                        .map(|window| window.window_start as f64)
                        .collect(),
                ),
                profile
                    .windows
                    .iter()
                    .map(|window| window.mean_charge)
                    .collect(),
            )
            .with_legend_label("Charge profile")
            .with_semantic_group("charge")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Line)
                    .with_color_role("primary"),
            ),
        ],
    );

    plot.validate().map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("bridge.charge.plot.invalid")
    })?;

    Ok(plot)
}

fn map_charge_error(error: ProteinChargeError) -> PlatformError {
    let code = match error {
        ProteinChargeError::NonProteinSequence => "bridge.charge.input.non_protein",
        ProteinChargeError::UnsupportedResidue { .. } => "bridge.charge.input.unsupported_residue",
        ProteinChargeError::InvalidWindow { .. } => "bridge.charge.window.invalid",
        ProteinChargeError::InvalidStep { .. } => "bridge.charge.step.invalid",
        ProteinChargeError::SequenceShorterThanWindow { .. } => {
            "bridge.charge.window.sequence_too_short"
        }
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}

fn build_fragment_record(
    record: &SequenceRecord,
    suffix: &str,
    residues: &str,
) -> Result<SequenceRecord, PlatformError> {
    let identifier =
        SequenceIdentifier::new(format!("{}.{}", record.identifier().accession(), suffix))
            .map_err(|error| {
                PlatformError::new(ErrorCategory::Validation, error.to_string())
                    .with_code("bridge.cut_sequences.identifier.invalid")
            })?;
    let fragment = SequenceRecord::new(identifier, record.molecule(), residues)
        .map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("bridge.cut_sequences.record.invalid")
        })?
        .with_metadata(record.metadata().clone());
    Ok(fragment)
}

fn backtranslate_sequences(
    inputs: &[BridgeSequenceInput],
    ambiguous: bool,
) -> Result<Vec<BridgeSequenceRecord>, PlatformError> {
    build_sequence_records(inputs)?
        .into_iter()
        .map(|record| {
            ensure_protein_record("backtranslate", &record)?;
            let translated = if ambiguous {
                backtranslate_ambiguous(record.residues())
            } else {
                backtranslate_representative(record.residues())
            }
            .map_err(map_translation_error)?;

            let translated =
                SequenceRecord::new(record.identifier().clone(), MoleculeKind::Dna, translated)
                    .map_err(|error| {
                        PlatformError::new(ErrorCategory::Validation, error.to_string())
                            .with_code("bridge.backtranslate.sequence.invalid")
                    })?
                    .with_metadata(record.metadata().clone());
            Ok(project_sequence_record(&translated))
        })
        .collect()
}

fn normalize_terminal_stop(protein: &str) -> String {
    protein.strip_suffix('*').unwrap_or(protein).to_owned()
}

fn ensure_nucleotide_record(tool: &str, record: &SequenceRecord) -> Result<(), PlatformError> {
    if record.molecule().is_protein() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} expects nucleotide input but '{}' was classified as {}",
                record.identifier().accession(),
                record.molecule()
            ),
        )
        .with_code(format!("bridge.{tool}.input.not_nucleotide")));
    }
    Ok(())
}

fn ensure_protein_record(tool: &str, record: &SequenceRecord) -> Result<(), PlatformError> {
    if record.molecule().is_nucleotide() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} expects protein input but '{}' was classified as {}",
                record.identifier().accession(),
                record.molecule()
            ),
        )
        .with_code(format!("bridge.{tool}.input.not_protein")));
    }
    Ok(())
}

fn project_pattern_hit(
    identifier: &str,
    pattern: &str,
    hit: PatternMatch,
    strand: Option<String>,
    frame: Option<usize>,
    amino_start: Option<usize>,
    nucleotide_start: Option<usize>,
    nucleotide_end: Option<usize>,
) -> BridgePatternHit {
    BridgePatternHit {
        identifier: identifier.to_owned(),
        pattern: pattern.to_owned(),
        strand,
        frame,
        start: hit.start(),
        end: hit.end(),
        amino_start,
        amino_end: amino_start.map(|_| hit.end()),
        nucleotide_start,
        nucleotide_end,
        matched: hit.matched().to_owned(),
    }
}

fn project_composition_rows(
    scope: &str,
    identifier: Option<String>,
    molecule: Option<String>,
    sequence_length: Option<usize>,
    composition: &ResidueComposition,
) -> Vec<BridgeCompositionRow> {
    composition
        .counts()
        .iter()
        .map(|(residue, count)| BridgeCompositionRow {
            scope: scope.to_owned(),
            identifier: identifier.clone(),
            molecule: molecule.clone(),
            sequence_length,
            counted_symbols: composition.counted_symbols(),
            ignored_gap_symbols: composition.ignored_gap_symbols(),
            residue: residue.to_string(),
            count: *count,
            frequency: composition.frequency_for(*residue),
        })
        .collect()
}

fn project_gc_row(
    scope: &str,
    identifier: Option<String>,
    sequence_length: usize,
    gc: &GcSummary,
) -> BridgeGcRow {
    BridgeGcRow {
        scope: scope.to_owned(),
        identifier,
        sequence_length,
        counted_symbols: gc.counted_symbols,
        canonical_symbols: gc.canonical_symbols,
        gc_symbols: gc.gc_symbols,
        ambiguous_symbols: gc.ambiguous_symbols,
        ignored_gap_symbols: gc.ignored_gap_symbols,
        gc_percent: gc.gc_percent(),
    }
}

fn project_complexity_summary(summary: &SequenceComplexity) -> BridgeComplexitySummary {
    BridgeComplexitySummary {
        identifier: summary.record_id.clone(),
        sequence_length: summary.sequence_length,
        k_min: summary.k_min,
        k_max: summary.k_max,
        complexity: summary.complexity,
    }
}

fn project_complexity_window(window: &WindowComplexity) -> BridgeComplexityWindow {
    BridgeComplexityWindow {
        identifier: window.record_id.clone(),
        window_start: window.start + 1,
        window_end: window.end,
        window_length: window.window_length,
        complexity: window.complexity,
    }
}

fn project_match_summary(summary: &DirectMatchSummary) -> BridgeMatcherSummary {
    BridgeMatcherSummary {
        mode: match summary.mode {
            AlignmentMode::Nucleotide => "nucleotide",
            AlignmentMode::Protein => "protein",
        }
        .to_owned(),
        query_length: summary.query_length,
        target_length: summary.target_length,
        compared_length: summary.compared_length,
        identity_count: summary.identity_count,
        mismatch_count: summary.mismatch_count,
        identity_percent: summary.identity_percent,
        length_difference: summary.length_difference,
    }
}

fn project_distance_matrix(matrix: &DistanceMatrix) -> BridgeDistanceMatrix {
    BridgeDistanceMatrix {
        identifiers: matrix.identifiers.clone(),
        mode: match matrix.mode {
            AlignmentMode::Nucleotide => "nucleotide",
            AlignmentMode::Protein => "protein",
        }
        .to_owned(),
        sequence_length: matrix.sequence_length,
        values: matrix.values.clone(),
    }
}

fn build_alignment(input: BridgeAlignmentInput) -> Result<Alignment, PlatformError> {
    if input.rows.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "alignment input must contain at least one row",
        )
        .with_code("bridge.alignment.empty"));
    }

    let molecule = match input.molecule {
        Some(label) => MoleculeKind::from_str(&label).map_err(|_| {
            PlatformError::new(
                ErrorCategory::Validation,
                format!("unsupported molecule label '{label}'"),
            )
            .with_code("bridge.alignment.molecule.invalid")
        })?,
        None => infer_alignment_molecule(&input.rows),
    };

    let rows = input
        .rows
        .into_iter()
        .map(|row| {
            let identifier = SequenceIdentifier::new(row.identifier).map_err(|error| {
                PlatformError::new(ErrorCategory::Validation, error.to_string())
                    .with_code("bridge.alignment.identifier.invalid")
            })?;
            let mut aligned =
                AlignmentRow::new(identifier, molecule, row.aligned).map_err(|error| {
                    PlatformError::new(ErrorCategory::Validation, error.to_string())
                        .with_code("bridge.alignment.row.invalid")
                })?;
            if let Some(description) = row.description {
                aligned =
                    aligned.with_metadata(SequenceMetadata::new().with_description(description));
            }
            Ok(aligned)
        })
        .collect::<Result<Vec<_>, PlatformError>>()?;

    Alignment::with_identifier(input.identifier, rows).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("bridge.alignment.invalid")
    })
}

fn infer_alignment_molecule(rows: &[BridgeAlignmentRowInput]) -> MoleculeKind {
    let merged: String = rows
        .iter()
        .flat_map(|row| row.aligned.chars().filter(|symbol| *symbol != '-'))
        .collect();
    infer_molecule_kind(&merged)
}

fn consensus_from_alignment(
    input: BridgeAlignmentInput,
    strategy: ConsensusStrategy,
    identifier: &str,
) -> Result<BridgeSequenceRecord, PlatformError> {
    let alignment = build_alignment(input)?;
    let consensus = consensus_sequence(
        &alignment,
        strategy,
        SequenceIdentifier::new(identifier).expect("static identifier is valid"),
    )
    .map_err(map_alignment_analysis_error)?;
    Ok(project_sequence_record(&consensus))
}

fn map_translation_error(error: TranslationError) -> PlatformError {
    let code = match error {
        TranslationError::UnsupportedResidue(_) => "bridge.translation.unsupported_residue",
        TranslationError::InvalidCodon(_) => "bridge.translation.invalid_codon",
        TranslationError::NonCodingLength { .. } => "bridge.translation.non_coding_length",
        TranslationError::InvalidFrameOffset { .. } => "bridge.translation.invalid_frame",
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}

fn map_pattern_error(error: PatternError) -> PlatformError {
    PlatformError::new(ErrorCategory::Validation, error.to_string())
        .with_code("bridge.pattern.invalid")
}

fn map_revseq_error(error: epithema_core::RevseqError) -> PlatformError {
    let code = match error {
        epithema_core::RevseqError::UnsupportedReverseComplement { .. } => {
            "bridge.reverse_sequences.complement.unsupported"
        }
        epithema_core::RevseqError::UnsupportedResidue { .. } => {
            "bridge.reverse_sequences.complement.residue_unsupported"
        }
        epithema_core::RevseqError::UnsupportedAnnotatedRecord => {
            "bridge.reverse_sequences.features.unsupported"
        }
        epithema_core::RevseqError::InvalidSequence(_) => {
            "bridge.reverse_sequences.sequence.invalid"
        }
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}

fn map_composition_error(error: epithema_core::CompositionError) -> PlatformError {
    PlatformError::new(ErrorCategory::Validation, error.to_string())
        .with_code("bridge.composition.invalid")
}

fn map_complexity_error(error: ComplexityError) -> PlatformError {
    PlatformError::new(ErrorCategory::Validation, error.to_string())
        .with_code("bridge.complexity.invalid")
}

fn map_alignment_analysis_error(error: AlignmentAnalysisError) -> PlatformError {
    PlatformError::new(ErrorCategory::Validation, error.to_string())
        .with_code("bridge.alignment_analysis.invalid")
}

fn map_tool_error(error: PlatformError) -> PlatformError {
    error
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::{
        BridgeAlignmentInput, BridgeSequenceInput, backtranslate_representative_sequences,
        charge_profile, compare_translation_sets, complexity_profile, composition_summary,
        consensus_simple, copy_features, count_gc_content, describe_sequence_file,
        describe_sequences, direct_match_sequences, extract_features, extract_sequences,
        fuzz_nucleotide, mask_features, mask_sequences, new_sequence, not_sequence, nth_sequence,
        p_distance_for_sequences, reverse_sequences, sequence_count, skip_sequences,
    };

    fn fixture(name: &str) -> String {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/epithema-tools/tests/fixtures/{name}"))
            .display()
            .to_string()
    }

    #[test]
    fn creates_bridge_sequence_records() {
        let record = new_sequence(BridgeSequenceInput {
            identifier: Some("seq1".to_owned()),
            sequence: "acgt".to_owned(),
            description: Some("example".to_owned()),
            molecule: None,
        })
        .expect("sequence should build");

        assert_eq!(record.identifier, "seq1");
        assert_eq!(record.sequence, "ACGT");
        assert_eq!(record.molecule, "dna");
    }

    #[test]
    fn counts_and_selects_sequences() {
        let inputs = vec![
            BridgeSequenceInput {
                identifier: Some("a".to_owned()),
                sequence: "ACGT".to_owned(),
                description: None,
                molecule: None,
            },
            BridgeSequenceInput {
                identifier: Some("b".to_owned()),
                sequence: "MSTN".to_owned(),
                description: None,
                molecule: Some("protein".to_owned()),
            },
            BridgeSequenceInput {
                identifier: Some("c".to_owned()),
                sequence: "AUGA".to_owned(),
                description: None,
                molecule: None,
            },
        ];

        assert_eq!(sequence_count(&inputs).expect("count should succeed"), 3);
        assert_eq!(
            nth_sequence(&inputs, 2)
                .expect("nth should succeed")
                .identifier,
            "b"
        );
        assert_eq!(
            skip_sequences(&inputs, 1)
                .expect("skip should succeed")
                .iter()
                .map(|record| record.identifier.clone())
                .collect::<Vec<_>>(),
            vec!["b".to_owned(), "c".to_owned()]
        );
        assert_eq!(
            not_sequence(&inputs, 2)
                .expect("not should succeed")
                .iter()
                .map(|record| record.identifier.clone())
                .collect::<Vec<_>>(),
            vec!["a".to_owned(), "c".to_owned()]
        );
    }

    #[test]
    fn reverses_and_masks_sequences_with_bridge_modes() {
        let reversed = reverse_sequences(
            &[BridgeSequenceInput {
                identifier: Some("dna1".to_owned()),
                sequence: "AAGT".to_owned(),
                description: None,
                molecule: Some("dna".to_owned()),
            }],
            Some("auto"),
        )
        .expect("reverse should succeed");
        assert_eq!(reversed[0].sequence, "ACTT");

        let masked = mask_sequences(
            &[BridgeSequenceInput {
                identifier: Some("dna1".to_owned()),
                sequence: "ACGTAC".to_owned(),
                description: None,
                molecule: Some("dna".to_owned()),
            }],
            &[crate::types::BridgeIntervalInput { start: 2, end: 4 }],
            None,
        )
        .expect("mask should succeed");
        assert_eq!(masked[0].sequence, "ANNNAC");
    }

    #[test]
    fn computes_charge_profile_with_plot_contract() {
        let profile = charge_profile(
            BridgeSequenceInput {
                identifier: Some("charge1".to_owned()),
                sequence: "AKRHDDE".to_owned(),
                description: None,
                molecule: Some("protein".to_owned()),
            },
            5,
            1,
        )
        .expect("charge profile should compute");

        assert_eq!(profile.windows.len(), 3);
        assert!((profile.windows[0].mean_charge - 0.3).abs() < 1e-9);

        let json: Value =
            serde_json::from_str(&profile.plot_contract_json).expect("plot contract should parse");
        assert_eq!(json["kind"], "line");
        assert_eq!(json["metadata"]["provenance"]["tool"], "charge");
    }

    #[test]
    fn extracts_backtranslates_and_checks_translation() {
        let extracted = extract_sequences(
            &[BridgeSequenceInput {
                identifier: Some("seq1".to_owned()),
                sequence: "ACGTAC".to_owned(),
                description: None,
                molecule: Some("dna".to_owned()),
            }],
            2,
            4,
        )
        .expect("extract should succeed");
        assert_eq!(extracted[0].sequence, "CGT");

        let backtranslated = backtranslate_representative_sequences(&[BridgeSequenceInput {
            identifier: Some("prot1".to_owned()),
            sequence: "MA".to_owned(),
            description: None,
            molecule: Some("protein".to_owned()),
        }])
        .expect("backtranslation should succeed");
        assert_eq!(backtranslated[0].sequence, "ATGGCT");

        let checked = compare_translation_sets(
            &[BridgeSequenceInput {
                identifier: Some("dna1".to_owned()),
                sequence: "ATGGCTTAA".to_owned(),
                description: None,
                molecule: Some("dna".to_owned()),
            }],
            &[BridgeSequenceInput {
                identifier: Some("prot1".to_owned()),
                sequence: "MA*".to_owned(),
                description: None,
                molecule: Some("protein".to_owned()),
            }],
        )
        .expect("checktrans should succeed");
        assert!(checked[0].matches);
    }

    #[test]
    fn scans_patterns_and_reports_statistics() {
        let hits = fuzz_nucleotide(
            &[BridgeSequenceInput {
                identifier: Some("dna1".to_owned()),
                sequence: "AACCGGTT".to_owned(),
                description: None,
                molecule: Some("dna".to_owned()),
            }],
            "CCG",
        )
        .expect("pattern scan should succeed");
        assert_eq!(hits[0].start, 2);

        let composition = composition_summary(&[
            BridgeSequenceInput {
                identifier: Some("dna1".to_owned()),
                sequence: "ACGT".to_owned(),
                description: None,
                molecule: Some("dna".to_owned()),
            },
            BridgeSequenceInput {
                identifier: Some("dna2".to_owned()),
                sequence: "AAGT".to_owned(),
                description: None,
                molecule: Some("dna".to_owned()),
            },
        ])
        .expect("composition should succeed");
        assert!(composition.iter().any(|row| row.scope == "aggregate"));

        let gc = count_gc_content(&[BridgeSequenceInput {
            identifier: Some("dna1".to_owned()),
            sequence: "GGCC".to_owned(),
            description: None,
            molecule: Some("dna".to_owned()),
        }])
        .expect("gc should succeed");
        assert_eq!(gc[0].gc_percent, 100.0);
    }

    #[test]
    fn reports_descseq_rows_for_memory_and_files() {
        let in_memory = describe_sequences(&[BridgeSequenceInput {
            identifier: Some("seq1".to_owned()),
            sequence: "ACGT".to_owned(),
            description: Some("example".to_owned()),
            molecule: Some("dna".to_owned()),
        }])
        .expect("in-memory descseq should succeed");
        assert_eq!(in_memory[0].identifier, "seq1");

        let file_rows =
            describe_sequence_file(&fixture("annotated_feature.gbk")).expect("file descseq");
        assert_eq!(file_rows[0].identifier, "FEAT1");
        assert_eq!(file_rows[0].feature_count, 2);
    }

    #[test]
    fn bridges_feature_tools_from_local_files() {
        let annotated = fixture("annotated_feature.gbk");
        let target = fixture("featcopy_target.fasta");

        let extracted =
            extract_features(&annotated, Some("gene"), None, None, None).expect("extractfeat");
        assert!(!extracted.is_empty());
        assert!(extracted[0].feature_count >= 1);

        let masked =
            mask_features(&annotated, Some("gene"), None, None, None, None).expect("maskfeat");
        assert_eq!(masked[0].feature_count, 2);

        let copied =
            copy_features(&annotated, &target, Some("gene"), None, None, None).expect("featcopy");
        assert!(!copied.is_empty());
        assert!(copied[0].feature_count >= 1);
    }

    #[test]
    fn computes_complexity_and_alignment_summaries() {
        let complexity = complexity_profile(
            BridgeSequenceInput {
                identifier: Some("dna1".to_owned()),
                sequence: "ACGTACGT".to_owned(),
                description: None,
                molecule: Some("dna".to_owned()),
            },
            1,
            2,
            Some(4),
            Some(2),
        )
        .expect("complexity should succeed");
        assert_eq!(complexity.summary.k_max, 2);
        assert!(!complexity.windows.is_empty());

        let matcher = direct_match_sequences(
            BridgeSequenceInput {
                identifier: Some("q".to_owned()),
                sequence: "ACGT".to_owned(),
                description: None,
                molecule: Some("dna".to_owned()),
            },
            BridgeSequenceInput {
                identifier: Some("t".to_owned()),
                sequence: "ACGA".to_owned(),
                description: None,
                molecule: Some("dna".to_owned()),
            },
        )
        .expect("matcher should succeed");
        assert_eq!(matcher.identity_count, 3);

        let matrix = p_distance_for_sequences(&[
            BridgeSequenceInput {
                identifier: Some("a".to_owned()),
                sequence: "ACGT".to_owned(),
                description: None,
                molecule: Some("dna".to_owned()),
            },
            BridgeSequenceInput {
                identifier: Some("b".to_owned()),
                sequence: "ACGA".to_owned(),
                description: None,
                molecule: Some("dna".to_owned()),
            },
        ])
        .expect("distance matrix should succeed");
        assert_eq!(matrix.values[0][1], 0.25);

        let consensus = consensus_simple(BridgeAlignmentInput {
            identifier: Some("aln1".to_owned()),
            molecule: Some("dna".to_owned()),
            rows: vec![
                crate::types::BridgeAlignmentRowInput {
                    identifier: "a".to_owned(),
                    aligned: "AC-GT".to_owned(),
                    description: None,
                },
                crate::types::BridgeAlignmentRowInput {
                    identifier: "b".to_owned(),
                    aligned: "ACTGT".to_owned(),
                    description: None,
                },
                crate::types::BridgeAlignmentRowInput {
                    identifier: "c".to_owned(),
                    aligned: "ACCGT".to_owned(),
                    description: None,
                },
            ],
        })
        .expect("consensus should succeed");
        assert_eq!(consensus.sequence, "ACNGT");
    }
}
