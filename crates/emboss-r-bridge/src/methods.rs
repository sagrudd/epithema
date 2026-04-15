//! Typed analytical bridge methods for the first idiomatic R surface.

use std::str::FromStr;

use emboss_core::{
    Alphabet, MoleculeKind, ProteinChargeError, SequenceIdentifier, SequenceMetadata,
    SequenceRecord, protein_charge_profile,
};
use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_plot_contract::{
    AxisScaleHint, DataVector, GeometryHint, PlotAxis, PlotKind, PlotMetadata, PlotProvenance,
    PlotSeries, PlotSpec, SeriesStyle,
};

use crate::conversion::project_plot_contract;
use crate::types::{
    BridgeChargeProfile, BridgeChargeWindow, BridgeSequenceInput, BridgeSequenceRecord,
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
    }
}

fn build_charge_plot(
    profile: &emboss_core::ProteinChargeProfile,
) -> Result<emboss_plot_contract::PlotPayload, PlatformError> {
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

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::{
        BridgeSequenceInput, charge_profile, new_sequence, not_sequence, nth_sequence,
        sequence_count, skip_sequences,
    };

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
}
