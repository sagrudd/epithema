# EMBOSS-RS Documentation and Auto-Documentation Policy

**Status:** Formal development policy addendum  
**Applies to:** `emboss-rs`, `emboss-r`, project documentation, release process, and GitHub Pages publication  
**Date:** 2026-04-15

Canonical governance context:
[EMBOSS-RS Governance Manual](../emboss_rs_governance_manual.md)

## 1. Purpose

This policy establishes the mandatory documentation, auto-documentation, and publication standards for the EMBOSS-RS project.

The purpose of this policy is to ensure that EMBOSS-RS maintains credible, reproducible, and continuously refreshed documentation grounded in historical EMBOSS artefacts and executable evidence from the modern codebase.

This policy is part of the formal development governance of the product.

## 2. Scope

This policy applies to:

- all user-facing `emboss-rs` tools
- all generated method documentation
- all documentation data sources and artefacts
- all release workflows involving newly introduced tools
- all public project documentation websites
- the supporting relationship between historical EMBOSS artefacts and EMBOSS-RS documentation generation

## 3. Normative Requirements

### 3.1 Documentation framework

Product documentation **must** be maintained using **Sphinx** with the **Read the Docs** template or equivalent Read the Docs theme configuration.

The documentation system **must** be structured so that both hand-authored and auto-generated content can be built reproducibly as part of the project build and release workflows.

### 3.2 Canonical auto-documentation command

`emboss-rs` **must** expose an `autodoc` subcommand.

The canonical command form is:

```text
emboss-rs autodoc
```

This command is the authoritative entry point for core documentation generation.

### 3.3 Documentation input contract

`emboss-rs autodoc` **shall** consume a JSON document describing:

- the origin of sequences and other artefacts to be processed by individual methods
- the identity of the method or methods to be documented
- the input artefacts required for the documentation run
- the descriptive text to be included in the generated documentation
- any mappings from historical EMBOSS examples to EMBOSS-RS invocations
- comparison or validation metadata where applicable
- any plot or rendering metadata required for documentation assembly

This JSON is the formal machine-readable specification for an auto-documentation run.

### 3.4 Derivation of documentation JSON from historical EMBOSS artefacts

It is the responsibility of `emboss-rs autodoc` to derive documentation JSON from the original EMBOSS GitHub artefacts available at:

`https://github.com/kimrutherford/EMBOSS`

This derivation responsibility includes, where feasible:

- discovery of relevant historical example inputs
- discovery of tool documentation artefacts
- extraction of example invocation context
- extraction of descriptive narrative content suitable for migration or adaptation
- construction of EMBOSS-to-EMBOSS-RS example mappings
- generation of normalized documentation JSON suitable for downstream Sphinx rendering

Where historical artefacts are incomplete, inconsistent, or ambiguous, `emboss-rs autodoc` **must** record that fact explicitly in generated metadata or diagnostics.

### 3.5 Data provenance and acquisition discipline

Data used in documentation **shall** be downloaded for reporting purposes only by methods exposed elsewhere within the `emboss-rs` software.

This means:

- `autodoc` must not introduce an undocumented shadow retrieval path
- all downloads used for documentation must flow through supported `emboss-rs` retrieval or acquisition methods
- documentation evidence must therefore remain aligned with real product functionality
- provenance of all retrieved artefacts must be capturable and reportable

### 3.6 Continuous documentation obligation for new tools

`emboss-rs autodoc` **shall** be run each time a new tool is introduced into the software.

A newly introduced tool is not considered credibly documented unless:

- its documentation inputs are defined
- its documentation JSON is available or derivable
- its auto-documentation has been executed
- its generated documentation has been incorporated into the project documentation build

This requirement exists to ensure that documentation remains credible as methods are released.

### 3.7 GitHub Pages obligation

The project **must** maintain formal GitHub Pages for public documentation.

Those pages **must** be provisioned at the start of development rather than deferred until later stages.

The GitHub Pages site must serve as an official publication surface for:

- user documentation
- method documentation
- generated examples
- release-linked documentation snapshots where appropriate
- public project information relevant to the EMBOSS-RS reboot

## 4. Architectural Implications

This policy implies the following development requirements.

### 4.1 Documentation is a first-class subsystem

Documentation generation is not an auxiliary convenience. It is a core subsystem of EMBOSS-RS and must be reflected in architecture, build orchestration, testing, and release policy.

### 4.2 Auto-documentation must be reproducible

`emboss-rs autodoc` must operate reproducibly from declared inputs, declared provenance, and versioned source artefacts.

### 4.3 Separation of authored and generated content

The Sphinx documentation layout should distinguish between:

- hand-authored narrative documentation
- generated method documentation
- generated validation and example artefacts
- generated provenance and comparison reports

### 4.4 Auto-documentation must remain product-faithful

Any data acquisition performed for documentation must reuse product methods. Documentation must not depend on capabilities that are absent from the actual EMBOSS-RS user surface.

### 4.5 Historical grounding is mandatory

The historical EMBOSS repository is an authoritative source for deriving example content and documentation inputs, subject to explicit normalization and provenance tracking.

## 5. Minimum JSON Content Model

The exact schema may evolve, but documentation JSON should be capable of expressing at least the following fields:

- documentation unit identifier
- tool name
- tool family
- legacy EMBOSS source references
- EMBOSS-RS invocation definition
- input artefact list
- input artefact provenance
- acquisition method used
- descriptive narrative text
- expected outputs
- comparison rules
- plot requirements
- rendering instructions or hooks
- generated file inventory
- warnings, caveats, and ambiguity notes

## 6. Release and CI Policy

The development workflow should enforce the following:

- Sphinx documentation must build in CI
- `emboss-rs autodoc` runs should be automatable in CI or scheduled release workflows
- the introduction of a new tool should fail policy checks if required documentation generation has not been defined
- published GitHub Pages content should be updated through a controlled documentation publication workflow

## 7. Governance Rules

The following governance rules are adopted.

1. No tool should be treated as release-credible without documentation generation support.
2. No documentation download path may bypass existing EMBOSS-RS retrieval methods.
3. Historical EMBOSS artefacts are to be treated as source material, not copied blindly.
4. Ambiguity in historical documentation must be preserved as explicit provenance or caveat metadata.
5. Public documentation publication is mandatory from project inception.

## 8. Recommended Operational Interpretation

For implementation purposes, this policy should be interpreted to require:

- a dedicated documentation build pipeline
- a dedicated `autodoc` implementation in `emboss-rs`
- a versioned JSON schema for auto-documentation inputs
- historical artefact harvesting from the public EMBOSS repository
- tight linkage between validation, acquisition, and documentation generation
- early establishment of the official GitHub Pages site

## 9. Canonical Policy Statement

The EMBOSS-RS project shall maintain product documentation using Sphinx with the Read the Docs template. The project shall provide a canonical `emboss-rs autodoc` command that consumes documentation JSON describing input artefact provenance, method execution context, and documentation text. The `autodoc` capability shall derive documentation JSON from historical artefacts in the public EMBOSS repository where possible, and any data downloaded for documentation purposes shall be obtained only through methods already exposed elsewhere in EMBOSS-RS. Auto-documentation shall be run whenever a new tool is introduced, and the project shall maintain formal GitHub Pages from the start of development as an official public documentation surface.
