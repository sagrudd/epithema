# EMBOSS-RS Governance Manual

## 1. Purpose and Status

This manual is the canonical governance document for `emboss-rs`, the Rust-based
reboot of EMBOSS. It defines the project's governing expectations for platform
intent, scope, tool-surface discipline, documentation requirements, repository
change control, code organization, and release stewardship.

This manual is normative. Supporting policy documents under `docs/governance/`
expand specific governance topics, and the appendix material provides reference
support. Where wording overlaps, this manual is authoritative.

## 2. Project Intent and Platform Position

`emboss-rs` exists to re-establish EMBOSS as a maintainable, modern bioinformatics
tool suite with Rust as the implementation language for the core executable
surface. The project is not a loose collection of unrelated utilities. It is a
coherent family of command-line tools delivered through a shared project
governance model and a disciplined implementation structure.

The project must retain a first-class relationship with the sister repository
`emboss-r`. That repository provides the R-facing complement to `emboss-rs`,
including plotting and other R-native workflows that are explicitly governed as
part of the broader EMBOSS reboot effort.

## 3. Architecture and Platform Intent

The primary command-line surface is a single executable:
`emboss-rs <tool> ...`

This single-binary policy is mandatory. Individual EMBOSS-style tools are
exposed as stable subcommands rather than as a proliferation of unrelated
binaries. The intent is to keep operational behavior, packaging, help output,
and future autodocumentation consistent across the tool family.

The project architecture must preserve the distinction between:

- core Rust implementation and command dispatch in `emboss-rs`
- R-first functionality delivered through `../emboss-r`
- shared governance and documentation expectations spanning both efforts

Plotting is governed as an R responsibility. Where plotting or figure-generation
capabilities are required, they must be implemented through `emboss-r` rather
than recreated ad hoc inside `emboss-rs`, except where a future governance
decision explicitly documents a narrow exception.

## 4. Scope and Tool-Family Policy

`emboss-rs` should be managed as a family of related EMBOSS-compatible or
EMBOSS-inspired tools. Tool additions must fit a documented family structure and
must be introduced in a way that preserves discoverability, naming discipline,
and long-term maintainability.

The governing expectations are:

- new functionality should be attached to a defined tool family or an approved
  extension of an existing family
- the public execution surface remains `emboss-rs <tool> ...`
- tool-family naming must remain explicit and documented
- family-to-tool mappings should be tracked in a maintained reference document

The current reference location for this registry is
[Family-to-Tool Mapping Reference](./appendices/family_to_tool_mapping_reference.md).
That appendix is a reference artifact and should be updated as the governed tool
set expands.

Related normative detail is maintained in
[Tool Surface and Family Policy](./policies/tool_surface_and_family_policy.md).

## 5. Documentation and Autodoc Policy

Documentation is a mandatory part of the project, not a downstream convenience.
`autodoc` is required for `emboss-rs`. Tool definitions, command metadata,
shared interfaces, and other suitable structured project information should be
arranged so that future documentation builds can consume them systematically.

At the governance level, this means:

- every governed tool surface must be documentable from maintained source data
- documentation should be written in a normalized structure suitable for future
  Sphinx ingestion
- manual prose and generated documentation must reinforce the same governed
  command surface and terminology
- architecture and policy changes should update documentation as part of the
  same material change set

Related normative detail is maintained in
[Documentation and Autodoc Policy](./policies/documentation_and_autodoc_policy.md).

## 6. Codex Commit and Push Policy

Repository operations performed through Codex must follow a strict change-control
rule: after each material Codex prompt, the resulting work must be committed and
pushed. This rule exists to preserve traceability, reduce hidden local drift, and
maintain a reliable project record during the reboot phase.

This policy applies to documentation, code, and other material repository
changes. It should be interpreted conservatively: where a prompt produces a
meaningful repository state change, that change must conclude with review,
commit, and push.

Related normative detail is maintained in
[Codex Change Control Policy](./policies/codex_change_control_policy.md).

## 7. Code Structure and Module Naming Policy

The repository must maintain disciplined code structure and module naming.
Naming, module boundaries, and file layout should make tool ownership and shared
infrastructure legible. Ad hoc structure growth is not acceptable.

At a governance level, the expectations are:

- code organization should reflect stable project concepts
- module names should be professional, descriptive, and durable
- command dispatch, shared libraries, and family-specific logic should remain
  clearly separated
- documentation names and code names should remain aligned where they describe
  the same governed concepts

Related normative detail is maintained in
[Code Structure and Module Naming Policy](./policies/code_structure_and_module_naming_policy.md).

## 8. Release and Governance Expectations

Project releases and milestone decisions should be treated as governance events,
not only packaging events. Release readiness requires consistency across command
surface, family mapping, documentation coverage, and repository traceability.

At a minimum, release-governing expectations include:

- the public `emboss-rs <tool> ...` surface remains consistent with governance
- relevant tool-family mappings are recorded in the maintained reference
- documentation and `autodoc` inputs are current
- repository history reflects reviewed, committed, and pushed material changes
- cross-repository boundaries between `emboss-rs` and `emboss-r` remain explicit

Future release procedures may be documented in more operational detail, but they
must remain subordinate to the governance principles recorded here.

## 9. Document Structure

This governance manual is the canonical entry point. Additional governance
documents are organized as follows:

- [Governance Index](./index.md) provides the maintained entry point for this
  section
- [Policies](./policies/) contain focused normative elaborations of specific
  governance subjects
- [Appendices](./appendices/) contain supporting reference material

## 10. Change Management

Updates to governance documentation should prioritize normalization,
consolidation, and explicit rationale. New policy should not be created as a
flat accumulation of unlinked files. Instead, governance changes should:

- update this manual when project-governing meaning changes
- update the relevant policy module when a narrower topic requires expansion
- update appendix material when registries or references change
- preserve exact project-specific terminology where it is already governed,
  including `emboss-rs`, `emboss-r`, and `autodoc`
