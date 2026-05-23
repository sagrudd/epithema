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

So no reordering or widening is needed here. If plotting is later blocked by
contract sprawl or renderer-coupled pressure, the repository can still switch
to the retrieval fallback without reopening family-selection ambiguity first.

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
