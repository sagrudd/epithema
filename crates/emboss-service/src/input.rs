//! Typed user-input references and resolution seams.
//!
//! Tools should accept these service-level input references rather than raw
//! strings so that local paths, provider-backed accessions, inline literals,
//! and unresolved tokens all flow through one governed classification path.

use std::path::{Path, PathBuf};

use emboss_core::MoleculeKind;
use emboss_diagnostics::{ArtifactProvenance, Diagnostic, Severity};
use emboss_providers::{AcquisitionRequest, InputReference, ProviderId, ResolutionIntent};

use crate::ServiceError;

/// High-level classification of a tool-facing input token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToolInputKind {
    /// Existing or explicit local filesystem path.
    LocalPath,
    /// Provider-qualified accession or locator.
    ProviderQualified,
    /// Bare accession-like identifier.
    Accession,
    /// Inline literal sequence content.
    InlineSequence,
    /// Input could not be classified confidently.
    Unresolved,
}

/// Tool-facing typed input reference preserving the raw user token.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolInputReference {
    raw: String,
    kind: ToolInputKind,
    local_path: Option<PathBuf>,
    accession: Option<String>,
    provider: Option<ProviderId>,
    inline_sequence: Option<String>,
    molecule_hint: Option<MoleculeKind>,
}

impl ToolInputReference {
    /// Classifies a raw tool input conservatively.
    pub fn classify(raw: impl Into<String>) -> Result<Self, ServiceError> {
        Self::classify_in(raw, None)
    }

    /// Classifies a raw tool input relative to a working directory.
    pub fn classify_in(raw: impl Into<String>, cwd: Option<&Path>) -> Result<Self, ServiceError> {
        let raw = raw.into();
        let token = raw.trim().to_owned();
        if token.is_empty() {
            return Err(emboss_diagnostics::PlatformError::new(
                emboss_diagnostics::ErrorCategory::Validation,
                "tool input must not be empty",
            )
            .with_code("service.input.empty"));
        }

        if let Some((provider, locator)) = parse_provider_qualified(&token)? {
            return Ok(Self {
                raw,
                kind: ToolInputKind::ProviderQualified,
                local_path: None,
                accession: Some(locator.clone()),
                provider: Some(provider),
                inline_sequence: None,
                molecule_hint: None,
            });
        }

        let resolved_path = resolve_candidate_path(&token, cwd);
        if looks_like_explicit_path(&token) || resolved_path.is_some() {
            return Ok(Self {
                raw,
                kind: ToolInputKind::LocalPath,
                local_path: Some(expand_tilde(&token)),
                accession: None,
                provider: None,
                inline_sequence: None,
                molecule_hint: None,
            });
        }

        if let Some((sequence, molecule_hint)) = classify_inline_sequence(&token) {
            return Ok(Self {
                raw,
                kind: ToolInputKind::InlineSequence,
                local_path: None,
                accession: None,
                provider: None,
                inline_sequence: Some(sequence),
                molecule_hint: Some(molecule_hint),
            });
        }

        if looks_like_accession(&token) {
            return Ok(Self {
                raw,
                kind: ToolInputKind::Accession,
                local_path: None,
                accession: Some(token.to_owned()),
                provider: None,
                inline_sequence: None,
                molecule_hint: None,
            });
        }

        Ok(Self {
            raw,
            kind: ToolInputKind::Unresolved,
            local_path: None,
            accession: None,
            provider: None,
            inline_sequence: None,
            molecule_hint: None,
        })
    }

    /// Returns the raw user-supplied input token.
    #[must_use]
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Returns the classified input kind.
    #[must_use]
    pub fn kind(&self) -> ToolInputKind {
        self.kind
    }

    /// Returns the local-path candidate when the input is path-like.
    #[must_use]
    pub fn local_path(&self) -> Option<&Path> {
        self.local_path.as_deref()
    }

    /// Returns the accession or locator when present.
    #[must_use]
    pub fn accession(&self) -> Option<&str> {
        self.accession.as_deref()
    }

    /// Returns the hinted provider when present.
    #[must_use]
    pub fn provider(&self) -> Option<&ProviderId> {
        self.provider.as_ref()
    }

    /// Returns the inline sequence content when present.
    #[must_use]
    pub fn inline_sequence(&self) -> Option<&str> {
        self.inline_sequence.as_deref()
    }

    /// Returns an optional molecule hint inferred from inline sequence content.
    #[must_use]
    pub fn molecule_hint(&self) -> Option<MoleculeKind> {
        self.molecule_hint
    }
}

/// Typed resolution outcome for a tool-facing input reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ToolInputResolution {
    /// Input resolved to an existing local filesystem artefact.
    LocalFile {
        /// Original classified input reference.
        reference: ToolInputReference,
        /// Canonical local path.
        canonical_path: PathBuf,
        /// Structured provenance for the resolved artefact.
        provenance: ArtifactProvenance,
        /// Resolution diagnostics.
        diagnostics: Vec<Diagnostic>,
    },
    /// Input should be routed through provider-backed acquisition.
    ProviderRouted {
        /// Original classified input reference.
        reference: ToolInputReference,
        /// Lower-level provider acquisition request.
        request: AcquisitionRequest,
        /// Structured provenance describing the routed artefact.
        provenance: ArtifactProvenance,
        /// Resolution diagnostics.
        diagnostics: Vec<Diagnostic>,
    },
    /// Inline sequence content resolved without external acquisition.
    InlineSequence {
        /// Original classified input reference.
        reference: ToolInputReference,
        /// Normalized literal content.
        sequence: String,
        /// Inferred molecule hint when available.
        molecule_hint: Option<MoleculeKind>,
        /// Structured provenance for the inline artefact.
        provenance: ArtifactProvenance,
        /// Resolution diagnostics.
        diagnostics: Vec<Diagnostic>,
    },
    /// Input could not be resolved confidently.
    Unresolved {
        /// Original classified input reference.
        reference: ToolInputReference,
        /// Resolution diagnostics.
        diagnostics: Vec<Diagnostic>,
    },
}

/// Service-level resolver for tool-facing inputs.
#[derive(Clone, Debug, Default)]
pub struct ToolInputResolver {
    cwd: Option<PathBuf>,
}

impl ToolInputResolver {
    /// Creates a resolver using the process working directory.
    #[must_use]
    pub fn new() -> Self {
        Self { cwd: None }
    }

    /// Creates a resolver relative to a specific working directory.
    #[must_use]
    pub fn with_cwd(cwd: impl Into<PathBuf>) -> Self {
        Self {
            cwd: Some(cwd.into()),
        }
    }

    /// Classifies a raw token into a typed input reference.
    pub fn classify(&self, raw: impl Into<String>) -> Result<ToolInputReference, ServiceError> {
        ToolInputReference::classify_in(raw, self.cwd.as_deref())
    }

    /// Resolves a classified input reference into a typed service outcome.
    pub fn resolve(
        &self,
        reference: ToolInputReference,
        intent: ResolutionIntent,
    ) -> Result<ToolInputResolution, ServiceError> {
        match reference.kind() {
            ToolInputKind::LocalPath => self.resolve_local(reference),
            ToolInputKind::ProviderQualified | ToolInputKind::Accession => {
                Ok(self.resolve_provider_routed(reference, intent))
            }
            ToolInputKind::InlineSequence => Ok(self.resolve_inline(reference)),
            ToolInputKind::Unresolved => Ok(ToolInputResolution::Unresolved {
                reference,
                diagnostics: vec![
                    Diagnostic::new(
                        Severity::Warning,
                        "tool input could not be classified confidently",
                    )
                    .with_code("service.input.unresolved")
                    .with_context(
                        "consider using an explicit local path or provider-qualified token",
                    ),
                ],
            }),
        }
    }

    fn resolve_local(
        &self,
        reference: ToolInputReference,
    ) -> Result<ToolInputResolution, ServiceError> {
        let path = reference.local_path().ok_or_else(|| {
            emboss_diagnostics::PlatformError::new(
                emboss_diagnostics::ErrorCategory::Validation,
                "local input reference is missing a path payload",
            )
            .with_code("service.input.local.missing_path")
        })?;

        let canonical = canonicalize_candidate(path, self.cwd.as_deref()).map_err(|_| {
            emboss_diagnostics::PlatformError::new(
                emboss_diagnostics::ErrorCategory::Validation,
                format!("local input path '{}' does not exist", path.display()),
            )
            .with_code("service.input.local.not_found")
            .with_detail(path.display().to_string())
        })?;

        Ok(ToolInputResolution::LocalFile {
            provenance: ArtifactProvenance::local_file(canonical.display().to_string())
                .with_description("resolved local tool input"),
            diagnostics: vec![
                Diagnostic::new(
                    Severity::Notice,
                    "resolved tool input as an existing local file",
                )
                .with_code("service.input.local.resolved"),
            ],
            reference,
            canonical_path: canonical,
        })
    }

    fn resolve_provider_routed(
        &self,
        reference: ToolInputReference,
        intent: ResolutionIntent,
    ) -> ToolInputResolution {
        let (provider, locator) = match reference.kind() {
            ToolInputKind::ProviderQualified => (
                reference.provider().cloned(),
                reference
                    .accession()
                    .expect("provider-qualified reference must have locator")
                    .to_owned(),
            ),
            ToolInputKind::Accession => (
                None,
                reference
                    .accession()
                    .expect("accession reference must have accession")
                    .to_owned(),
            ),
            _ => unreachable!("provider-routed resolution only applies to accession inputs"),
        };

        let provider_input = if provider.is_some() {
            InputReference::provider_asset(provider.clone(), locator.clone())
        } else {
            InputReference::accession(locator.clone())
        };

        let mut request = AcquisitionRequest::new(intent, provider_input);
        if let Some(provider_id) = &provider {
            request = request.with_preferred_provider(provider_id.clone());
        }

        let provenance = if let Some(provider_id) = &provider {
            ArtifactProvenance::provider_asset(locator.clone())
                .with_provider(provider_id.as_str())
                .with_description("provider-qualified tool input")
        } else {
            ArtifactProvenance::accession(locator.clone())
                .with_description("bare accession-like tool input")
        };

        let diagnostics = if provider.is_some() {
            vec![
                Diagnostic::new(
                    Severity::Notice,
                    "tool input is explicitly provider-qualified and must use governed acquisition",
                )
                .with_code("service.input.provider_qualified"),
            ]
        } else {
            vec![Diagnostic::new(
                Severity::Warning,
                "tool input looks accession-like and is routed to provider-backed acquisition without a preferred provider",
            )
            .with_code("service.input.accession_routed")]
        };

        ToolInputResolution::ProviderRouted {
            reference,
            request,
            provenance,
            diagnostics,
        }
    }

    fn resolve_inline(&self, reference: ToolInputReference) -> ToolInputResolution {
        let sequence = reference
            .inline_sequence()
            .expect("inline reference must carry literal content")
            .to_owned();
        ToolInputResolution::InlineSequence {
            molecule_hint: reference.molecule_hint(),
            provenance: ArtifactProvenance::new(
                emboss_diagnostics::ArtifactOriginKind::Unknown,
                reference.raw().to_owned(),
            )
            .with_description("inline tool input sequence"),
            diagnostics: vec![
                Diagnostic::new(
                    Severity::Notice,
                    "tool input was classified as inline sequence content",
                )
                .with_code("service.input.inline_sequence"),
            ],
            reference,
            sequence,
        }
    }
}

fn parse_provider_qualified(token: &str) -> Result<Option<(ProviderId, String)>, ServiceError> {
    let Some((provider_raw, locator_raw)) = token.split_once(':') else {
        return Ok(None);
    };

    if provider_raw.contains('/') || locator_raw.is_empty() || locator_raw.contains('/') {
        return Ok(None);
    }

    match ProviderId::new(provider_raw) {
        Ok(provider) => Ok(Some((provider, locator_raw.trim().to_owned()))),
        Err(error) => Err(error.with_code("service.input.provider.invalid")),
    }
}

fn resolve_candidate_path(token: &str, cwd: Option<&Path>) -> Option<PathBuf> {
    let candidate = expand_tilde(token);
    if candidate.exists() {
        return Some(candidate);
    }

    cwd.map(|root| root.join(token))
        .filter(|path| path.exists())
}

fn canonicalize_candidate(path: &Path, cwd: Option<&Path>) -> std::io::Result<PathBuf> {
    if path.is_absolute() {
        return path.canonicalize();
    }

    if let Ok(canonical) = path.canonicalize() {
        return Ok(canonical);
    }

    if let Some(root) = cwd {
        return root.join(path).canonicalize();
    }

    path.canonicalize()
}

fn looks_like_explicit_path(token: &str) -> bool {
    token.starts_with('/')
        || token.starts_with("./")
        || token.starts_with("../")
        || token.starts_with("~/")
        || token.contains(std::path::MAIN_SEPARATOR)
}

fn expand_tilde(token: &str) -> PathBuf {
    if let Some(rest) = token.strip_prefix("~/")
        && let Ok(home) = std::env::var("HOME")
    {
        return PathBuf::from(home).join(rest);
    }
    PathBuf::from(token)
}

fn classify_inline_sequence(token: &str) -> Option<(String, MoleculeKind)> {
    let normalized: String = token
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .map(|ch| ch.to_ascii_uppercase())
        .collect();

    if normalized.len() < 8 {
        return None;
    }

    if !normalized
        .chars()
        .all(|ch| ch.is_ascii_alphabetic() || matches!(ch, '-' | '*'))
    {
        return None;
    }

    let molecule_hint = if normalized.contains('U') && !normalized.contains('T') {
        MoleculeKind::Rna
    } else if normalized.contains('T') && !normalized.contains('U') {
        MoleculeKind::Dna
    } else {
        MoleculeKind::Unknown
    };

    Some((normalized, molecule_hint))
}

fn looks_like_accession(token: &str) -> bool {
    let trimmed = token.trim();
    if trimmed.len() < 4
        || trimmed.contains(char::is_whitespace)
        || trimmed.contains('/')
        || trimmed.contains(':')
    {
        return false;
    }

    let has_digit = trimmed.chars().any(|ch| ch.is_ascii_digit());
    let has_alpha = trimmed.chars().any(|ch| ch.is_ascii_alphabetic());
    let valid_chars = trimmed
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '-'));

    has_digit && has_alpha && valid_chars
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use emboss_core::MoleculeKind;
    use emboss_diagnostics::ArtifactOriginKind;
    use emboss_providers::ResolutionIntent;

    use super::{ToolInputKind, ToolInputResolution, ToolInputResolver};

    #[test]
    fn resolves_existing_local_file() {
        let temp_dir = std::env::temp_dir().join(unique_name("emboss-rs-service-input"));
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let file = temp_dir.join("example.fa");
        fs::write(&file, b">seq\nACGT\n").expect("fixture file should be written");

        let resolver = ToolInputResolver::with_cwd(&temp_dir);
        let reference = resolver
            .classify("example.fa")
            .expect("path should classify");
        assert_eq!(reference.kind(), ToolInputKind::LocalPath);

        let resolution = resolver
            .resolve(reference, ResolutionIntent::SequenceInput)
            .expect("path should resolve");
        match resolution {
            ToolInputResolution::LocalFile {
                canonical_path,
                provenance,
                ..
            } => {
                assert_eq!(canonical_path, file.canonicalize().expect("canonical path"));
                assert_eq!(provenance.origin_kind, ArtifactOriginKind::LocalFile);
            }
            other => panic!("expected local file resolution, got {other:?}"),
        }

        fs::remove_file(&file).ok();
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn rejects_missing_explicit_local_path() {
        let resolver = ToolInputResolver::new();
        let reference = resolver
            .classify("./missing/example.fa")
            .expect("explicit path should classify");

        let error = resolver
            .resolve(reference, ResolutionIntent::SequenceInput)
            .expect_err("missing explicit path should fail");
        assert_eq!(error.code(), Some("service.input.local.not_found"));
    }

    #[test]
    fn routes_provider_qualified_accession() {
        let resolver = ToolInputResolver::new();
        let reference = resolver.classify("ena:AB000263").expect("should classify");
        assert_eq!(reference.kind(), ToolInputKind::ProviderQualified);

        let resolution = resolver
            .resolve(reference, ResolutionIntent::SequenceInput)
            .expect("provider-qualified input should route");
        match resolution {
            ToolInputResolution::ProviderRouted {
                request,
                provenance,
                ..
            } => {
                assert_eq!(
                    request.preferred_provider.as_ref().map(|id| id.as_str()),
                    Some("ena")
                );
                assert_eq!(provenance.origin_kind, ArtifactOriginKind::ProviderAsset);
            }
            other => panic!("expected provider-routed resolution, got {other:?}"),
        }
    }

    #[test]
    fn keeps_ambiguous_bare_token_unresolved() {
        let resolver = ToolInputResolver::new();
        let reference = resolver
            .classify("needle")
            .expect("bare token should classify");
        assert_eq!(reference.kind(), ToolInputKind::Unresolved);

        let resolution = resolver
            .resolve(reference, ResolutionIntent::SequenceInput)
            .expect("unresolved input should return unresolved outcome");
        assert!(matches!(resolution, ToolInputResolution::Unresolved { .. }));
    }

    #[test]
    fn classifies_inline_sequence_conservatively() {
        let resolver = ToolInputResolver::new();
        let reference = resolver
            .classify("ACGTACGT")
            .expect("inline sequence should classify");
        assert_eq!(reference.kind(), ToolInputKind::InlineSequence);
        assert_eq!(reference.molecule_hint(), Some(MoleculeKind::Dna));

        let resolution = resolver
            .resolve(reference, ResolutionIntent::SequenceInput)
            .expect("inline sequence should resolve");
        match resolution {
            ToolInputResolution::InlineSequence {
                sequence,
                molecule_hint,
                ..
            } => {
                assert_eq!(sequence, "ACGTACGT");
                assert_eq!(molecule_hint, Some(MoleculeKind::Dna));
            }
            other => panic!("expected inline sequence resolution, got {other:?}"),
        }
    }

    #[test]
    fn routes_strong_bare_accession_without_provider_hint() {
        let resolver = ToolInputResolver::new();
        let reference = resolver
            .classify("AB000263")
            .expect("accession should classify");
        assert_eq!(reference.kind(), ToolInputKind::Accession);

        let resolution = resolver
            .resolve(reference, ResolutionIntent::SequenceInput)
            .expect("accession should route");
        match resolution {
            ToolInputResolution::ProviderRouted { request, .. } => {
                assert!(request.preferred_provider.is_none());
            }
            other => panic!("expected provider-routed resolution, got {other:?}"),
        }
    }

    fn unique_name(prefix: &str) -> String {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should advance")
            .as_nanos();
        format!("{prefix}-{nanos}")
    }
}
