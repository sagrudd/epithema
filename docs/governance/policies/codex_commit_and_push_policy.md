# EMBOSS-RS Codex Commit and Push Policy

**Status:** Formal development policy  
**Date:** 2026-04-15  
**Applies to:** `emboss-rs`, `emboss-r`, and any directly related repository used in the EMBOSS-RS programme of work

Canonical governance context:
[EMBOSS-RS Governance Manual](../emboss_rs_governance_manual.md)

## 1. Purpose

This policy defines the minimum source-control discipline required for development work performed through Codex prompts.

Codex can change repository contents directly, so commit and push discipline is a
necessary part of reproducible development governance for EMBOSS-RS.

## 2. Policy Statement

After each **development prompt** executed in Codex that results in a material repository change, a commit **must** be created and **must** be pushed to the relevant remote repository.

For the purposes of this policy:

- A **development prompt** means any Codex instruction intended to create, modify, delete, refactor, reorganize, document, test, configure, or otherwise change repository contents.
- A **material repository change** means any non-trivial change to tracked files, including source code, tests, fixtures, configuration, build logic, documentation, schemas, workflow files, or project metadata.
- The **relevant remote repository** means the canonical upstream or designated working remote for the repository in which the change was made.

## 3. Canonical Rule

The default rule is:

1. execute one development prompt in Codex
2. review the resulting repository diff
3. create a commit covering that prompt's change set
4. push that commit to the applicable remote branch

No subsequent development prompt should begin until the prior prompt's resulting material changes have been committed and pushed.

## 4. Required Scope

This rule applies to changes in:

- Rust code
- R code
- tests and validation assets
- Sphinx and GitHub Pages documentation
- fixtures and documentation JSON inputs
- build scripts and Makefile logic
- CI and automation configuration
- repository governance documents

## 5. Exceptions

The following cases do **not** require a commit and push:

1. **No-op prompt**  
   The prompt results in no material repository change.

2. **Abandoned attempt**  
   The prompt produces work that is intentionally discarded before acceptance into the repository.

3. **Exploratory local scratch output**  
   Temporary artefacts are created only for investigation and are not intended to remain as tracked project state.

Empty commits should not be used merely to satisfy this policy.

## 6. Commit Quality Requirements

Each required commit should:

- correspond to a single development prompt or one tightly coupled change set arising from that prompt
- have a message that accurately summarizes the change
- leave the repository in a coherent state
- include associated documentation and tests where the prompt materially affects them

Where a development prompt introduces a new tool, the same commit or an immediately following prompt-derived commit should include the required documentation and validation updates mandated elsewhere in project policy.

## 7. Push Requirements

The resulting commit must be pushed promptly to the appropriate remote branch so that:

- work is not stranded only in a local Codex environment
- review and audit trails remain current
- downstream automation can observe the new state
- sponsor-visible progress remains reproducible

## 8. Relationship to Existing Project Policy

This rule complements, and does not replace, the existing requirements that:

- documentation be maintained through Sphinx using the Read the Docs template
- `emboss-rs autodoc` generate and maintain credible documentation inputs and outputs
- GitHub Pages be provisioned from project start
- validation and documentation be grounded in historical EMBOSS artefacts and reproducible runs

## 9. Governance Interpretation

This policy is intended to enforce small, reviewable, attributable increments of work. Its aim is not bureaucratic overhead; it is traceability, reproducibility, and disciplined delivery for a sponsored scientific software reboot.

## 10. Recommended Normative Text for Inclusion in the Main Governance Set

"After each development prompt executed in Codex that results in a material repository change, a commit shall be created and pushed to the relevant remote repository before further development prompts are undertaken. No-op prompts and discarded exploratory changes do not require empty commits."
