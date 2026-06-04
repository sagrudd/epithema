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

#### `isochore` method-level acceptance criteria

Before code changes begin for the selected bounded continuation method,
`isochore` should have explicit method-level acceptance criteria recorded as
follows.

##### Analytical output expectations

- the method should accept bounded nucleotide-sequence input only
- Rust should compute the analytical `isochore` profile directly rather than
  delegating analytical work to the renderer
- the analytical model should be described honestly as a bounded
  region-oriented nucleotide-composition profile, not as a generic plotting
  framework extension
- the primary output should remain a stable analytical table with one row per
  emitted window, span, or bucket and explicit columns sufficient to
  reconstruct the plotted continuation line
- the analytical output should stay inside the same table-first contract style
  already used by the governed plotting seam

##### Typed contract expectations

- the same run should emit a deterministic typed plot contract derived from the
  analytical table output
- the contract should remain single-series unless the analytical needs of
  `isochore` itself make broader structure unavoidable
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
  scope merely because `isochore` ships
- no broader plot-contract taxonomy unless the analytical needs of `isochore`
  make that pressure concrete enough to stop and reassess

If `isochore` cannot satisfy these criteria while remaining a bounded
method-associated plotting method, the repository should pause and reassess
before starting the next code-bearing continuation patch.

#### Exact start conditions for the first `isochore` implementation patch

The first code-bearing `isochore` patch should not begin until all of the
following are treated as explicit start conditions:

- the active family decision still remains:
  - plotting first
  - remote retrieval second
  - protein-property rework third
- bounded plotting Phase 1 has already passed explicit reassessment:
  - `hmoment`
  - `octanol`
  - `pepinfo`
- the post-`wobble` continuation gate has already passed:
  - bounded plotting continuation remains viable
  - `isochore` is the single selected next bounded continuation candidate
- the release-truth surface still remains in the current zero-burden state:
  - `full_compared_cohort == true`
  - `harvest_coverage_complete == true`
  - `retained_backlog_closed == true`
  - `gapped_method_count == 0`
  - `weakest_evidence_family == null`
  - `release_truth_current == true`
- the patch scope remains limited to `isochore` and the smallest shared
  support needed for:
  - deterministic analytical computation
  - typed plot-contract emission
  - governed docs and validation plumbing
- the patch does not widen into:
  - broader plotting-family members beyond `isochore`
  - Rust-side rendering behavior
  - a generalized plotting framework
  - broader plot-contract taxonomies unless `isochore` itself makes that
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

#### Bounded `isochore` implementation tier

Because `isochore` is now the single selected next bounded continuation
candidate, the repository should map its full method-associated implementation
tier explicitly before code starts.

The bounded `isochore` tier should be:

1. implement the bounded analytical core for `isochore`
2. add the typed `isochore` plot-contract emission path
3. expose `isochore` through the governed shipped surface
4. add canonical analytical and plot-contract fixtures plus compared evidence
5. re-run the full release-truth surface after shipping `isochore`
6. reassess the shipped `isochore` slice before any further continuation is
   mapped

The bounded `isochore` tier should preserve the same architectural
constraints already enforced for the earlier plotting continuation slices:

- method-associated implementation only
- table-first analytical output
- typed contract output from the same computation path
- no Rust-side rendering
- no generic plotting-framework widening
- no broader plot-contract taxonomy unless `isochore` itself makes that
  pressure concrete enough to stop and reassess

So the next code-bearing work is not an abstract "continue plotting" step. It
should begin only as this explicit bounded `isochore` tier.

#### Pre-code seam-pressure stop conditions for `isochore`

Before the first `isochore` code patch starts, the repository should make the
pause-and-reconsider conditions explicit rather than treating them as implied.

The repository should pause and reopen planning if any of the following
becomes true during the pre-code review:

1. `isochore` cannot remain table-first with a typed contract derived from the
   same computation path
2. `isochore` requires Rust-side rendering behavior, layout policy, styling,
   or other presentation logic
3. `isochore` cannot remain method-associated and instead demands a
   generalized plotting framework before one bounded shipped slice closes
4. `isochore` forces region-track, threshold-call, segmentation-call, or
   broader plot-contract taxonomy pressure that is not clearly local to the
   method

If one of these conditions becomes true, the repository should not force-fit
`isochore` through the bounded seam. It should stop and reassess whether the
active plotting branch still remains the right next implementation path.

#### Reassessment after the shipped `isochore` slice

After the bounded `isochore` tier closed as a shipped governed slice, the
repository should explicitly record whether the bounded plotting seam still
remains credible before mapping any further continuation.

The observed answer after the shipped `isochore` slice is still yes.

That conclusion is grounded in the actual shipped seam behavior:

- `isochore` stayed method-associated
- the analytical surface remained table-first and kept the richer GC, AT, and
  bounded isochore-band rows explicit rather than collapsing the method into a
  plot-only surface
- the emitted contract stayed bounded to a single derived GC-percent series
- no Rust-side rendering, presentation policy, or layout logic entered the
  Rust surface
- no generalized plotting-framework pressure emerged
- no retrieval-fallback activation stop condition tripped

The current generated release-truth surface also remained fully green after
the shipped `isochore` slice closed:

- shipped methods: `102`
- compared evidence: `102`
- executable evidence: `0`
- harvested legacy provenance present: `102`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

So the repository should remain on the plotting continuation branch at this
checkpoint, with the prepared retrieval fallback still documented and ready
but inactive unless a later bounded candidate trips the explicit stop
conditions.

#### Post-`isochore` plotting-family remainder inventory

Because `isochore` has now closed as another shipped bounded continuation
slice, any further plotting continuation should start from an explicit rebased
remainder set rather than from the older post-`wobble` shortlist.

The governed plotting seam and bounded shipped continuation set now excludes:

- governed seam precedents:
  - `charge`
  - `pepwindow`
- bounded shipped plotting-family continuations:
  - `hmoment`
  - `octanol`
  - `pepinfo`
  - `density`
  - `wobble`
  - `isochore`

So the explicit post-`isochore` plotting-family remainder pool is now the
remaining `20` plotting-family methods:

- `abiview`
- `banana`
- `chaos`
- `cirdna`
- `cpgplot`
- `dotmatcher`
- `dotpath`
- `dottup`
- `findkm`
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

This inventory remains governance-only. It does not yet reclassify seam
pressure or choose the next bounded continuation candidate.

#### Post-`isochore` seam-pressure reclassification

Now that `isochore` has shipped as another bounded continuation slice, the
remainder can be reclassified against a more concrete seam than the earlier
post-`wobble` shortlist.

The key new signal from `isochore` is:

- the bounded plotting seam can still support a nucleotide analytical profile
  that keeps richer categorical information in the table while emitting one
  derived continuation line
- but the method still stayed table-first, method-associated, and free of
  renderer-coupled region-track or segmentation-framework pressure

That means the strongest remaining seam-compatible continuation candidates are
now:

- `banana`
- `syco`

`cpgplot` remains plausible, but only as a more conditional region-oriented
candidate whose surface may still drift toward threshold or track pressure.

The explicit post-`isochore` remainder buckets are therefore:

- strongest seam-compatible continuation candidates:
  - `banana`
  - `syco`
- region- or threshold-conditional candidate:
  - `cpgplot`
- broader contract-taxonomy candidates:
  - `chaos`
  - `pepwindowall`
  - `plotcon`
- dotplot or comparative-matrix heavy:
  - `dotmatcher`
  - `dotpath`
  - `dottup`
  - `polydot`
- diagram, layout, or presentation-heavy:
  - `cirdna`
  - `lindna`
  - `pepnet`
  - `pepwheel`
  - `prettyplot`
  - `showfeat`
  - `showpep`
  - `showseq`
- specialized laboratory-trace or kinetic plotting:
  - `abiview`
  - `findkm`

This reclassification remains governance-only. It narrows the strongest
next-candidate pool more sharply than the post-`wobble` view, but it does not
yet choose the next bounded continuation candidate.

#### Post-`isochore` bounded-candidate viability decision

After the shipped `isochore` slice and the rebased post-`isochore`
seam-pressure reclassification, the repository can now close the next
pass/fail question explicitly: does another bounded plotting continuation
candidate still exist, or has the seam narrowed to the point where the
prepared retrieval fallback should activate?

The decision is affirmative. Another bounded plotting continuation candidate
still exists after `isochore`.

The current viable post-`isochore` continuation pool is:

- `banana`
- `syco`

So the no-candidate branch is not taken here. The repository remains on the
plotting continuation path, and the prepared retrieval fallback remains
documented, ready, and inactive unless a later selection or shipment gate
trips the explicit stop conditions.

That means the conditional “activate retrieval fallback now” branch is closed
as untriggered at this checkpoint. The repository should not promote the
fallback by inertia when the narrowed plotting continuation pool remains
credible.

#### Untriggered retrieval-activation branch after post-`isochore` viability

Because the post-`isochore` viability gate remained affirmative, the prepared
retrieval fallback should remain documented and ready but inactive at this
checkpoint.

The repository should therefore not map a retrieval implementation start or a
retrieval lead-method decision here. The active planning branch remains the
bounded plotting continuation path until a later candidate-selection or
shipment gate trips the explicit fallback-activation stop conditions.

#### Post-`isochore` bounded-candidate selection

Because the post-`isochore` viability gate still leaves a non-empty narrowed
continuation pool, the repository should choose exactly one next bounded
plotting continuation candidate before any further implementation planning
starts.

The selected candidate is:

- `banana`

The selection basis is:

- `banana` looks like the closest remaining extension of the proven bounded
  nucleotide plotting seam as an analytical, table-first profile with a likely
  single derived continuation line
- it appears less likely than `syco` to force codon-usage-specific structure,
  coding-sequence-only assumptions, or heavier translation-adjacent coupling
- it still looks compatible with the established renderer-agnostic typed
  contract seam rather than immediately pushing toward a broader taxonomy

The non-selected viable method remains documented but inactive:

- `syco`

So the next bounded planning step should be to capture `banana`-specific
acceptance criteria and exact patch start conditions before any code starts.

That also keeps the retrieval-activation branch closed as untriggered after
candidate selection. Retrieval fallback remains documented, prepared, and
inactive while the active bounded `banana` continuation branch remains
credible.

#### Untriggered branch closeout after post-`isochore` candidate selection

Because `banana` has now been selected as the single active bounded
continuation candidate, the repository should close the opposite branch
explicitly rather than leaving it implied.

At this checkpoint:

- the plotting-continuation branch remains active
- the prepared retrieval fallback remains documented, prepared, and inactive
- the non-selected viable method `syco` remains documented but inactive

So no retrieval implementation tier or retrieval lead-method choice should be
mapped here. The next bounded planning step remains `banana`-specific method
acceptance criteria and patch start conditions.

#### `banana` method-level acceptance criteria

Before code changes begin for the selected bounded continuation method,
`banana` should have explicit method-level acceptance criteria recorded as
follows.

##### Analytical output expectations

- the method should accept bounded nucleotide-sequence input only
- Rust should compute the analytical `banana` profile directly rather than
  delegating analytical work to the renderer
- the analytical model should be described honestly as a bounded
  bendability-or-curvature-oriented continuation profile, not as a generic
  plotting framework extension
- the analytical surface should remain table-first and stable, with enough
  explicit columns to reconstruct the plotted continuation line from the same
  computation path

##### Typed contract expectations

- the emitted plot contract should remain typed, deterministic, and derived
  from the same governed analytical computation path
- the first shipped slice should remain single-series unless the analytical
  model proves that a broader contract is method-locally unavoidable
- Rust should not own figure rendering, layout policy, styling, legend policy,
  or presentation defaults

##### Fixture and evidence expectations

- the method should ship with governed autodoc and generated validation
  metadata
- the method should have canonical checked-in analytical fixtures
- the method should have a canonical checked-in plot-contract fixture
- the method should close with compared evidence for both table and contract
  outputs

##### Explicit non-goals

- no Rust-side figure rendering
- no silent widening into a generic plotting framework
- no implicit promotion into a broader plot-contract taxonomy unless the
  analytical needs of `banana` itself make that pressure unavoidable
- no family-wide continuation claims merely because `banana` ships

#### Exact start conditions for the first `banana` implementation patch

The first code-bearing `banana` patch should not begin until all of the
following remain true:

- the current shortlist still remains:
  1. plotting
  2. remote retrieval
  3. protein-property rework
- bounded plotting Phase 1 still remains passed:
  - `hmoment`
  - `octanol`
  - `pepinfo`
- the post-`isochore` continuation gate has already passed:
  - `banana` is the single selected next bounded continuation candidate
- the zero-burden release-truth surface still remains intact
- the patch scope remains limited to `banana` and the smallest shared support
  needed for:
  - deterministic analytical computation
  - typed contract emission
  - governed docs and validation plumbing
- the patch lands as a full governed slice rather than a half-start

That means the first `banana` patch should not widen immediately into:

- `syco`
- retrieval fallback work
- generalized plotting-framework seams
- broader plot-contract taxonomies unless `banana` itself makes that pressure
  concrete enough to stop and reassess

#### Bounded `banana` implementation tier

Because `banana` is now the single selected next bounded continuation
candidate, the repository should map its full method-associated implementation
tier explicitly before code starts.

The bounded `banana` tier should be:

1. implement the bounded analytical core for `banana`
2. add the typed `banana` plot-contract emission path
3. expose `banana` through the governed shipped surface
4. add canonical analytical and plot-contract fixtures plus compared evidence
5. re-run the full release-truth surface after shipping `banana`
6. reassess the shipped `banana` slice before any further continuation is
   mapped

The bounded `banana` tier should preserve the same architectural constraints
already enforced for the earlier plotting continuation slices:

- method-associated implementation only
- table-first analytical output
- typed contract output from the same computation path
- no Rust-side rendering
- no generic plotting-framework widening
- no broader plot-contract taxonomy unless `banana` itself makes that pressure
  concrete enough to stop and reassess

So the next code-bearing work is not an abstract “continue plotting” step. It
should begin only as this explicit bounded `banana` tier.

#### Explicit seam-pressure stop conditions for `banana`

Before code changes begin for `banana`, the repository should treat the
following as explicit pause-and-reassess conditions.

1. `banana` cannot remain table-first with a typed contract derived from the
   same governed computation path
2. `banana` requires Rust-side rendering, layout, styling, or other
   presentation policy to make the shipped slice coherent
3. `banana` cannot remain method-associated and instead demands a generalized
   plotting framework before one bounded shipped slice closes
4. `banana` forces curvature-track, region-call, threshold-call, or broader
   plot-contract taxonomy pressure that is not clearly local to the method

If any of those conditions becomes true before code starts, the repository
should stop the `banana` continuation path and reopen planning rather than
continuing under an implicitly widened plotting seam.

#### Post-ship reassessment of the bounded `banana` slice

After the bounded `banana` slice shipped and the compared-evidence closure
completed, the repository should record whether the method still fits the
proven plotting seam before any further continuation is mapped.

The result for `banana` is affirmative.

- the shipped `banana` slice stayed bounded and method-associated
- the analytical surface remained table-first and retained the richer local
  bend and curvature columns in the governed table output
- the emitted contract stayed bounded to a single derived curvature
  continuation line over the analytically defined positions
- no Rust-side rendering, generalized plotting-framework pressure, or broader
  non-local plot-contract taxonomy pressure emerged
- no retrieval-fallback activation condition tripped

The governed release-truth surface also remained fully green at this
checkpoint:

- shipped methods: `103`
- compared evidence: `103`
- executable evidence: `0`
- harvested legacy provenance present: `103`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

So `banana` should be treated as having passed its shipped-slice
reassessment, and any further plotting continuation should be planned from
that observed bounded state rather than from pre-implementation assumptions.

#### Post-`banana` plotting-family remainder inventory

After the bounded `banana` slice shipped and passed reassessment, the plotting
continuation pool should be rebased again onto the actual shipped state rather
than left on the older post-`isochore` shortlist.

The plotting-family remainder now excludes:

- the two preexisting governed seam precedents:
  - `charge`
  - `pepwindow`
- the bounded shipped plotting-family continuations:
  - `hmoment`
  - `octanol`
  - `pepinfo`
  - `density`
  - `wobble`
  - `isochore`
  - `banana`

So the explicit post-`banana` plotting-family continuation pool is now the
remaining `19` plotting-family methods:

- `abiview`
- `chaos`
- `cirdna`
- `cpgplot`
- `dotmatcher`
- `dotpath`
- `dottup`
- `findkm`
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

This checkpoint is inventory only. It does not yet reclassify the remainder by
seam pressure, and it does not yet decide whether the final narrowed plotting
continuation candidate still passes an honest seam review.

#### Reclassification of the post-`banana` remainder by seam pressure

Now that the repository has shipped and reassessed `banana`, the post-`banana`
remainder can be reclassified against a still tighter proven plotting seam
than the earlier post-`isochore` view.

The strongest remaining seam-compatible continuation candidate is now:

- `syco`

`cpgplot` remains the only plausibly arguable adjacent case, but only as a
more conditional region-oriented candidate rather than as part of the current
narrowed active continuation pool.

The explicit post-`banana` remainder buckets are therefore:

- strongest remaining seam-compatible continuation candidate:
  - `syco`
- conditional region-oriented adjacent case:
  - `cpgplot`
- broader contract-taxonomy pressure:
  - `chaos`
  - `pepwindowall`
  - `plotcon`
- dotplot or comparative-matrix heavy:
  - `dotmatcher`
  - `dotpath`
  - `dottup`
  - `polydot`
- diagram, layout, or presentation-heavy:
  - `cirdna`
  - `lindna`
  - `pepnet`
  - `pepwheel`
  - `prettyplot`
  - `showfeat`
  - `showpep`
  - `showseq`
- specialized laboratory-trace or kinetic plotting:
  - `abiview`
  - `findkm`

This checkpoint still does not choose the next continuation action by itself.
It narrows the final candidate pool more sharply than the post-`isochore`
view, but it does not yet decide whether `syco` still passes an honest seam
review strongly enough to remain active instead of activating the prepared
retrieval fallback.

#### Pass/fail gate for the final narrowed plotting continuation candidate

After the post-`banana` seam-pressure reclassification, the repository should
close the pass/fail gate on whether the last narrowed plotting candidate still
passes an honest seam review strongly enough to remain active.

The decision at this checkpoint is affirmative.

- a final bounded plotting continuation candidate still remains credible after
  `banana`
- the current viable final continuation candidate is:
  - `syco`
- the no-candidate branch is therefore not taken here
- the prepared retrieval fallback remains documented, ready, and inactive

So the repository should stay on the plotting continuation branch for the next
bounded planning step, which is now simply whether to activate `syco` as the
final active candidate or to trigger the fallback only if that final
continuation later fails its own bounded seam review.

#### Untriggered fallback-activation branch after final-candidate viability

Because the final-candidate viability gate remained affirmative, the prepared
retrieval fallback should not be promoted at this checkpoint.

The recorded outcome is therefore:

- the plotting-continuation branch remains active
- the prepared retrieval fallback remains documented, ready, and inactive
- retrieval is not promoted by inertia while `syco` remains the final bounded
  active continuation candidate

So no retrieval implementation tier or retrieval lead-method choice should be
mapped here. The next bounded planning step remains the explicit activation of
`syco` as the final active continuation candidate.

#### Activation of the final bounded plotting continuation candidate

Because the final-candidate viability gate remained affirmative and the
fallback branch stayed untriggered, the repository should now explicitly
activate `syco` as the single final bounded plotting continuation candidate.

That activation is justified because:

- `syco` is now the only remaining seam-compatible continuation candidate in
  the narrowed post-`banana` pool
- the prepared retrieval fallback remains documented, ready, and inactive
  because the no-candidate branch was not taken
- no other plotting-family method remains comparably bounded without heavier
  contract, presentation, or comparative-matrix pressure

So the next bounded planning step should now become `syco`-specific rather
than generic:

- capture method-level acceptance criteria
- capture exact patch start conditions
- capture explicit seam-pressure stop conditions

Only after those are made explicit should the repository map and ship the
final bounded `syco` slice.

#### `syco` method-level acceptance criteria

Before code changes begin for the final bounded continuation method, `syco`
should have explicit method-level acceptance criteria recorded as follows.

##### Analytical output expectations

- the method should accept bounded coding nucleotide-sequence input only
- Rust should compute the analytical `syco` surface directly rather than
  delegating analytical work to the renderer
- the analytical model should be described honestly as a bounded synonymous
  codon-usage continuation profile rather than as a generic plotting-framework
  extension
- the analytical surface should remain table-first and stable, with enough
  explicit columns to reconstruct the plotted continuation line from the same
  computation path

##### Typed contract expectations

- the emitted plot contract should remain typed, deterministic, and derived
  from the same governed analytical computation path
- the first shipped slice should remain single-series unless the analytical
  needs of `syco` itself prove a broader local structure is unavoidable
- even if a broader local structure becomes necessary, it should still remain
  method-associated, table-derived, and renderer-agnostic rather than widening
  into a generic plotting framework
- Rust should not own figure rendering, layout policy, styling, legend
  policy, or presentation defaults

##### Fixture and evidence expectations

- the method should ship with governed autodoc and generated validation
  metadata
- the method should have canonical checked-in analytical fixtures
- the method should have a canonical checked-in plot-contract fixture
- the method should close with compared evidence for both table and contract
  outputs

##### Explicit non-goals

- no Rust-side figure rendering
- no silent widening into a generic plotting framework
- no broader plot-contract taxonomy unless the analytical needs of `syco`
  itself make that pressure unavoidable enough to force a real local
  reassessment
- no family-wide continuation claims merely because `syco` ships

#### Exact start conditions for the first `syco` implementation patch

The first code-bearing `syco` patch should not begin until all of the
following remain true:

- the current shortlist still remains:
  1. plotting
  2. remote retrieval
  3. protein-property rework
- bounded plotting Phase 1 still remains passed:
  - `hmoment`
  - `octanol`
  - `pepinfo`
- the bounded Phase 2 and later continuation gates still remain passed:
  - `density`
  - `wobble`
  - `isochore`
  - `banana`
- the post-`banana` final-candidate viability gate has already passed:
  - `syco` is the single selected final bounded continuation candidate
- the zero-burden release-truth surface still remains intact
- the patch scope remains limited to `syco` and the smallest shared support
  needed for:
  - deterministic analytical computation
  - typed contract emission
  - governed docs and validation plumbing
- the patch lands as a full governed slice rather than a half-start

That means the first `syco` patch should not widen immediately into:

- retrieval fallback work
- generalized plotting-framework seams
- broader plot-contract taxonomies unless `syco` itself makes that pressure
  concrete enough to stop and reassess
- any implicit continuation beyond the final bounded `syco` slice

Only when those start conditions still hold should code work begin for the
final bounded plotting continuation candidate.

#### Full bounded `syco` implementation tier

Before code work begins for the final bounded plotting continuation candidate,
the complete bounded `syco` implementation tier should be recorded explicitly
as follows:

1. implement the bounded analytical core
2. add the typed plot-contract emission path
3. expose `syco` through the governed shipped surface
4. add canonical analytical and plot-contract fixtures plus compared evidence
5. re-run the full release-truth surface after shipping `syco`
6. reassess the shipped `syco` slice before any further continuation is mapped

That tier should remain bounded by the same constraints already established for
the final continuation candidate:

- method-associated implementation only
- table-first analytical output
- typed contract output derived from the same computation path
- no Rust-side rendering
- no silent widening into a generic plotting framework
- no broader plot-contract taxonomy unless `syco` itself forces a real local
  reassessment
- no implicit continuation beyond the final bounded `syco` slice

#### Explicit seam-pressure stop conditions for `syco`

Before code begins for the final bounded continuation method, the repository
should pause and reopen planning if any of the following becomes true:

1. `syco` cannot remain table-first with a typed contract derived from the
   same computation path
2. `syco` requires Rust-side rendering, layout, styling, or other
   presentation-policy logic
3. `syco` cannot remain method-associated and instead demands a generalized
   plotting framework before one bounded shipped slice closes
4. `syco` forces codon-usage panelization, coding-region segmentation,
   threshold-call behavior, or broader plot-contract taxonomy pressure that is
   not clearly local to the method

Those stop conditions should be treated as planning boundaries, not as
implementation details to absorb informally into the first `syco` patch.

#### Reassessment after the shipped `syco` slice

After the bounded `syco` slice shipped, closed its compared-evidence gap, and
passed the release-truth rerun, the repository should make the post-ship seam
assessment explicit before mapping any further continuation.

The reassessment result is affirmative.

- the shipped `syco` slice stayed bounded and method-associated
- the analytical surface stayed coding-sequence-specific and table-first
- the richer codon-window scoring surface remained in the analytical table
- the emitted contract stayed bounded to a single derived `syco_score` series
- no Rust-side rendering pressure emerged
- no generalized plotting-framework pressure emerged
- no fallback-activation stop condition tripped

The governed release-truth surface remained fully green after the shipped
`syco` slice closed:

- shipped methods: `104`
- compared evidence: `104`
- executable evidence: `0`
- harvested legacy provenance present: `104`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

So `syco` should be treated as having passed its shipped-slice reassessment
gate, and any next roadmap extension should be derived from that observed
post-closure state rather than from the earlier executable-only interim.

#### Closure of the bounded plotting continuation branch

After the shipped `syco` slice closed cleanly, the repository should make the
branch-resolution outcome explicit: the bounded plotting continuation program
is now exhausted as an active branch.

That conclusion follows from the full observed bounded plotting path:

- bounded Phase 1 closed and passed reassessment:
  - `hmoment`
  - `octanol`
  - `pepinfo`
- bounded continuation slices also closed and passed reassessment:
  - `density`
  - `wobble`
  - `isochore`
  - `banana`
  - `syco`

The resulting planning consequence is explicit:

- no comparably bounded plotting continuation candidate remains active after
  the final selected `syco` slice
- continued plotting should no longer be treated as the default next planning
  branch
- the prepared remote-retrieval fallback should now become the active next
  planning program by explicit branch resolution
- protein-property rework remains documented but stays third until the
  retrieval branch is reassessed

The governed release-truth surface still remains fully green at this branch
closure point:

- shipped methods: `104`
- compared evidence: `104`
- executable evidence: `0`
- harvested legacy provenance present: `104`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Activation of the remote-retrieval fallback as the next active planning program

Once the bounded plotting continuation branch is closed explicitly, the
prepared remote-retrieval fallback should become the active next planning
program rather than remaining a merely documented contingency.

That activation should be treated as a formal branch-resolution step, not as a
family-order rewrite:

- plotting completed its bounded continuation path and no longer remains an
  active continuation branch
- remote retrieval now becomes the next active planning program because it is
  the already-prepared bounded fallback with explicit method candidates,
  provider constraints, and evidence expectations
- protein-property rework remains third and is not promoted by this step

The active bounded retrieval planning subset remains:

- `seqretsetall`
- `seqretsplit`
- `infoassembly`

The activation does not widen retrieval scope beyond the already-governed
fallback posture:

- no broad provider-parity claims
- no implicit live-network validation
- no automatic promotion of broader historical retrieval members such as
  `assemblyget`, `whichdb`, or `entret`

So after the shipped `syco` closeout, the repository should treat
remote-retrieval planning as active and bounded, with protein-property
rework still held in reserve behind it.

#### Reconfirmation that protein-property rework still remains the third candidate after retrieval activation

After the repository explicitly closes the bounded plotting continuation
branch and activates the prepared remote-retrieval fallback as the next active
planning program, protein-property rework should still remain the third
candidate in the shortlist.

That conclusion remains explicit because:

- retrieval now has the strongest active bounded plan after plotting closure:
  - `seqretsetall`
  - `seqretsplit`
  - `infoassembly`
- retrieval also preserves a clearer operational seam than protein-property at
  this checkpoint:
  - explicit provider-aware orchestration
  - deterministic mocked-provider or managed-asset validation
  - compared evidence on normalized returned sequence or metadata outputs
- protein-property rework still has a credible scientific substrate, but it
  still lacks the same immediate bounded lead-method activation detail now
  attached to the active retrieval branch

So the shortlist should remain:

1. completed bounded plotting continuation program
2. active remote-retrieval planning program
3. protein-property rework

This remains a planning checkpoint only. It does not promote protein-property
work or weaken the explicit retrieval activation decision.

#### Bounded retrieval lead-method selection after retrieval activation

After activating remote retrieval as the next planning program and
reconfirming that protein-property remains third, the repository should choose
exactly one bounded retrieval lead method from the already-governed subset:

- `seqretsetall`
- `seqretsplit`
- `infoassembly`

The selected lead method should be:

- `seqretsetall`

That choice is the narrowest and most defensible bounded continuation because:

- it stays closest to the already-shipped governed retrieval slice by
  extending the normalized sequence-return path already proven by `seqret`
- it appears easier to keep deterministic under mocked-provider or
  managed-asset validation than a split-output filesystem-oriented lead path
- it avoids starting retrieval Phase 1 with the broader assembly metadata
  shape and provider-surface questions that would come with `infoassembly`

The non-selected bounded retrieval candidates should remain documented but
inactive:

- `seqretsplit`
- `infoassembly`

#### Method-level acceptance criteria for `seqretsetall`

After selecting `seqretsetall` as the bounded retrieval lead method, the
repository should make its acceptance criteria explicit before code starts.

The governed criteria should be:

- bounded provider-aware many-set retrieval/write workflow only
- deterministic normalized sequence-return output built on the same governed
  retrieval substrate already proven by `seqret`
- stable output surface with explicit source and grouping behavior defined by
  the same computation path rather than by ad hoc filesystem side effects
- canonical managed-asset or mocked-provider fixtures
- compared evidence on normalized returned sequence sets rather than on
  orchestration intent alone

The non-goals should remain explicit:

- no hidden live-network validation
- no broad provider-parity claims
- no generic retrieval-family widening merely because `seqretsetall` ships

#### Exact patch start conditions for `seqretsetall`

After making `seqretsetall` the active bounded retrieval lead method and
capturing its method-level acceptance criteria, the repository should make the
first implementation-patch start conditions explicit.

The start gate should require:

- the current shortlist to remain intact:
  1. completed bounded plotting continuation program
  2. active remote-retrieval planning program
  3. protein-property rework
- the bounded retrieval planning subset to remain limited to:
  - `seqretsetall`
  - `seqretsplit`
  - `infoassembly`
- `seqretsetall` to remain the single selected bounded retrieval lead method
- the zero-burden release-truth surface to remain intact
- the first patch to stay limited to `seqretsetall` plus the smallest support
  needed for deterministic provider-aware orchestration, normalized
  sequence-return output, and governed docs/validation plumbing
- the patch to land as a full governed slice rather than a half-start

The same guardrails should remain explicit:

- no hidden live-network dependencies
- no broad provider-parity claims
- no retrieval-family widening beyond the selected lead-method slice

#### Bounded implementation tier for `seqretsetall`

After making `seqretsetall` the active bounded retrieval lead method and
capturing its exact patch start conditions, the repository should map the full
bounded implementation tier explicitly before code starts.

The bounded `seqretsetall` tier should be:

1. implement the bounded provider-aware orchestration and normalized many-set
   return core
2. expose the governed output surface for deterministic many-set
   retrieval/write behavior
3. expose `seqretsetall` through the governed shipped surface
4. add canonical managed-asset or mocked-provider fixtures plus compared
   evidence
5. re-run the full release-truth surface after shipping `seqretsetall`
6. reassess the shipped `seqretsetall` slice before any further retrieval
   continuation is mapped

#### Provider and seam stop conditions for `seqretsetall`

After selecting `seqretsetall` as the bounded retrieval lead method and
mapping its implementation tier, the repository should make the pre-code stop
conditions explicit.

Planning should pause and reopen before code starts if:

1. `seqretsetall` cannot remain deterministic under mocked-provider or
   managed-asset validation
2. `seqretsetall` requires hidden live-network dependencies, implicit
   provider fallback chains, or unclear provider precedence
3. `seqretsetall` cannot remain a bounded extension of the normalized
   `seqret` return path and instead demands broader retrieval-family
   orchestration before one shipped slice closes
4. `seqretsetall` forces broad filesystem-policy, batching-policy, or
   provider-parity claims that are not clearly local to the method

#### Reassessment of the shipped `seqretsetall` slice

After shipping `seqretsetall`, closing its compared-evidence slice, and
rerunning the full release-truth surface, the repository should reassess
whether the shipped retrieval seam stayed bounded enough to continue from
honestly.

The result is affirmative:

- the shipped `seqretsetall` slice stayed bounded, provider-aware, and
  deterministic
- it remained a local extension of the normalized `seqret` return path rather
  than forcing broader retrieval-family orchestration
- it did not require hidden live-network behavior, implicit provider fallback
  chains, or unclear provider precedence
- it did not force broad filesystem-policy, batching-policy, or provider-parity
  claims beyond the local method slice

The governed release-truth surface remained fully green:

- shipped methods: `105`
- compared evidence: `105`
- executable evidence: `0`
- harvested legacy provenance present: `105`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Rebasing the bounded retrieval continuation pool after `seqretsetall`

After the first bounded retrieval continuation slice closes cleanly through
`seqretsetall`, the repository should rebase the active continuation pool onto
the actual post-ship state before making any further viability or lead-method
decision.

The resulting bounded retrieval continuation inventory is:

- shipped bounded retrieval slice:
  - `seqretsetall`
- remaining bounded retrieval continuation pool:
  - `seqretsplit`
  - `infoassembly`

This checkpoint is inventory only:

- it does not yet decide whether both remaining methods still pass honest seam
  review strongly enough to remain active continuation candidates
- it does not promote protein-property rework
- it does not widen retrieval scope beyond the already-governed bounded pool

#### Post-`seqretsetall` viability decision for bounded retrieval continuation

After rebasing the bounded retrieval continuation pool onto the actual
post-`seqretsetall` state, the repository should decide explicitly whether a
further bounded retrieval continuation candidate still exists or whether the
retrieval branch should stop and hand off to protein-property rework.

The decision is affirmative:

- another bounded retrieval continuation candidate still exists after
  `seqretsetall`
- the no-candidate branch is not taken here
- retrieval therefore remains the active planning program

The current viable bounded continuation pool remains:

- `seqretsplit`
- `infoassembly`

Protein-property rework therefore remains the reserve third program and should
not be promoted at this checkpoint.

#### Explicit closeout of the untriggered protein-property promotion branch

Because the post-`seqretsetall` retrieval viability gate stayed affirmative,
the opposite branch should now be closed explicitly rather than left implicit.

The resulting branch consequence is:

- retrieval continuation remains the active planning program
- protein-property rework remains documented as the reserve third program
- protein-property is not promoted by inertia while bounded retrieval
  continuation still has viable candidates

The active viable bounded continuation pool remains:

- `seqretsplit`
- `infoassembly`

#### Bounded retrieval continuation lead-method selection after `seqretsetall`

After keeping retrieval active through the post-`seqretsetall` viability gate
and closing the untriggered protein-property branch, the repository should
choose exactly one next bounded retrieval continuation candidate from the
narrowed viable pool.

The selected bounded continuation candidate should be:

- `seqretsplit`

That choice is the narrowest and most defensible continuation because:

- it stays closest to the already-shipped governed retrieval slice by
  extending normalized sequence-return behavior with deterministic split-output
  partitioning
- it appears easier to keep deterministic under mocked-provider or
  managed-asset validation than `infoassembly`
- it avoids starting the next retrieval slice with the broader assembly
  metadata shape and provider-surface questions that `infoassembly` would
  introduce

The non-selected bounded continuation candidate should remain documented but
inactive:

- `infoassembly`

#### Method-level acceptance criteria for `seqretsplit`

After selecting `seqretsplit` as the bounded retrieval continuation lead
method, the repository should make its acceptance criteria explicit before
code starts.

The governed criteria should be:

- bounded provider-aware split-output retrieval workflow only
- deterministic normalized sequence-return output built on the same governed
  retrieval substrate as `seqret` and `seqretsetall`
- stable partitioned output surface with explicit file-naming and grouping
  behavior defined by the same computation path
- canonical managed-asset or mocked-provider fixtures
- compared evidence required on normalized split-output sequence sets, not
  just orchestration intent

The non-goals should remain explicit:

- no hidden live-network validation
- no broad provider-parity claims
- no implicit promotion of `infoassembly`
- no generic retrieval-family widening merely because `seqretsplit` ships

#### Exact patch start conditions for `seqretsplit`

After selecting `seqretsplit` as the bounded retrieval continuation lead
method and capturing its method-level acceptance criteria, the repository
should make the exact start conditions explicit before code begins.

The start gate should require:

- the current shortlist to remain intact:
  1. completed bounded plotting continuation program
  2. active remote-retrieval planning program
  3. protein-property rework
- the bounded retrieval continuation pool to remain limited to:
  - `seqretsplit`
  - `infoassembly`
- `seqretsplit` to remain the single selected bounded retrieval continuation
  lead method
- the zero-burden release-truth surface to remain intact
- the first patch to stay limited to `seqretsplit` plus the smallest support
  needed for deterministic provider-aware split-output orchestration,
  normalized sequence-return output, and governed docs/validation plumbing
- the patch to land as a full governed slice rather than a half-start

The same guardrails should remain explicit:

- no hidden live-network dependencies
- no broad provider-parity claims
- no implicit widening into `infoassembly`
- no retrieval-family widening beyond the selected lead-method slice

#### Bounded implementation tier for `seqretsplit`

After selecting `seqretsplit` as the bounded retrieval continuation lead
method and capturing its exact patch start conditions, the repository should
map the full bounded implementation tier explicitly before code starts.

The bounded `seqretsplit` tier should be:

1. implement the bounded provider-aware split-output orchestration and
   normalized return core
2. expose the governed output surface for deterministic split-output retrieval
   behavior
3. expose `seqretsplit` through the governed shipped surface
4. add canonical managed-asset or mocked-provider fixtures plus compared
   evidence
5. re-run the full release-truth surface after shipping `seqretsplit`
6. reassess the shipped `seqretsplit` slice before any further retrieval
   continuation is mapped

#### Provider and seam stop conditions for `seqretsplit`

After selecting `seqretsplit` as the bounded retrieval continuation lead
method and mapping its implementation tier, the repository should make the
pre-code stop conditions explicit.

Planning should pause and reopen before code starts if:

1. `seqretsplit` cannot remain deterministic under mocked-provider or
   managed-asset validation
2. `seqretsplit` requires hidden live-network dependencies, implicit provider
   fallback chains, or unclear provider precedence
3. `seqretsplit` cannot remain a bounded extension of normalized
   sequence-return behavior and instead demands broader filesystem-policy
   orchestration before one shipped slice closes
4. `seqretsplit` forces broad filename-policy, directory-policy,
   batching-policy, or provider-parity claims that are not clearly local to
   the method

#### Reassessment of the shipped `seqretsplit` slice

After shipping `seqretsplit`, closing its compared-evidence slice, and
rerunning the full release-truth surface, the repository should reassess
whether the shipped retrieval seam stayed bounded enough to continue from
honestly.

The result is affirmative:

- the shipped `seqretsplit` slice stayed bounded, provider-aware, and
  deterministic
- it remained a local extension of normalized sequence-return behavior rather
  than forcing broader retrieval-family orchestration
- it did not require hidden live-network behavior, implicit provider fallback
  chains, or unclear provider precedence
- it did not force broad filename-policy, directory-policy, batching-policy,
  or provider-parity claims beyond the local method slice

The governed release-truth surface remained fully green:

- shipped methods: `106`
- compared evidence: `106`
- executable evidence: `0`
- harvested legacy provenance present: `106`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Rebasing the bounded retrieval continuation pool after `seqretsplit`

After the second bounded retrieval continuation slice closes cleanly through
`seqretsplit`, the repository should rebase the active continuation pool onto
the actual post-ship state before making any further viability or lead-method
decision.

The resulting bounded retrieval continuation inventory is:

- shipped bounded retrieval slices:
  - `seqretsetall`
  - `seqretsplit`
- remaining bounded retrieval continuation pool:
  - `infoassembly`

This checkpoint is inventory only:

- it does not yet decide whether the final remaining method still passes
  honest seam review strongly enough to remain an active continuation
  candidate
- it does not promote protein-property rework
- it does not widen retrieval scope beyond the already-governed bounded pool

#### Post-`seqretsplit` viability decision for bounded retrieval continuation

After rebasing the bounded retrieval continuation pool onto the actual
post-`seqretsplit` state, the repository should decide explicitly whether a
further bounded retrieval continuation candidate still exists or whether the
retrieval branch should stop and hand off to protein-property rework.

The decision is affirmative:

- another bounded retrieval continuation candidate still exists after
  `seqretsplit`
- the no-candidate branch is not taken here
- retrieval therefore remains the active planning program

The current viable bounded continuation pool remains:

- `infoassembly`

Protein-property rework therefore remains the reserve third program and should
not be promoted at this checkpoint.

#### Explicit closeout of the untriggered protein-property promotion branch after `seqretsplit`

Because the post-`seqretsplit` retrieval viability gate stayed affirmative,
the opposite branch should now be closed explicitly rather than left implicit.

The resulting branch consequence is:

- retrieval continuation remains the active planning program
- protein-property rework remains documented as the reserve third program
- protein-property is not promoted by inertia while bounded retrieval
  continuation still has a viable candidate

The active viable bounded continuation pool remains:

- `infoassembly`

#### Bounded retrieval continuation lead-method selection after `seqretsplit`

After keeping retrieval active through the post-`seqretsplit` viability gate
and closing the untriggered protein-property branch, the repository should
activate the single final bounded retrieval continuation candidate explicitly.

The selected bounded continuation candidate should be:

- `infoassembly`

That activation is the honest next step because:

- `infoassembly` is now the only remaining bounded continuation candidate in
  the narrowed post-`seqretsplit` pool
- the prepared protein-property reserve branch remains documented but
  inactive because the no-candidate branch was not taken
- no other retrieval-family method remains comparably bounded without
  widening provider-surface, orchestration, or scope claims beyond the
  already-governed continuation plan

#### Method-level acceptance criteria for `infoassembly`

After activating `infoassembly` as the single final bounded retrieval
continuation candidate, the repository should make its acceptance criteria
explicit before code starts.

The governed criteria should be:

- bounded provider-aware assembly metadata retrieval workflow only
- deterministic normalized assembly metadata output built from the same
  governed provider-resolution and execution path
- stable metadata-first output surface with explicit identifiers, provider
  source, and assembly fields defined by the same computation path
- canonical managed-asset or mocked-provider fixtures
- compared evidence required on normalized returned assembly metadata, not
  just orchestration intent

The non-goals should remain explicit:

- no hidden live-network validation
- no broad provider-parity claims
- no implicit widening into broader retrieval-family members such as
  `assemblyget`, `whichdb`, or `entret`
- no generic retrieval-family widening merely because `infoassembly` ships

#### Exact patch start conditions for `infoassembly`

After activating `infoassembly` as the single final bounded retrieval
continuation candidate and capturing its method-level acceptance criteria, the
repository should make the exact start conditions explicit before code begins.

The start gate should require:

- the current shortlist to remain intact:
  1. completed bounded plotting continuation program
  2. active remote-retrieval planning program
  3. protein-property rework
- the bounded retrieval continuation pool to remain limited to:
  - `infoassembly`
- `infoassembly` to remain the single selected bounded retrieval continuation
  candidate
- the zero-burden release-truth surface to remain intact
- the first patch to stay limited to `infoassembly` plus the smallest support
  needed for deterministic provider-aware metadata retrieval, normalized
  assembly metadata output, and governed docs/validation plumbing
- the patch to land as a full governed slice rather than a half-start

The same guardrails should remain explicit:

- no hidden live-network dependencies
- no broad provider-parity claims
- no implicit widening into broader retrieval-family members such as
  `assemblyget`, `whichdb`, or `entret`
- no retrieval-family widening beyond the selected lead-method slice

#### Full bounded implementation tier for `infoassembly`

After capturing the exact patch start conditions for `infoassembly`, the
repository should map the full bounded implementation tier explicitly before
code starts.

The bounded `infoassembly` tier should be:

1. implement the bounded provider-aware assembly metadata retrieval and
   normalized metadata return core
2. expose the governed output surface for deterministic assembly metadata
   retrieval behavior
3. expose `infoassembly` through the governed shipped surface
4. add canonical managed-asset or mocked-provider fixtures plus compared
   evidence
5. re-run the full release-truth surface after shipping `infoassembly`
6. reassess the shipped `infoassembly` slice before any further retrieval
   continuation is mapped

This bounded tier should preserve the same architectural rules:

- provider-aware but deterministic execution
- metadata-first normalized output
- method-local scope only
- no hidden live-network validation
- no broad provider-parity claims
- no retrieval-family widening unless `infoassembly` itself forces a real
  reassessment

#### Explicit provider/seam stop conditions for `infoassembly`

After mapping the full bounded implementation tier for `infoassembly`, the
repository should make the provider/seam stop conditions explicit before code
starts.

The repository should pause and reassess before implementation if:

1. `infoassembly` cannot remain deterministic under mocked-provider or
   managed-asset validation
2. `infoassembly` requires hidden live-network dependencies, implicit provider
   fallback chains, or unclear provider precedence
3. `infoassembly` cannot remain a bounded provider-aware metadata-first slice
   and instead demands broader retrieval-family orchestration before one
   shipped slice closes
4. `infoassembly` forces broad assembly-schema, provider-parity, or archive-
   scale acquisition claims that are not clearly local to the method

#### Reassessment after the shipped `infoassembly` slice

After the bounded `infoassembly` tier closed as a shipped governed slice, the
repository should record explicitly whether that slice stayed inside the
intended provider-aware retrieval seam before any further continuation is
mapped.

That conclusion is explicit rather than inferred because the shipped slice
stayed narrow in each important dimension:

- the analytical/result surface remained metadata-first and method-local
- the governed shipped surface remained a bounded retrieval member:
  - exactly one additional shipped method: `infoassembly`
- the provider seam remained bounded and deterministic:
  - mocked-provider evidence was sufficient for the governed slice
  - no hidden live-network dependency was required
  - no implicit provider fallback chain or broad provider-parity claim was
    introduced
- the evidence path closed completely for the shipped method:
  - canonical compared evidence exists
  - harvested legacy provenance is recorded
  - the method is no longer executable-only

So the repository should record the shipped `infoassembly` slice as having
passed its post-ship reassessment.

The governed release-truth surface remained fully green:

- shipped methods: `107`
- compared evidence: `107`
- executable evidence: `0`
- harvested legacy provenance present: `107`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Explicit closeout of the bounded retrieval continuation branch

After the final bounded retrieval continuation candidate closed cleanly
through `infoassembly`, the repository should make the branch-resolution
consequence explicit rather than leaving retrieval continuation implicitly
active.

That closeout is justified by the bounded continuation record now being
complete:

- shipped bounded retrieval continuation slices:
  - `seqretsetall`
  - `seqretsplit`
  - `infoassembly`
- each bounded retrieval continuation slice passed:
  - compared-evidence closure
  - harvested legacy provenance closure
  - release-truth rerun
  - post-ship reassessment
- no comparably bounded retrieval continuation candidate remains active after
  the shipped `infoassembly` reassessment

So the repository should record the bounded retrieval continuation program as
closed and should no longer treat retrieval continuation as the default next
planning branch.

The governed release-truth surface remained fully green:

- shipped methods: `107`
- compared evidence: `107`
- executable evidence: `0`
- harvested legacy provenance present: `107`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Activation of protein-property rework as the next active planning program

After the bounded retrieval continuation branch closes explicitly, the
repository should promote the next reserve program by branch resolution rather
than by inertia.

The resulting next-program activation is:

- plotting is no longer the active continuation branch
- bounded retrieval continuation is no longer the active continuation branch
- protein-property rework now becomes the next active planning program
- restriction-analysis rework remains the reserve next program and is not
  promoted by this step

The active bounded protein-property planning subset is now narrowed to:

- `psiphi`

This activation should remain bounded:

- it does not imply broad structural-biology family activation
- it does not reopen already-shipped `iep` or `pepdigest` as active planning
  candidates
- it does not revive omitted molecular-weight utilities by inertia
- it preserves the existing expectation that any shipped protein-property
  continuation must remain table-first, typed, and scientifically narrow

#### Reconfirmation of the reserve next program behind active protein-property rework

After activating protein-property rework as the next active planning program,
the repository should make the reserve next-program ordering explicit rather
than leaving it implicit.

That reserve ordering remains:

- active planning program:
  - protein-property rework
- reserve next program:
  - restriction-analysis rework
- reserve bounded restriction-analysis subset:
  - `recoder`
  - `silent`

This remains an ordering checkpoint only:

- it does not activate a restriction-analysis implementation tier
- it does not displace the active bounded `psiphi` planning branch
- it does not widen the reserve restriction-analysis family beyond the already
  governed retained subset

#### Bounded lead-candidate activation inside active protein-property rework

After protein-property rework becomes the active planning program, the
repository should activate the single bounded lead candidate explicitly rather
than implying a broader active pool.

That bounded lead-candidate selection is:

- active bounded protein-property lead candidate:
  - `psiphi`

The basis is narrow and explicit:

- `iep` is already shipped and fully evidenced
- `pepdigest` is already shipped and fully evidenced
- `psiphi` is the only remaining governed protein-property rework member that
  is neither already shipped nor explicitly omitted

So the repository should record that no broader protein-property candidate pool
remains active at this checkpoint; `psiphi` is the single bounded lead
candidate for the active program.

#### Method-level acceptance criteria for `psiphi`

After activating `psiphi` as the bounded lead candidate inside the active
protein-property rework program, the repository should make the method-level
acceptance criteria explicit before code starts.

The bounded governed criteria for `psiphi` are:

- bounded protein-coordinate analytical surface only
- stable table-first output with enough explicit columns to reconstruct
  per-residue phi/psi reporting from the same computation path
- deterministic typed result-surface output derived directly from the same
  coordinate-processing path
- canonical analytical fixtures and compared evidence on normalized
  torsion-angle rows
- explicit honesty about residue eligibility, chain continuity, missing-atom
  handling, and coordinate-model limitations

The non-goals remain explicit:

- no Ramachandran plotting or renderer-coupled figure logic
- no broad structural-analysis family activation
- no implicit widening into general structure parsing, modeling, or
  comparative coordinate analytics
- no family-wide continuation claim merely because `psiphi` ships

#### Exact start conditions for the first `psiphi` implementation patch

After capturing the method-level acceptance criteria for `psiphi`, the
repository should make the exact patch start conditions explicit before code
begins.

The following are treated as explicit start conditions:

- the current active-program ordering remains intact:
  1. protein-property rework
  2. restriction-analysis rework
- `psiphi` remains the single bounded lead candidate for the active
  protein-property branch
- the zero-burden release-truth surface remains intact
- the first patch stays limited to `psiphi` plus the smallest support needed
  for deterministic coordinate-derived computation, typed result-surface
  emission, and governed docs/validation plumbing
- the patch lands as a full bounded slice rather than a half-start

The same guardrails remain explicit:

- no Ramachandran plotting
- no broad structural-analysis family activation
- no generalized coordinate-processing framework
- no promotion of reserve restriction-analysis work while the bounded
  `psiphi` branch remains active

Only when those start conditions still hold should code work begin for the
bounded `psiphi` slice.

#### Full bounded `psiphi` implementation tier

After capturing the exact start conditions for the first `psiphi`
implementation patch, the repository should map the full bounded
implementation tier explicitly before code starts.

The bounded `psiphi` tier is:

1. implement the bounded protein-coordinate analytical core
2. expose the deterministic typed result surface for normalized per-residue
   torsion-angle reporting
3. expose `psiphi` through the governed shipped surface
4. add canonical analytical fixtures plus compared evidence on normalized
   torsion-angle rows
5. re-run the full release-truth surface after shipping `psiphi`
6. reassess the shipped `psiphi` slice before any further continuation is
   mapped

The bounded tier preserves the same architectural rules:

- protein-coordinate, table-first scope only
- deterministic typed result output from the same computation path
- method-local implementation only
- no Ramachandran plotting
- no generalized coordinate-processing framework
- no structural-analysis family widening unless `psiphi` itself forces a
  real reassessment

#### Explicit coordinate/seam stop conditions for `psiphi`

After mapping the full bounded `psiphi` implementation tier, the repository
should make the pre-code pause-and-reassess triggers explicit before
implementation begins.

The repository should now pause and reassess before implementation if:

1. `psiphi` cannot remain table-first with a deterministic typed result
   surface derived from the same coordinate-processing path
2. `psiphi` requires Ramachandran plotting, renderer-coupled figure logic, or
   other presentation-policy behavior
3. `psiphi` cannot remain method-associated and instead demands a generalized
   coordinate-processing or structural-analysis framework before one shipped
   slice closes
4. `psiphi` forces broad chain-model normalization, missing-atom imputation,
   alternate-conformer policy, or comparative structure-analysis claims that
   are not clearly local to the method

#### Reassessment after the shipped `psiphi` slice

After shipping `psiphi`, closing its compared-evidence and harvested-
provenance slice, and rerunning the full release-truth surface, the
repository should reassess whether the shipped protein-property seam stayed
bounded enough to continue from honestly.

The reassessment result is affirmative:

- the shipped `psiphi` slice stayed bounded, method-associated, and
  scientifically honest
- it remained a protein-coordinate, table-first analytical surface rather
  than forcing Ramachandran plotting or renderer-coupled figure logic
- it kept the normalized per-residue torsion-angle rows and typed result
  surface derived from the same coordinate-processing path
- it did not force broad chain-model normalization, missing-atom imputation,
  alternate-conformer policy, or comparative structure-analysis claims
  beyond the local method slice

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Post-ship reassessment of the bounded `primersearch` slice

After the bounded `primersearch` slice ships, the repository should reassess
it before mapping any further primer-family continuation.

The result is affirmative:

- the shipped `primersearch` slice stayed bounded, deterministic, and
  search-first
- it remained a primer-pair sequence-search workflow rather than widening
  into primer-design optimization
- it kept the normalized table-first hit rows and typed result surface
  derived from the same matching path
- it did not force assay-ranking, thermodynamic scoring, or a generalized
  primer-analysis framework beyond the local method slice
- it did not force broader ambiguity-resolution or mismatch-taxonomy claims
  beyond the bounded local seam

The governed release-truth surface remained fully green:

- shipped methods: `109`
- compared evidence: `109`
- executable evidence: `0`
- harvested legacy provenance present: `109`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Post-`primersearch` bounded continuation inventory

After the shipped `primersearch` reassessment closes, the repository should
rebase the remaining primer-family continuation pool onto the actual shipped
state before deciding whether another bounded continuation candidate still
exists.

The first bounded primer-family continuation slice is now treated as closed:

- shipped bounded primer-family slice:
  - `primersearch`

The remaining bounded primer-family continuation pool is now:

- `eprimer3`
- `sirna`

This checkpoint is inventory only. It does not yet decide whether both
remaining methods still pass honest seam review strongly enough to remain
active continuation candidates.

#### Post-`primersearch` viability decision for the narrowed primer-family branch

After the shipped `primersearch` reassessment closes and the remaining bounded
primer-family continuation pool is inventoried, the repository should decide
explicitly whether another bounded continuation candidate still exists or
whether the active primer-family branch should stop and hand off to the
reserve next program.

That decision is affirmative at this checkpoint:

- another bounded primer-family continuation candidate still exists after
  `primersearch`
- the no-candidate branch is not taken here
- primer and assay-oriented search therefore remains the active planning
  program

The currently viable bounded continuation pool remains:

- `eprimer3`
- `sirna`

Reserve promotion does not happen at this checkpoint:

- command discovery and help-navigation remains the reserve next program and
  is not promoted here

#### Explicit closeout of the untriggered reserve-promotion branch after post-`primersearch` viability

Once the narrowed primer-family continuation gate stays affirmative after the
shipped `primersearch` reassessment, the repository should make the branch
consequence explicit rather than leaving reserve promotion ambiguous.

That reserve-promotion branch is not taken at this checkpoint:

- the narrowed primer-family seam-review gate stayed affirmative
- primer and assay-oriented search therefore remains the active planning
  program

The active bounded continuation pool remains:

- `eprimer3`
- `sirna`

Command discovery and help-navigation remains the documented reserve next
program rather than being promoted by inertia.

#### Selection of the next bounded primer-family continuation candidate

Once the narrowed primer-family continuation gate stays affirmative, the
repository should choose exactly one bounded lead candidate from the remaining
active pool rather than keeping multiple implementation branches live at once.

The selected bounded continuation candidate is:

- `eprimer3`

`eprimer3` leads the remaining active pool because:

- it stays closer to the core primer-and-oligo design seam than `sirna`
- it is still easier to bound as a local primer-design modernization than a
  broader siRNA-selection workflow with stronger biological-efficacy policy
  pressure
- it keeps the active family continuation inside primer/oligo design rather
  than immediately widening into RNA-interference-specific selection semantics

The non-selected bounded family member remains documented but inactive:

- `sirna`

#### Explicit `eprimer3` method-level acceptance criteria

With `eprimer3` selected as the bounded continuation candidate, the repository
should make the method-specific acceptance criteria explicit before
implementation.

The governed criteria now make `eprimer3` concrete before code starts:

- bounded primer-and-oligo design workflow only
- deterministic table-first output derived from the same local design path
- explicit reporting of candidate oligo identity, strand/orientation, genomic
  interval, and method-local scoring fields from the same computation path
- canonical analytical fixtures and compared evidence on normalized
  primer-candidate rows
- honest handling of primer length policy, product-size bounds, ambiguity
  rules, and design-eligibility filtering

The non-goals remain:

- no generalized assay-ranking framework
- no broad thermodynamic optimization platform
- no widening into `sirna`
- no family-wide continuation claim merely because `eprimer3` ships

#### Exact start conditions for the first `eprimer3` implementation patch

Before any code starts, the repository should make the start gate for the
first bounded `eprimer3` patch explicit.

That start gate now requires:

- primer and assay-oriented search to remain the active planning program
- `eprimer3` to remain the single selected bounded continuation candidate
- the active bounded primer-family subset to remain limited to:
  - `eprimer3`
  - `primersearch`
  - `sirna`
- the zero-burden release-truth surface to remain intact
- the first patch to stay limited to `eprimer3` plus the smallest support
  needed for deterministic primer/oligo candidate generation, normalized
  table-first reporting, and governed docs/validation plumbing
- the patch to land as a full bounded slice rather than a half-start

The same guardrails remain:

- no generalized assay-ranking framework
- no broad thermodynamic optimization platform
- no widening into `sirna`
- no family-wide continuation claim merely because one bounded `eprimer3`
  slice ships

#### Full bounded `eprimer3` implementation tier

With the acceptance criteria and start gate in place, the repository should
map the full bounded `eprimer3` implementation tier before any code starts.

The bounded `eprimer3` tier is now:

1. implement the bounded primer-and-oligo design analytical core
2. expose the deterministic typed result surface for normalized
   primer-candidate reporting
3. expose `eprimer3` through the governed shipped surface
4. add canonical analytical fixtures plus compared evidence on normalized
   primer-candidate rows
5. re-run the full release-truth surface after shipping `eprimer3`
6. reassess the shipped `eprimer3` slice before any further primer-family
   continuation is mapped

The same bounded rules remain:

- primer-and-oligo design scope only
- deterministic table-first reporting from the same local design path
- method-local implementation only
- no generalized assay-ranking framework
- no broad thermodynamic optimization platform
- no primer-family widening unless `eprimer3` itself forces a real
  reassessment

#### Explicit biological-design/seam stop conditions for `eprimer3`

After mapping the full bounded `eprimer3` implementation tier, the repository
should make the pre-code stop conditions explicit before implementation.

The repository should pause and reassess before implementation if:

1. `eprimer3` cannot remain table-first with a deterministic typed result
   surface derived from the same local design path
2. `eprimer3` requires generalized assay-ranking, broad thermodynamic
   optimization policy, or other non-local biological-design behavior
3. `eprimer3` cannot remain method-associated and instead demands a
   generalized primer-analysis framework before one shipped slice closes
4. `eprimer3` forces broad ambiguity-resolution policy, product-scoring
   taxonomy, or design-eligibility semantics that are not clearly local to
   the method

#### Explicit closeout of the bounded protein-property rework branch

After the shipped `psiphi` slice closed its compared-evidence and harvested-
provenance work, reran release truth, and passed post-ship reassessment, the
repository should make the protein-property branch-resolution consequence
explicit rather than leaving the program implicitly active.

That closeout is now honest because the governed protein-property rework set
has no remaining bounded active candidate:

- `iep` is already shipped and fully evidenced
- `pepdigest` is already shipped and fully evidenced
- `psiphi` has now shipped, closed compared evidence and harvested
  provenance, rerun release truth, and passed post-ship reassessment
- no comparably bounded protein-property rework candidate remains active
  after the shipped `psiphi` reassessment

So the repository should record the bounded protein-property rework program as
closed and should no longer treat protein-property rework as the default next
planning branch.

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Activation of restriction-analysis rework as the next active planning program

After the bounded protein-property rework branch closes explicitly, the
repository should promote the next reserve program by branch resolution rather
than by inertia.

The resulting next-program activation is:

- plotting is no longer the active continuation branch
- bounded retrieval continuation is no longer the active continuation branch
- protein-property rework is no longer the active continuation branch
- restriction-analysis rework now becomes the next active planning program

The active bounded restriction-analysis planning subset is now narrowed to:

- `recoder`
- `silent`

This activation should remain bounded:

- it does not imply broad enzyme-catalog or provider-parity claims
- it does not reopen omitted broader restriction workflows by inertia
- it does not preemptively activate any later reserve-program ordering
- it preserves the existing expectation that any shipped
  restriction-analysis continuation must remain deterministic, method-local,
  and scientifically explicit

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Honest seam review of the narrowed bounded restriction-analysis branch

After restriction-analysis rework becomes the active planning program, the
repository should close the pass/fail gate on whether the narrowed bounded
branch still remains honest enough to continue before selecting any single
lead method.

The decision is affirmative:

- a bounded restriction-analysis continuation candidate still exists
- the no-candidate branch is not taken here
- restriction-analysis rework therefore remains the active planning program

The current viable bounded continuation pool remains:

- `recoder`
- `silent`

The broader family remains outside the active bounded branch at this
checkpoint:

- database extraction and provider-heavy surfaces such as `rebaseextract`
  and `redata`
- reporting or visualization-heavy surfaces such as `remap`
- broader enzyme-scan workflows such as `restrict` and `restover`

So the next bounded planning step should be to choose exactly one lead method
from the viable bounded pool rather than widening to the broader family.

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Explicit closeout of the bounded restriction-analysis continuation branch

After reconciling the previously assumed bounded restriction-analysis
continuation tier against the actual shipped state, the repository should make
the branch-resolution consequence explicit rather than leaving
restriction-analysis continuation implicitly active.

That closeout is now honest because the retained bounded members of the family
are already shipped and evidenced:

- `recoder`
- `silent`

No comparably bounded restriction-analysis continuation candidate remains
active after that reconciliation step.

So the repository should record the bounded restriction-analysis continuation
branch as closed and should no longer treat restriction-analysis continuation
as an active implementation branch.

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Continued inactivity of the broader restriction-analysis family

Closing the bounded restriction-analysis continuation branch should not be
misread as activation of the broader family by inertia.

The broader restriction-analysis family therefore remains inactive at this
checkpoint:

- `rebaseextract`
- `redata`
- `remap`
- `restrict`
- `restover`

Those methods remain inactive because their seams remain broader than the
already-shipped local recoding primitives:

- database-extraction and provider-surface pressure remains unbounded
- reporting and visualization surfaces remain broader than the closed local
  recoding seam
- broader enzyme-scan workflows remain outside the already-shipped bounded
  edit-design primitives

So any future activation of those methods would need a fresh bounded local
justification rather than inherited family momentum.

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Remaining wholly unshipped `Rework` and `Add` family inventory

With plotting, remote retrieval, protein-property rework, and the bounded
restriction-analysis retained kernel now all represented in the shipped
surface, the repository should make the remaining wholly unshipped
implementation-program families explicit before generating a new
recommendation.

The already-activated `Rework` families are therefore not part of the
remaining wholly unshipped family inventory:

- plotting and visualization tools
- remote retrieval and archive acquisition
- protein property and structural-summary utilities
- restriction-enzyme design and analysis

The remaining wholly unshipped `Rework` families are:

- primer and assay-oriented search
- external database preparation helpers
- legacy prediction methods with enduring scientific value
- command discovery and help-navigation

The remaining wholly unshipped `Add` families are:

- HMM and probabilistic homology workflows
- modern archive-scale raw data ingestion

Those six families now form the live inventory for the next-program
recommendation step, rather than any stale inherited family order left over
from the already-completed plotting, retrieval, protein-property, or
restriction planning cycles.

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Explicit next-program recommendation from the current shipped-state truth

The current generated surfaces remain neutral at this checkpoint:

- no reprioritization signals were generated
- no recommendation changes are required automatically

That means the repository still needs an explicit next-program recommendation
from the remaining live inventory rather than a report-driven reorder.

The recommended next active planning program is:

- primer and assay-oriented search

That family leads the remaining inventory because:

- it is the most bounded remaining user-facing `Rework` family
- it has only three surviving `Rework` methods after the existing `Omit`
  decisions:
  - `eprimer3`
  - `primersearch`
  - `sirna`
- it does not depend on broad provider-ingestion seams or strategic-add
  platform claims
- it appears materially less open-ended than legacy prediction or
  archive-scale `Add` programs

The remaining family order behind it is now:

1. primer and assay-oriented search
2. command discovery and help-navigation
3. external database preparation helpers
4. legacy prediction methods with enduring scientific value
5. HMM and probabilistic homology workflows
6. modern archive-scale raw data ingestion

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Activation of primer and assay-oriented search as the next active planning program

After the recommendation step is made explicit, the repository should activate
that family deliberately rather than leaving it as an advisory note only.

Primer and assay-oriented search is now the active planning program by branch
resolution rather than by inherited family order.

The currently live bounded family subset is:

- `eprimer3`
- `primersearch`
- `sirna`

The remaining families stay inactive behind the newly activated program:

- command discovery and help-navigation
- external database preparation helpers
- legacy prediction methods with enduring scientific value
- HMM and probabilistic homology workflows
- modern archive-scale raw data ingestion

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Honest seam review of the narrowed primer and assay-oriented search branch

After primer and assay-oriented search becomes the active planning program,
the repository should close the pass/fail gate on whether the narrowed bounded
family branch remains honest enough to continue before choosing any single
lead method.

The decision is affirmative.

A bounded continuation candidate still exists inside the active primer family.

The viable bounded continuation pool remains:

- `eprimer3`
- `primersearch`
- `sirna`

The omitted family members do not re-enter the active branch:

- `eprimer32`
- `stssearch`

So the family remains active for the next bounded lead-method selection step
rather than forcing an immediate return to reserve-program ordering.

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Lead-method selection inside the active primer-family branch

After the bounded primer-family seam-review gate remains affirmative, the
repository should choose exactly one lead method from the active bounded pool
before capturing method-specific acceptance criteria and start conditions.

The selected bounded lead candidate is:

- `primersearch`

`primersearch` leads the active pool because:

- it is the narrowest remaining search-first surface in the family
- it appears easier to keep deterministic than primer-design or
  siRNA-selection workflows
- it avoids immediately forcing optimization-heavy scoring or broader
  biological-design policy claims

The non-selected bounded family members remain documented but inactive:

- `eprimer3`
- `sirna`

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Explicit `primersearch` method-level acceptance criteria

With `primersearch` selected as the bounded lead method, the repository should
make the method-specific acceptance criteria explicit before implementation.

The governed criteria now make `primersearch` concrete before code starts:

- bounded primer-pair sequence-search workflow only
- deterministic table-first output derived from the same matching path
- explicit reporting of primer-pair identity, strand/orientation, and matched
  interval coordinates from the same computation path
- canonical analytical fixtures and compared evidence on normalized
  primer-hit rows
- honest handling of mismatch policy, orientation rules, ambiguity handling,
  and pair-completion rules

The non-goals remain:

- no primer-design optimization
- no broad assay-scoring or thermodynamic ranking framework
- no widening into `eprimer3` or `sirna`
- no family-wide continuation claim merely because `primersearch` ships

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Exact start conditions for the first `primersearch` implementation patch

Before any code starts, the repository should make the start gate for the
first bounded `primersearch` patch explicit.

That start gate now requires:

- primer and assay-oriented search to remain the active planning program
- `primersearch` to remain the single selected bounded lead candidate
- the active bounded family subset to remain limited to:
  - `primersearch`
  - `eprimer3`
  - `sirna`
- the zero-burden release-truth surface to remain intact
- the first patch to stay limited to `primersearch` plus the smallest support
  needed for deterministic primer-pair matching, normalized table-first
  reporting, and governed docs/validation plumbing
- the patch to land as a full bounded slice rather than a half-start

The same guardrails remain:

- no primer-design optimization
- no assay-ranking or thermodynamic scoring framework
- no widening into `eprimer3` or `sirna`
- no family-wide continuation claim merely because one bounded `primersearch`
  slice ships

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Full bounded `primersearch` implementation tier

With the acceptance criteria and start gate in place, the repository should
map the full bounded `primersearch` implementation tier before any code
starts.

The bounded `primersearch` tier is now:

1. implement the bounded primer-pair matching analytical core
2. expose the deterministic typed result surface for normalized primer-hit
   reporting
3. expose `primersearch` through the governed shipped surface
4. add canonical analytical fixtures plus compared evidence on normalized
   primer-hit rows
5. re-run the full release-truth surface after shipping `primersearch`
6. reassess the shipped `primersearch` slice before any further
   primer-family continuation is mapped

The same bounded rules remain:

- primer-pair search scope only
- deterministic table-first reporting from the same matching path
- method-local implementation only
- no primer-design optimization
- no assay-ranking or thermodynamic scoring framework
- no primer-family widening unless `primersearch` itself forces a real
  reassessment

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Explicit seam-pressure stop conditions for `primersearch`

Before implementation starts, the repository should make the pause-and-reassess
triggers for the bounded `primersearch` slice explicit.

The repository should pause and reassess before implementation if:

1. `primersearch` cannot remain table-first with a deterministic typed result
   surface derived from the same primer-matching path
2. `primersearch` requires primer-design optimization, assay-ranking,
   thermodynamic scoring policy, or other non-local biological-design
   behavior
3. `primersearch` cannot remain method-associated and instead demands a
   generalized primer-analysis framework before one shipped slice closes
4. `primersearch` forces broad ambiguity-resolution policy, mismatch search
   taxonomy, or pair-completion semantics that are not clearly local to the
   method

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Explicit closeout of the untriggered reserve-ordering revisit branch

Because the narrowed bounded restriction-analysis seam-review gate remained
affirmative, the repository should close the opposite branch explicitly rather
than informally drifting toward reserve-program reshuffling.

The resulting branch consequence is:

- restriction-analysis rework remains the active planning program
- reserve-program ordering behind it remains deferred and inactive
- the active bounded continuation pool remains:
  - `recoder`
  - `silent`

The broader family is still not activated by inertia:

- no promotion of `rebaseextract` or `redata`
- no promotion of `remap`, `restrict`, or `restover`
- no reserve-program reshuffle while the bounded restriction-analysis branch
  still has viable candidates

So the repository should not revisit reserve ordering at this checkpoint. The
next bounded planning step remains selection of exactly one lead method from
the viable bounded pool.

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

#### Reconciliation of the bounded restriction-analysis lead-candidate assumption

After the narrowed bounded restriction-analysis branch stayed affirmative, the
next planned step would ordinarily be explicit lead-method selection.
However, the actual repository state must be checked before that step is
treated as real.

That check shows the previously assumed bounded candidate pool is already
fully shipped and evidenced:

- `recoder`
- `silent`

Each already appears in the governed shipped registry and already carries:

- compared evidence
- harvested legacy provenance
- fully green release-truth coverage

So no new lead candidate needs to be selected at this checkpoint. Instead,
the planning consequence should be recorded honestly:

- the bounded restriction-analysis continuation plan written before this
  check is stale relative to the actual shipped state
- further restriction-analysis planning must be rebased on the actual
  post-`recoder` and post-`silent` shipped state before any new continuation
  tier is mapped

The governed release-truth surface remained fully green:

- shipped methods: `108`
- compared evidence: `108`
- executable evidence: `0`
- harvested legacy provenance present: `108`
- `full_compared_cohort: true`
- `harvest_coverage_complete: true`
- `retained_backlog_closed: true`
- `release_truth_current: true`

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
