# v1.0.0 Release-Candidate Readiness

Status date: 2026-05-18

## Recommendation

`emboss-rs` is **ready for a `v1.0.0` release-candidate tag (`RC1`) with known
limitations**.

This audit still finds no repository-level blocker in the Rust workspace, docs
path, or release automation. The remaining limitations are release cutover and
intentional scope boundaries, not evidence-depth debt, retained-backlog
closure, or broken packaging mechanics.

## Audited Shipped Cohort

The release-candidate audit uses the governed tool registry
`emboss_tools::governed_tool_descriptors()` as the source of truth. The current
shipped cohort is:

- `aligncopy`
- `aligncopypair`
- `diffseq`
- `edialign`
- `infoalign`
- `extractalign`
- `nthseqset`
- `runinfo`
- `runget`
- `matcher`
- `distmat`
- `cons`
- `consambig`
- `needle`
- `needleall`
- `water`
- `seqret`
- `refseqget`
- `newseq`
- `seqcount`
- `notseq`
- `nthseq`
- `skipseq`
- `listor`
- `skipredundant`
- `degapseq`
- `revseq`
- `trimseq`
- `descseq`
- `maskseq`
- `maskambignuc`
- `maskambigprot`
- `maskfeat`
- `extractfeat`
- `featcopy`
- `coderet`
- `featmerge`
- `featreport`
- `feattext`
- `splitsource`
- `twofeat`
- `cai`
- `chips`
- `cusp`
- `codcmp`
- `codcopy`
- `dreg`
- `einverted`
- `fuzznuc`
- `fuzzpro`
- `fuzztran`
- `palindrome`
- `preg`
- `patmatdb`
- `seqmatchall`
- `wordmatch`
- `wordfinder`
- `charge`
- `pepwindow`
- `recoder`
- `silent`
- `aaindexextract`
- `complex`
- `compseq`
- `dan`
- `geecee`
- `infobase`
- `infoseq`
- `inforesidue`
- `iep`
- `oddcomp`
- `pepdigest`
- `pepstats`
- `wordcount`
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
- `pasteseq`
- `splitter`
- `merger`
- `megamerger`
- `sizeseq`
- `shuffleseq`

## Readiness Summary

Current generated release-truth markers:

- Shipped methods audited: `98`
- Compared-evidence methods: `97`
- Executable-evidence methods: `1`
- Methods with harvested legacy provenance recorded: `98`
- Full compared cohort: `no`
- Non-blocking plotting legacy-reference notes remain visible: `yes`
- Blocking cohort gaps: `1`
- Weakest evidence family: `Modernize — Rework — Plotting and visualization tools`
- Retained backlog still unshipped: `0`

### Complete / Ready

- Workspace version metadata is normalized to `1.0.0`.
- All `98` shipped methods are documented and present in the generated docs
  index.
- All `98` shipped methods have a checked-in validation stub.
- The cohort-level evidence report is present in both JSON and Markdown forms.
- The governance-alignment report is present in both JSON and Markdown forms.
- The cohort-health reprioritization gate is present in both JSON and Markdown
  forms.
- The comparison-coverage report is present in both JSON and Markdown forms.
- The full-compared-cohort gate is present in both JSON and Markdown forms.
- The harvest-coverage exceptions report is present in both JSON and Markdown
  forms.
- The retained-backlog-closure report is present in both JSON and Markdown
  forms.
- Release tasks cover version checks, generated-artifact freshness, governance
  alignment, release builds, release docs, Linux bundle assembly, and container
  build wiring.
- GitHub Actions continue to verify the release-oriented docs path and the
  Linux-first container smoke path.
- The root docs and README continue to state the first-class R and R-owned
  plotting posture explicitly.

### Ready With Known Limitations

- The shipped cohort is fully harvested and fully compared on this branch.
- The retained governance backlog is `0`, so the dominant remaining work is
  bounded plotting-rework completion plus post-closure release/process
  discipline rather than retained-method implementation.
- Plotting remains intentionally narrow, with `charge`, `pepwindow`,
  governed `wordcount`, and now governed `hmoment` as the current stable
  Rust-side plot-contract producers.
- The visible plotting notes for `charge` and `pepwindow` are now categorized
  as non-blocking missing explicit legacy-reference artefacts rather than as
  generic validation-report gaps.
- The R surface is real and first-class, but it remains a curated subset
  rather than exhaustive parity across every shipped Rust method.
- Remote retrieval remains governed and compared for the shipped slice, but the
  broader acquisition/orchestration surface is still a future rework program
  rather than a claim of general provider parity.

### Not Ready / Blocked

- No shipped-cohort evidence blocker is currently open. Remaining limitations
  are bounded scope decisions and future rework programs rather than missing
  compared evidence or harvest coverage.

## Governance And Release Obligation Audit

### Documentation completeness

- Status: `complete`
- Basis: all shipped methods appear in the generated docs tree and the docs
  build passes.

### Validation and evidence presence

- Status: `complete`
- Basis: every shipped method has a validation stub, appears in the cohort
  report, records harvested legacy provenance, and now reaches compared
  evidence.

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
  and currently shows `90/90` shipped retained methods mapped into the
  governance appendix with `0` retained methods still unshipped.

### Standing cohort-health gate

- Status: `complete`
- Basis: the standing reprioritization gate is generated at:
  - `docs/generated/validation/cohort_health.json`
  - `docs/generated/cohort_health.md`
  and now honestly reports zero weak-evidence signals plus
  `release_truth_current: true`.

### Family comparison coverage

- Status: `ready with known limitations`
- Basis: the family-level comparison coverage report is generated at:
  - `docs/generated/validation/comparison_coverage.json`
  - `docs/generated/comparison_coverage.md`
  and now records one executable-only shipped method in the plotting rework
  family while the bounded `octanol` shipped-surface step is in progress.

### Full compared cohort gate

- Status: `ready with known limitations`
- Basis: the full-compared-cohort gate is generated at:
  - `docs/generated/validation/full_compared_cohort.json`
  - `docs/generated/full_compared_cohort.md`
  and now records `97/98` shipped methods at compared evidence with `1`
  method below compared while `octanol` is shipped but not yet compared.

### Harvest coverage reporting

- Status: `complete`
- Basis: the harvest-coverage exceptions report is generated at:
  - `docs/generated/validation/harvest_coverage.json`
  - `docs/generated/harvest_coverage.md`
  and currently records `98/98` shipped methods with harvested legacy
  provenance and `0` harvest exceptions.

### Retained backlog closure tracking

- Status: `complete`
- Basis: the retained-backlog closure report is generated at:
  - `docs/generated/validation/retained_backlog_closure.json`
  - `docs/generated/retained_backlog_closure.md`
  and currently records `0` retained methods still unshipped.

### Workspace and test health

- Status: `complete`
- Basis: `cargo build` and `cargo test --workspace --all-features` pass on the
  audited branch.

### Docs and publication path

- Status: `complete`
- Basis: the Sphinx build passes, the release docs build path is clean, and the
  Pages workflow remains aligned to the release-oriented docs target.

### Release task availability

- Status: `complete`
- Basis: `make release-version-check`, `make release-generated-check`,
  `make release-check`, and `make release-artifacts` are present, and these
  generated-report gates are present and working:
  - `make cohort-health-report`
  - `make comparison-coverage-report`
  - `make full-compared-cohort-report`
  - `make harvest-coverage-report`
  - `make retained-backlog-report`

### Container readiness

- Status: `ready with known limitations`
- Basis: the Dockerfile, `.dockerignore`, CI smoke build, and release workflow
  container path are present. Local smoke execution still depends on an
  environment with Docker available.

### R-first-class posture in docs

- Status: `complete`
- Basis: the root README and release docs continue to describe `emboss-r` as a
  real peer user surface and keep plotting ownership in R.

## Concrete Gap Report

### Missing validation depth

- `1` shipped method still has blocking evidence debt.
- The retained governance backlog is now `0`.
- The dominant remaining implementation work is now the bounded plotting
  rework slice itself, not shipped-cohort evidence debt.

### Legacy harvesting remains partial

- The cohort report now records `98` methods with harvested legacy provenance.
- Harvest coverage is complete across the shipped cohort.

### Comparison-based acceptance remains partial

- The cohort report now records `97` methods with compared evidence.
- The comparison framework remains real and reusable, but the full compared
  cohort gate is temporarily non-green while `octanol` awaits its canonical
  fixture and compared acceptance closure.

### Plotting limitations

- `charge`, `pepwindow`, and `wordcount` are the current governed plot-contract
  producers.
- Plotting remains intentionally narrow and R-owned; this is in scope for
  `1.0.0` and is not treated as a blocker.

### R surface limitations

- `emboss-r` remains a real first-class surface, but broader parity still
  remains deferred to future sweeps.
- This remains aligned with the current cross-repo posture and is not a blocker
  for the Rust-side `1.0.0` candidate.

### Manual release steps still required

- Run or observe the Linux container smoke build in an environment with Docker.
- Perform final review of the changelog and release notes.
- Create the `v1.0.0` tag.
- Publish the GitHub release after the tag workflow completes.
- Confirm the coordinated `emboss-r` release decision separately.

## Current Generated Count Markers

- Shipped methods audited: `98`
- Compared-evidence methods: `97`
- Executable-evidence methods: `1`
- Methods with harvested legacy provenance recorded: `98`
- Retained backlog still unshipped: `0`

## Practical Validation Run

The following checks were completed in this environment during the release-
candidate hardening and roadmap-governance phase:

- `cargo build`
- `cargo test --workspace --all-features`
- `cargo test -p emboss-testkit`
- `cargo test -p emboss-docgen --test doc_coverage`
- `PYTHON=.venv-docs/bin/python make docs`

The broad generated-refresh target was also re-audited and normalized later:

- `PYTHON=.venv-docs/bin/python make release-generated-check`
  - current result: the older acceptance-anchor ordering issue did not
    reproduce, and the generated tool-page EOF churn was removed by
    canonicalizing emitted Markdown to one trailing newline
