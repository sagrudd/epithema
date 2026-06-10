# Epithema Governance Documentation

This section contains the project-governing documentation for `epithema`.
It consolidates the architecture and policy material already produced for the
EMBOSS reboot into a single maintained location that future Sphinx
documentation can ingest without a disruptive content migration.

## Canonical Entry Point

The canonical governance document is
[Epithema Governance Manual](./epithema_governance_manual.md). That manual is
the authoritative entry point for project intent, scope, operating rules, and
release-governing expectations.

## Normative Documents

The following documents are normative:

- [Epithema Governance Manual](./epithema_governance_manual.md)
- [Scope and Tool-Family Policy](./policies/scope_and_tool_family_policy.md)
- [Documentation and Autodoc Policy](./policies/documentation_and_autodoc_policy.md)
- [Codex Commit and Push Policy](./policies/codex_commit_and_push_policy.md)
- [Code Structure and Module Naming Policy](./policies/code_structure_and_module_naming_policy.md)

Where wording overlaps, the governance manual governs.

## Appendices and Reference Material

The following documents are supporting reference material:

- [Foundational Architecture Brief](./appendices/foundational_architecture_brief.md)
- [Family-to-Tool Mapping Reference](./appendices/family_to_tool_mapping_reference.md)
- [Full Scope Matrix](./appendices/full_scope_matrix.md)

Appendices support planning, implementation, and release review, but they do not
supersede the normative governance documents.

## Operational Guidance

Operational workflow guidance for contributors and Codex sessions is maintained
separately under [Development Workflow](../development/index.md). That material
explains how to follow the governance rules in practice but does not supersede
the normative governance documents.

## Maintenance Notes

New governance material should be added under `docs/governance/` and linked from
this index. Normative policy belongs in the governance manual and, where useful,
the `policies/` directory. Extended registries, detailed inventories, and other
supporting material belong in `appendices/`.

## Governance Contents

```{toctree}
:maxdepth: 2

epithema_governance_manual
policies/scope_and_tool_family_policy
policies/documentation_and_autodoc_policy
policies/codex_commit_and_push_policy
policies/code_structure_and_module_naming_policy
appendices/foundational_architecture_brief
appendices/family_to_tool_mapping_reference
appendices/full_scope_matrix
```
