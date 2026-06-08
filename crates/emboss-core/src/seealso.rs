//! Bounded related-program discovery support for `seealso`.

use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

/// Errors for bounded `seealso` profile computation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SeealsoError {
    /// The provided query tool name did not contain any stable text.
    EmptyQueryToolName,
    /// A catalog entry contained no stable tool name.
    EmptyToolName {
        /// Zero-based entry offset.
        entry_index: usize,
    },
    /// A catalog entry contained no governed family name.
    EmptyFamily {
        /// Zero-based entry offset.
        entry_index: usize,
    },
    /// A catalog entry contained no governed short description.
    EmptyShortDescription {
        /// Zero-based entry offset.
        entry_index: usize,
    },
    /// The catalog contained duplicate normalized tool names.
    DuplicateToolName {
        /// First zero-based entry offset.
        first_index: usize,
        /// Duplicate zero-based entry offset.
        duplicate_index: usize,
        /// Stable normalized tool name that collided.
        tool_name: String,
    },
    /// The requested tool name does not exist in the bounded catalog.
    QueryToolNotFound {
        /// Trimmed original query tool name.
        tool_name: String,
    },
}

impl Display for SeealsoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyQueryToolName => {
                write!(f, "seealso requires one non-empty tool name query")
            }
            Self::EmptyToolName { entry_index } => write!(
                f,
                "catalog entry {entry_index} must include a non-empty tool name"
            ),
            Self::EmptyFamily { entry_index } => write!(
                f,
                "catalog entry {entry_index} must include a non-empty family name"
            ),
            Self::EmptyShortDescription { entry_index } => write!(
                f,
                "catalog entry {entry_index} must include a non-empty short description"
            ),
            Self::DuplicateToolName {
                first_index,
                duplicate_index,
                tool_name,
            } => write!(
                f,
                "catalog entries {first_index} and {duplicate_index} share duplicate tool name `{tool_name}`"
            ),
            Self::QueryToolNotFound { tool_name } => {
                write!(f, "seealso could not find governed tool `{tool_name}`")
            }
        }
    }
}

impl std::error::Error for SeealsoError {}

/// One bounded metadata row that can participate in `seealso` discovery.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeealsoCatalogEntry {
    /// Stable governed tool identity.
    pub tool_name: String,
    /// Stable governed tool family.
    pub family: String,
    /// Governed short description used for bounded relationship resolution.
    pub short_description: String,
}

impl SeealsoCatalogEntry {
    /// Creates one bounded catalog entry.
    #[must_use]
    pub fn new(
        tool_name: impl Into<String>,
        family: impl Into<String>,
        short_description: impl Into<String>,
    ) -> Self {
        Self {
            tool_name: tool_name.into(),
            family: family.into(),
            short_description: short_description.into(),
        }
    }
}

/// One bounded metadata field that can support a `seealso` relationship.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SeealsoRelationshipField {
    /// Shared governed family.
    Family,
    /// Shared significant summary terms.
    ShortDescription,
}

impl SeealsoRelationshipField {
    /// Stable label for rendered table rows.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Family => "family",
            Self::ShortDescription => "short_description",
        }
    }
}

impl Display for SeealsoRelationshipField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One bounded related-program row for `seealso`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeealsoRelatedTool {
    /// Stable governed related-tool identity.
    pub tool_name: String,
    /// Governed tool family for the related tool.
    pub family: String,
    /// Governed short description for the related tool.
    pub short_description: String,
    /// Stable ordered normalized relationship terms.
    pub relationship_terms: Vec<String>,
    /// Stable ordered metadata fields that justify the relationship.
    pub relationship_fields: Vec<SeealsoRelationshipField>,
}

/// Full bounded `seealso` profile for one tool query.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeealsoProfile {
    /// Original free-text query tool name after trimming.
    pub query_tool_name: String,
    /// Resolved governed tool identity.
    pub resolved_tool_name: String,
    /// Resolved governed family.
    pub resolved_family: String,
    /// Resolved governed short description.
    pub resolved_short_description: String,
    /// Count of catalog entries searched.
    pub searched_entry_count: usize,
    /// Stable ordered related-program rows.
    pub related_tools: Vec<SeealsoRelatedTool>,
}

/// Computes a deterministic bounded `seealso` discovery profile.
pub fn seealso_profile(
    query_tool_name: impl AsRef<str>,
    entries: &[SeealsoCatalogEntry],
) -> Result<SeealsoProfile, SeealsoError> {
    let query_tool_name = query_tool_name.as_ref().trim().to_owned();
    if query_tool_name.is_empty() {
        return Err(SeealsoError::EmptyQueryToolName);
    }

    let normalized_query_tool_name = normalize_text(&query_tool_name);
    let mut seen_tool_names = HashMap::new();
    let mut resolved_index = None;

    for (entry_index, entry) in entries.iter().enumerate() {
        let tool_name = entry.tool_name.trim();
        if tool_name.is_empty() {
            return Err(SeealsoError::EmptyToolName { entry_index });
        }
        if entry.family.trim().is_empty() {
            return Err(SeealsoError::EmptyFamily { entry_index });
        }
        if entry.short_description.trim().is_empty() {
            return Err(SeealsoError::EmptyShortDescription { entry_index });
        }

        let normalized_tool_name = normalize_text(tool_name);
        if let Some(first_index) = seen_tool_names.insert(normalized_tool_name.clone(), entry_index) {
            return Err(SeealsoError::DuplicateToolName {
                first_index,
                duplicate_index: entry_index,
                tool_name: normalized_tool_name,
            });
        }

        if normalized_tool_name == normalized_query_tool_name {
            resolved_index = Some(entry_index);
        }
    }

    let resolved_index = resolved_index.ok_or_else(|| SeealsoError::QueryToolNotFound {
        tool_name: query_tool_name.clone(),
    })?;

    let resolved = &entries[resolved_index];
    let resolved_tool_name = resolved.tool_name.trim().to_owned();
    let resolved_family = resolved.family.trim().to_owned();
    let resolved_short_description = resolved.short_description.trim().to_owned();
    let normalized_resolved_family = normalize_text(&resolved_family);
    let resolved_terms = tokenize_relationship_terms(&resolved_short_description);

    let mut related_tools = Vec::new();
    for (entry_index, entry) in entries.iter().enumerate() {
        if entry_index == resolved_index {
            continue;
        }

        let tool_name = entry.tool_name.trim();
        let family = entry.family.trim();
        let short_description = entry.short_description.trim();
        let normalized_family = normalize_text(family);

        let mut relationship_fields = Vec::new();
        let mut relationship_terms = Vec::new();

        if normalized_family == normalized_resolved_family {
            relationship_fields.push(SeealsoRelationshipField::Family);
            relationship_terms.push(normalized_resolved_family.clone());
        }

        let candidate_terms = tokenize_relationship_terms(short_description);
        let overlapping_terms: Vec<String> = resolved_terms
            .iter()
            .filter(|term| candidate_terms.contains(term))
            .cloned()
            .collect();
        if overlapping_terms.len() >= 2 {
            relationship_fields.push(SeealsoRelationshipField::ShortDescription);
            relationship_terms.extend(overlapping_terms);
        }

        deduplicate_preserving_order(&mut relationship_terms);
        if relationship_fields.is_empty() {
            continue;
        }

        related_tools.push(SeealsoRelatedTool {
            tool_name: tool_name.to_owned(),
            family: family.to_owned(),
            short_description: short_description.to_owned(),
            relationship_terms,
            relationship_fields,
        });
    }

    Ok(SeealsoProfile {
        query_tool_name,
        resolved_tool_name,
        resolved_family,
        resolved_short_description,
        searched_entry_count: entries.len(),
        related_tools,
    })
}

fn tokenize_relationship_terms(text: &str) -> Vec<String> {
    let mut terms = Vec::new();
    for term in text
        .split(|ch: char| !ch.is_alphanumeric())
        .map(normalize_text)
        .filter(|term| term.len() >= 5)
    {
        if !terms.contains(&term) {
            terms.push(term);
        }
    }
    terms
}

fn normalize_text(text: impl AsRef<str>) -> String {
    text.as_ref().chars().flat_map(char::to_lowercase).collect()
}

fn deduplicate_preserving_order(values: &mut Vec<String>) {
    let mut seen = HashSet::new();
    values.retain(|value| seen.insert(value.clone()));
}

#[cfg(test)]
mod tests {
    use super::{
        SeealsoCatalogEntry, SeealsoError, SeealsoRelationshipField, seealso_profile,
    };

    #[test]
    fn rejects_blank_or_unknown_query_tool_names() {
        let error = seealso_profile("   ", &[]).expect_err("blank query should fail");
        assert_eq!(error, SeealsoError::EmptyQueryToolName);

        let error = seealso_profile(
            "needle",
            &[SeealsoCatalogEntry::new(
                "water",
                "pairwise_alignment",
                "perform deterministic local pairwise alignment",
            )],
        )
        .expect_err("unknown query should fail");
        assert_eq!(
            error,
            SeealsoError::QueryToolNotFound {
                tool_name: "needle".to_owned(),
            }
        );
    }

    #[test]
    fn rejects_blank_catalog_rows_and_duplicate_tool_names() {
        let error = seealso_profile(
            "needle",
            &[SeealsoCatalogEntry::new("", "pairwise_alignment", "alignment tool")],
        )
        .expect_err("blank tool name should fail");
        assert_eq!(error, SeealsoError::EmptyToolName { entry_index: 0 });

        let error = seealso_profile(
            "needle",
            &[SeealsoCatalogEntry::new("needle", "", "alignment tool")],
        )
        .expect_err("blank family should fail");
        assert_eq!(error, SeealsoError::EmptyFamily { entry_index: 0 });

        let error = seealso_profile(
            "needle",
            &[SeealsoCatalogEntry::new("needle", "pairwise_alignment", "   ")],
        )
        .expect_err("blank description should fail");
        assert_eq!(
            error,
            SeealsoError::EmptyShortDescription { entry_index: 0 }
        );

        let error = seealso_profile(
            "needle",
            &[
                SeealsoCatalogEntry::new(
                    "needle",
                    "pairwise_alignment",
                    "perform deterministic global pairwise alignment",
                ),
                SeealsoCatalogEntry::new(
                    "Needle",
                    "pairwise_alignment",
                    "perform deterministic second alignment",
                ),
            ],
        )
        .expect_err("duplicate names should fail");
        assert_eq!(
            error,
            SeealsoError::DuplicateToolName {
                first_index: 0,
                duplicate_index: 1,
                tool_name: "needle".to_owned(),
            }
        );
    }

    #[test]
    fn reports_related_tools_in_stable_catalog_order() {
        let profile = seealso_profile(
            "needle",
            &[
                SeealsoCatalogEntry::new(
                    "needle",
                    "pairwise_alignment",
                    "perform deterministic global pairwise alignment between exactly one query and one target",
                ),
                SeealsoCatalogEntry::new(
                    "aligncopypair",
                    "alignment_tools",
                    "copy a single pairwise alignment unchanged and reject non-pairwise inputs",
                ),
                SeealsoCatalogEntry::new(
                    "water",
                    "pairwise_alignment",
                    "perform deterministic local pairwise alignment between exactly one query and one target",
                ),
                SeealsoCatalogEntry::new(
                    "wossname",
                    "command_tools",
                    "find programs by keywords in their short description",
                ),
                SeealsoCatalogEntry::new(
                    "needleall",
                    "pairwise_alignment",
                    "perform deterministic many-vs-many global pairwise alignment and report comparison summaries",
                ),
            ],
        )
        .expect("seealso should succeed");

        assert_eq!(profile.query_tool_name, "needle");
        assert_eq!(profile.resolved_tool_name, "needle");
        assert_eq!(profile.resolved_family, "pairwise_alignment");
        assert_eq!(profile.searched_entry_count, 5);
        assert_eq!(profile.related_tools.len(), 3);
        assert_eq!(profile.related_tools[0].tool_name, "aligncopypair");
        assert_eq!(profile.related_tools[1].tool_name, "water");
        assert_eq!(profile.related_tools[2].tool_name, "needleall");
        assert_eq!(
            profile.related_tools[0].relationship_fields,
            vec![SeealsoRelationshipField::ShortDescription]
        );
        assert_eq!(
            profile.related_tools[1].relationship_fields,
            vec![
                SeealsoRelationshipField::Family,
                SeealsoRelationshipField::ShortDescription,
            ]
        );
        assert_eq!(
            profile.related_tools[0].relationship_terms,
            vec!["pairwise".to_owned(), "alignment".to_owned()]
        );
        assert_eq!(
            profile.related_tools[1].relationship_terms,
            vec![
                "pairwise_alignment".to_owned(),
                "perform".to_owned(),
                "deterministic".to_owned(),
                "pairwise".to_owned(),
                "alignment".to_owned(),
                "between".to_owned(),
                "exactly".to_owned(),
                "query".to_owned(),
                "target".to_owned(),
            ]
        );
    }
}
