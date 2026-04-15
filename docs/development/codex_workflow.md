# Codex Workflow

## Purpose

This document explains how Codex-driven development work should be conducted in
`emboss-rs` in practical terms. The normative policy source is the
[Codex Commit and Push Policy](../governance/policies/codex_commit_and_push_policy.md).
This document translates that policy into an operating workflow.

## Scope a Prompt Narrowly

A Codex development prompt should describe one coherent unit of work. Good prompt
scope is narrow enough that the resulting diff can be reviewed, explained, and
committed as one change set.

As a practical rule:

- one material prompt should correspond to one commitable unit of work
- unrelated refactors or follow-on ideas should be split into later prompts
- speculative future behavior should not be implemented just to make a prompt
  feel more complete

## Material Prompt Rule

After each material Codex development prompt, the resulting repository changes
must be:

1. reviewed
2. validated with the relevant checks
3. committed
4. pushed

No subsequent material development prompt should begin until that sequence has
been completed.

## Narrow Exceptions

The commit-and-push rule does not require a commit for:

- a true no-op prompt that changes no tracked project state
- exploratory work that is intentionally discarded
- temporary scratch output that is not accepted into the repository

These exceptions should be interpreted narrowly. Empty commits should not be
used merely to satisfy the rule.

## Diff Review Expectations

Before committing a Codex-driven change:

- inspect the diff for scope drift
- remove accidental or unrelated edits
- confirm file locations and names follow repository structure expectations
- confirm documentation updates are included where required
- confirm the sister repository `../emboss-r` was not modified unless the prompt
  explicitly authorized that work

## Validation Expectations

Run the smallest relevant check set for the change, but do not skip checks that
are directly implicated by the work.

For the current repository state, the main checks are:

- `make build`
- `make fmt`
- `make lint`
- `make test`
- `make lint-repo`
- `make check-sister-repo`
- `make lint-docs`
- `make docs`
- `make ci`

If a prompt changes documentation, repository workflow, CI, or policy, the docs
checks should normally be run before commit.

## Prompt Atomicity and Traceability

Prompt-driven work should remain traceable from prompt to diff to commit. In
practice this means:

- keep each prompt focused
- use a commit message that reflects that prompt’s delivered change
- avoid bundling several unrelated prompt outcomes into one commit
- push promptly so the branch history remains current and auditable

## Documentation and Policy Changes

When a Codex prompt changes contributor workflow, governance-adjacent material,
CI, docs build behavior, or repository structure, update the relevant
documentation in the same prompt-derived change set where practical.

## Pull Request Use

When work is presented through a pull request, the PR description should make it
clear:

- what prompt or task the change addresses
- what checks were run
- whether documentation changed
- whether any intentionally deferred work remains

The repository PR template is designed to reinforce that discipline.
