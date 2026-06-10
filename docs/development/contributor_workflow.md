# Contributor Workflow

## Purpose

This document explains the practical contributor workflow for `epithema`. It is
operational guidance for human contributors and Codex-assisted sessions. It does
not replace the normative governance documents; where there is any conflict, the
[Epithema Governance Manual](../governance/epithema_governance_manual.md) and
the linked governance policies govern.

## Working Model

`epithema` is developed as a governed, auditable reboot of EMBOSS. Changes are
expected to be small, explicit, and reviewable. Contributors should prefer a
single coherent task per change set rather than accumulating unrelated work in
one branch or pull request.

The project has two first-class surfaces:

- the Rust and CLI surface in `epithema`
- the R surface in the sister repository `epithemaR`

Unless a prompt or task explicitly authorizes cross-repository work, this
repository should be changed independently and references to `epithemaR` should
remain documentary or compatibility-oriented only.

## Standard Change Flow

For a normal contribution:

1. Read the relevant governance and workflow documents before changing files.
2. Scope the task narrowly enough that the resulting diff is coherent and
   reviewable.
3. Make the repository changes.
4. Review the diff before commit.
5. Run the relevant local checks for the affected scope.
6. Update documentation when behavior, policy, build workflow, or repository
   structure changes.
7. Commit with a message that accurately describes the change.
8. Push the change promptly to the relevant remote branch.
9. Open or update a pull request using the repository checklist.

## Canonical Local Task Surface

The root `Makefile` is the canonical entry point for common repository tasks.
Run `make help` for the current task surface.

At present the most relevant targets are:

- `make build`
- `make fmt`
- `make lint`
- `make test`
- `make lint-repo`
- `make check-sister-repo`
- `make lint-docs`
- `make docs`
- `make ci`

Use the smallest relevant set of checks for the change, but prefer `make ci`
before finalizing a material repository update when the current repository state
makes that practical.

## Documentation Expectations

Documentation is a first-class subsystem in this project. Contributors should
update documentation whenever they change:

- project policy
- contributor workflow
- repository structure
- docs build or publication behavior
- user-facing commands or interfaces

For this repository state, documentation changes are often part of the same
material change set rather than a follow-up task.

For exposed bioinformatics methods, the canonical documentation-preparation path
is the committed autodoc contract set under `docs/autodoc/tools/` plus
generation through `epithema autodoc`. Adding or exposing a tool without a
matching autodoc contract, generated page, and generated index entry is treated
as a repository error and should fail the completeness checks.

## Structure and Naming Discipline

Contributors must preserve disciplined structure and naming. The current Rust
workspace is organized under `crates/` by architectural role, and future code
should extend that layout coherently rather than bypassing it. Documentation and
repository structure should follow the same principle.

## Pull Request Expectations

A pull request should make it easy for a reviewer to confirm:

- what task or prompt the change addresses
- which checks were run
- whether documentation was updated
- whether the change touched only `epithema`
- whether any deferred work remains

The PR template is intended to keep that review surface consistent, not to add
bureaucracy.
