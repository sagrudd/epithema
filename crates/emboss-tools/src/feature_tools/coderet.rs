//! `coderet` implementation.

use emboss_core::{FeatureSelector, MoleculeKind, SequenceIdentifier, SequenceRecord, extract_selected_regions, translate_dna_strict};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::feature_tools::shared::map_feature_error;
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `coderet`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoderetParams {
    /// Local annotated sequence input path.
    pub input: SequenceInput,
    /// Shared selector.
    pub selector: FeatureSelector,
    /// Translate extracted records to protein after sequence extraction.
    pub translate: bool,
}

/// Structured `coderet` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoderetOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Shared selector.
    pub selector: FeatureSelector,
    /// Translation flag.
    pub translate: bool,
    /// Extracted record count.
    pub extracted_record_count: usize,
    /// Derived records in stable order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `coderet` help text.
#[must_use]
pub fn coderet_help() -> &'static str {
    "Usage: emboss-rs coderet <input> [--translate] [--kind <kind>] [--name <name>] [--qualifier <key[=value]>] [--strand <forward|reverse|unknown>]\n\nExtract selected simple feature-defined coding regions from an annotated EMBL or GenBank input. If no selector flags are supplied, v1 defaults to `--kind cds`. With `--translate`, the extracted nucleotide regions are translated strictly with the standard genetic code and emitted as protein FASTA records."
}

/// Executes `coderet`.
pub fn run_coderet(params: CoderetParams) -> Result<CoderetOutcome, ToolExecutionError> {
    let mut records = Vec::new();

    for record in load_sequence_records(&params.input)? {
        match extract_selected_regions(&record, &params.selector) {
            Ok(extracted) => {
                for extracted_record in extracted {
                    if params.translate {
                        records.push(translate_extracted_record(extracted_record.record)?);
                    } else {
                        records.push(extracted_record.record);
                    }
                }
            }
            Err(emboss_core::FeatureOperationError::NoMatchingFeatures) => {}
            Err(error) => return Err(map_feature_error("coderet", error)),
        }
    }

    if records.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "coderet did not find any features matching the requested selector",
        )
        .with_code("tools.coderet.feature.no_match"));
    }

    Ok(CoderetOutcome {
        input: params.input,
        selector: params.selector,
        translate: params.translate,
        extracted_record_count: records.len(),
        records,
    })
}

fn translate_extracted_record(record: SequenceRecord) -> Result<SequenceRecord, ToolExecutionError> {
    let protein = translate_dna_strict(&dna_equivalent_residues(record.residues())).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.coderet.translation.invalid_coding_sequence")
    })?;
    let identifier = SequenceIdentifier::new(format!("{}.pep", record.identifier().accession()))
        .map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.coderet.identifier.invalid")
        })?;
    let identifier = match record.identifier().display_name() {
        Some(display_name) => identifier.with_display_name(format!("{display_name}.pep")),
        None => identifier,
    };
    let metadata = record.metadata().clone();
    SequenceRecord::new(identifier, MoleculeKind::Protein, protein)
        .map(|translated| translated.with_metadata(metadata))
        .map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.coderet.translation.output_invalid")
        })
}

fn dna_equivalent_residues(residues: &str) -> String {
    residues.chars().map(|symbol| if symbol == 'U' { 'T' } else { symbol }).collect()
}

#[cfg(test)]
mod tests {
    use super::{CoderetParams, run_coderet};
    use crate::sequence_stream::SequenceInput;

    #[test]
    fn defaults_to_cds_selection_and_can_translate() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/annotated_feature.gbk");
        let outcome = run_coderet(CoderetParams {
            input: SequenceInput::new(&fixture),
            selector: emboss_core::FeatureSelector::Kind(emboss_core::FeatureKind::CodingSequence),
            translate: true,
        })
        .expect("coderet should execute");

        assert_eq!(outcome.records.len(), 1);
        assert_eq!(outcome.records[0].molecule(), emboss_core::MoleculeKind::Protein);
        assert_eq!(outcome.records[0].residues(), "Y");
    }
}
