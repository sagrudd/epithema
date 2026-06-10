# Epithema Governance Manual

## 1. Purpose and Status

This manual is the canonical governance document for `epithema`, the Rust-based
reboot of EMBOSS. It consolidates the governing intent already recorded across
the architecture brief and the formal policy documents into one maintained
project source document.

This manual is normative. The policy modules under `docs/governance/policies/`
elaborate specific topics, and the appendix documents under
`docs/governance/appendices/` preserve planning and reference detail. Where
there is overlap, this manual governs.

## 2. Project Intent and Platform Position

`epithema` is a full-platform reboot of the historic EMBOSS collection of
bioinformatics tools. The reboot is concept-compatible with EMBOSS rather than
strictly command-line compatible. The objective is to preserve and modernize the
scientific utility, breadth, and reputation of EMBOSS while replacing the legacy
implementation model with a contemporary Rust architecture, a first-class R
surface, and a modern validation and documentation pipeline.

The reboot has two first-class user surfaces:

- the Rust and CLI surface delivered through `epithema`
- the R surface delivered through the sister repository `epithemaR`

`epithemaR` is a peer surface, not merely a plotting adapter. It is responsible
for R-facing access to the Epithema method catalog and is the exclusive
plotting backend for graphical tools.

The detailed architecture background is preserved in the
[Foundational Architecture Brief](./appendices/foundational_architecture_brief.md).

## 3. Architecture and Platform Intent

The governing executable surface is a single binary:

`epithema <tool> ...`

This single-binary rule is mandatory. Individual EMBOSS-style tools are exposed
as governed subcommands rather than as a proliferation of standalone binaries.
This keeps packaging, discovery, help output, autodocumentation, and release
behavior disciplined and consistent.

The architecture must preserve a shared execution and service layer so that CLI,
R, and future API surfaces dispatch through the same scientific core. The future
API is not yet a delivery target, but readiness for it is a governance
requirement.

Plotting is governed as an R responsibility. `epithema` may emit plot-ready
data, but graphical rendering belongs in `epithemaR`.

## 4. Scope and Tool-Family Policy

`epithema` is governed as a coherent family of related EMBOSS-compatible or
EMBOSS-inspired tools, not as an unrelated utility collection. Tool additions
must fit an explicit family structure and preserve naming discipline,
discoverability, and long-term maintainability.

The governing rules are:

- the public execution surface remains `epithema <tool> ...`
- no standalone legacy command shims are introduced
- tool families are managed through explicit governance buckets
- family-level defaults may be refined by explicit per-tool decisions
- `complex` remains an explicit retained capability
- R-facing plotting workflows remain in `epithemaR`

The canonical normative detail is maintained in
[Scope and Tool-Family Policy](./policies/scope_and_tool_family_policy.md).
The maintained family-to-tool registry location is
[Family-to-Tool Mapping Reference](./appendices/family_to_tool_mapping_reference.md),
with the per-tool inventory preserved in
[Full Scope Matrix](./appendices/full_scope_matrix.md).

## 5. Documentation and Autodoc Policy

Documentation is a first-class subsystem of the project. `autodoc` is mandatory
for `epithema`, and governance changes must leave the repository ready for a
future Sphinx-based documentation build rather than forcing a later structural
migration.

The governing rules are:

- `epithema` must expose `epithema autodoc`
- documentation source material must be organized for future Sphinx ingestion
- documentation generation must be grounded in declared inputs and provenance
- historical EMBOSS artefacts are source material for executable documentation
- documentation downloads must reuse product retrieval methods rather than a
  shadow acquisition path
- a new tool is not release-credible until its documentation generation support
  is defined and executed

The canonical normative detail is maintained in
[Documentation and Autodoc Policy](./policies/documentation_and_autodoc_policy.md).

## 6. Codex Commit and Push Policy

Repository work performed through Codex must follow strict source-control
discipline. After each development prompt that results in a material repository
change, the resulting work must be reviewed, committed, and pushed to the
relevant remote repository before further development prompts proceed.

This rule applies to documentation, code, tests, configuration, validation
assets, and other material repository changes. It exists to preserve
traceability, auditability, and sponsor-visible progress during the Epithema
reboot.

The canonical normative detail is maintained in
[Codex Commit and Push Policy](./policies/codex_commit_and_push_policy.md).

## 7. Code Structure and Module Naming Policy

`epithema` must maintain disciplined repository structure and module naming as
the codebase grows. Module names should reflect implemented methods or durable
capabilities, and code should be grouped by explicit functional domains rather
than allowed to accumulate in flat or ad hoc structures.

The governing rules are:

- module and file names should remain method-aligned where practical
- code should be grouped by explicit functional domain
- shared code should be placed by purpose, not convenience
- generated code must conform to the same structural rules as hand-written code
- structural clarity is a first-class review criterion

The canonical normative detail is maintained in
[Code Structure and Module Naming Policy](./policies/code_structure_and_module_naming_policy.md).

## 8. Release and Governance Expectations

Release readiness is a governance question, not only a packaging question. A
release or milestone state is credible only when the public command surface,
family mapping, documentation, and repository traceability remain aligned.

At a minimum, release-governing expectations include:

- the `epithema <tool> ...` surface remains consistent with governance
- the family-to-tool registry is current at the appropriate level of detail
- the documentation and `autodoc` inputs are current
- historical-example validation remains part of the acceptance model
- repository history reflects reviewed, committed, and pushed material changes
- cross-repository boundaries between `epithema` and `epithemaR` remain explicit

## 9. Document Structure and Maintenance

This manual is the obvious canonical entry point for governance. Supporting
documents are organized as follows:

- [Governance Index](./index.md) provides the maintained table of contents
- policy modules under `docs/governance/policies/` hold focused normative
  elaborations
- appendix documents under `docs/governance/appendices/` hold architecture
  background, registries, and inventory reference material

Governance updates should prefer normalization, consolidation, and
de-duplication over a flat accumulation of disconnected files. Important
project-specific terms, including `epithema`, `epithemaR`, `autodoc`, and the
governance bucket names, should be preserved exactly where governed.
