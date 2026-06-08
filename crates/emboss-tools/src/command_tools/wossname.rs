//! `wossname` implementation.

use emboss_core::{WossnameCatalogEntry, wossname_profile};

use crate::sequence_stream::ToolExecutionError;
use crate::{ToolDescriptor, governed_tool_descriptors};

/// Typed parameters for `wossname`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WossnameParams {
    /// Free-text keyword query.
    pub query: String,
}

/// Stable summary row for one bounded `wossname` match.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WossnameRow {
    /// Stable governed tool identity.
    pub tool_name: String,
    /// Governed tool family for the matched tool.
    pub family: String,
    /// Governed short description for the matched tool.
    pub short_description: String,
    /// Stable ordered normalized query terms that matched this row.
    pub matched_terms: Vec<String>,
    /// Stable ordered text fields that satisfied at least one matched term.
    pub matched_fields: Vec<String>,
}

/// Structured `wossname` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WossnameOutcome {
    /// Original query after trimming.
    pub query: String,
    /// Stable ordered normalized query terms.
    pub normalized_terms: Vec<String>,
    /// Count of governed tool entries searched.
    pub searched_entry_count: usize,
    /// Stable ordered keyword-match rows.
    pub rows: Vec<WossnameRow>,
}

/// Returns the bounded `wossname` help text.
#[must_use]
pub fn wossname_help() -> &'static str {
    "Usage: emboss-rs wossname <keyword-query>\n\nReport deterministic keyword matches against the governed local tool catalog. The bounded v1 seam searches only governed tool names and short descriptions, uses normalized case-folded substring matching, and emits table-first rows rather than a broader semantic-ranking or ontology-driven discovery workflow."
}

/// Executes bounded `wossname`.
pub fn run_wossname(params: WossnameParams) -> Result<WossnameOutcome, ToolExecutionError> {
    let descriptors = governed_tool_descriptors();
    let entries: Vec<_> = descriptors
        .iter()
        .map(|descriptor| WossnameCatalogEntry::new(descriptor.name, descriptor.summary))
        .collect();

    let profile = wossname_profile(&params.query, &entries).map_err(|error| {
        emboss_diagnostics::PlatformError::new(
            emboss_diagnostics::ErrorCategory::Validation,
            error.to_string(),
        )
        .with_code("tools.wossname.profile.invalid")
    })?;

    let rows = profile
        .matches
        .into_iter()
        .map(|hit| {
            let descriptor = descriptor_for_tool(descriptors, &hit.tool_name)
                .expect("wossname rows always derive from governed descriptors");
            WossnameRow {
                tool_name: hit.tool_name,
                family: descriptor.family.to_owned(),
                short_description: hit.short_description,
                matched_terms: hit.matched_terms,
                matched_fields: hit
                    .matched_fields
                    .into_iter()
                    .map(|field| field.as_str().to_owned())
                    .collect(),
            }
        })
        .collect();

    Ok(WossnameOutcome {
        query: profile.query,
        normalized_terms: profile.normalized_terms,
        searched_entry_count: profile.searched_entry_count,
        rows,
    })
}

fn descriptor_for_tool<'a>(
    descriptors: &'a [ToolDescriptor],
    tool_name: &str,
) -> Option<&'a ToolDescriptor> {
    descriptors
        .iter()
        .find(|descriptor| descriptor.name == tool_name)
}

#[cfg(test)]
mod tests {
    use super::{WossnameParams, run_wossname};

    #[test]
    fn reports_expected_keyword_match_rows() {
        let outcome = run_wossname(WossnameParams {
            query: "pairwise align".to_owned(),
        })
        .expect("wossname should execute");

        assert_eq!(outcome.query, "pairwise align");
        assert_eq!(outcome.normalized_terms, vec!["pairwise", "align"]);
        assert!(outcome.searched_entry_count >= 3);
        assert_eq!(outcome.rows.len(), 4);
        let matched_tools: Vec<_> = outcome
            .rows
            .iter()
            .map(|row| row.tool_name.as_str())
            .collect();
        assert_eq!(
            matched_tools,
            vec!["aligncopypair", "needle", "needleall", "water"]
        );
        assert_eq!(outcome.rows[0].family, "alignment_tools");
        assert_eq!(
            outcome.rows[0].matched_fields,
            vec!["tool_name".to_owned(), "short_description".to_owned()]
        );
    }

    #[test]
    fn rejects_blank_queries() {
        let error = run_wossname(WossnameParams {
            query: "   ".to_owned(),
        })
        .expect_err("blank query should fail");

        assert!(
            error
                .to_string()
                .contains("requires at least one keyword query term")
        );
    }
}
