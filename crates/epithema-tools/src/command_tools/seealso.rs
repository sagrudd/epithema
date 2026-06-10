//! `seealso` implementation.

use epithema_core::{SeealsoCatalogEntry, seealso_profile};

use crate::sequence_stream::ToolExecutionError;
use crate::{ToolDescriptor, governed_tool_descriptors};

/// Typed parameters for `seealso`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeealsoParams {
    /// Governed tool name query.
    pub tool_name: String,
}

/// Stable summary row for one bounded `seealso` related-program result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeealsoRow {
    /// Stable governed query tool identity.
    pub query_tool: String,
    /// Stable governed related-tool identity.
    pub related_tool: String,
    /// Governed tool family for the related tool.
    pub related_family: String,
    /// Governed short description for the related tool.
    pub related_short_description: String,
    /// Stable ordered normalized terms that justify the relationship.
    pub relationship_terms: Vec<String>,
    /// Stable ordered metadata fields that justify the relationship.
    pub relationship_fields: Vec<String>,
}

/// Structured `seealso` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeealsoOutcome {
    /// Original query tool name after trimming.
    pub query_tool_name: String,
    /// Resolved governed tool identity.
    pub resolved_tool_name: String,
    /// Resolved governed family.
    pub resolved_family: String,
    /// Resolved governed short description.
    pub resolved_short_description: String,
    /// Count of governed tool entries searched.
    pub searched_entry_count: usize,
    /// Stable ordered related-program rows.
    pub rows: Vec<SeealsoRow>,
}

/// Returns the bounded `seealso` help text.
#[must_use]
pub fn seealso_help() -> &'static str {
    "Usage: epithema seealso <tool-name>\n\nReport deterministic related-program rows for one governed local tool. The bounded v1 seam resolves relationships from governed family metadata and bounded short-description term overlap, emits table-first rows, and does not perform semantic ranking, ontology expansion, or provider-backed discovery."
}

/// Executes bounded `seealso`.
pub fn run_seealso(params: SeealsoParams) -> Result<SeealsoOutcome, ToolExecutionError> {
    let descriptors = governed_tool_descriptors();
    let entries: Vec<_> = descriptors
        .iter()
        .map(|descriptor| {
            SeealsoCatalogEntry::new(descriptor.name, descriptor.family, descriptor.summary)
        })
        .collect();

    let profile = seealso_profile(&params.tool_name, &entries).map_err(|error| {
        epithema_diagnostics::PlatformError::new(
            epithema_diagnostics::ErrorCategory::Validation,
            error.to_string(),
        )
        .with_code("tools.seealso.profile.invalid")
    })?;

    let rows = profile
        .related_tools
        .into_iter()
        .map(|related| {
            let descriptor = descriptor_for_tool(descriptors, &related.tool_name)
                .expect("seealso rows always derive from governed descriptors");
            SeealsoRow {
                query_tool: profile.resolved_tool_name.clone(),
                related_tool: related.tool_name,
                related_family: descriptor.family.to_owned(),
                related_short_description: related.short_description,
                relationship_terms: related.relationship_terms,
                relationship_fields: related
                    .relationship_fields
                    .into_iter()
                    .map(|field| field.as_str().to_owned())
                    .collect(),
            }
        })
        .collect();

    Ok(SeealsoOutcome {
        query_tool_name: profile.query_tool_name,
        resolved_tool_name: profile.resolved_tool_name,
        resolved_family: profile.resolved_family,
        resolved_short_description: profile.resolved_short_description,
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
    use super::{SeealsoParams, run_seealso};

    #[test]
    fn reports_expected_related_program_rows() {
        let outcome = run_seealso(SeealsoParams {
            tool_name: "needle".to_owned(),
        })
        .expect("seealso should execute");

        assert_eq!(outcome.query_tool_name, "needle");
        assert_eq!(outcome.resolved_tool_name, "needle");
        assert_eq!(outcome.resolved_family, "pairwise_alignment");
        assert!(outcome.searched_entry_count >= 3);
        assert!(outcome.rows.iter().any(|row| row.related_tool == "water"));
        let water = outcome
            .rows
            .iter()
            .find(|row| row.related_tool == "water")
            .expect("water should be related to needle");
        assert_eq!(water.query_tool, "needle");
        assert_eq!(water.related_family, "pairwise_alignment");
        assert!(water.relationship_fields.contains(&"family".to_owned()));
    }

    #[test]
    fn rejects_unknown_tool_names() {
        let error = run_seealso(SeealsoParams {
            tool_name: "missing-tool".to_owned(),
        })
        .expect_err("unknown query should fail");

        assert!(error.to_string().contains("could not find governed tool"));
    }
}
