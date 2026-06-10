use std::fs;
use std::path::Path;

use epithema_core::{
    CodonUsageError, CodonUsageProfile, amino_acid_for_sense_codon, derive_cai_weights,
    sense_codons, summarize_coding_sequence,
};
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

pub const PROFILE_HEADER: &str = "codon\tamino_acid\tcount\tfrequency\tweight";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodingProfileRecord {
    pub record_id: String,
    pub profile: CodonUsageProfile,
    pub sense_codon_count: usize,
    pub terminal_stop: Option<String>,
}

pub fn derive_coding_profile_records(
    input: &SequenceInput,
) -> Result<Vec<CodingProfileRecord>, ToolExecutionError> {
    load_sequence_records(input)?
        .into_iter()
        .map(|record| {
            if record.molecule().is_protein() {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    format!(
                        "coding-bias tools expect nucleotide coding input but '{}' was classified as {}",
                        record.identifier().accession(),
                        record.molecule()
                    ),
                )
                .with_code("tools.codon.input.not_nucleotide"));
            }
            let summary = summarize_coding_sequence(record.residues())
                .map_err(|error| map_codon_usage_error("codon", error))?;
            Ok(CodingProfileRecord {
                record_id: record.identifier().accession().to_owned(),
                profile: summary.profile,
                sense_codon_count: summary.sense_codon_count,
                terminal_stop: summary.terminal_stop,
            })
        })
        .collect()
}

pub fn aggregate_profile(records: &[CodingProfileRecord]) -> CodonUsageProfile {
    let mut aggregate = CodonUsageProfile::new();
    for record in records {
        aggregate.merge(&record.profile);
    }
    aggregate
}

pub fn load_profile_source(path: &Path) -> Result<CodonUsageProfile, ToolExecutionError> {
    if looks_like_profile_file(path)? {
        parse_profile_tsv(&fs::read_to_string(path).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Validation,
                "failed to read codon profile input",
            )
            .with_code("tools.codon.profile.read_failed")
            .with_detail(format!("{}: {error}", path.display()))
        })?)
    } else {
        let records = derive_coding_profile_records(&SequenceInput::new(path.to_path_buf()))?;
        Ok(aggregate_profile(&records))
    }
}

pub fn render_profile_rows(profile: &CodonUsageProfile) -> Vec<Vec<String>> {
    let weights = derive_cai_weights(profile);
    sense_codons()
        .into_iter()
        .map(|codon| {
            vec![
                codon.to_owned(),
                amino_acid_for_sense_codon(codon)
                    .expect("sense codon should have amino acid")
                    .to_string(),
                profile.count_for(codon).to_string(),
                format!("{:.6}", profile.frequency_for(codon)),
                format!("{:.6}", weights.get(codon).copied().unwrap_or_default()),
            ]
        })
        .collect()
}

pub fn write_profile_tsv(
    path: &Path,
    profile: &CodonUsageProfile,
) -> Result<(), ToolExecutionError> {
    let mut body = String::new();
    body.push_str(PROFILE_HEADER);
    body.push('\n');
    for row in render_profile_rows(profile) {
        body.push_str(&row.join("\t"));
        body.push('\n');
    }
    fs::write(path, body).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write codon profile output",
        )
        .with_code("tools.codcopy.profile.write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

fn looks_like_profile_file(path: &Path) -> Result<bool, ToolExecutionError> {
    let content = fs::read_to_string(path).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Validation,
            "failed to inspect codon profile input",
        )
        .with_code("tools.codon.profile.inspect_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })?;
    Ok(content
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .is_some_and(|line| line == PROFILE_HEADER))
}

fn parse_profile_tsv(content: &str) -> Result<CodonUsageProfile, ToolExecutionError> {
    let mut lines = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty());
    match lines.next() {
        Some(header) if header == PROFILE_HEADER => {}
        _ => {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "codon profile input must start with the normalized profile header",
            )
            .with_code("tools.codon.profile.header_invalid"));
        }
    }

    let mut profile = CodonUsageProfile::new();
    for line in lines {
        let columns: Vec<_> = line.split('\t').collect();
        if columns.len() != 5 {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "codon profile rows must have five tab-separated columns",
            )
            .with_code("tools.codon.profile.row_invalid")
            .with_detail(line.to_owned()));
        }
        let codon = columns[0].trim().to_ascii_uppercase();
        if amino_acid_for_sense_codon(&codon).is_none() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "codon profile contains an unsupported codon row",
            )
            .with_code("tools.codon.profile.codon_invalid")
            .with_detail(codon));
        }
        let count = columns[2].trim().parse::<usize>().map_err(|_| {
            PlatformError::new(
                ErrorCategory::Validation,
                "codon profile count column must be an integer",
            )
            .with_code("tools.codon.profile.count_invalid")
            .with_detail(columns[2].trim().to_owned())
        })?;
        for _ in 0..count {
            profile.add_codon(&codon);
        }
    }

    Ok(profile)
}

pub fn map_codon_usage_error(tool: &str, error: CodonUsageError) -> ToolExecutionError {
    let code = match error {
        CodonUsageError::NonCodingLength { .. } => format!("tools.{tool}.coding.non_coding_length"),
        CodonUsageError::InvalidCodon(_) => format!("tools.{tool}.codon.invalid"),
        CodonUsageError::AmbiguousCodon(_) => format!("tools.{tool}.codon.ambiguous"),
        CodonUsageError::InternalStopCodon(_) => format!("tools.{tool}.codon.internal_stop"),
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}
