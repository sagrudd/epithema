# v1.0.0 Release-Candidate Readiness

Status date: 2026-05-18

## Recommendation

`emboss-rs` is **ready for a `v1.0.0` release-candidate tag (`RC1`) with known limitations**.

This audit found no repository-level blocker in the Rust workspace, docs path,
or release automation that should prevent a conservative release-candidate cut.
The remaining gaps are evidence-depth and operational cutover items, not
missing method exposure or broken packaging mechanics. The current candidate
should be interpreted as release-mechanically ready with partial biological
acceptance depth, not as a claim of full historical EMBOSS equivalence.

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
- `pepwindow`
- `complex`
- `compseq`
- `geecee`
- `pepstats`
- `backtranseq`
- `backtranambig`
- `checktrans`
- `transeq`
- `getorf`
- `prettyseq`
- `tranalign`
- `extractseq`
- `cutseq`
- `union`
- `splitter`
- `water`
- `coderet`
- `featmerge`
- `featreport`
- `feattext`
- `infoseq`
- `wordcount`
- `cusp`
- `dan`

## Readiness Summary

### Complete / Ready

- Workspace version metadata is normalized to `1.0.0`.
- All 60 shipped methods are documented and present in the generated docs index.
- All 60 shipped methods have a checked-in validation stub.
- The cohort-level evidence report is present in both JSON and Markdown forms.
- The governance-alignment report is present in both JSON and Markdown forms.
- Release tasks now cover:
  - version-alignment verification
  - generated-doc/evidence freshness verification
  - governance-registry alignment reporting
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
  - `13` methods currently show `compared_evidence`
  - `42` methods currently show `executable_evidence`
  - `1` method shows `declared_evidence`
  - `4` methods remain at `documented_only`
- No shipped method yet records harvested historical evidence in the cohort
  report.
- Plotting remains intentionally narrow, with `charge` and `pepwindow` as the
  only governed plot-contract producers.
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

### Governance-to-registry alignment

- Status: `complete`
- Basis: the governance alignment report is generated at:
  - `docs/generated/validation/governance_alignment.json`
  - `docs/generated/governance_alignment.md`
  and currently shows `60/60` shipped methods mapped into the governance
  appendix.

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

- `4` methods remain at `documented_only` in the cohort report:
  - `degapseq`
  - `trimseq`
  - `complex`
  - `splitter`
- `newseq` is only at `declared_evidence`.
- `47` shipped methods still have one or more visible evidence gaps, because
  executable evidence without harvested or compared grounding is still treated
  as partial.

### Legacy harvesting remains partial

- The cohort report still records `0` methods with harvested legacy evidence.
- This is acceptable for an RC, but it remains the main credibility gap for a
  later post-`1.0.0` acceptance-hardening phase.

### Comparison-based acceptance remains partial

- The cohort report now records `13` methods with compared evidence.
- The comparison framework is therefore real and reusable, but it is still not
  populated across most of the shipped cohort.

### Plotting limitations

- `charge` and `pepwindow` are the only end-to-end plot-producing analytical
  paths.
- Plotting is intentionally narrow and R-owned; this remains in scope for
  `1.0.0` and is not treated as a blocker.

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
- The broader hardening phase did surface two small documentation/reporting
  defects that are now resolved:
  - generated index normalization now retains `cohort_validation`
  - shipped `runinfo` and `runget` are now represented in the governance
    appendix and in the generated governance-alignment report
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
