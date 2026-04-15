//! Integration tests for the versioned autodoc JSON contract.

use emboss_docgen::{AutodocContractError, AutodocDocument};

fn fixture(path: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    std::fs::read_to_string(path).expect("fixture should be readable")
}

#[test]
fn parses_minimal_valid_autodoc_contract() {
    let json = fixture("tests/fixtures/minimal_autodoc.json");
    let document = AutodocDocument::from_json_str(&json).expect("contract should parse");

    assert_eq!(document.document_id, "needle-minimal");
    assert_eq!(document.examples.len(), 1);
}

#[test]
fn parses_rich_valid_autodoc_contract() {
    let json = fixture("tests/fixtures/rich_autodoc.json");
    let document = AutodocDocument::from_json_str(&json).expect("contract should parse");

    assert_eq!(document.tool.name, "seqret");
    assert_eq!(document.artifacts.len(), 3);
    assert_eq!(document.examples.len(), 2);
}

#[test]
fn rejects_unsupported_schema_version() {
    let mut json = fixture("tests/fixtures/minimal_autodoc.json");
    json = json.replace("emboss-rs.autodoc/v1", "emboss-rs.autodoc/v999");

    let error = AutodocDocument::from_json_str(&json).expect_err("schema version should fail");
    assert!(matches!(error, AutodocContractError::Validation(_)));
    assert!(
        error
            .to_string()
            .contains("unsupported autodoc schema version")
    );
}

#[test]
fn rejects_duplicate_artifact_identifier() {
    let json = r#"
    {
      "schema_version": "emboss-rs.autodoc/v1",
      "document_id": "dup-artifact",
      "tool": { "name": "needle", "family": null, "summary": null, "legacy_names": [] },
      "sections": [],
      "artifacts": [
        {
          "id": "a1",
          "label": "one",
          "origin": { "kind": "local_file" },
          "acquisition": { "mode": "local_path" },
          "reference": { "kind": "path", "path": "example.txt" },
          "description": null
        },
        {
          "id": "a1",
          "label": "two",
          "origin": { "kind": "local_file" },
          "acquisition": { "mode": "local_path" },
          "reference": { "kind": "path", "path": "example2.txt" },
          "description": null
        }
      ],
      "examples": [],
      "provenance": { "source_mode": "curated", "curated_by": null, "source_references": [] },
      "validation": null
    }"#;

    let error = AutodocDocument::from_json_str(json).expect_err("duplicate id should fail");
    assert!(error.to_string().contains("duplicate artifact identifier"));
}

#[test]
fn rejects_example_reference_to_missing_artifact() {
    let json = r#"
    {
      "schema_version": "emboss-rs.autodoc/v1",
      "document_id": "missing-artifact-ref",
      "tool": { "name": "needle", "family": null, "summary": null, "legacy_names": [] },
      "sections": [],
      "artifacts": [],
      "examples": [
        {
          "id": "example1",
          "title": "Broken example",
          "description": null,
          "artifact_ids": ["missing"],
          "parameters": [],
          "expected_outputs": [],
          "legacy_reference": null
        }
      ],
      "provenance": { "source_mode": "curated", "curated_by": null, "source_references": [] },
      "validation": null
    }"#;

    let error = AutodocDocument::from_json_str(json).expect_err("missing artifact should fail");
    assert!(error.to_string().contains("undeclared artifact"));
}
