# v1.0.0 Release-Candidate Readiness

Status date: 2026-04-21

## Recommendation

`emboss-rs` is **ready for a `v1.0.0` release-candidate tag (`RC1`) with known limitations**.

This audit found no repository-level blocker in the Rust workspace, docs path,
or release automation that should prevent a conservative release-candidate cut.
The remaining gaps are evidence-depth and operational cutover items, not
missing method exposure or broken packaging mechanics.

## Audited Shipped Cohort

The release-candidate audit uses the governed tool registry
`emboss_tools::governed_tool_descriptors()` as the source of truth. The current
shipped cohort is:

- `aligncopy`
- `aligncopypair`
- `infoalign`
- `extractalign`
- `runinfo`
- `runget`
- `matcher`
- `distmat`
- `cons`
- `consambig`
- `needle`
- `needleall`
- `seqret`
- `refseqget`
- `newseq`
- `seqcount`
- `notseq`
- `nthseq`
- `skipseq`
- `degapseq`
- `revseq`
- `trimseq`
- `descseq`
- `maskseq`
- `maskfeat`
- `extractfeat`
- `featcopy`
- `cai`
- `chips`
- `codcmp`
- `codcopy`
- `fuzznuc`
- `fuzzpro`
- `fuzztran`
- `charge`
- `complex`
- `compseq`
- `geecee`
- `pepstats`
- `backtranseq`
- `backtranambig`
- `checktrans`
- `extractseq`
- `cutseq`
- `union`
- `splitter`

## Readiness Summary

### Complete / Ready

- Workspace version metadata is normalized to `1.0.0`.
- All 46 shipped methods are documented and present in the generated docs index.
- All 46 shipped methods have a checked-in validation stub.
- The cohort-level evidence report is present in both JSON and Markdown forms.
- Release tasks now cover:
  - version-alignment verification
  - generated-doc/evidence freshness verification
  - release-mode Rust build
  - docs build for release
  - Linux release bundle assembly
  - container build wiring
- GitHub Actions now verify the release-oriented docs path and a Linux-first
  container smoke build.
- GitHub Pages publication remains wired through the release-oriented docs path.
- The root docs and README continue to state the first-class R and R-owned
  plotting posture explicitly.

### Ready With Known Limitations

- Evidence depth is uneven across the shipped cohort.
  - `20` methods currently show `executable_evidence`
  - `1` method shows `declared_evidence`
  - `25` methods remain at `documented_only`
- No shipped method yet records harvested historical evidence in the cohort
  report.
- No shipped method yet records compared-against-expected evidence in the
  cohort report.
- The first plotting slice remains limited to `charge`, by design.
- The R surface is first-class and real, but it is still a curated subset of
  the full Rust registry rather than exhaustive parity across retrieval/archive
  methods.

### Not Ready / Blocked

- No repository-level blocker was found in this audit.

## Governance And Release Obligation Audit

### Documentation completeness

- Status: `complete`
- Basis: all shipped methods appear in the generated docs tree and in the
  cohort report; Sphinx builds cleanly.

### Validation and evidence presence

- Status: `ready with known limitations`
- Basis: every shipped method has a validation stub and appears in the cohort
  report, but most methods still lack harvested or compared evidence depth.

### Cohort-level acceptance reporting

- Status: `complete`
- Basis: the shipped cohort report is generated at:
  - `docs/generated/validation/shipped_cohort.validation.json`
  - `docs/generated/cohort_validation.md`

### Workspace and test health

- Status: `complete`
- Basis: `cargo build` and `cargo test` pass on the audited branch.

### Docs and publication path

- Status: `complete`
- Basis: the Sphinx build passes, the release docs build path is clean, and the
  Pages workflow now runs the release-oriented docs target.

### Release task availability

- Status: `complete`
- Basis: `make release-version-check`, `make release-generated-check`,
  `make release-check`, and `make release-artifacts` are present and working.

### Container readiness

- Status: `ready with known limitations`
- Basis: the Dockerfile, `.dockerignore`, CI smoke build, and release workflow
  container path are present. Local smoke execution was not possible in this
  environment because Docker is unavailable.

### R-first-class posture in docs

- Status: `complete`
- Basis: the root README and release docs describe `emboss-r` as a real peer
  user surface and keep plotting ownership in R.

## Concrete Gap Report

### Missing validation depth

- `25` methods remain at `documented_only` in the cohort report:
  - `aligncopy`, `aligncopypair`, `infoalign`, `extractalign`
  - `runinfo`, `runget`
  - `matcher`, `distmat`, `cons`, `consambig`
  - `needleall`
  - `seqret`, `refseqget`
  - `degapseq`, `trimseq`
  - `cai`, `chips`, `codcmp`, `codcopy`
  - `charge`, `complex`
  - `backtranseq`, `backtranambig`, `checktrans`
  - `splitter`
- `newseq` is only at `declared_evidence`.

### Legacy harvesting remains partial

- The cohort report still records `0` methods with harvested legacy evidence.
- This is acceptable for an RC, but it remains the main credibility gap for a
  later post-`1.0.0` acceptance-hardening phase.

### Comparison-based acceptance remains absent

- The cohort report still records `0` methods with compared evidence.
- The current validation foundation is structurally ready, but the expected
  output comparison layer is not yet populated across the cohort.

### Plotting limitations

- `charge` remains the only end-to-end plot-producing analytical path.
- This is in scope for `1.0.0` and not treated as a blocker.

### R surface limitations

- `emboss-r` now provides a real first-class surface, but retrieval/archive
  methods and broader parity remain deferred.
- This is aligned with the current README in the sister repository and is not
  a blocker for the Rust-side `1.0.0` candidate.

### Manual release steps still required

- Run or observe the Linux container smoke build in an environment with Docker.
- Perform final review of the changelog and release notes.
- Create the `v1.0.0` tag.
- Publish the GitHub release after the tag workflow completes.
- Confirm the coordinated `emboss-r` release decision separately.

## Small Blocking Defects Resolved In This Audit

- No new repository-level code defect required a release-blocking fix in this
  prompt.
- The practical validation rerun did surface one transient generated-docs race:
  `docs/generated/index.md` must retain the `cohort_validation` entry before a
  strict Sphinx pass. Re-running the existing generated-index normalization
  step resolved that cleanly, and the release-generated check now passes again.
- The main output of this audit is therefore the explicit readiness and gap
  report, not another round of release automation changes.

## Practical Validation Run

The following checks were completed in this environment during the release-
candidate audit:

- `cargo build`
- `cargo test`
- `python3 scripts/release_metadata.py check`
- `PYTHON=.venv-docs/bin/python make release-generated-check`
- `PYTHON=.venv-docs/bin/python make release-artifacts`
- workflow YAML parse validation for:
  - `.github/workflows/ci.yml`
  - `.github/workflows/docs-pages.yml`
  - `.github/workflows/release.yml`
  - `.github/release.yml`

Environment-limited checks that could not be executed locally here:

- `make release-container`, because `docker` is not installed in this
  environment
- any `emboss-r` package checks, because this prompt does not modify that
  repository and R is not available locally here

## Final RC Interpretation

The current `emboss-rs` branch is suitable for a conservative `v1.0.0 RC1`
tag. It is not evidence-complete, but it is release-mechanically ready and
honest about the remaining acceptance gaps.
