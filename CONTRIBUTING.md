# Contributing to EMBOSS-RS

This repository follows a governed, prompt-driven development model. The
canonical policy source is the
[EMBOSS-RS Governance Manual](./docs/governance/emboss_rs_governance_manual.md).
Contributor workflow guidance is documented under
[docs/development/](./docs/development/index.md).

## Start Here

New contributors should read, in order:

1. [Contributor Workflow](./docs/development/contributor_workflow.md)
2. [Codex Workflow](./docs/development/codex_workflow.md)
3. [Governance Manual](./docs/governance/emboss_rs_governance_manual.md)

## Core Working Rules

- Treat the root `Makefile` as the canonical entry point for common repository
  tasks.
- Keep changes atomic and reviewable.
- Update documentation when behavior, policy, or repository structure changes.
- Do not modify the sister repository `../emboss-r` unless a prompt explicitly
  authorizes work there.
- For Codex-driven work, each material development prompt must end with a commit
  and push, subject only to the narrow no-op or discarded-work exceptions
  documented in the Codex workflow guidance.

## Local Validation

The current baseline local checks are:

- `make build`
- `make fmt`
- `make lint`
- `make test`
- `make lint-repo`
- `make check-sister-repo`
- `make lint-docs`
- `make docs`
- `make ci`

## Pull Requests

Pull requests should describe the prompt or task scope they cover, identify the
checks run locally, and confirm whether documentation was updated as part of the
change. The repository PR template provides the current checklist.
