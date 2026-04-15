//! `codcopy` implementation.

use std::path::PathBuf;

use emboss_core::CodonUsageProfile;

use crate::codon_tools::shared::{load_profile_source, write_profile_tsv};
use crate::sequence_stream::ToolExecutionError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodcopyParams {
    pub source: PathBuf,
    pub profile_out: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodcopyOutcome {
    pub source: PathBuf,
    pub profile: CodonUsageProfile,
    pub profile_out: Option<PathBuf>,
}

#[must_use]
pub fn codcopy_help() -> &'static str {
    "Usage: emboss-rs codcopy <coding-or-profile-input> [--profile-out <path>]\n\nNormalize a coding-sequence input or an existing normalized codon-profile input into a reusable codon-usage profile. When --profile-out is supplied, the normalized profile is written as tab-separated text suitable for later cai or codcmp runs."
}

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
