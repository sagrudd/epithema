//! Coverage checks for generated tool documentation against the exposed registry.

use std::collections::BTreeSet;
use std::path::Path;

use epithema_docgen::load_document_from_path;
use epithema_tools::governed_tool_descriptors;

fn repo_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve")
}

#[test]
fn every_exposed_tool_has_autodoc_input_generated_page_and_validation_stub() {
    let root = repo_root();
    let generated_index =
        std::fs::read_to_string(root.join("docs/generated/index.md")).expect("index should exist");

    let mut seen_slugs = BTreeSet::new();

    for descriptor in governed_tool_descriptors() {
        let slug = descriptor.name;
        assert!(
            seen_slugs.insert(slug),
            "duplicate generated-doc slug for exposed tool '{slug}'"
        );

        let contract_path = root.join("docs/autodoc/tools").join(format!("{slug}.json"));
        assert!(
            contract_path.exists(),
            "missing autodoc input for exposed tool '{slug}'"
        );
        let document = load_document_from_path(&contract_path)
            .unwrap_or_else(|error| panic!("invalid autodoc input for '{slug}': {error}"));
        assert_eq!(document.tool.name, descriptor.name);
        assert_eq!(document.tool.family.as_deref(), Some(descriptor.family));

        let generated_page = root.join("docs/generated/tools").join(format!("{slug}.md"));
        assert!(
            generated_page.exists(),
            "missing generated docs page for exposed tool '{slug}'"
        );

        let validation_path = root
            .join("docs/generated/validation")
            .join(format!("{slug}.validation.json"));
        assert!(
            validation_path.exists(),
            "missing validation stub for exposed tool '{slug}'"
        );

        assert!(
            generated_index.contains(&format!("tools/{slug}")),
            "generated index does not contain exposed tool '{slug}'"
        );
    }
}

#[test]
fn generated_index_contains_only_unique_tool_entries() {
    let root = repo_root();
    let generated_index =
        std::fs::read_to_string(root.join("docs/generated/index.md")).expect("index should exist");

    let mut seen = BTreeSet::new();
    for line in generated_index.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("tools/") {
            continue;
        }
        assert!(
            seen.insert(trimmed.to_owned()),
            "generated docs index contains duplicate entry '{trimmed}'"
        );
    }
}
