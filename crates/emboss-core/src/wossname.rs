//! Bounded keyword-discovery support for `wossname`.

use std::fmt::{Display, Formatter};

/// Errors for bounded `wossname` profile computation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WossnameError {
    /// The provided query did not contain any searchable text.
    EmptyQuery,
    /// A catalog entry contained no stable tool name.
    EmptyToolName {
        /// Zero-based entry offset.
        entry_index: usize,
    },
    /// A catalog entry contained no governed short description.
    EmptyShortDescription {
        /// Zero-based entry offset.
        entry_index: usize,
    },
}

impl Display for WossnameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyQuery => write!(f, "wossname requires at least one keyword query term"),
            Self::EmptyToolName { entry_index } => write!(
                f,
                "catalog entry {entry_index} must include a non-empty tool name"
            ),
            Self::EmptyShortDescription { entry_index } => write!(
                f,
                "catalog entry {entry_index} must include a non-empty short description"
            ),
        }
    }
}

impl std::error::Error for WossnameError {}

/// One bounded metadata row that can participate in `wossname` discovery.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WossnameCatalogEntry {
    /// Stable governed tool identity.
    pub tool_name: String,
    /// Governed short description used for local keyword discovery.
    pub short_description: String,
}

impl WossnameCatalogEntry {
    /// Creates one bounded catalog entry.
    #[must_use]
    pub fn new(tool_name: impl Into<String>, short_description: impl Into<String>) -> Self {
        Self {
            tool_name: tool_name.into(),
            short_description: short_description.into(),
        }
    }
}

/// One bounded text field that can satisfy a keyword match.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WossnameMatchField {
    /// The governed tool name field.
    ToolName,
    /// The governed short-description field.
    ShortDescription,
}

impl WossnameMatchField {
    /// Stable label for rendered table rows.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ToolName => "tool_name",
            Self::ShortDescription => "short_description",
        }
    }
}

impl Display for WossnameMatchField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One bounded `wossname` keyword-match row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WossnameMatch {
    /// Stable governed tool identity.
    pub tool_name: String,
    /// Governed short description reported alongside the match.
    pub short_description: String,
    /// Stable ordered normalized query terms that matched this row.
    pub matched_terms: Vec<String>,
    /// Stable ordered text fields that satisfied at least one matched term.
    pub matched_fields: Vec<WossnameMatchField>,
}

/// Full bounded `wossname` profile for one keyword query.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WossnameProfile {
    /// Original free-text query after trimming.
    pub query: String,
    /// Stable ordered normalized query terms.
    pub normalized_terms: Vec<String>,
    /// Count of catalog entries searched.
    pub searched_entry_count: usize,
    /// Stable ordered keyword-match rows.
    pub matches: Vec<WossnameMatch>,
}

/// Computes a deterministic bounded `wossname` discovery profile.
pub fn wossname_profile(
    query: impl AsRef<str>,
    entries: &[WossnameCatalogEntry],
) -> Result<WossnameProfile, WossnameError> {
    let query = query.as_ref().trim().to_owned();
    let normalized_terms = normalize_query_terms(&query);
    if normalized_terms.is_empty() {
        return Err(WossnameError::EmptyQuery);
    }

    let mut matches = Vec::new();
    for (entry_index, entry) in entries.iter().enumerate() {
        let tool_name = entry.tool_name.trim();
        if tool_name.is_empty() {
            return Err(WossnameError::EmptyToolName { entry_index });
        }

        let short_description = entry.short_description.trim();
        if short_description.is_empty() {
            return Err(WossnameError::EmptyShortDescription { entry_index });
        }

        let normalized_tool_name = normalize_text(tool_name);
        let normalized_short_description = normalize_text(short_description);

        let matches_tool_name = normalized_terms
            .iter()
            .all(|term| normalized_tool_name.contains(term));
        let matches_description = normalized_terms
            .iter()
            .all(|term| normalized_short_description.contains(term));

        let matched_terms: Vec<String> = normalized_terms
            .iter()
            .filter(|term| {
                normalized_tool_name.contains(term.as_str())
                    || normalized_short_description.contains(term.as_str())
            })
            .cloned()
            .collect();

        if matched_terms.len() != normalized_terms.len() {
            continue;
        }

        let mut matched_fields = Vec::new();
        if matches_tool_name
            || normalized_terms
                .iter()
                .any(|term| normalized_tool_name.contains(term))
        {
            matched_fields.push(WossnameMatchField::ToolName);
        }
        if matches_description
            || normalized_terms
                .iter()
                .any(|term| normalized_short_description.contains(term))
        {
            matched_fields.push(WossnameMatchField::ShortDescription);
        }

        matches.push(WossnameMatch {
            tool_name: tool_name.to_owned(),
            short_description: short_description.to_owned(),
            matched_terms,
            matched_fields,
        });
    }

    Ok(WossnameProfile {
        query,
        normalized_terms,
        searched_entry_count: entries.len(),
        matches,
    })
}

fn normalize_query_terms(query: &str) -> Vec<String> {
    let mut terms = Vec::new();
    for term in query
        .split(|ch: char| !ch.is_alphanumeric())
        .map(normalize_text)
        .filter(|term| !term.is_empty())
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

#[cfg(test)]
mod tests {
    use super::{WossnameCatalogEntry, WossnameError, WossnameMatchField, wossname_profile};

    #[test]
    fn rejects_blank_queries() {
        let error = wossname_profile("   ", &[]).expect_err("blank query should fail");
        assert_eq!(error, WossnameError::EmptyQuery);
    }

    #[test]
    fn rejects_blank_catalog_rows() {
        let error = wossname_profile(
            "align",
            &[WossnameCatalogEntry::new("", "global alignment")],
        )
        .expect_err("blank tool name should fail");
        assert_eq!(error, WossnameError::EmptyToolName { entry_index: 0 });

        let error = wossname_profile("align", &[WossnameCatalogEntry::new("needle", "   ")])
            .expect_err("blank description should fail");
        assert_eq!(
            error,
            WossnameError::EmptyShortDescription { entry_index: 0 }
        );
    }

    #[test]
    fn matches_normalized_query_terms_in_stable_catalog_order() {
        let profile = wossname_profile(
            " ALIGN, pairwise ",
            &[
                WossnameCatalogEntry::new("needle", "global pairwise sequence alignment"),
                WossnameCatalogEntry::new("water", "local pairwise sequence alignment"),
                WossnameCatalogEntry::new("seqret", "read and rewrite sequence records"),
            ],
        )
        .expect("wossname should succeed");

        assert_eq!(profile.query, "ALIGN, pairwise");
        assert_eq!(profile.normalized_terms, vec!["align", "pairwise"]);
        assert_eq!(profile.searched_entry_count, 3);
        assert_eq!(profile.matches.len(), 2);
        assert_eq!(profile.matches[0].tool_name, "needle");
        assert_eq!(profile.matches[1].tool_name, "water");
        assert_eq!(
            profile.matches[0].matched_fields,
            vec![WossnameMatchField::ShortDescription]
        );
        assert_eq!(
            profile.matches[0].matched_terms,
            vec!["align".to_owned(), "pairwise".to_owned()]
        );
    }

    #[test]
    fn records_both_name_and_description_fields_when_both_match() {
        let profile = wossname_profile(
            "word match",
            &[WossnameCatalogEntry::new(
                "wordmatch",
                "match identical words between two sequences",
            )],
        )
        .expect("wossname should succeed");

        assert_eq!(
            profile.matches[0].matched_fields,
            vec![
                WossnameMatchField::ToolName,
                WossnameMatchField::ShortDescription
            ]
        );
    }

    #[test]
    fn requires_every_normalized_term_to_match_some_local_field() {
        let profile = wossname_profile(
            "alignment remote",
            &[WossnameCatalogEntry::new(
                "needle",
                "global pairwise sequence alignment",
            )],
        )
        .expect("wossname should succeed");

        assert!(profile.matches.is_empty());
    }
}
