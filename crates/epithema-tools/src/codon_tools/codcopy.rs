//! `codcopy` implementation.

use std::path::PathBuf;

use epithema_core::CodonUsageProfile;

use crate::codon_tools::shared::{load_profile_source, write_profile_tsv};
use crate::sequence_stream::ToolExecutionError;

/// Parameters for a `codcopy` execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodcopyParams {
    /// Coding-sequence or normalized codon-profile input to normalize.
    pub source: PathBuf,
    /// Optional destination for the normalized profile TSV.
    pub profile_out: Option<PathBuf>,
}

/// Complete outcome from a `codcopy` execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodcopyOutcome {
    /// Input used to produce the normalized profile.
    pub source: PathBuf,
    /// Normalized codon-usage profile.
    pub profile: CodonUsageProfile,
    /// Optional destination where the profile was written.
    pub profile_out: Option<PathBuf>,
}

/// Returns command-line help text for `codcopy`.
#[must_use]
pub fn codcopy_help() -> &'static str {
    "Usage: epithema codcopy <coding-or-profile-input> [--profile-out <path>]\n\nNormalize a coding-sequence input or an existing normalized codon-profile input into a reusable codon-usage profile. When --profile-out is supplied, the normalized profile is written as tab-separated text suitable for later cai or codcmp runs."
}

/// Normalizes a coding-sequence or profile input into a codon-usage profile.
pub fn run_codcopy(params: CodcopyParams) -> Result<CodcopyOutcome, ToolExecutionError> {
    let profile = load_profile_source(&params.source)?;
    if let Some(path) = &params.profile_out {
        write_profile_tsv(path, &profile)?;
    }
    Ok(CodcopyOutcome {
        source: params.source,
        profile,
        profile_out: params.profile_out,
    })
}
