# EMBOSS-RS Family-to-Tool Mapping Reference

Status: governance appendix and maintained reference registry

Canonical governance context:
[EMBOSS-RS Governance Manual](../emboss_rs_governance_manual.md)

This appendix ties the named tool families from the governance policy back to individual tools from the full scope matrix.

## How to read this appendix

- The **family name** and **default bucket** come from the governance policy.
- The **per-tool decision** comes from the full scope matrix and may override the family default where needed.
- Some mappings are **closest-fit mappings** rather than perfect historical taxonomies. This is intentional: the policy is a governance layer, not a claim that the original EMBOSS catalog was designed around these exact family boundaries.
- `complex` remains an explicit retain regardless of broader family heuristics.

## Summary

| Family | Default bucket | Historical/core tools mapped | Decision split |
|---|---:|---:|---|
| Core Retain — Basic sequence IO and conversion | Core Retain | 18 | Retain: 18 |
| Core Retain — Sequence editing and manipulation | Core Retain | 23 | Retain: 23 |
| Core Retain — Alignment read-write and post-processing | Core Retain | 18 | Retain: 13, Rework: 5 |
| Core Retain — Core sequence statistics and composition | Core Retain | 17 | Retain: 16, Rework: 1 |
| Core Retain — Simple motif, pattern, and regular-expression search | Core Retain | 12 | Retain: 12 |
| Core Retain — ORF and translation-adjacent utilities | Core Retain | 8 | Retain: 4, Rework: 4 |
| Modernize — Rework — Restriction-enzyme design and analysis | Modernize | 7 | Retain: 2, Rework: 5 |
| Modernize — Rework — Primer and assay-oriented search | Modernize | 5 | Rework: 3, Omit: 2 |
| Modernize — Rework — Plotting and visualization tools | Modernize | 28 | Rework: 28 |
| Modernize — Rework — Remote retrieval and archive acquisition | Modernize | 10 | Rework: 10 |
| Modernize — Rework — External database preparation helpers | Modernize | 5 | Rework: 5 |
| Modernize — Rework — Legacy prediction methods with enduring scientific value | Modernize | 21 | Rework: 21 |
| Modernize — Rework — Protein property and structural-summary utilities | Modernize | 6 | Retain: 2, Rework: 1, Omit: 3 |
| Modernize — Rework — Command discovery and help-navigation | Modernize | 6 | Rework: 3, Omit: 3 |
| Defer — Ontology command group | Defer | 24 | Omit: 24 |
| Defer — Specialized metadata and semantic lookup utilities | Defer | 18 | Rework: 18 |
| Exclude Permanently — ACD developer tooling | Exclude Permanently | 5 | Omit: 5 |
| Exclude Permanently — EMBOSS-era server-cache-registry plumbing | Exclude Permanently | 6 | Omit: 6 |
| Exclude Permanently — EMBOSS local database indexing administration | Exclude Permanently | 16 | Omit: 16 |
| Exclude Permanently — Wrapper-only compatibility commands | Exclude Permanently | 6 | Omit: 6 |
| Strategic Add — HMM and probabilistic homology workflows | Strategic Add | 0 | 0 historical core tools; 2 Add rows |
| Strategic Add — Modern archive-scale raw data ingestion | Strategic Add | 0 | 0 historical core tools; 4 Add rows |

## Post-retained-backlog closure reassessment

Status date: 2026-05-21

The retained backlog is now closed. This appendix therefore moves from
implementation-backlog triage to governance review of the remaining `Rework`
surface.

### Outcome of this reassessment

- No family is reclassified in this pass.
- The remaining `Rework` set is **reordered**, not narrowed or expanded.
- The reassessment now narrows actual implementation-planning attention to the
  top of the reordered list rather than treating the entire `Rework` surface as
  equally ripe.
- Any future bucket promotion or demotion should still follow the rules in
  `scope_and_tool_family_policy.md`, especially the stabilization and
  validation expectations for moving a `Rework` family toward operational core
  treatment.

### Why no family was silently promoted

Several rework families now have meaningful enabling infrastructure in place,
but they do not yet satisfy the promotion rule as a whole-family claim:

- plotting has a real Rust-to-R plot-contract seam, but only a narrow governed
  producer subset is validated today
- remote retrieval has governed provider seams and compared examples, but the
  wider acquisition surface still needs redesign decisions around scope,
  orchestration, and operational guarantees
- restriction-analysis has retained edit-design primitives, but the broader
  database and reporting surface remains explicitly modernize-first
- translation and alignment presentation-heavy rework members still need a
  deliberate redesign, not a compatibility-only port

For that reason, this pass preserves the bucket assignments and instead records
the next recommended rework order.

### Recommended next rework order

| Priority | Family | Reason for ordering after retained backlog closure |
|---:|---|---|
| 1 | Modernize — Rework — Plotting and visualization tools | The governed plot-contract seam, R rendering ownership, and validated `charge` / `pepwindow` / `wordcount` path now exist. This is the most mature platform seam for scaling a broader rework family without inventing new architecture. |
| 2 | Modernize — Rework — Remote retrieval and archive acquisition | Provider-backed acquisition seams, mocked compared evidence, and governed release wiring already exist. Additional tools can now be judged against a real operational model rather than a speculative one. |
| 3 | Modernize — Rework — Protein property and structural-summary utilities | The residue-property, hydropathy, pI, and digestion foundations are now present, which lowers implementation risk for adjacent rework members while keeping scientific scope bounded. |
| 4 | Core Retain — ORF and translation-adjacent utilities (rework members only) | The retained translation cohort is now shipped and substantially evidenced. Presentation-heavy or visualization-heavy members can be revisited from a stable translation substrate instead of as first-pass ports. |
| 5 | Core Retain — Alignment read-write and post-processing (rework members only) | The retained alignment substrate is now broad and heavily compared. Remaining wrapper-heritage members should only advance if they are redesigned around current Rust alignment outputs rather than historical UI compatibility. |
| 6 | Modernize — Rework — Restriction-enzyme design and analysis | `recoder` and `silent` establish a useful retained kernel, but the broader family still depends on deliberate redesign of enzyme sources, reporting, and workflow shape. |
| 7 | Modernize — Rework — Primer and assay-oriented search | The problem domain remains relevant, but it still lacks the same enabling substrate and evidence path now available to plotting, retrieval, and core analytics rework families. |
| 8 | Modernize — Rework — Legacy prediction methods with enduring scientific value | These methods remain scientifically interesting, but they are the least ready for quiet rollout because they demand the heaviest algorithm, dataset, and validation reconsideration. |
| 9 | Modernize — Rework — External database preparation helpers | These remain downstream of more user-facing retrieval and analysis priorities. They should not advance ahead of the workflows that would actually consume them. |
| 10 | Modernize — Rework — Command discovery and help-navigation | Important for polish, but not urgent while the governed docs, generated index, and release-truth reports already provide a strong discoverability baseline. |

### Post-full-compared-cohort planning consequence

The shipped retained cohort is now fully compared and fully harvested. That
changes the practical question from "which families still need retained-family
stabilization?" to "which rework family is mature enough to become the first
deliberate post-v1.x implementation program?"

This reassessment now picks the first implementation-planning candidate while
still preserving the broader shortlist:

- plotting is the chosen first post-v1.x family implementation program
  candidate because it combines the clearest governed platform seam with the
  lowest architectural ambiguity among the remaining rework families
- remote retrieval is the chosen explicit next alternative if plotting-first
  is later blocked, because its provider-aware seams and mocked compared
  evidence already describe the strongest remaining operational model
- protein-property rework remains the next fallback after those two because the
  scientific and implementation substrate is present, but the user-facing
  redesign pressure is lower

- plotting remains the default first candidate because it already has a clear
  governed computation-to-contract seam and a bounded rendering handoff
- remote retrieval remains the strongest alternative because its provider-aware
  seams and mocked compared evidence already describe a plausible operational
  model

Accordingly, future implementation planning should start from the top of this
shortlist rather than reopening family-wide reorder debates unless the
generated reports show a material regression or a new dependency.

### Post-summary-semantics recheck

The later post-closure summary cleanup did not change the ordering rationale.
The cleaned generated surface now shows:

- `gapped_method_count: 0`
- `weakest_evidence_family: null`
- `weak_evidence_method_count: 0`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`

Those results remove misleading summary noise, but they do not create a new
reason to displace plotting from the first implementation-program slot.
Remote retrieval therefore remains the explicit fallback rather than the new
lead candidate.

This is still a planning decision only. It does **not** authorize silent
surface widening, whole-family implementation claims, or bucket reassignment
without a later explicit rework program.

### Dedicated plotting rework sub-roadmap

The first plotting rework program should stay tightly inside the already-proven
Rust-to-R handoff seam. It should not begin by trying to absorb the entire
historical plotting and visualization family.

#### Bounded initial method subset

Phase 1 should be limited to methods that can plausibly reuse the current
typed-contract model without inventing a brand-new rendering architecture:

- `hmoment` — protein-sequence analytical profile with a governed line-plot
  handoff
- `octanol` — alternate hydropathy-style protein profile with a governed
  line-plot handoff
- `pepinfo` — multi-series protein property profile rendered through a bounded
  comparative line-plot contract

These are the preferred first candidates because they sit closest to the
already-governed `charge`, `pepwindow`, and `wordcount` seam:

- single-record or bounded-record analytical inputs
- explicit numerical series that Rust can compute deterministically
- output shapes that can remain table-first with typed plot-contract payloads
- no need for circular maps, trace visualization, feature-layout engines, or
  unrestricted alignment pretty-print rendering

Phase 1 should explicitly exclude the broader plotting family members that
would require a materially wider contract taxonomy or a heavier rendering
orchestration model, including:

- dotplot-style methods such as `dotmatcher`, `dotpath`, `dottup`, and
  `polydot`
- layout/diagram methods such as `cirdna`, `lindna`, `pepnet`, and `pepwheel`
- presentation-heavy formatted display methods such as `prettyplot`,
  `showfeat`, `showpep`, and `showseq`
- trace or specialized laboratory-plot methods such as `abiview` and `findkm`

#### Plot-contract evidence model

The plotting rework program should preserve the same evidence rules that now
govern the shipped retained cohort:

- each newly shipped method must emit a stable analytical table payload first
- any plot payload must be a typed contract derived from that same analytical
  run rather than a second independent computation path
- each method must gain governed autodoc, generated validation metadata, and a
  canonical checked-in plot-contract fixture
- compared evidence should validate both the analytical table and the canonical
  plot-contract JSON before a method is treated as complete

#### R-rendering handoff constraints

The program should preserve the existing cross-surface division of ownership:

- Rust owns deterministic computation, table emission, and typed contract
  production
- `emboss-r` owns graphical rendering, presentation tuning, and any
  user-facing figure ergonomics
- Phase 1 should not widen the Rust surface into figure rendering, ad hoc image
  generation, or renderer-specific layout policy
- new contract types should only be introduced when at least one bounded method
  requires them and the resulting shape remains reusable

#### Release-risk framing

The first plotting rework program should be judged as a narrow platform
expansion, not as a promise to revive the full historical plotting family.

- success for Phase 1 means proving that the existing seam scales beyond the
  current governed trio without destabilizing release-truth reporting
- failure conditions include contract sprawl, renderer-coupled Rust logic, or
  attempts to absorb diagrammatic/layout-heavy methods before the narrow
  analytical-profile path is stable
- if those risks become dominant during detailed planning, the repository
  should fall back to the already-recorded remote-retrieval alternative instead
  of switching families informally

#### Acceptance criteria for starting the program

The plotting rework program should not be treated as started until all of the
following are true:

- the family remains the explicitly chosen first `Rework` implementation
  program in this appendix and the roadmap
- the bounded Phase 1 scope remains limited to `hmoment`, `octanol`, and
  `pepinfo`
- no newly proposed method requires renderer-owned layout or diagram logic to
  move into Rust
- the expected contract shapes can still be expressed as analytical table
  output plus typed plot-contract payloads
- the release-truth surface remains in the current zero-burden state:
  - `full_compared_cohort == true`
  - `harvest_coverage_complete == true`
  - `retained_backlog_closed == true`
  - `gapped_method_count == 0`
  - `weakest_evidence_family == null`

If any of those conditions stop being true during pre-implementation planning,
the repository should pause the plotting program rather than widening scope
implicitly.

#### Acceptance criteria for completing Phase 1

The first plotting rework program should only be considered complete when all
bounded Phase 1 members are shipped and the platform constraints remain intact.
That means:

- `hmoment`, `octanol`, and `pepinfo` are present in the governed shipped
  registry
- each method has governed autodoc, generated docs, and generated validation
  metadata committed in the same repository-truth path as the existing shipped
  cohort
- each method emits:
  - deterministic analytical table output
  - deterministic typed plot-contract output from the same computation path
- each method has committed canonical plot-contract fixtures and compared
  evidence that validates both table and contract outputs
- no Phase 1 method requires Rust-side rendering logic or renderer-coupled
  layout policy
- no additional plotting-family members are implied as “effectively started”
  merely because the bounded Phase 1 slice shipped
- the release-truth gates continue to pass without special-case exceptions for
  the new plotting methods

If the repository can ship fewer than all three bounded methods while still
proving the seam is sound, that should be recorded as an explicit replanning
decision, not silently treated as “Phase 1 complete”.

#### Implementation-readiness checklist

Before the repository translates the plotting Phase 1 slice into a concrete
implementation sequence, the following checklist should be satisfied
explicitly:

- the plotting family is still the chosen first `Rework` implementation
  program in both this appendix and the roadmap
- the bounded Phase 1 set is still exactly:
  - `hmoment`
  - `octanol`
  - `pepinfo`
- the release-truth gates are still green in the current zero-burden state:
  - `full_compared_cohort == true`
  - `harvest_coverage_complete == true`
  - `retained_backlog_closed == true`
  - `gapped_method_count == 0`
  - `weakest_evidence_family == null`
- the current governed plotting seam remains intact and compared:
  - `charge`
  - `pepwindow`
  - `wordcount`
- no proposed Phase 1 method needs:
  - Rust-side rendering logic
  - renderer-coupled layout policy
  - a diagrammatic or presentation-heavy contract taxonomy
- each Phase 1 method can still be described as:
  - deterministic analytical table output first
  - typed plot-contract output from that same computation path
  - governed autodoc plus canonical contract fixtures plus compared evidence
- the retrieval fallback remains documented and operational as the next
  alternative if this checklist stops holding during sequencing

If any item in this checklist cannot be affirmed directly, the repository
should pause before beginning method-order sequencing and resolve the ambiguity
explicitly rather than letting the plotting program expand informally.

#### Phase 1 implementation sequence

The bounded plotting slice should be implemented in the following order:

1. `hmoment`
2. `octanol`
3. `pepinfo`

This order is preferred because it keeps the program method-associated and
evidence-aware:

- `hmoment` should go first because it is the narrowest extension of the
  current line-profile seam:
  - single analytical series
  - protein-input profile behavior closest to `pepwindow`
  - smallest likely contract expansion from the already-governed plotting trio
- `octanol` should go second because it can reuse the same single-series
  profile shape after the first method proves the path, while still testing a
  second analytical model rather than mere duplication
- `pepinfo` should go third because it is the first bounded Phase 1 method
  likely to require a broader comparative or multi-series contract surface, so
  it should only begin after the single-series path is already stable

The sequencing rules for implementation should remain explicit:

- each method should complete its full governed path before the next one is
  treated as started:
  - method-associated Rust computation
  - typed plot-contract emission
  - governed autodoc and generated docs
  - canonical contract fixtures
  - compared evidence for both table and contract outputs
- if `hmoment` cannot stay inside the current seam cleanly, the repository
  should pause and reassess before beginning `octanol`
- if `octanol` requires materially new contract taxonomy beyond the single-line
  profile path, the repository should pause before beginning `pepinfo`
- `pepinfo` should only be treated as in scope for this Phase 1 if its
  comparative line-series output still avoids renderer-coupled layout policy

This sequence establishes implementation order only. It does not broaden the
bounded plotting subset or imply that any later plotting-family members are
already approved.

#### Reassessment of the first method

After writing the bounded implementation sequence down explicitly, `hmoment`
still remains the best first plotting method.

The reassessment does not change the Phase 1 order because:

- `hmoment` is still the closest extension of the existing governed plotting
  seam:
  - protein-sequence input
  - deterministic numerical profile
  - single-series line-contract shape
- `octanol` remains a better second method than a first one because it tests a
  second analytical profile after the single-series path is proven, rather
  than reducing the uncertainty of the first expansion step
- `pepinfo` still remains the least suitable first method because it is the
  first bounded candidate likely to force comparative or multi-series contract
  decisions

So the repository should treat `hmoment` as the explicit lead method for the
first code-bearing plotting patch unless a later method-level acceptance review
uncovers a concrete blocker that invalidates this reasoning.

#### `hmoment` method-level acceptance criteria

Before code changes begin for the first plotting rework method, `hmoment`
should have explicit method-level acceptance criteria recorded as follows.

##### Analytical output expectations

- the method should accept bounded protein-sequence input only
- Rust should compute a deterministic hydrophobic-moment profile rather than
  delegating any analytical calculation to the renderer
- the primary output should be a stable analytical table with one row per
  emitted profile window
- each row should be sufficient to reconstruct the plotted line without a
  second computation path
- the analytical output should stay within the same table-first contract style
  already used by `charge` and `pepwindow`

##### Typed contract expectations

- the same run should emit a deterministic typed line-plot contract derived
  from the analytical table output
- the contract should stay single-series unless a concrete analytical need
  proves otherwise during implementation
- the contract shape should avoid renderer-coupled styling or layout policy
- Rust should own only:
  - numerical series construction
  - axis/domain metadata needed for faithful rendering
  - stable contract serialization
- `emboss-r` should remain responsible for presentation choices and figure
  rendering

##### Fixture and evidence expectations

- the method must gain governed autodoc before it is treated as shipped
- the method must gain generated docs and generated validation metadata in the
  same governed path as the retained cohort
- a canonical checked-in plot-contract fixture must be committed
- compared evidence must validate both:
  - the analytical table output
  - the canonical plot-contract JSON
- the method should not be considered complete on executable-only evidence

##### Explicit non-goals

- no Rust-side figure rendering
- no diagram, wheel, map, or presentation-heavy layout behavior
- no implicit widening from single-series analytical profile work into the
  broader plotting family
- no reliance on hidden renderer defaults to fill in missing analytical
  metadata
- no second independent computation path used only for plotting

If `hmoment` cannot satisfy these criteria while remaining a narrow
single-series profile method, the repository should pause and reassess before
starting the first implementation patch.

#### Exact start conditions for the first `hmoment` implementation patch

The first code-bearing plotting patch should not begin until all of the
following are treated as explicit start conditions:

- the active family decision still remains:
  - plotting first
  - remote retrieval second
  - protein-property rework third
- the bounded plotting Phase 1 order still remains:
  1. `hmoment`
  2. `octanol`
  3. `pepinfo`
- the release-truth surface still remains in the current zero-burden state:
  - `full_compared_cohort == true`
  - `harvest_coverage_complete == true`
  - `retained_backlog_closed == true`
  - `gapped_method_count == 0`
  - `weakest_evidence_family == null`
- the patch scope remains limited to `hmoment` and the smallest shared support
  needed for:
  - deterministic analytical computation
  - typed single-series plot-contract emission
  - governed docs and validation plumbing
- the patch does not widen into:
  - `octanol`
  - `pepinfo`
  - broader plot-contract taxonomies
  - Rust-side rendering behavior
- the patch is expected to land with all method-level governed surfaces, not
  as a half-start:
  - method-associated Rust implementation
  - registry/service exposure
  - governed autodoc contract
  - generated docs and validation metadata
  - canonical plot-contract fixture
  - compared evidence for both table and contract outputs
- if any required support code spans multiple methods, it must still be named
  and scoped as a narrow plotting-profile helper rather than a broad shared
  plotting framework

If any of these start conditions cease to hold before code changes begin, the
repository should re-open planning rather than starting the first patch under a
looser scope.

#### Post-ship reassessment of the bounded `hmoment` slice

After shipping `hmoment` and closing its compared-evidence follow-on, the
repository should treat the first plotting slice as having passed its
post-ship reassessment.

That conclusion is explicit rather than inferred because the shipped slice
stayed inside the boundaries set by the plotting-first program:

- the analytical implementation remained method-associated and narrow:
  - one bounded core helper for the deterministic hydrophobic-moment profile
  - one method-specific tool implementation for the line-profile and typed
    plot-contract emission
- the governed shipped surface remained narrow:
  - exactly one new shipped method: `hmoment`
  - no widening into `octanol`, `pepinfo`, or broader plotting-family members
- the typed contract seam remained narrow:
  - single-series line contract only
  - no renderer-coupled styling or layout policy added in Rust
  - no broader plot-contract taxonomy introduced
- the evidence path closed completely for the shipped method:
  - committed analytical fixture
  - committed canonical plot-contract fixture
  - compared acceptance evidence for both table and contract outputs

The release-truth surface also remained clean after shipping the method:

- shipped methods: `97`
- compared evidence: `97`
- executable evidence: `0`
- harvested legacy provenance present: `97`
- `full_compared_cohort == true`
- `harvest_coverage_complete == true`
- `retained_backlog_closed == true`
- `gapped_method_count == 0`
- `weakest_evidence_family == null`
- `release_truth_current == true`

No concrete signal emerged that would justify pausing plotting in favor of the
remote-retrieval fallback:

- no contract sprawl appeared
- no renderer-coupled pressure appeared
- no release-truth exception was needed
- no family-order ambiguity re-opened

So the repository should treat plotting-first as still valid and may proceed to
the bounded `octanol` planning tier without reopening the higher-level family
selection question first.

#### Bounded `octanol` implementation tier after the `hmoment` reassessment

Now that the first plotting slice has shipped and passed explicit
reassessment, the repository should map the second plotting method as a
bounded tier before writing any `octanol` code.

That tier should stay parallel to the `hmoment` sequence rather than widening
the plotting program informally:

1. capture `octanol` method-level acceptance criteria
2. capture exact start conditions for the first `octanol` implementation patch
3. implement the bounded analytical core for `octanol`
4. add the typed `octanol` plot-contract emission path
5. expose `octanol` through the governed shipped surface
6. add canonical analytical and plot-contract fixtures plus compared evidence
7. re-run the full release-truth surface after shipping `octanol`
8. reassess the shipped `octanol` slice before any `pepinfo` work is mapped

The bounded `octanol` tier should preserve the same architectural constraints
already proven by `hmoment`:

- method-associated implementation only
- single-series line-contract output unless implementation proves broader shape
  is analytically required
- no widening into `pepinfo`
- no renderer-coupled Rust logic
- no broader plot-contract taxonomy unless the analytical needs of `octanol`
  make that pressure concrete enough to stop and reassess

So the next code-bearing plotting work should not begin as a generic “continue
plotting” step. It should begin as this explicit bounded `octanol` tier.

#### `octanol` method-level acceptance criteria

Before code changes begin for the second plotting rework method, `octanol`
should have explicit method-level acceptance criteria recorded as follows.

##### Analytical output expectations

- the method should accept bounded protein-sequence input only
- Rust should compute a deterministic `octanol` profile directly rather than
  delegating analytical work to the renderer
- the analytical model should be recorded honestly as a distinct hydropathy-
  style profile rather than being treated as a cosmetic variant of
  `pepwindow`
- the primary output should be a stable analytical table with one row per
  emitted profile window
- each row should remain sufficient to reconstruct the plotted line without a
  second computation path
- the analytical output should stay within the same table-first contract style
  already used by `charge`, `pepwindow`, and `hmoment`

##### Typed contract expectations

- the same run should emit a deterministic typed line-plot contract derived
  from the analytical table output
- the contract should stay single-series unless a concrete analytical need
  proves otherwise during implementation
- the contract shape should avoid renderer-coupled styling or layout policy
- Rust should own only:
  - numerical series construction
  - axis/domain metadata needed for faithful rendering
  - stable contract serialization
- `emboss-r` should remain responsible for presentation choices and figure
  rendering

##### Fixture and evidence expectations

- the method must gain governed autodoc before it is treated as shipped
- the method must gain generated docs and generated validation metadata in the
  same governed path as the shipped cohort
- a canonical checked-in plot-contract fixture must be committed
- compared evidence must validate both:
  - the analytical table output
  - the canonical plot-contract JSON
- the method should not be considered complete on executable-only evidence

##### Explicit non-goals

- no Rust-side figure rendering
- no implicit widening into multi-series comparative plotting
- no diagram, wheel, map, or presentation-heavy layout behavior
- no inference that `pepinfo` is already in scope merely because `octanol`
  shipped
- no broader plot-contract taxonomy unless the distinct analytical needs of
  `octanol` make that pressure concrete enough to stop and reassess

If `octanol` cannot satisfy these criteria while remaining a narrow
single-series profile method, the repository should pause and reassess before
starting the second plotting implementation patch.

#### Exact start conditions for the first `octanol` implementation patch

The first code-bearing `octanol` patch should not begin until all of the
following are treated as explicit start conditions:

- the active family decision still remains:
  - plotting first
  - remote retrieval second
  - protein-property rework third
- the bounded plotting Phase 1 order still remains:
  1. `hmoment`
  2. `octanol`
  3. `pepinfo`
- the shipped `hmoment` slice has already passed explicit post-ship
  reassessment without reopening family-selection ambiguity
- the release-truth surface still remains in the current zero-burden state:
  - `full_compared_cohort == true`
  - `harvest_coverage_complete == true`
  - `retained_backlog_closed == true`
  - `gapped_method_count == 0`
  - `weakest_evidence_family == null`
  - `release_truth_current == true`
- the patch scope remains limited to `octanol` and the smallest shared support
  needed for:
  - deterministic analytical computation
  - typed single-series plot-contract emission
  - governed docs and validation plumbing
- the patch does not widen into:
  - `pepinfo`
  - broader plot-contract taxonomies
  - Rust-side rendering behavior
  - a generalized plotting framework
- the patch is expected to land with all method-level governed surfaces, not
  as a half-start:
  - method-associated Rust implementation
  - registry/service exposure
  - governed autodoc contract
  - generated docs and validation metadata
  - canonical analytical and plot-contract fixtures
  - compared evidence for both table and contract outputs
- if any required support code spans more than one plotting method, it must
  still be named and scoped as a narrow plotting-profile helper rather than a
  broad shared plotting framework

If any of these start conditions cease to hold before code changes begin, the
repository should re-open planning rather than starting the second plotting
patch under a looser scope.

#### Post-ship reassessment of the bounded `octanol` slice

After shipping `octanol`, closing its compared-evidence follow-on, and rerunning
the full release-truth surface, the repository should treat the second plotting
slice as having passed its post-ship reassessment.

That conclusion is explicit rather than inferred because the shipped slice
stayed inside the same narrow boundaries already proven by `hmoment`:

- the analytical implementation remained method-associated and narrow:
  - one bounded core helper for the deterministic `octanol` profile
  - one method-specific tool implementation for the analytical table and typed
    line-contract emission
- the governed shipped surface remained narrow:
  - exactly one additional shipped method: `octanol`
  - no widening into `pepinfo` or other plotting-family members
- the typed contract seam remained narrow:
  - single-series line contract only
  - no renderer-coupled styling or layout policy added in Rust
  - no broader plot-contract taxonomy introduced
- the evidence path closed completely for the shipped method:
  - committed analytical fixture
  - committed canonical plot-contract fixture
  - compared acceptance evidence for both table and contract outputs

The release-truth surface also remained clean after shipping the method:

- shipped methods: `98`
- compared evidence: `98`
- executable evidence: `0`
- harvested legacy provenance present: `98`
- `full_compared_cohort == true`
- `harvest_coverage_complete == true`
- `retained_backlog_closed == true`
- `gapped_method_count == 0`
- `weakest_evidence_family == null`
- `release_truth_current == true`

No concrete signal emerged that would justify pausing plotting in favor of the
remote-retrieval fallback:

- no contract sprawl appeared
- no renderer-coupled pressure appeared
- no plotting-framework pressure appeared
- no release-truth exception was needed

So the repository should treat plotting-first as still valid after the second
shipped plotting slice and may proceed to the bounded `pepinfo` planning gate
without reopening the higher-level family selection question first.

#### Bounded `pepinfo` implementation tier after the `octanol` reassessment

Now that the second plotting slice has shipped and passed explicit
reassessment, the repository should map the third bounded Phase 1 method as an
explicit `pepinfo` tier before writing any `pepinfo` code.

That tier should remain method-associated and stop short of any broader
plotting-family expansion:

1. capture `pepinfo` method-level acceptance criteria
2. capture exact start conditions for the first `pepinfo` implementation patch
3. implement the bounded analytical core for `pepinfo`
4. add the typed `pepinfo` plot-contract emission path
5. expose `pepinfo` through the governed shipped surface
6. add canonical analytical and plot-contract fixtures plus compared evidence
7. re-run the full release-truth surface after shipping `pepinfo`
8. reassess the shipped `pepinfo` slice before any broader plotting expansion
   is mapped

The bounded `pepinfo` tier should preserve the same architectural constraints
already proven by `hmoment` and `octanol`, while acknowledging that `pepinfo`
is the first likely pressure point for a broader contract shape:

- method-associated implementation only
- Rust owns deterministic analytical computation and typed contract emission
- no Rust-side rendering or presentation-heavy layout behavior
- no silent widening into a generic plotting framework
- no broader plot-contract taxonomy unless the analytical needs of `pepinfo`
  make that pressure concrete enough to stop and reassess before further work

So the next code-bearing plotting work should not begin as a vague “expand
plotting again” step. It should begin as this explicit bounded `pepinfo` tier,
with the retrieval fallback still preserved as the next alternative if the seam
widens too far.

#### `pepinfo` method-level acceptance criteria

Before code changes begin for the third bounded plotting method, `pepinfo`
should have explicit method-level acceptance criteria recorded as follows.

##### Analytical output expectations

- the method should accept bounded protein-sequence input only
- Rust should compute the analytical `pepinfo` profile directly rather than
  delegating analytical work to the renderer
- the analytical model should be recorded honestly as a bounded multi-property
  protein profile rather than as a cosmetic extension of `hmoment` or
  `octanol`
- the primary output should remain a stable analytical table with one row per
  emitted window and explicit property columns sufficient to reconstruct the
  emitted plotted series
- the analytical output should stay within the same table-first contract style
  already used by the governed plotting seam, even if `pepinfo` needs more
  than one numerical series

##### Typed contract expectations

- the same run should emit a deterministic typed plot contract derived from the
  analytical table output
- the contract may expand beyond the single-series line shape only if the
  analytical needs of `pepinfo` make that necessary and explicit
- any multi-series contract shape must still avoid renderer-coupled styling or
  layout policy
- Rust should own only:
  - numerical series construction
  - axis or domain metadata needed for faithful rendering
  - stable contract serialization
- `emboss-r` should remain responsible for presentation choices and figure
  rendering

##### Fixture and evidence expectations

- the method must gain governed autodoc before it is treated as shipped
- the method must gain generated docs and generated validation metadata in the
  same governed path as the shipped cohort
- canonical checked-in analytical and plot-contract fixtures must be committed
- compared evidence must validate both:
  - the analytical table output
  - the canonical plot-contract JSON
- the method should not be considered complete on executable-only evidence

##### Explicit non-goals

- no Rust-side figure rendering
- no silent widening into a generic plotting framework
- no diagram, wheel, map, or presentation-heavy layout behavior
- no inference that broader plotting-family members are already in scope merely
  because `pepinfo` ships
- no broader plot-contract taxonomy unless the distinct analytical needs of
  `pepinfo` make that pressure concrete enough to stop and reassess

If `pepinfo` cannot satisfy these criteria while remaining a bounded
method-associated plotting method, the repository should pause and reassess
before starting the next code-bearing plotting patch.

#### Exact start conditions for the first `pepinfo` implementation patch

The first code-bearing `pepinfo` patch should not begin until all of the
following are treated as explicit start conditions:

- the active family decision still remains:
  - plotting first
  - remote retrieval second
  - protein-property rework third
- the bounded plotting Phase 1 order still remains:
  1. `hmoment`
  2. `octanol`
  3. `pepinfo`
- both shipped plotting slices have already passed explicit post-ship
  reassessment without reopening family-selection ambiguity:
  - `hmoment`
  - `octanol`
- the release-truth surface still remains in the current zero-burden state:
  - `full_compared_cohort == true`
  - `harvest_coverage_complete == true`
  - `retained_backlog_closed == true`
  - `gapped_method_count == 0`
  - `weakest_evidence_family == null`
  - `release_truth_current == true`
- the patch scope remains limited to `pepinfo` and the smallest shared support
  needed for:
  - deterministic analytical computation
  - typed plot-contract emission
  - governed docs and validation plumbing
- the patch does not widen into:
  - broader plotting-family members beyond `pepinfo`
  - Rust-side rendering behavior
  - a generalized plotting framework
  - broader plot-contract taxonomies unless `pepinfo` itself makes that
    pressure concrete enough to stop and reassess
- the patch is expected to land with all method-level governed surfaces, not
  as a half-start:
  - method-associated Rust implementation
  - registry and service exposure
  - governed autodoc contract
  - generated docs and validation metadata
  - canonical analytical and plot-contract fixtures
  - compared evidence for both table and contract outputs

If any of these start conditions cease to hold before code changes begin, the
repository should re-open planning rather than starting the third plotting
patch under a looser scope.

#### Post-ship reassessment of the bounded `pepinfo` slice

After shipping `pepinfo`, closing its compared-evidence follow-on, and rerunning
the full release-truth surface, the repository should treat the third bounded
plotting slice as having passed its post-ship reassessment.

That conclusion is explicit rather than inferred because the shipped slice
stayed inside the same bounded Phase 1 rules, even though it was the first
method to require more than one governed series:

- the analytical implementation remained method-associated and narrow:
  - one bounded core helper for the deterministic `pepinfo` profile
  - one method-specific tool implementation for the analytical table and typed
    multi-series contract emission
- the governed shipped surface remained narrow:
  - exactly one additional shipped method: `pepinfo`
  - no widening into broader plotting-family members
- the typed contract seam broadened only as far as `pepinfo` itself required:
  - multi-series contract, but still table-derived
  - no renderer-coupled styling or layout policy added in Rust
  - no generalized plotting framework introduced
  - no broader contract taxonomy beyond the bounded needs of this method
- the evidence path closed completely for the shipped method:
  - committed analytical fixture
  - committed canonical plot-contract fixture
  - compared acceptance evidence for both table and contract outputs

The release-truth surface also remained clean after shipping the method:

- shipped methods: `99`
- compared evidence: `99`
- executable evidence: `0`
- harvested legacy provenance present: `99`
- `full_compared_cohort == true`
- `harvest_coverage_complete == true`
- `retained_backlog_closed == true`
- `gapped_method_count == 0`
- `weakest_evidence_family == null`
- `release_truth_current == true`

No concrete signal emerged that would justify pausing plotting in favor of the
remote-retrieval fallback:

- the first multi-series method still stayed method-associated and bounded
- no contract sprawl beyond the bounded `pepinfo` need appeared
- no renderer-coupled pressure appeared
- no plotting-framework pressure appeared
- no release-truth exception was needed

So the repository should treat bounded plotting Phase 1 as having passed its
full three-method reassessment and may proceed to the next explicit planning
decision rather than switching families by inertia.

### Dedicated remote-retrieval fallback sub-roadmap

If plotting is later blocked, the remote-retrieval family should become the
next bounded implementation program without reopening the broader family order.
That fallback should stay inside the provider-aware seams already proven by the
shipped `refseqget`, `runinfo`, and `runget` slice.

#### Bounded initial method subset

The retrieval fallback Phase 1 should focus on methods that extend the existing
retrieval and normalization substrate without immediately forcing a broad
provider-orchestration redesign:

- `seqretsetall` — bounded many-set retrieval/write workflow built on the same
  normalized sequence-return path as `seqret`
- `seqretsplit` — deterministic split-output sequence return path built on the
  same governed retrieval substrate
- `infoassembly` — metadata-first assembly information path that can reuse the
  existing provider-aware reporting model before full data acquisition is
  widened

These are the preferred fallback candidates because they are the closest
extensions of the current governed retrieval slice:

- they reuse sequence-return or metadata-reporting patterns that already exist
- they can stay deterministic under mocked provider fixtures and managed local
  assets
- they avoid starting with the heaviest orchestration or “search every
  configured source” redesign questions

The fallback Phase 1 should explicitly exclude broader retrieval members that
would demand a larger operational redesign at the start, including:

- `assemblyget`, which would widen the acquisition/orchestration surface before
  the metadata and return-path extensions are settled
- `whichdb`, whose old “search all databases” model needs a deliberate modern
  provider-discovery redesign rather than a compatibility-first port
- `entret`, whose historical flatfile and remote entry semantics should not be
  revived without a clear modern source model

#### Evidence model

The retrieval fallback program should preserve the same evidence discipline used
for the current governed retrieval slice:

- each new method must have a deterministic mocked-provider or managed-asset
  validation path
- compared evidence should validate normalized returned sequence or metadata
  outputs rather than only request orchestration intent
- governed autodoc, generated validation metadata, and committed expected
  outputs remain required before a method is treated as complete
- release-truth reporting should continue to describe provider seams honestly
  rather than implying generic remote-provider parity

#### Provider and orchestration constraints

The fallback program should preserve the current operational boundaries:

- Rust owns retrieval orchestration, normalization, and stable returned data
  artifacts
- provider integrations remain explicit seams with mocked or managed validation
  inputs, not hidden live-network dependencies inside the core validation path
- Phase 1 should not introduce best-effort provider fallback chains, silent
  source switching, or implicit online test dependencies
- modern retrieval scope should stay provider-aware and explicit rather than
  reproducing the historical EMBOSS “configured database universe” abstraction

#### Release-risk framing

The remote-retrieval fallback should be treated as a narrow operational
expansion, not as a claim of general archive and accession parity.

- success for fallback Phase 1 means extending the existing governed retrieval
  slice through a few well-bounded methods without destabilizing release-truth
  reporting or offline validation
- failure conditions include hidden network dependence, unclear provider
  precedence, or unbounded compatibility pressure around historical database
  behaviors
- if those risks dominate the fallback planning pass, the repository should
  stop and reassess rather than informally widening the retrieval family

#### Reconfirmation after plotting sequencing

After fixing the bounded plotting implementation order and confirming
`hmoment` as the explicit lead method, the retrieval fallback remains ready
for a clean switch if plotting becomes architecturally noisy.

That conclusion still holds because:

- the fallback Phase 1 remains bounded to:
  - `seqretsetall`
  - `seqretsplit`
  - `infoassembly`
- the fallback still has a distinct operational model instead of depending on
  the plotting contract seam
- its evidence path remains explicit and deterministic:
  - mocked-provider or managed-asset validation
  - compared evidence on normalized returned sequence or metadata outputs
- its exclusions remain clear enough to prevent an informal expansion into
  broad provider search or acquisition parity work

#### Reconfirmation after the second shipped plotting slice

After shipping both `hmoment` and `octanol`, closing their compared-evidence
follow-ons, and rerunning the full release-truth surface, the remote-retrieval
fallback should still be treated as ready but inactive.

That conclusion remains explicit because the second shipped plotting slice did
not create the kind of pressure that would justify activating the fallback:

- the plotting seam still remained narrow:
  - both shipped plotting methods stayed method-associated
  - both shipped plotting methods stayed single-series and table-derived
  - no renderer-coupled logic or broader plot-contract taxonomy emerged
- the release-truth surface stayed clean while plotting advanced:
  - shipped methods: `98`
  - compared evidence: `98`
  - harvested legacy provenance present: `98`
  - `full_compared_cohort == true`
  - `harvest_coverage_complete == true`
  - `release_truth_current == true`
- the retrieval fallback itself still remains bounded and operationally
  distinct:
  - Phase 1 remains limited to `seqretsetall`, `seqretsplit`, and
    `infoassembly`
  - mocked-provider or managed-asset validation remains the expected evidence
    model
  - the fallback still does not depend on the plotting contract seam or on
    live-network validation

So the repository should preserve the same fallback ordering after the second
shipped plotting method:

- plotting remains the active first implementation program
- remote retrieval remains the explicit next alternative if plotting later
  becomes noisy
- no family-order change is justified at this checkpoint

So no reordering or widening is needed here. If plotting is later blocked by
contract sprawl or renderer-coupled pressure, the repository can still switch
to the retrieval fallback without reopening family-selection ambiguity first.

#### Reconfirmation after the third shipped plotting slice

After shipping `pepinfo`, closing its compared-evidence follow-on, and
rerunning the full release-truth surface, the remote-retrieval fallback should
still be treated as ready but inactive.

That conclusion remains explicit because the third bounded plotting slice did
not create the kind of pressure that would justify activating the fallback:

- the plotting seam still remained bounded enough after Phase 1 completion:
  - all three shipped plotting methods stayed method-associated
  - the first multi-series method still remained table-derived and
    renderer-agnostic
  - no generic plotting framework or broader contract taxonomy emerged
- the release-truth surface stayed clean while plotting advanced through the
  full bounded Phase 1:
  - shipped methods: `99`
  - compared evidence: `99`
  - harvested legacy provenance present: `99`
  - `full_compared_cohort == true`
  - `harvest_coverage_complete == true`
  - `retained_backlog_closed == true`
  - `release_truth_current == true`
- the retrieval fallback itself still remains bounded and operationally
  distinct:
  - Phase 1 remains limited to `seqretsetall`, `seqretsplit`, and
    `infoassembly`
  - mocked-provider or managed-asset validation remains the expected evidence
    model
  - the fallback still does not depend on the plotting contract seam or on
    live-network validation

So the repository should preserve the same fallback ordering after the full
bounded plotting Phase 1:

- plotting remains the active first implementation program
- remote retrieval remains the explicit next alternative if plotting later
  becomes noisy
- no family-order change is justified at this checkpoint

So no switch, reordering, or widening is needed here. If later plotting work
crosses the bounded Phase 1 seam into genuine contract sprawl or
renderer-coupled pressure, the repository can still activate the retrieval
fallback without reopening the broader family-selection decision first.

#### Explicit post-Phase-1 family decision

After the full bounded plotting Phase 1 has shipped and passed explicit
reassessment, the repository should make the family decision explicit rather
than leaving it implied by the earlier checkpoints.

The correct post-Phase-1 decision is:

- continue plotting-family planning as the active path
- keep remote retrieval as the explicit prepared fallback
- do not switch families at this checkpoint

That decision is justified because:

- the full bounded plotting Phase 1 stayed inside the intended architectural
  seam:
  - `hmoment` and `octanol` stayed single-series and method-associated
  - `pepinfo` introduced the first multi-series case without widening into a
    generic plotting framework
  - all three methods remained table-derived and renderer-agnostic
- the release-truth surface remained fully green through the full Phase 1:
  - shipped methods: `99`
  - compared evidence: `99`
  - harvested legacy provenance present: `99`
  - `full_compared_cohort == true`
  - `harvest_coverage_complete == true`
  - `retained_backlog_closed == true`
  - `release_truth_current == true`
- the remote-retrieval fallback remains strong, but it is not being activated
  in response to a real plotting failure condition here:
  - no contract sprawl
  - no renderer-coupled pressure
  - no release-truth exception
  - no family-order ambiguity

So the next planning task should be to map the next bounded post-Phase-1 gate
explicitly, not to switch implementation families by inertia.

#### Next bounded post-Phase-1 gate

Because the bounded plotting Phase 1 passed, the next gate should stay inside
plotting-family planning, but it should still be a planning gate rather than a
direct code-start decision.

The next bounded post-Phase-1 gate should therefore be:

- a Phase 2 candidate-selection and seam-compatibility gate for the remaining
  plotting-family methods

That gate should happen before any further plotting implementation starts, and
it should answer one bounded question:

- does at least one remaining plotting-family method still fit the proven
  method-associated, table-derived, renderer-agnostic seam closely enough to
  justify another bounded plotting tier?

The gate should explicitly do all of the following:

- inventory the remaining plotting-family methods not already covered by:
  - the preexisting governed seam: `charge`, `pepwindow`, `wordcount`
  - the bounded Phase 1 slice: `hmoment`, `octanol`, `pepinfo`
- classify the remaining methods by seam pressure:
  - likely seam-compatible
  - requires broader contract taxonomy
  - requires dotplot-style or layout-heavy behavior
  - requires specialized laboratory-plot or trace behavior
- decide whether plotting still has a bounded Phase 2 candidate that is:
  - method-associated
  - table-first
  - typed-contract-friendly
  - renderer-agnostic in Rust
- if such a candidate exists:
  - map exactly one bounded next-method tier before code starts
- if no such candidate exists:
  - stop plotting-family continuation cleanly and activate the already-mapped
    retrieval fallback planning path instead

The gate should **not** do any of the following:

- start implementation for a new plotting method
- imply that all remaining plotting-family methods are in scope
- widen the seam into a generic plotting framework by default
- demote retrieval fallback readiness just because plotting Phase 1 succeeded

So the next bounded post-Phase-1 gate is not “continue plotting” in the
abstract. It is “prove that a bounded Phase 2 plotting candidate still exists,
or switch planning cleanly to retrieval fallback.”

#### Phase 2 candidate-pool inventory

Before any Phase 2 classification or candidate selection begins, the remaining
plotting-family method pool should be stated explicitly.

The plotting-family methods already covered by the current proven seam are:

- preexisting governed seam:
  - `charge`
  - `pepwindow`
  - `wordcount`
- bounded Phase 1 shipped slice:
  - `hmoment`
  - `octanol`
  - `pepinfo`

Important scope note:

- `wordcount` is part of the governed plot-contract seam, but it is not itself
  a member of the historical plotting-family mapping
- the actual plotting-family remainder set therefore excludes only the five
  plotting-family members already covered inside that family:
  - `charge`
  - `pepwindow`
  - `hmoment`
  - `octanol`
  - `pepinfo`

So the explicit Phase 2 plotting-family candidate pool is the remaining `23`
plotting-family methods:

- `abiview`
- `banana`
- `chaos`
- `cirdna`
- `cpgplot`
- `density`
- `dotmatcher`
- `dotpath`
- `dottup`
- `findkm`
- `isochore`
- `lindna`
- `pepnet`
- `pepwheel`
- `pepwindowall`
- `plotcon`
- `polydot`
- `prettyplot`
- `showfeat`
- `showpep`
- `showseq`
- `syco`
- `wobble`

This section is inventory only. It does not yet classify seam pressure, select
the next candidate, or imply that all of these methods remain plausible Phase 2
continuations.

#### Phase 2 seam-pressure classification

The explicit Phase 2 candidate pool can now be split into bounded seam-pressure
buckets without yet choosing the next implementation candidate.

##### Likely seam-compatible

These methods still look closest to the proven bounded seam because they appear
most likely to remain analytical, table-first, and renderer-agnostic with a
method-associated Rust computation path:

- `banana`
- `cpgplot`
- `density`
- `isochore`
- `syco`
- `wobble`

##### Requires broader contract taxonomy

These methods still look biologically relevant, but they appear more likely to
need a broader typed-contract vocabulary than the current bounded line-profile
seam:

- `chaos`
- `pepwindowall`
- `plotcon`

##### Dotplot-style or comparative-matrix heavy

These methods lean directly into comparative matrix or dotplot behavior, which
is outside the currently proven bounded seam:

- `dotmatcher`
- `dotpath`
- `dottup`
- `polydot`

##### Diagram, layout, or presentation-heavy

These methods appear to depend more on structural layout, diagram rendering, or
pretty-display behavior than on the narrow analytical-profile seam proven so
far:

- `cirdna`
- `lindna`
- `pepnet`
- `pepwheel`
- `prettyplot`
- `showfeat`
- `showpep`
- `showseq`

##### Specialized laboratory-trace or kinetic plotting

These methods appear to need specialized trace or laboratory-kinetic plot
handling rather than the current general analytical-profile seam:

- `abiview`
- `findkm`

This classification is still governance-only. It does not yet choose the next
Phase 2 candidate, and it does not imply that every method in the seam-
compatible bucket is equally suitable.

#### Phase 2 viability decision

The Phase 2 gate now has enough information to answer its pass/fail question.

The decision is:

- yes, plotting still has bounded Phase 2 candidates that fit the proven seam
  closely enough to justify another bounded plotting tier

That decision is justified because the seam-compatible bucket is non-empty and
contains multiple methods that still look plausibly:

- method-associated
- analytical and table-first
- typed-contract-friendly
- renderer-agnostic in Rust

The current viable Phase 2 pool is therefore:

- `banana`
- `cpgplot`
- `density`
- `isochore`
- `syco`
- `wobble`

At this gate, the repository should therefore:

- remain on the plotting-family path
- not activate the retrieval fallback
- move to the next bounded decision: choose exactly one Phase 2 candidate from
  this viable pool

What this decision does **not** do:

- it does not say that all six viable methods are equally strong
- it does not start implementation
- it does not widen scope beyond one bounded next-method tier
- it does not weaken retrieval-fallback readiness

So the no-candidate branch is not taken here. The repository should continue to
the single-candidate selection step.

#### Selected bounded Phase 2 candidate

The repository should now choose exactly one bounded Phase 2 plotting candidate
from the viable pool.

The selected candidate is:

- `density`

This is the best bounded next-method choice because it appears to be the
closest extension of the already-proven seam:

- it most plausibly stays a single-series nucleotide analytical profile
- it appears more naturally table-first and typed-contract-friendly than the
  more event- or region-oriented candidates such as `cpgplot` or `isochore`
- it avoids the extra coding-sequence or codon-usage specificity likely to
  arise in `syco` or `wobble`
- it does not immediately signal the broader contract-taxonomy pressure that
  sits closer to `chaos`, `pepwindowall`, or `plotcon`

This choice also keeps the next tier bounded in the intended way:

- exactly one candidate is promoted
- the other seam-compatible methods remain viable but inactive
- no implementation begins yet
- retrieval fallback readiness is preserved

The non-selected viable methods at this checkpoint remain:

- `banana`
- `cpgplot`
- `isochore`
- `syco`
- `wobble`

So the next bounded planning step should be to capture `density`-specific
acceptance criteria and patch start conditions before any code starts.

#### `density` method-level acceptance criteria

Before code changes begin for the selected bounded Phase 2 plotting method,
`density` should have explicit method-level acceptance criteria recorded as
follows.

##### Analytical output expectations

- the method should accept bounded nucleotide-sequence input only
- Rust should compute the analytical `density` profile directly rather than
  delegating analytical work to the renderer
- the analytical model should be described honestly as a bounded nucleotide
  density profile, not as a generic plotting framework extension
- the primary output should remain a stable analytical table with one row per
  emitted window or segment and explicit columns sufficient to reconstruct the
  plotted line
- the analytical output should stay inside the same table-first contract style
  already used by the governed plotting seam

##### Typed contract expectations

- the same run should emit a deterministic typed plot contract derived from the
  analytical table output
- the contract should remain single-series unless the analytical needs of
  `density` itself make broader structure unavoidable
- any contract emitted should stay renderer-agnostic and avoid styling or
  layout policy in Rust
- Rust should own only:
  - numerical series construction
  - domain metadata needed for faithful rendering
  - stable contract serialization
- `emboss-r` should remain responsible for presentation choices and final
  figure rendering

##### Fixture and evidence expectations

- the method must gain governed autodoc before it is treated as shipped
- the method must gain generated docs and generated validation metadata in the
  same governed path as the shipped cohort
- canonical checked-in analytical and plot-contract fixtures must be committed
- compared evidence must validate both:
  - the analytical table output
  - the canonical plot-contract JSON
- the method should not be considered complete on executable-only evidence

##### Explicit non-goals

- no Rust-side figure rendering
- no silent widening into a generic plotting framework
- no dotplot, matrix, circular-map, or pretty-display behavior
- no inference that other remaining plotting-family members are already in
  scope merely because `density` ships
- no broader plot-contract taxonomy unless the analytical needs of `density`
  make that pressure concrete enough to stop and reassess

If `density` cannot satisfy these criteria while remaining a bounded
method-associated plotting method, the repository should pause and reassess
before starting the next code-bearing Phase 2 patch.

#### Exact start conditions for the first `density` implementation patch

The first code-bearing `density` patch should not begin until all of the
following are treated as explicit start conditions:

- the active family decision still remains:
  - plotting first
  - remote retrieval second
  - protein-property rework third
- bounded plotting Phase 1 has already passed explicit reassessment:
  - `hmoment`
  - `octanol`
  - `pepinfo`
- the Phase 2 candidate-selection gate has already passed:
  - bounded plotting continuation remains viable
  - `density` is the single selected Phase 2 candidate
- the release-truth surface still remains in the current zero-burden state:
  - `full_compared_cohort == true`
  - `harvest_coverage_complete == true`
  - `retained_backlog_closed == true`
  - `gapped_method_count == 0`
  - `weakest_evidence_family == null`
  - `release_truth_current == true`
- the patch scope remains limited to `density` and the smallest shared support
  needed for:
  - deterministic analytical computation
  - typed plot-contract emission
  - governed docs and validation plumbing
- the patch does not widen into:
  - broader plotting-family members beyond `density`
  - Rust-side rendering behavior
  - a generalized plotting framework
  - broader plot-contract taxonomies unless `density` itself makes that
    pressure concrete enough to stop and reassess
- the patch is expected to land with all method-level governed surfaces, not
  as a half-start:
  - method-associated Rust implementation
  - registry and service exposure
  - governed autodoc contract
  - generated docs and validation metadata
  - canonical analytical and plot-contract fixtures
  - compared evidence for both table and contract outputs

If any of these start conditions cease to hold before code changes begin, the
repository should re-open planning rather than starting the Phase 2 patch under
a looser scope.

#### Bounded `density` implementation tier

Because `density` is now the single selected Phase 2 candidate, the repository
should map its full bounded implementation tier explicitly before any code
starts.

That tier should stay parallel to the earlier bounded method sequences rather
than widening the plotting program informally:

1. implement the bounded analytical core for `density`
2. add the typed `density` plot-contract emission path
3. expose `density` through the governed shipped surface
4. add canonical analytical and plot-contract fixtures plus compared evidence
5. re-run the full release-truth surface after shipping `density`
6. reassess the shipped `density` slice before any further Phase 2 plotting
   continuation is mapped

The bounded `density` tier should preserve the same architectural constraints
already proven by the bounded plotting seam:

- method-associated implementation only
- table-first analytical output
- typed plot-contract output from the same computation path
- no Rust-side rendering behavior
- no silent widening into a generic plotting framework
- no broader contract taxonomy unless `density` itself makes that pressure
  concrete enough to stop and reassess

So the next code-bearing plotting work should not begin as a vague “continue
Phase 2” step. It should begin as this explicit bounded `density` tier.

#### Explicit fallback-activation stop conditions for continued plotting

Because the repository has now stayed on the plotting path beyond bounded Phase
1 and selected `density` as the active Phase 2 continuation candidate, the
conditions that would finally force fallback activation should be concrete
rather than described only as generic “contract sprawl.”

The repository should stop continued plotting work and activate the prepared
remote-retrieval fallback path if any of the following becomes true during the
bounded `density` tier or any later plotting continuation gate:

1. `density` cannot be expressed as a table-first analytical output plus a
   typed contract derived from the same computation path without adding a
   second non-derived plotting-only computation path.
2. `density` requires Rust-side rendering behavior, layout policy, styling
   logic, or presentation orchestration rather than remaining
   renderer-agnostic.
3. `density` cannot remain method-associated and instead demands a generalized
   plotting framework before even one bounded shipped slice can close.
4. `density` forces a broader plot-contract taxonomy that is not clearly
   method-local, such as introducing fundamentally new chart classes,
   interactive state, or presentation semantics that are better treated as a
   separate planning program.
5. the bounded `density` slice cannot land as a full governed slice with:
   - implementation
   - governed shipped-surface exposure
   - canonical analytical and contract fixtures
   - compared evidence for both outputs
   - a green release-truth surface after shipment
6. a later plotting continuation candidate after `density` fails the same seam
   test and no equally bounded next plotting candidate remains.

The repository should **not** activate the fallback merely because plotting
work remains non-trivial, because a method has more than one analytical series,
or because another viable plotting candidate might also exist. Activation
should happen only when the bounded seam itself fails in one of the concrete
ways above.

If activation is required, the repository should switch directly to the already
prepared retrieval-fallback path without reopening broader family-order
ambiguity first.

#### Reassessment after the shipped `density` slice

After the bounded `density` tier closed as a shipped governed slice, the
repository should make the seam reassessment explicit before mapping any
further plotting continuation.

The result of that reassessment is affirmative:

- `density` stayed method-associated
- the analytical surface remained table-first and honest about richer
  nucleotide-density rows rather than collapsing the method into a plot-only
  contract
- the emitted plot contract stayed bounded to a single derived GC-density
  series
- no Rust-side rendering or presentation-policy pressure emerged
- no generalized plotting-framework pressure emerged
- no fallback-activation stop condition tripped

The concrete basis is:

- `density` shipped as one bounded nucleotide analytical helper plus one
  method-specific plotting path
- the richer analytical table remained the primary truth surface, while the
  contract stayed a bounded renderer-agnostic derivative rather than widening
  into a broader taxonomy
- compared evidence closed both analytical and contract outputs
- the governed release-truth surface remained fully green:
  - shipped methods: `100`
  - compared evidence: `100`
  - executable evidence: `0`
  - harvested legacy provenance present: `100`
  - `full_compared_cohort: true`
  - `harvest_coverage_complete: true`
  - `retained_backlog_closed: true`
  - `release_truth_current: true`

So the repository remains on the plotting continuation path, and the prepared
retrieval fallback remains documented and ready but inactive.

#### Post-`density` Phase 2 remainder inventory

Because `density` has now closed as the shipped bounded Phase 2 slice, any
further plotting continuation should start from an explicit rebased remainder
set rather than the older pre-`density` inventory.

The plotting-family methods already covered by the current proven seam are now:

- preexisting governed seam:
  - `charge`
  - `pepwindow`
  - `wordcount`
- bounded Phase 1 shipped slice:
  - `hmoment`
  - `octanol`
  - `pepinfo`
- bounded Phase 2 shipped slice:
  - `density`

Important scope note:

- `wordcount` remains part of the governed plot-contract seam, but it is not
  itself a member of the historical plotting-family mapping
- the actual post-`density` plotting-family remainder therefore excludes the
  six plotting-family members already covered inside that family:
  - `charge`
  - `pepwindow`
  - `hmoment`
  - `octanol`
  - `pepinfo`
  - `density`

So the explicit post-`density` plotting-family remainder pool is the remaining
`22` plotting-family methods:

- `abiview`
- `banana`
- `chaos`
- `cirdna`
- `cpgplot`
- `dotmatcher`
- `dotpath`
- `dottup`
- `findkm`
- `isochore`
- `lindna`
- `pepnet`
- `pepwheel`
- `pepwindowall`
- `plotcon`
- `polydot`
- `prettyplot`
- `showfeat`
- `showpep`
- `showseq`
- `syco`
- `wobble`

This section is inventory only. It does not yet reclassify seam pressure or
choose the next bounded continuation candidate.

#### Post-`density` seam-pressure reclassification

Now that an actual bounded Phase 2 shipment exists, the post-`density`
remainder pool can be reclassified with a more concrete seam reference than
the earlier pre-shipment guesswork.

The key new signal from `density` is:

- the current seam comfortably supports a nucleotide analytical table that is
  richer than the emitted plot
- the emitted contract can still stay bounded to one derived renderer-agnostic
  series
- this makes “table-first with one bounded derived line” look proven, but it
  does not automatically justify region-track, threshold-call, matrix, or
  presentation-heavy continuations

The explicit post-`density` remainder buckets are therefore:

##### Strongest remaining seam-compatible candidates

These methods still look closest to the now-proven seam because they appear
most likely to remain method-associated, analytical, table-first, and
renderer-agnostic without forcing broader contract vocabulary:

- `banana`
- `isochore`
- `syco`
- `wobble`

##### Conditional region-oriented candidates

These methods may still be bounded, but they now look less direct than the
strongest bucket because they appear more likely to pressure the seam toward
region calling, threshold semantics, or event-style reporting rather than a
simple derived analytical profile:

- `cpgplot`

##### Requires broader contract taxonomy

These methods still look biologically relevant, but they appear more likely to
need a broader typed-contract vocabulary than the currently proven bounded
profile seam:

- `chaos`
- `pepwindowall`
- `plotcon`

##### Dotplot-style or comparative-matrix heavy

These methods still lean directly into comparative matrix or dotplot behavior,
which remains outside the currently proven bounded seam:

- `dotmatcher`
- `dotpath`
- `dottup`
- `polydot`

##### Diagram, layout, or presentation-heavy

These methods still appear to depend more on structural layout, diagram
rendering, or pretty-display behavior than on the bounded analytical-profile
seam proven so far:

- `cirdna`
- `lindna`
- `pepnet`
- `pepwheel`
- `prettyplot`
- `showfeat`
- `showpep`
- `showseq`

##### Specialized laboratory-trace or kinetic plotting

These methods still appear to need specialized trace or laboratory-kinetic
handling rather than the current analytical-profile seam:

- `abiview`
- `findkm`

This reclassification is still governance-only. It does not yet decide whether
another bounded Phase 2 plotting candidate still exists, but it does narrow
the strongest next-candidate pool more sharply than the pre-`density`
classification did.

#### Post-`density` bounded-candidate viability decision

After the `density` shipment gate and the rebased seam-pressure
reclassification, the repository can now answer the next pass/fail question
explicitly: does another bounded plotting continuation candidate still exist,
or has the seam narrowed to the point where the prepared retrieval fallback
should activate?

The decision is:

- yes, another bounded plotting continuation candidate still exists

That decision is justified because the strongest current seam-compatible pool
remains non-empty and still contains multiple methods that look plausibly:

- method-associated
- analytical and table-first
- typed-contract-friendly without broad taxonomy expansion
- renderer-agnostic in Rust

The strongest current next-candidate pool is therefore:

- `banana`
- `isochore`
- `syco`
- `wobble`

At this gate, the repository should therefore:

- remain on the plotting-family path
- not activate the retrieval fallback
- move to the next bounded decision: choose exactly one next continuation
  candidate from this narrowed pool

What this decision does **not** do:

- it does not say that all four remaining strong candidates are equally good
- it does not start implementation
- it does not widen scope beyond one next-method selection
- it does not weaken retrieval-fallback readiness

So the no-candidate branch is not taken here. The repository should continue
to the next explicit bounded candidate-selection step.

#### Post-`density` bounded-candidate selection

Because the post-`density` viability gate still left a non-empty narrowed
strong pool, the repository should now choose exactly one bounded next-method
continuation candidate rather than carrying all four forward as if they were
interchangeable.

The selected next bounded plotting candidate is:

- `wobble`

The selection basis is:

- `wobble` appears to be the closest remaining extension of the currently
  proven seam as a likely single-series nucleotide analytical profile
- it appears more naturally table-first and typed-contract-friendly than the
  more region-oriented `isochore`
- it appears narrower and easier to keep method-associated than `banana`,
  which signals a heavier biophysical model surface, or `syco`, which signals
  a broader synonymous-codon-usage interpretation surface
- it does not immediately imply the broader contract-taxonomy or presentation
  pressure already excluded elsewhere in the plotting family

The non-selected strong candidates remain viable but inactive:

- `banana`
- `isochore`
- `syco`

So the next bounded planning step should be to capture `wobble`-specific
acceptance criteria and exact patch start conditions before any code starts.

#### Untriggered retrieval-activation branch after candidate selection

The conditional retrieval-activation branch is not taken at this checkpoint.

That branch would only activate if the narrowed plotting pool had proven
deceptive against the explicit fallback stop conditions. Instead:

- the narrowed plotting pool remained credible
- `wobble` was selected as the active next bounded continuation candidate

So the repository should not promote remote retrieval here. The prepared
retrieval shortlist remains documented, ready, and inactive while the
repository stays on the plotting continuation branch.

#### Explicit closeout of the untriggered retrieval-activation branch

Because plotting remains active after the bounded selection review, the
untriggered retrieval-activation branch should now be closed explicitly rather
than left implied.

That closeout means:

- the retrieval shortlist remains documented and prepared
- retrieval is not promoted by inertia
- the repository continues on the active `wobble` plotting branch

So there is no branch ambiguity at this checkpoint. The next planning work
should stay method-associated to `wobble` rather than reopening fallback
promotion.

#### `wobble` method-level acceptance criteria

Before code changes begin for the selected bounded continuation method,
`wobble` should have explicit method-level acceptance criteria recorded as
follows.

##### Analytical output expectations

- the method should accept bounded coding nucleotide-sequence input only
- Rust should compute the analytical `wobble` profile directly rather than
  delegating analytical work to the renderer
- the analytical model should be described honestly as a bounded
  third-base-position variability profile, not as a generic plotting framework
  extension
- the primary output should remain a stable analytical table with one row per
  emitted position, window, or bucket and explicit columns sufficient to
  reconstruct the plotted series
- the analytical output should stay inside the same table-first contract style
  already used by the governed plotting seam

##### Typed contract expectations

- the same run should emit a deterministic typed plot contract derived from the
  analytical table output
- the contract should remain single-series unless the analytical needs of
  `wobble` itself make broader structure unavoidable
- any contract emitted should stay renderer-agnostic and avoid styling or
  layout policy in Rust
- Rust should own only:
  - numerical series construction
  - domain metadata needed for faithful rendering
  - stable contract serialization
- `emboss-r` should remain responsible for presentation choices and final
  figure rendering

##### Fixture and evidence expectations

- the method must gain governed autodoc before it is treated as shipped
- the method must gain generated docs and generated validation metadata in the
  same governed path as the shipped cohort
- canonical checked-in analytical and plot-contract fixtures must be committed
- compared evidence must validate both:
  - the analytical table output
  - the canonical plot-contract JSON
- the method should not be considered complete on executable-only evidence

##### Explicit non-goals

- no Rust-side figure rendering
- no silent widening into a generic plotting framework
- no dotplot, matrix, circular-map, or pretty-display behavior
- no inference that other remaining plotting-family members are already in
  scope merely because `wobble` ships
- no broader plot-contract taxonomy unless the analytical needs of `wobble`
  make that pressure concrete enough to stop and reassess

If `wobble` cannot satisfy these criteria while remaining a bounded
method-associated plotting method, the repository should pause and reassess
before starting the next code-bearing continuation patch.

#### Exact start conditions for the first `wobble` implementation patch

The first code-bearing `wobble` patch should not begin until all of the
following are treated as explicit start conditions:

- the active family decision still remains:
  - plotting first
  - remote retrieval second
  - protein-property rework third
- bounded plotting Phase 1 has already passed explicit reassessment:
  - `hmoment`
  - `octanol`
  - `pepinfo`
- the post-`density` continuation gate has already passed:
  - bounded plotting continuation remains viable
  - `wobble` is the single selected next bounded continuation candidate
- the release-truth surface still remains in the current zero-burden state:
  - `full_compared_cohort == true`
  - `harvest_coverage_complete == true`
  - `retained_backlog_closed == true`
  - `gapped_method_count == 0`
  - `weakest_evidence_family == null`
  - `release_truth_current == true`
- the patch scope remains limited to `wobble` and the smallest shared support
  needed for:
  - deterministic analytical computation
  - typed plot-contract emission
  - governed docs and validation plumbing
- the patch does not widen into:
  - broader plotting-family members beyond `wobble`
  - Rust-side rendering behavior
  - a generalized plotting framework
  - broader plot-contract taxonomies unless `wobble` itself makes that
    pressure concrete enough to stop and reassess
- the patch is expected to land with all method-level governed surfaces, not
  as a half-start:
  - method-associated Rust implementation
  - registry and service exposure
  - governed autodoc contract
  - generated docs and validation metadata
  - canonical analytical and plot-contract fixtures
  - compared evidence for both table and contract outputs

If any of these start conditions cease to hold before code changes begin, the
repository should re-open planning rather than starting the continuation patch
under a looser scope.

#### Bounded `wobble` implementation tier

Because `wobble` is now the single selected next bounded continuation
candidate, the repository should map its full bounded implementation tier
explicitly before any code starts.

That tier should stay parallel to the earlier bounded method sequences rather
than widening the plotting program informally:

1. implement the bounded analytical core for `wobble`
2. add the typed `wobble` plot-contract emission path
3. expose `wobble` through the governed shipped surface
4. add canonical analytical and plot-contract fixtures plus compared evidence
5. re-run the full release-truth surface after shipping `wobble`
6. reassess the shipped `wobble` slice before any further continuation is
   mapped

The bounded `wobble` tier should preserve the same architectural constraints
already proven by the governed plotting seam and the bounded continuation
methods shipped so far:

- method-associated implementation only
- table-first analytical output
- typed contract output from the same computation path
- no Rust-side rendering
- no generic plotting-framework widening
- no broader contract taxonomy unless `wobble` itself makes that pressure
  concrete enough to stop and reassess

So the next code-bearing continuation should not begin as a vague “keep going
with plotting” step. It should begin as this explicit bounded `wobble` tier.

#### Pre-code seam-pressure stop conditions for `wobble`

Before the first `wobble` code patch starts, the repository should make the
pause-and-reconsider conditions explicit rather than treating them as implied.

The repository should pause and reopen planning if any of the following
becomes true during the pre-code review:

1. `wobble` cannot remain table-first with a typed contract derived from the
   same computation path
2. `wobble` requires Rust-side rendering behavior, layout policy, styling, or
   other presentation logic
3. `wobble` cannot remain method-associated and instead demands a generalized
   plotting framework before one bounded shipped slice closes
4. `wobble` forces region-track, threshold-call, or broader plot-contract
   taxonomy pressure that is not clearly local to the method

If one of these conditions becomes true, the repository should not force-fit
`wobble` through the bounded seam. It should stop and reassess whether the
active plotting branch still remains the right next implementation path.

#### Reassessment after the shipped `wobble` slice

After the bounded `wobble` implementation tier closed as a shipped governed
slice, the repository should reassess whether the plotting seam still remains
bounded or whether the prepared retrieval fallback should finally activate.

The reassessment result is affirmative. The shipped `wobble` slice stayed
bounded:

- `wobble` stayed method-associated.
- The analytical surface stayed table-first and coding-sequence-specific.
- The richer third-position composition remained in the analytical table rather
  than widening the emitted contract.
- The emitted contract stayed bounded to a single derived
  `wobble_variability` series.
- No renderer-coupled pressure emerged.
- No generalized plotting-framework pressure emerged.
- No fallback-activation stop condition tripped.

The governed release-truth surface remained fully green after `wobble` closed:

- shipped methods: `101`
- compared evidence: `101`
- executable evidence: `0`
- harvested legacy provenance present: `101`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

So the repository remains on the plotting continuation path. The retrieval
fallback remains documented, prepared, and inactive unless a later bounded
continuation slice trips the explicit stop conditions.

#### Post-`wobble` plotting-family remainder inventory

After the bounded `wobble` slice shipped and passed its reassessment gate, the
repository should rebase the active plotting-family continuation pool on the
actual post-`wobble` state rather than on the earlier post-`density`
shortlist.

The plotting-family remainder now excludes:

- the preexisting governed seam precedents:
  - `charge`
  - `pepwindow`
- the bounded shipped plotting-family continuations:
  - `hmoment`
  - `octanol`
  - `pepinfo`
  - `density`
  - `wobble`

So the explicit post-`wobble` plotting-family continuation pool is the
remaining `21` plotting-family methods:

- `abiview`
- `banana`
- `chaos`
- `cirdna`
- `cpgplot`
- `dotmatcher`
- `dotpath`
- `dottup`
- `findkm`
- `isochore`
- `lindna`
- `pepnet`
- `pepwheel`
- `pepwindowall`
- `plotcon`
- `polydot`
- `prettyplot`
- `showfeat`
- `showpep`
- `showseq`
- `syco`

This section is inventory only. It does not yet reclassify seam pressure or
choose the next bounded continuation candidate.

#### Post-`wobble` seam-pressure reclassification

Now that `wobble` has shipped as another bounded continuation slice, the
post-`wobble` remainder can be reclassified against a more concrete seam than
the earlier post-`density` shortlist.

The key new signal from `wobble` is:

- the current seam comfortably supports a coding-sequence-specific analytical
  table that is richer than the emitted plot
- the emitted contract can still stay bounded to one derived
  renderer-agnostic series
- this makes “table-first with one bounded derived line” look proven for
  another nucleotide-specific continuation, but it still does not justify
  region tracks, threshold calls, matrix-heavy comparison, or
  presentation-heavy behavior by default

The explicit post-`wobble` remainder buckets are therefore:

##### Strongest remaining seam-compatible candidates

These methods still look closest to the now-proven seam because they appear
most likely to remain method-associated, analytical, table-first, and
renderer-agnostic without forcing broader contract vocabulary:

- `banana`
- `isochore`
- `syco`

##### Conditional region-oriented candidates

These methods may still be bounded, but they now look less direct than the
strongest bucket because they appear more likely to pressure the seam toward
region calling, threshold semantics, or event-style reporting rather than a
simple derived analytical profile:

- `cpgplot`

##### Requires broader contract taxonomy

These methods still look biologically relevant, but they appear more likely to
need a broader typed-contract vocabulary than the currently proven bounded
profile seam:

- `chaos`
- `pepwindowall`
- `plotcon`

##### Dotplot-style or comparative-matrix heavy

These methods still lean directly into comparative matrix or dotplot behavior,
which remains outside the currently proven bounded seam:

- `dotmatcher`
- `dotpath`
- `dottup`
- `polydot`

##### Diagram, layout, or presentation-heavy

These methods still appear to depend more on structural layout, diagram
rendering, or pretty-display behavior than on the bounded analytical-profile
seam proven so far:

- `cirdna`
- `lindna`
- `pepnet`
- `pepwheel`
- `prettyplot`
- `showfeat`
- `showpep`
- `showseq`

##### Specialized laboratory-trace or kinetic plotting

These methods still appear to need specialized trace or laboratory-kinetic
handling rather than the current analytical-profile seam:

- `abiview`
- `findkm`

This reclassification remains governance-only. It narrows the strongest
next-candidate pool more sharply than the post-`density` view, but it does not
yet choose the next bounded continuation candidate.

#### Post-`wobble` bounded-candidate viability decision

After the shipped `wobble` slice and the rebased post-`wobble` seam-pressure
reclassification, the repository can now answer the next pass/fail question
explicitly: does another bounded plotting continuation candidate still exist,
or has the seam narrowed to the point where the prepared retrieval fallback
should activate?

The decision is affirmative. Another bounded plotting continuation candidate
still exists after `wobble`.

The current viable post-`wobble` continuation pool is:

- `banana`
- `isochore`
- `syco`

So the no-candidate branch is not taken here. The repository remains on the
plotting continuation path, and the prepared retrieval fallback remains
documented, ready, and inactive unless a later selection or shipment gate
trips the explicit stop conditions.

That means the conditional “activate retrieval fallback now” branch is closed
as untriggered at this checkpoint. The repository should not promote the
fallback by inertia when the narrowed plotting continuation pool remains
credible.

#### Post-`wobble` bounded-candidate selection

Because the post-`wobble` viability gate still leaves a non-empty narrowed
continuation pool, the repository should choose exactly one next bounded
plotting continuation candidate before any further implementation planning
starts.

The selected candidate is:

- `isochore`

The selection basis is:

- `isochore` looks like the narrowest remaining extension of the proven
  bounded nucleotide plotting seam as an analytical, table-first profile with
  a likely single derived continuation line
- it appears less likely than `banana` to force a heavier biophysical model
  surface
- it appears less likely than `syco` to force codon-usage-specific structure
  or coding-sequence-only seam pressure

The non-selected viable methods remain documented but inactive:

- `banana`
- `syco`

So the next bounded planning step should be to capture `isochore`-specific
acceptance criteria and exact patch start conditions before any code starts.

That also closes the retrieval-activation branch explicitly as untriggered
after candidate selection. Retrieval fallback remains documented, prepared, and
inactive while the active bounded `isochore` continuation branch remains
credible.

#### Reconfirmation after the `density` shipment gate

After the repository stayed on the plotting path through the bounded
`density` shipment gate, protein-property rework should still remain the third
candidate in the shortlist.

That conclusion remains explicit because the two higher-ranked programs are
still more implementation-ready in concrete terms:

- plotting remains the active path because:
  - bounded plotting Phase 1 passed without forcing generic plotting-framework
    widening
  - the Phase 2 seam-compatibility gate also passed
  - `density` has now shipped as a bounded Phase 2 slice without forcing
    fallback activation
  - the post-`density` viability gate still leaves a non-empty narrowed
    next-candidate pool:
    - `banana`
    - `isochore`
    - `syco`
    - `wobble`
- remote retrieval still remains the clearest prepared fallback because its
  bounded Phase 1 subset, provider-aware seams, and deterministic evidence
  model are already documented and still inactive
- protein-property rework still has a credible analytical substrate, but it
  still lacks the same immediate post-gate implementation-readiness detail now
  written down for the two higher-ranked programs

The fully green release-truth surface still does not alter that ordering:

- shipped methods: `100`
- compared evidence: `100`
- harvested legacy provenance present: `100`
- `full_compared_cohort == true`
- `harvest_coverage_complete == true`
- `retained_backlog_closed == true`
- `release_truth_current == true`

So the shortlist should still remain:

1. plotting
2. remote retrieval
3. protein-property rework

This remains a planning checkpoint only. It does not promote
protein-property work, and it does not weaken the active plotting-first
decision.

### Post-sub-roadmap third-candidate check

Drafting the first plotting program and the explicit retrieval fallback does
not materially change the rationale for the third position in the shortlist.
Protein-property rework still remains the next candidate after those two.

That ordering still holds for the same bounded reasons:

- plotting now has the clearest immediate implementation path because its
  governed contract seam and bounded Phase 1 subset are already written down
- remote retrieval now has the strongest explicit fallback plan because its
  provider-aware validation seam and bounded Phase 1 subset are also written
  down
- protein-property rework still has a strong scientific substrate, but it does
  not yet displace either of the two more explicitly prepared implementation
  programs

This remains a planning check only. It does **not** promote protein-property
rework, and it does **not** imply that plotting or retrieval are blocked.

#### Reconfirmation after the plotting lead-method pass

After fixing the plotting implementation order, confirming `hmoment` as the
explicit lead method, and recording its method-level acceptance criteria,
protein-property rework still remains the third candidate.

That conclusion still holds because:

- plotting now has the most operationally prepared first patch boundary:
  - bounded Phase 1 order
  - explicit lead method
  - method-level start criteria for `hmoment`
- remote retrieval still has the clearest prepared fallback if plotting needs
  to pause
- protein-property rework still has a credible analytical substrate, but it
  still lacks the same immediate implementation-readiness detail now written
  down for the two higher-ranked programs

So the shortlist remains:

1. plotting
2. remote retrieval
3. protein-property rework

This is still a planning checkpoint only. It does not start protein-property
work and does not weaken the plotting-first decision.

#### Reconfirmation after the second shipped plotting slice

After shipping both `hmoment` and `octanol`, closing their compared-evidence
follow-ons, and confirming that the retrieval fallback remains ready but
inactive, protein-property rework should still remain the third candidate in
the shortlist.

That conclusion remains explicit because the two higher-ranked programs are now
still more implementation-ready in concrete terms:

- plotting now has two shipped bounded slices:
  - `hmoment`
  - `octanol`
- plotting still has the active bounded Phase 1 continuation path toward
  `pepinfo` without having triggered contract sprawl or renderer-coupled
  pressure
- remote retrieval still remains the clearest prepared fallback because its
  bounded Phase 1 subset, provider-aware seams, and deterministic evidence
  model are already written down
- protein-property rework still has a credible scientific substrate, but it
  still lacks the same immediate start-boundary detail now recorded for the
  two higher-ranked programs

The fully green release-truth surface does not alter that ordering:

- shipped methods: `98`
- compared evidence: `98`
- harvested legacy provenance present: `98`
- `full_compared_cohort == true`
- `harvest_coverage_complete == true`
- `release_truth_current == true`

So the shortlist should remain:

1. plotting
2. remote retrieval
3. protein-property rework

This remains a planning checkpoint only. It does not promote protein-property
work, and it does not weaken the active plotting-first decision.

#### Reconfirmation after the third shipped plotting slice

After shipping `pepinfo`, closing its compared-evidence follow-on, and
confirming that the retrieval fallback remains ready but inactive,
protein-property rework should still remain the third candidate in the
shortlist.

That conclusion remains explicit because the two higher-ranked programs are
still more implementation-ready in concrete terms after the full bounded
plotting Phase 1 exists:

- plotting now has a full bounded Phase 1 with three shipped methods:
  - `hmoment`
  - `octanol`
  - `pepinfo`
- plotting advanced through its first multi-series method without forcing a
  generic plotting framework, broader contract taxonomy, or renderer-coupled
  pressure
- remote retrieval still remains the clearest prepared fallback because its
  bounded Phase 1 subset, provider-aware seams, and deterministic evidence
  model are already written down and still inactive
- protein-property rework still has a credible scientific substrate, but it
  still lacks the same immediate post-Phase-1 start-boundary detail now
  recorded for the two higher-ranked programs

The fully green release-truth surface still does not alter that ordering:

- shipped methods: `99`
- compared evidence: `99`
- harvested legacy provenance present: `99`
- `full_compared_cohort == true`
- `harvest_coverage_complete == true`
- `retained_backlog_closed == true`
- `release_truth_current == true`

So the shortlist should still remain:

1. plotting
2. remote retrieval
3. protein-property rework

This remains a planning checkpoint only. It does not promote protein-property
work, and it does not weaken the active plotting-first decision.

### Explicit no-change decisions

This reassessment does **not** do any of the following:

- promote a `Rework` family to `Core Retain`
- demote any existing `Rework` family to `Defer`
- expand the retained set beyond the current governed shipped cohort
- rewrite per-tool decisions in the full scope matrix

Those changes should only happen in a future governance pass with explicit
supporting evidence, not as a side effect of roadmap execution.

## Core Retain — Basic sequence IO and conversion

Foundational sequence and feature-table ingest, extraction, conversion, and stream-handling utilities.

**Mapped historical/core tools:** 18

**Decision split:** Retain 18

- `coderet` — **Retain** — Extract CDS, mRNA and translations from feature tables
- `extractfeat` — **Retain** — Extract features from sequence(s)
- `extractseq` — **Retain** — Extract regions from a sequence
- `featcopy` — **Retain** — Read and write a feature table
- `featmerge` — **Retain** — Merge two overlapping feature tables
- `featreport` — **Retain** — Read and write a feature table
- `feattext` — **Retain** — Return a feature table original text
- `newseq` — **Retain** — Create a sequence file from a typed-in sequence
- `notseq` — **Retain** — Write to file a subset of an input stream of sequences
- `nthseq` — **Retain** — Write to file a single sequence from an input stream of sequences
- `nthseqset` — **Retain** — Read and write (return) one set of sequences from many
- `seqcount` — **Retain** — Read and count sequences
- `skipseq` — **Retain** — Read and write (return) sequences, skipping first few
- `splitsource` — **Retain** — Split sequence(s) into original source sequences
- `union` — **Retain** — Concatenate multiple sequences into a single sequence
- `backtranambig` — **Retain** — Back-translate a protein sequence to ambiguous nucleotide sequence
- `backtranseq` — **Retain** — Back-translate a protein sequence to a nucleotide sequence
- `degapseq` — **Retain** — Remove non-alphabetic (e.g. gap) characters from sequences

## Core Retain — Sequence editing and manipulation

Direct sequence alteration, masking, shuffling, slicing, merging, and related manipulations.

**Mapped historical/core tools:** 23

**Decision split:** Retain 23

- `biosed` — **Retain** — Replace or delete sequence sections
- `cutseq` — **Retain** — Remove a section from a sequence
- `descseq` — **Retain** — Alter the name or description of a sequence
- `listor` — **Retain** — Write a list file of the logical OR of two sets of sequences
- `makenucseq` — **Retain** — Create random nucleotide sequences
- `makeprotseq` — **Retain** — Create random protein sequences
- `maskambignuc` — **Retain** — Mask all ambiguity characters in nucleotide sequences with N
- `maskambigprot` — **Retain** — Mask all ambiguity characters in protein sequences with X
- `maskfeat` — **Retain** — Write a sequence with masked features
- `maskseq` — **Retain** — Write a sequence with masked regions
- `megamerger` — **Retain** — Merge two large overlapping DNA sequences
- `merger` — **Retain** — Merge two overlapping sequences
- `msbar` — **Retain** — Mutate a sequence
- `pasteseq` — **Retain** — Insert one sequence into another
- `revseq` — **Retain** — Reverse and complement a nucleotide sequence
- `shuffleseq` — **Retain** — Shuffle a set of sequences maintaining composition
- `sizeseq` — **Retain** — Sort sequences by size
- `skipredundant` — **Retain** — Remove redundant sequences from an input set
- `splitter` — **Retain** — Split sequence(s) into smaller sequences
- `trimest` — **Retain** — Remove poly-A tails from nucleotide sequences
- `trimseq` — **Retain** — Remove unwanted characters from start and end of sequence(s)
- `vectorstrip` — **Retain** — Remove vectors from the ends of nucleotide sequence(s)
- `twofeat` — **Retain** — Find neighbouring pairs of features in sequence(s)

## Core Retain — Alignment read-write and post-processing

Pairwise/multiple alignment generation, alignment IO, and alignment-derived summaries; includes a small number of wrapper-heritage or large-sequence exceptions marked Rework.

**Family note:** Mixed family: `emma`, `showalign`, `stretcher`, and `supermatcher` remain alignment-relevant but are individually Rework.

**Mapped historical/core tools:** 18

**Decision split:** Retain 13, Rework 5

- `aligncopy` — **Retain** — Read and write alignments
- `aligncopypair` — **Retain** — Read and write pairs from alignments
- `cons` — **Retain** — Create a consensus sequence from a multiple alignment
- `consambig` — **Retain** — Create an ambiguous consensus sequence from a multiple alignment
- `diffseq` — **Retain** — Compare and report features of two similar sequences
- `distmat` — **Retain** — Create a distance matrix from a multiple sequence alignment
- `edialign` — **Retain** — Local multiple alignment of sequences
- `est2genome` — **Rework** — Align EST sequences to genomic DNA sequence
- `extractalign` — **Retain** — Extract regions from a sequence alignment
- `infoalign` — **Retain** — Display basic information about a multiple sequence alignment
- `matcher` — **Retain** — Waterman-Eggert local alignment of two sequences
- `needle` — **Retain** — Needleman-Wunsch global alignment of two sequences
- `needleall` — **Retain** — Many-to-many pairwise alignments of two sequence sets
- `showalign` — **Rework** — Display a multiple sequence alignment in pretty format
- `stretcher` — **Rework** — Needleman-Wunsch rapid global alignment of two sequences
- `supermatcher` — **Rework** — Calculate approximate local pair-wise alignments of larger sequences
- `water` — **Retain** — Smith-Waterman local alignment of sequences
- `emma` — **Rework** — Multiple sequence alignment (ClustalW wrapper)

## Core Retain — Core sequence statistics and composition

Durable descriptive statistics, codon/composition summaries, and residue/base information utilities.

**Family note:** Mixed family: `freak` is mapped here as a statistics utility even though its plotting mode pushes it toward Rework.

**Mapped historical/core tools:** 17

**Decision split:** Retain 16, Rework 1

- `aaindexextract` — **Retain** — Extract amino acid property data from AAINDEX
- `cai` — **Retain** — Calculate codon adaptation index
- `chips` — **Retain** — Calculate Nc codon usage statistic
- `codcmp` — **Retain** — Codon usage table comparison
- `codcopy` — **Retain** — Copy and reformat a codon usage table
- `compseq` — **Retain** — Calculate the composition of unique words in sequences
- `cusp` — **Retain** — Create a codon usage table from nucleotide sequence(s)
- `dan` — **Retain** — Calculate nucleic acid melting temperature
- `freak` — **Rework** — Generate residue/base frequency table or plot
- `geecee` — **Retain** — Calculate fractional GC content of nucleic acid sequences
- `infobase` — **Retain** — Return information on a given nucleotide base
- `inforesidue` — **Retain** — Return information on a given amino acid residue
- `infoseq` — **Retain** — Display basic information about sequences
- `oddcomp` — **Retain** — Identify proteins with specified sequence word composition
- `pepstats` — **Retain** — Calculate statistics of protein properties
- `checktrans` — **Retain** — Report STOP codons and ORF statistics of a protein
- `wordcount` — **Retain** — Count and extract unique words in molecular sequence(s)

## Core Retain — Simple motif, pattern, and regular-expression search

Lightweight exact/pattern search utilities; includes the explicit `complex` retain.

**Mapped historical/core tools:** 12

**Decision split:** Retain 12

- `dreg` — **Retain** — Regular expression search of nucleotide sequence(s)
- `einverted` — **Retain** — Find inverted repeats in nucleotide sequences
- `fuzznuc` — **Retain** — Search for patterns in nucleotide sequences
- `fuzzpro` — **Retain** — Search for patterns in protein sequences
- `fuzztran` — **Retain** — Search for patterns in protein sequences (translated)
- `palindrome` — **Retain** — Find inverted repeats in nucleotide sequence(s)
- `patmatdb` — **Retain** — Search protein sequences with a sequence motif
- `preg` — **Retain** — Regular expression search of protein sequence(s)
- `seqmatchall` — **Retain** — All-against-all word comparison of a sequence set
- `wordfinder` — **Retain** — Match large sequences against one or more other sequences
- `wordmatch` — **Retain** — Find regions of identity (exact matches) of two sequences
- `complex` — **Retain** — Complexity / low-complexity analysis tool (EMBASSY; explicit user retain)

## Core Retain — ORF and translation-adjacent utilities

ORF finding, translation, and presentation of coding context; several presentation-heavy members are Rework rather than Retain.

**Family note:** Mixed family: `plotorf`, `showorf`, `sixpack`, and `tcode` are individually Rework because presentation or algorithmic modernization is warranted.

**Mapped historical/core tools:** 8

**Decision split:** Retain 4, Rework 4

- `getorf` — **Retain** — Find and extract open reading frames (ORFs)
- `plotorf` — **Rework** — Plot potential open reading frames in a nucleotide sequence
- `showorf` — **Rework** — Display a nucleotide sequence and translation in pretty format
- `sixpack` — **Rework** — Display a DNA sequence with 6-frame translation and ORFs
- `tcode` — **Rework** — Identify protein-coding regions using Fickett TESTCODE statistic
- `transeq` — **Retain** — Translate nucleic acid sequences
- `tranalign` — **Retain** — Generate an alignment of nucleic coding regions from aligned proteins
- `prettyseq` — **Retain** — Write a nucleotide sequence and its translation to file

## Modernize — Rework — Restriction-enzyme design and analysis

Restriction workflows remain valuable, but databases, reporting, and visualization should be modernized; `recoder` and `silent` are retained as durable edit-design primitives.

**Family note:** Mixed family: `recoder` and `silent` are individually Retain despite the family’s default Rework stance.

**Mapped historical/core tools:** 7

**Decision split:** Retain 2, Rework 5

- `recoder` — **Retain** — Find restriction sites to remove (mutate) with no translation change
- `rebaseextract` — **Rework** — Process the REBASE database for use by restriction enzyme applications
- `redata` — **Rework** — Retrieve information from REBASE restriction enzyme database
- `remap` — **Rework** — Display restriction enzyme binding sites in a nucleotide sequence
- `restover` — **Rework** — Find restriction enzymes producing a specific overhang
- `restrict` — **Rework** — Report restriction enzyme cleavage sites in a nucleotide sequence
- `silent` — **Retain** — Find restriction sites to insert (mutate) with no translation change

## Modernize — Rework — Primer and assay-oriented search

Primer and assay workflows remain in scope, but legacy wrappers and dated assay-specific surfaces may be omitted or redesigned.

**Family note:** Mixed family: `eprimer32` and `stssearch` are individually Omit.

**Mapped historical/core tools:** 5

**Decision split:** Rework 3, Omit 2

- `eprimer3` — **Rework** — Pick PCR primers and hybridization oligos
- `eprimer32` — **Omit** — Pick PCR primers and hybridization oligos
- `primersearch` — **Rework** — Search DNA sequences for matches with primer pairs
- `sirna` — **Rework** — Find siRNA duplexes in mRNA
- `stssearch` — **Omit** — Search a DNA database for matches with a set of STS primers

## Modernize — Rework — Plotting and visualization tools

Rendering moves to `emboss-r`; Rust emits plot-ready data only.

**Mapped historical/core tools:** 28

**Decision split:** Rework 28

- `abiview` — **Rework** — Display the trace in an ABI sequencer file
- `banana` — **Rework** — Plot bending and curvature data for B-DNA
- `chaos` — **Rework** — Draw a chaos game representation plot for a nucleotide sequence
- `charge` — **Rework** — Draw a protein charge plot
- `cirdna` — **Rework** — Draw circular map of DNA constructs
- `cpgplot` — **Rework** — Identify and plot CpG islands in nucleotide sequence(s)
- `density` — **Rework** — Draw a nucleic acid density plot
- `dotmatcher` — **Rework** — Draw a threshold dotplot of two sequences
- `dotpath` — **Rework** — Draw a non-overlapping wordmatch dotplot of two sequences
- `dottup` — **Rework** — Display a wordmatch dotplot of two sequences
- `findkm` — **Rework** — Calculate and plot enzyme reaction data
- `hmoment` — **Rework** — Calculate and plot hydrophobic moment for protein sequence(s)
- `isochore` — **Rework** — Plot isochores in DNA sequences
- `lindna` — **Rework** — Draw linear maps of DNA constructs
- `octanol` — **Rework** — Draw a White-Wimley protein hydropathy plot
- `pepinfo` — **Rework** — Plot amino acid properties of a protein sequence in parallel
- `pepnet` — **Rework** — Draw a helical net for a protein sequence
- `pepwheel` — **Rework** — Draw a helical wheel diagram for a protein sequence
- `pepwindow` — **Rework** — Draw a hydropathy plot for a protein sequence
- `pepwindowall` — **Rework** — Draw Kyte-Doolittle hydropathy plot for a protein alignment
- `plotcon` — **Rework** — Plot conservation of a sequence alignment
- `polydot` — **Rework** — Draw dotplots for all-against-all comparison of a sequence set
- `prettyplot` — **Rework** — Draw a sequence alignment with pretty formatting
- `showfeat` — **Rework** — Display features of a sequence in pretty format
- `showpep` — **Rework** — Display protein sequences with features in pretty format
- `showseq` — **Rework** — Display sequences with features in pretty format
- `syco` — **Rework** — Draw synonymous codon usage statistic plot for a nucleotide sequence
- `wobble` — **Rework** — Plot third base position variability in a nucleotide sequence

## Modernize — Rework — Remote retrieval and archive acquisition

Accession-driven retrieval is retained as a user need, but the old EMBOSS server/database model is replaced with provider-aware integrations.

**Mapped historical/core tools:** 10

**Decision split:** Rework 10

- `assemblyget` — **Rework** — Get assembly of sequence reads
- `entret` — **Rework** — Retrieve sequence entries from flatfile databases and files
- `infoassembly` — **Rework** — Display information about assemblies
- `refseqget` — **Rework** — Get reference sequence
- `runget` — **Rework** — Download archive-run-associated data through a modern provider seam
- `runinfo` — **Rework** — Report archive-run-associated metadata through a modern provider seam
- `seqret` — **Rework** — Read and write (return) sequences
- `seqretsetall` — **Rework** — Read and write (return) many sets of sequences
- `seqretsplit` — **Rework** — Read sequences and write them to individual files
- `whichdb` — **Rework** — Search all sequence databases for an entry and retrieve it

## Modernize — Rework — External database preparation helpers

Keep only where the underlying curated resource remains useful; redesign around modern data-source preparation.

**Mapped historical/core tools:** 5

**Decision split:** Rework 5

- `cutgextract` — **Rework** — Extract codon usage tables from CUTG database
- `jaspextract` — **Rework** — Extract data from JASPAR
- `printsextract` — **Rework** — Extract data from PRINTS database for use by pscan
- `prosextract` — **Rework** — Process the PROSITE motif database for use by patmatmotifs
- `tfextract` — **Rework** — Process TRANSFAC transcription factor database for use by tfscan

## Modernize — Rework — Legacy prediction methods with enduring scientific value

Keep the biological problem domain, but rework algorithms, reference resources, and output models aggressively.

**Mapped historical/core tools:** 21

**Decision split:** Rework 21

- `antigenic` — **Rework** — Find antigenic sites in proteins
- `btwisted` — **Rework** — Calculate the twisting in a B-DNA sequence
- `cpgreport` — **Rework** — Identify and report CpG-rich regions in nucleotide sequence(s)
- `epestfind` — **Rework** — Find PEST motifs as potential proteolytic cleavage sites
- `equicktandem` — **Rework** — Find tandem repeats in nucleotide sequences
- `etandem` — **Rework** — Find tandem repeats in a nucleotide sequence
- `garnier` — **Rework** — Predict protein secondary structure using GOR method
- `helixturnhelix` — **Rework** — Identify nucleic acid-binding motifs in protein sequences
- `jaspscan` — **Rework** — Scan DNA sequences for transcription factors
- `marscan` — **Rework** — Find matrix/scaffold recognition (MRS) signatures in DNA sequences
- `newcpgreport` — **Rework** — Identify CpG islands in nucleotide sequence(s)
- `newcpgseek` — **Rework** — Identify and report CpG-rich regions in nucleotide sequence(s)
- `patmatmotifs` — **Rework** — Scan a protein sequence with motifs from the PROSITE database
- `pepcoil` — **Rework** — Predict coiled coil regions in protein sequences
- `profit` — **Rework** — Scan one or more sequences with a simple frequency matrix
- `prophecy` — **Rework** — Create frequency matrix or profile from a multiple alignment
- `prophet` — **Rework** — Scan one or more sequences with a Gribskov or Henikoff profile
- `pscan` — **Rework** — Scan protein sequence(s) with fingerprints from the PRINTS database
- `sigcleave` — **Rework** — Report on signal cleavage sites in a protein sequence
- `tfscan` — **Rework** — Identify transcription factor binding sites in DNA sequences
- `tmap` — **Rework** — Predict and plot transmembrane segments in protein sequences

## Modernize — Rework — Protein property and structural-summary utilities

Protein/biophysical summaries remain relevant; older molecular-weight niche commands are likely to drop.

**Family note:** Mixed family: `iep` and `pepdigest` are individually Retain, `psiphi` is Rework, and older molecular-weight utilities are Omit.

**Mapped historical/core tools:** 6

**Decision split:** Retain 2, Rework 1, Omit 3

- `emowse` — **Omit** — Search protein sequences by digest fragment molecular weight
- `iep` — **Retain** — Calculate the isoelectric point of proteins
- `mwcontam` — **Omit** — Find weights common to multiple molecular weights files
- `mwfilter` — **Omit** — Filter noisy data from molecular weights file
- `pepdigest` — **Retain** — Report on protein proteolytic enzyme or reagent cleavage sites
- `psiphi` — **Rework** — Calculates phi and psi torsion angles from protein coordinates

## Modernize — Rework — Command discovery and help-navigation

Replace scattered discovery/help commands with a coherent `emboss-rs` discovery model.

**Family note:** Mixed family: `embossupdate`, `embossversion`, and `tfm` are individually Omit.

**Mapped historical/core tools:** 6

**Decision split:** Rework 3, Omit 3

- `embossdata` — **Rework** — Find and retrieve EMBOSS data files
- `embossupdate` — **Omit** — Checks for more recent updates to EMBOSS
- `embossversion` — **Omit** — Report the current EMBOSS version number
- `seealso` — **Rework** — Find programs with similar function to a specified program
- `tfm` — **Omit** — Display full documentation for an application
- `wossname` — **Rework** — Find programs by keywords in their short description

## Defer — Ontology command group

Omit ontology command surfaces initially, while preserving an extension path for future ontology-aware metadata.

**Mapped historical/core tools:** 24

**Decision split:** Omit 24

- `edamdef` — **Omit** — Find EDAM ontology terms by definition
- `edamhasinput` — **Omit** — Find EDAM ontology terms by has_input relation
- `edamhasoutput` — **Omit** — Find EDAM ontology terms by has_output relation
- `edamisformat` — **Omit** — Find EDAM ontology terms by is_format_of relation
- `edamisid` — **Omit** — Find EDAM ontology terms by is_identifier_of relation
- `edamname` — **Omit** — Find EDAM ontology terms by name
- `godef` — **Omit** — Find GO ontology terms by definition
- `goname` — **Omit** — Find GO ontology terms by name
- `ontocount` — **Omit** — Count ontology term(s)
- `ontoget` — **Omit** — Get ontology term(s)
- `ontogetcommon` — **Omit** — Get common ancestor for terms
- `ontogetdown` — **Omit** — Get ontology term(s) by parent id
- `ontogetobsolete` — **Omit** — Get ontology ontology terms
- `ontogetroot` — **Omit** — Get ontology root terms by child identifier
- `ontogetsibs` — **Omit** — Get ontology term(s) by id with common parent
- `ontogetup` — **Omit** — Get ontology term(s) by id of child
- `ontoisobsolete` — **Omit** — Report whether an ontology term id is obsolete
- `ontotext` — **Omit** — Get ontology term(s) original full text
- `wossdata` — **Omit** — Find programs by EDAM data
- `wossinput` — **Omit** — Find programs by EDAM input data
- `wossoperation` — **Omit** — Find programs by EDAM operation
- `wossoutput` — **Omit** — Find programs by EDAM output data
- `wossparam` — **Omit** — Find programs by EDAM parameter
- `wosstopic` — **Omit** — Find programs by EDAM topic

## Defer — Specialized metadata and semantic lookup utilities

Family remains deferrable in general, but several accession/resource/taxonomy discovery commands are individually promoted to Rework because the user need persists.

**Family note:** Important override family: every mapped historical tool is currently Rework rather than Defer because modern provider-aware metadata lookup remains valuable.

**Mapped historical/core tools:** 18

**Decision split:** Rework 18

- `drfinddata` — **Rework** — Find public databases by data type
- `drfindformat` — **Rework** — Find public databases by format
- `drfindid` — **Rework** — Find public databases by identifier
- `drfindresource` — **Rework** — Find public databases by resource
- `drget` — **Rework** — Get data resource entries
- `drtext` — **Rework** — Get data resource entries complete text
- `seqxref` — **Rework** — Retrieve all database cross-references for a sequence entry
- `seqxrefget` — **Rework** — Retrieve all cross-referenced data for a sequence entry
- `showdb` — **Rework** — Display information on configured databases
- `taxget` — **Rework** — Get taxon(s)
- `taxgetdown` — **Rework** — Get descendants of taxon(s)
- `taxgetrank` — **Rework** — Get parents of taxon(s)
- `taxgetspecies` — **Rework** — Get all species under taxon(s)
- `taxgetup` — **Rework** — Get parents of taxon(s)
- `textget` — **Rework** — Get text data entries
- `textsearch` — **Rework** — Search the textual description of sequence(s)
- `urlget` — **Rework** — Get URLs of data resources
- `variationget` — **Rework** — Get sequence variations

## Exclude Permanently — ACD developer tooling

Pure ACD-era developer/test plumbing; superseded by Rust-native definitions.

**Mapped historical/core tools:** 5

**Decision split:** Omit 5

- `acdc` — **Omit** — Test an application ACD file
- `acdpretty` — **Omit** — Correctly reformat an application ACD file
- `acdtable` — **Omit** — Generate an HTML table of parameters from an application ACD file
- `acdtrace` — **Omit** — Trace processing of an application ACD file (for testing)
- `acdvalid` — **Omit** — Validate an application ACD file

## Exclude Permanently — EMBOSS-era server-cache-registry plumbing

Obsolete remote-server/cache/registry machinery.

**Mapped historical/core tools:** 6

**Decision split:** Omit 6

- `cachedas` — **Omit** — Generate server cache file for DAS servers or for the DAS registry
- `cachedbfetch` — **Omit** — Generate server cache file for Dbfetch/WSDbfetch data sources
- `cacheebeyesearch` — **Omit** — Generate server cache file for EB-eye search domains
- `cacheensembl` — **Omit** — Generate server cache file for an Ensembl server
- `servertell` — **Omit** — Display information about a public server
- `showserver` — **Omit** — Display information on configured servers

## Exclude Permanently — EMBOSS local database indexing administration

Historic local indexing/admin commands tied to EMBOSS’s legacy database layer.

**Mapped historical/core tools:** 16

**Decision split:** Omit 16

- `dbiblast` — **Omit** — Index a BLAST database
- `dbifasta` — **Omit** — Index a fasta file database
- `dbiflat` — **Omit** — Index a flat file database
- `dbigcg` — **Omit** — Index a GCG formatted database
- `dbxcompress` — **Omit** — Compress an uncompressed dbx index
- `dbxedam` — **Omit** — Index the EDAM ontology using b+tree indices
- `dbxfasta` — **Omit** — Index a fasta file database using b+tree indices
- `dbxflat` — **Omit** — Index a flat file database using b+tree indices
- `dbxgcg` — **Omit** — Index a GCG formatted database using b+tree indices
- `dbxobo` — **Omit** — Index an obo ontology using b+tree indices
- `dbxreport` — **Omit** — Validate index and report internals for dbx databases
- `dbxresource` — **Omit** — Index a data resource catalogue using b+tree indices
- `dbxstat` — **Omit** — Dump statistics for dbx databases
- `dbxtax` — **Omit** — Index NCBI taxonomy using b+tree indices
- `dbxuncompress` — **Omit** — Uncompress a compressed dbx index
- `dbtell` — **Omit** — Display information about a public database

## Exclude Permanently — Wrapper-only compatibility commands

Generic utility baggage or legacy compatibility-only surfaces outside the reboot’s scientific core.

**Family note:** Closest-fit bucket also absorbs generic text-cleaning utilities and the obsolete USA/list helper `yank`.

**Mapped historical/core tools:** 6

**Decision split:** Omit 6

- `nohtml` — **Omit** — Remove mark-up (e.g. HTML tags) from an ASCII text file
- `noreturn` — **Omit** — Remove carriage return from ASCII files
- `nospace` — **Omit** — Remove whitespace from an ASCII text file
- `notab` — **Omit** — Replace tabs with spaces in an ASCII text file
- `trimspace` — **Omit** — Remove extra whitespace from an ASCII text file
- `yank` — **Omit** — Add a sequence reference (a full USA) to a list file

## Strategic Add — HMM and probabilistic homology workflows

Modern profile-HMM capability should exist in the reboot, but as contemporary methods rather than EMBOSS wrapper compatibility.

**Family note:** No direct core-app-index tools map here; the appendix ties this family to the explicit Add rows in the scope matrix.

### Strategic additions

- `hmmbuild / hmmsearch / hmmscan / hmmalign` — **Add** — Primary modern profile-HMM capability block.
- `jackhmmer / nhmmer / nhmmscan` — **Add** — Iterative protein search and nucleotide-profile extensions.

### Historical precursors or adjacent tools from the scope matrix

- `emma` — **Rework** — Multiple sequence alignment (ClustalW wrapper)

## Strategic Add — Modern archive-scale raw data ingestion

New ENA/SRA-scale ingest capabilities that were not adequately covered by historical EMBOSS commands.

**Family note:** No direct historical core tool maps cleanly here; related historical precursors are listed alongside the explicit Add rows.

### Strategic additions

- `ena_get` — **Add** — Accession-first ENA record and metadata retrieval.
- `ena_fetch_runs` — **Add** — Bulk ENA run / assembly / file retrieval.
- `sra_fetch_runs` — **Add** — Bulk SRA run download workflow.
- `sra_fetch_original` — **Add** — Original submitted-file retrieval where available.

### Historical precursors or adjacent tools from the scope matrix

- `assemblyget` — **Rework** — Get assembly of sequence reads
- `entret` — **Rework** — Retrieve sequence entries from flatfile databases and files
- `refseqget` — **Rework** — Get reference sequence
- `seqret` — **Rework** — Read and write (return) sequences
- `whichdb` — **Rework** — Search all sequence databases for an entry and retrieve it
- `infoassembly` — **Rework** — Display information about assemblies

## Cross-check notes

- Every historical/core tool from the scope matrix, plus `complex`, appears exactly once in this appendix.
- The two Strategic Add families are anchored to the explicit Add rows from the scope matrix rather than to historical/core EMBOSS commands.
- Where a family contains mixed decisions, the per-tool decision in the scope matrix takes precedence over the family default in the governance policy.
