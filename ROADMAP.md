# ROADMAP

This roadmap converts the current EMBOSS-RS appraisal into a discrete task map.
The ordering is intentional: earlier items improve truthfulness and confidence
in the platform, while later items expand method coverage in governed sweeps.

## Standing Rules For All Tasks

1. Inspect and maintain `git status` throughout the task.
2. Any modification to code or documentation must remain tightly scoped to the
   essential files for that change.
3. Documentation must be maintained honestly and religiously.
   - Do not overstate provenance, validation depth, example coverage, or
     implementation completeness.
4. Rust code should prefer method-associated `.rs` files.
5. Where logic spans multiple methods, use discrete, logically named files
   rather than generic catch-all modules.
6. Any task that modifies repository content should end with review, commit, and
   push when the execution prompt requires commit and push.

## Task Map

1. Separate autodoc stub provenance from curated documentation.
   - Introduce an explicit distinction between registry-generated stub
     contracts and genuinely curated autodoc contracts.
   - Remove any implication that a generated baseline stub is equivalent to
     harvested or reviewer-curated method documentation.
   - Update generated page metadata so readers can tell immediately whether a
     page is stub-backed or richly curated.

2. Harden the documentation truth model.
   - Audit the current autodoc contract schema, generated Markdown rendering,
     and cohort-report projection for terminology that overstates maturity.
   - Align `source_mode`, provenance fields, and validation-intent wording with
     actual evidence levels.
   - Ensure that the docs site clearly distinguishes:
     - documented only
     - declared evidence
     - executable evidence
     - compared evidence

3. Complete a curated documentation sweep for the alignment family.
   - Target:
     - `aligncopy`
     - `aligncopypair`
     - `infoalign`
     - `extractalign`
     - `matcher`
     - `distmat`
     - `cons`
     - `consambig`
     - `needleall`
   - Replace baseline stubs with curated autodoc contracts containing real
     artifacts, declared examples, and honest validation intent.

4. Complete a curated documentation sweep for retrieval and archive methods.
   - Target:
     - `seqret`
     - `refseqget`
     - `runinfo`
     - `runget`
   - Document the current provider seams, local-versus-remote behavior,
     unsupported cases, and provenance boundaries explicitly.

5. Complete a curated documentation sweep for codon-analysis methods.
   - Target:
     - `cai`
     - `chips`
     - `codcmp`
     - `codcopy`
   - Add explicit example declarations, fixture references, and current
     limitations rather than leaving these methods on generic stub pages.

6. Build an acceptance anchor set with real compared evidence.
   - Select a small cross-family anchor cohort, for example:
     - `needle`
     - `seqret`
     - `extractfeat`
     - `maskseq`
     - `compseq`
     - `pepstats`
   - For each anchor:
     - harvest one historical example where practical
     - execute one governed validation case
     - capture at least one expected-output comparison

7. Add the first translation and ORF completion sweep.
   - Implement the highest-value retained translation-adjacent gaps:
     - `transeq`
     - `getorf`
     - `prettyseq`
     - `tranalign`
   - Reuse existing translation, interval, and typed reporting foundations.
   - Keep method logic in method-associated files and add only clearly named
     shared helpers where needed.

8. Add the first local-alignment completion sweep.
   - Implement:
     - `water`
   - Then assess the immediately adjacent retained gaps for follow-on work:
     - `diffseq`
     - `wordmatch`
     - `wordfinder`
   - Keep this sweep focused on local alignment and direct comparison rather
     than bundling broader alignment redesign.

9. Add the next feature and sequence-IO completion sweep.
   - Prioritise retained tools that extend the existing annotated-record path:
     - `coderet`
     - `featmerge`
     - `featreport`
     - `feattext`
   - Follow the existing feature-selection, extraction, and copying seams
     rather than introducing parallel feature models.

10. Add the next core statistics and metadata sweep.
    - Prioritise:
      - `infoseq`
      - `wordcount`
      - `cusp`
      - `dan`
    - Reuse the existing typed table-report path already established by
      `descseq`, `compseq`, `geecee`, and `pepstats`.

11. Expand plotting as a governed family, not a one-off.
    - Preserve R-owned rendering.
    - After `charge`, choose the next narrow plot-capable methods with simple
      contracts, such as one of:
      - `pepwindow`
      - `plotorf`
      - `freak`
    - Define plot contracts in Rust and render them only in `emboss-r`.

12. Reduce the “documented only” cohort count family by family.
    - Use the cohort validation report as the source of truth.
    - Each family sweep should explicitly reduce:
      - `documented_only`
      - `missing_validation_cases`
      - `missing_executable_evidence`
      - `missing_compared_evidence`

13. Keep the governance backlog and the shipped registry aligned.
    - Periodically reconcile:
      - governed retained methods in the scope matrix
      - currently shipped Rust registry methods
      - curated autodoc coverage
      - executable evidence coverage
    - Update roadmap ordering if a family’s retained gaps become more urgent
      than the current sequence.

14. Preserve release honesty while coverage grows.
    - Keep release/readiness reports explicit about what is complete versus
      what is merely scaffolded.
    - Do not treat page count, stub presence, or registry inclusion as a proxy
      for biological acceptance.

## Next Tier Task Map

The first fourteen tasks established the truth model, the governed reporting
path, the first compared-evidence anchors, and the current shipped cohort of
`60` methods. The next tier should be driven by the generated governance and
cohort reports rather than by ad hoc tool additions.

Current baseline for this next tier:

- shipped methods: `60`
- documented-only shipped methods: `4`
- declared-only shipped methods: `1`
- compared-evidence shipped methods: `13`
- retained backlog still unshipped: `36`

15. Eliminate the remaining documented-only shipped methods.
    - Target:
      - `degapseq`
      - `trimseq`
      - `complex`
      - `splitter`
    - Convert each one to:
      - curated autodoc
      - executable validation intent
      - honest generated validation metadata
    - The specific goal is to drive `documented_only_count` from `4` to `0`
      without inflating evidence claims beyond what actually runs.

16. Upgrade `newseq` from declared-only to executable evidence.
    - Keep the current conservative molecule policy.
    - Add at least one governed executed case that proves:
      - DNA creation
      - protein creation
      - invalid residue rejection
    - The goal is to remove the last `declared_evidence` method from the
      shipped cohort.

17. Expand compared-evidence anchors across the strongest existing families.
    - Prioritise methods that already have executable coverage and stable
      output forms, for example:
      - `water`
      - `descseq`
      - `fuzznuc`
      - `fuzzpro`
      - `fuzztran`
      - `geecee`
      - `infoseq`
      - `cusp`
    - Focus on durable expected-output fixtures, not high-volume superficial
      comparisons.

18. Add the next sequence editing and manipulation sweep.
    - The governance report identifies this as the largest retained backlog.
    - Prioritise the most structurally adjacent tools:
      - `merger`
      - `megamerger`
      - `sizeseq`
      - `shuffleseq`
    - Keep the implementations aligned to existing sequence-stream and
      transform seams, with method-associated files.

19. Add the ambiguity and masking follow-on sweep.
    - Prioritise:
      - `maskambignuc`
      - `maskambigprot`
      - `pasteseq`
      - `twofeat`
    - Reuse existing masking, feature, and sequence-edit infrastructure rather
      than introducing a second ambiguity model.

20. Add the alignment post-processing follow-on sweep.
    - The immediate retained backlog is:
      - `diffseq`
      - `edialign`
    - Keep this sweep narrowly about comparison and alignment editing rather
      than extending the pairwise scoring core unnecessarily.

21. Add the next motif and regular-expression sweep.
    - The retained backlog is still large here.
    - Prioritise the most practically useful cluster:
      - `preg`
      - `patmatdb`
      - `wordmatch`
      - `wordfinder`
    - Keep the first release for each tool bounded and typed rather than
      reproducing the whole historical fuzzy-language surface at once.

22. Add the inverted-repeat and palindrome sweep.
    - Prioritise:
      - `palindrome`
      - `einverted`
      - `dreg`
      - `seqmatchall`
    - Reuse interval and pattern-reporting structures where possible and keep
      strand/coordinate semantics explicit.

23. Add the next core statistics sweep.
    - The retained backlog is now:
      - `aaindexextract`
      - `infobase`
      - `inforesidue`
      - `oddcomp`
    - Follow the same deterministic table-report model used by `infoseq`,
      `compseq`, `geecee`, `pepstats`, and `wordcount`.

24. Add the next sequence IO and set-handling sweep.
    - Prioritise:
      - `nthseqset`
      - `splitsource`
      - `listor`
      - `skipredundant`
    - Keep ordering semantics and duplicate-handling behavior explicit and
      deterministic.

25. Add the next protein-property rework sweep.
    - Governance already marks these as retained rework candidates:
      - `iep`
      - `pepdigest`
    - Keep them scientifically narrow and typed.
    - Do not add broad biochemical “kitchen sink” summaries; prefer explicit
      v1 metric scopes.

26. Add the restriction-analysis rework sweep.
    - Governance backlog:
      - `recoder`
      - `silent`
    - Treat these as modernized analytical tools, not literal ports of
      EMBOSS-era UI behavior.

27. Expand governed plotting carefully.
    - After `charge` and `pepwindow`, choose the next narrow plot-contract
      additions only where the analytical payload is already stable.
    - Candidate next plots:
      - `plotorf`
      - a governed visualization for `dan`
      - a governed visualization for `wordcount`
    - Rendering must remain R-owned in `emboss-r`.

28. Begin historical evidence harvesting for the compared-anchor cohort.
    - The current framework can distinguish harvested evidence, but that count
      is still `0`.
    - Start with the methods that already have compared fixtures.
    - The goal is to prove provenance depth, not merely to add more snapshots.

29. Add a standing cohort-health gate for roadmap reprioritization.
    - On each future family sweep, review:
      - `docs/generated/cohort_validation.md`
      - `docs/generated/governance_alignment.md`
      - `docs/release/v1_0_0_rc_readiness.md`
    - Reorder the next roadmap items whenever:
      - a different family becomes the largest retained backlog
      - a shipped family accumulates too many weak-evidence methods
      - release truth would otherwise fall behind shipped capability

30. Preserve the release candidate truth model after `1.0.0`.
    - After the stable cut, carry the same rules forward into `Unreleased`.
    - New shipped tools must not bypass:
      - governance mapping
      - autodoc presence
      - validation-stub generation
      - cohort-report inclusion
      - honest release-note wording

## Next Tier Task Map

This next tier is derived from the current generated reports, not from stale
pre-implementation assumptions.

Current basis at the time this tier was written:

- shipped methods: `90`
- compared-evidence methods: `21`
- executable-evidence methods: `69`
- methods with harvested legacy provenance recorded: `47`
- retained backlog still unshipped: `6`
- largest retained backlog family:
  - `Core Retain — Sequence editing and manipulation`
- weakest shipped evidence family:
  - `Core Retain — Basic sequence IO and conversion`

31. Close the remaining retained sequence-editing backlog.
    - The retained governance backlog is now entirely:
      - `biosed`
      - `makenucseq`
      - `makeprotseq`
      - `msbar`
      - `trimest`
      - `vectorstrip`
    - Treat this as the highest implementation priority until retained backlog
      reaches `0`.
    - Keep method logic in method-associated `.rs` files and avoid creating a
      second sequence-editing abstraction layer unless it is genuinely shared.

32. Deepen compared evidence in the weakest shipped family first.
    - The cohort-health gate now identifies `Core Retain — Basic sequence IO and
      conversion` as the largest below-compared burden.
    - Prioritise compared anchors for methods such as:
      - `newseq`
      - `seqcount`
      - `notseq`
      - `nthseq`
      - `skipseq`
      - `listor`
      - `skipredundant`
      - `extractseq`
      - `cutseq`
      - `union`
      - `pasteseq`
      - `splitter`
      - `merger`
      - `megamerger`
      - `sizeseq`
      - `shuffleseq`
    - Focus on stable expected-output fixtures and real legacy provenance where
      it exists.

33. Raise the harvested-legacy footprint beyond the current anchor cohort.
    - `47` methods now record harvested legacy provenance, but this remains
      concentrated in curated and anchor-backed families.
    - Extend harvested legacy references to strong executable-but-not-compared
      methods family by family.
    - Prefer historically meaningful references over placeholder URLs.

34. Convert the remaining executable-only alignment family methods to compared evidence.
    - Prioritise:
      - `aligncopy`
      - `aligncopypair`
      - `infoalign`
      - `extractalign`
      - `matcher`
      - `distmat`
      - `cons`
      - `consambig`
      - `needleall`
      - `diffseq`
      - `edialign`
      - `nthseqset`
    - This should complete the alignment utility and summary surface as a
      comparatively strong evidence family.

35. Convert the retrieval and archive family from executable-only to compared evidence where feasible.
    - Prioritise:
      - `runinfo`
      - `runget`
      - `refseqget`
    - Keep mocked-provider seams explicit and do not imply live-provider
      equivalence unless the comparison really exercises that path.

36. Strengthen the feature-tools family with compared anchors and harvested provenance.
    - Prioritise:
      - `featcopy`
      - `coderet`
      - `featmerge`
      - `featreport`
      - `feattext`
      - `splitsource`
      - `twofeat`
      - `maskambignuc`
      - `maskambigprot`
    - Feature-rich methods are user-visible and structurally easy to regress,
      so evidence depth here pays off quickly.

37. Deepen codon and protein-statistics acceptance beyond current spot checks.
    - Prioritise compared or harvested evidence for:
      - `cai`
      - `chips`
      - `codcmp`
      - `codcopy`
      - `aaindexextract`
      - `infobase`
      - `inforesidue`
      - `oddcomp`
      - `iep`
      - `pepdigest`
      - `wordcount`
    - Keep scientific assumptions explicit and do not broaden metric scope
      unless required by real use cases.

38. Deepen the pattern and repeat family from executable-heavy to comparison-backed.
    - Prioritise:
      - `preg`
      - `patmatdb`
      - `wordmatch`
      - `wordfinder`
      - `dreg`
      - `palindrome`
      - `einverted`
      - `seqmatchall`
    - Use stable coordinate tables and overlapping-hit semantics consistently.

39. Complete the restriction and recoding family as an evidence-backed rework surface.
    - Prioritise:
      - `recoder`
      - `silent`
    - Add compared fixtures that demonstrate exact site removal/creation and
      stable synonymous-edit ordering.

40. Revisit governed plotting from the evidence side before adding more plot-capable methods.
    - The current Rust-side plotting surface is:
      - `charge`
      - `pepwindow`
      - `wordcount`
    - Before adding new plot contracts, strengthen docs and validation around
      contract stability and deterministic emitted payloads.
    - Only then consider the next narrow candidate such as `dan`.

41. Add a generated “comparison coverage” report alongside the cohort report.
    - The current cohort report distinguishes evidence levels, but it does not
      yet make family-by-family compared coverage easy to scan.
    - Add a small typed report or section that surfaces:
      - compared count by family
      - executable-only count by family
      - harvested-but-not-compared count by family
    - Use it to drive evidence prioritization after each sweep.

42. Add a generated “retained backlog closure” report or section.
    - Once the retained backlog falls below `10`, each remaining method becomes
      strategically important.
    - Surface for each unshipped retained method:
      - governance family
      - nearest implemented Rust family
      - recommended next sweep
      - whether the blocker is implementation, validation, or documentation

43. Tighten release gating after retained backlog reaches `0`.
    - Once all retained methods are shipped, raise the bar from “docs and stubs”
      to stronger evidence requirements for new releases.
    - Candidate future gate:
      - no new shipped family may land without at least one compared anchor
      - no release note may claim a family without a generated cohort entry
      - no drift between `cohort_health`, `governance_alignment`, and
        release-facing docs

44. Reassess the governance appendix after retained backlog closure.
    - Once the six retained backlog methods are implemented, review whether the
      remaining `rework` set should be narrowed, expanded, or reordered.
    - Keep this as a governance pass, not a silent code change.

45. Carry the same roadmap discipline into the next planning cycle.
    - After Task `44`, extend this roadmap again from generated evidence and
      backlog truth rather than from memory.
    - Future roadmap updates must continue to:
      - preserve honest documentation
      - keep changes scoped
      - commit and push after changed prompts
      - avoid touching unrelated code when implementing method or evidence work

## Next Tier Task Map

This next planning cycle is derived from the current generated truth surface:

- shipped methods: `96`
- compared evidence: `84`
- executable evidence: `12`
- harvested legacy provenance present: `94`
- retained backlog: `0`
- weakest evidence family:
  `Core Retain — Sequence editing and manipulation`
- weak-evidence method count in that family: `9`
- release-truth state: `true`

The remaining pressure is no longer method implementation backlog. It is the
last mile of evidence deepening, harvested-provenance cleanup, and stronger
post-`1.0.0` release discipline.

46. Convert the remaining sequence-editing executable-only methods to compared evidence.
    - Current targets:
      - `biosed`
      - `degapseq`
      - `revseq`
      - `msbar`
      - `trimest`
      - `trimseq`
      - `vectorstrip`
    - This is the largest remaining evidence hole and should be addressed as a
      coherent family sweep.

47. Convert the remaining sequence-construction executable-only methods to compared evidence.
    - Current targets:
      - `makenucseq`
      - `makeprotseq`
    - Keep this focused on anchor execution and deterministic expected outputs,
      not tool redesign.

48. Convert the remaining statistics-family executable-only methods to compared evidence.
    - Current targets:
      - `dan`
      - `complex`
    - `complex` is explicitly retained, so it should not remain an evidence
      outlier once the rest of the retained cohort is mostly compared.

49. Convert the remaining feature-family executable-only method to compared evidence.
    - Current target:
      - `maskfeat`
    - Keep this as a narrow acceptance-anchor and generated-report refresh.

50. Finish harvested legacy provenance for the last non-harvested shipped methods.
    - Current known laggards: none.
    - The shipped cohort now records harvested legacy provenance for all `96`
      shipped methods.
    - Keep this task as a no-regression verification point rather than
      reopening provenance debt that has already been closed honestly.

51. Drive the shipped cohort to full compared evidence.
    - This milestone is now satisfied.
    - Current generated state:
      - `compared_evidence_count == shipped_method_count == 96`
      - `executable_evidence_count == 0`
    - Treat this as a maintained release-truth condition rather than pending
      implementation work.

52. Add a generated “full compared cohort” release gate.
    - Once every shipped method has compared evidence, encode that milestone as
      a generated gate rather than a one-time claim in release notes.
    - The gate should fail if any shipped method drops back below compared
      evidence.

53. Add a generated “harvest coverage exceptions” report.
    - Surface only the shipped methods that still lack harvested legacy
      provenance, with explicit reasons where known.
    - This prevents the remaining provenance debt from being hidden once the
      compared-evidence milestone is met.

54. Reassess whether `harvested_legacy_presence_count` should become a harder release gate.
    - This reassessment is now complete.
    - Because the generated `harvest_coverage` report currently records `0`
      exceptions, release truth now treats harvested coverage as a hard gate:
      - `harvest_coverage_complete == true`
      - release-facing docs must link to the harvest-coverage report
    - If future exceptions reappear, they must be surfaced explicitly through
      the generated report rather than silently tolerated or implied away.

55. Tighten release gating again after full compared-cohort closure.
    - Complete.
    - Release truth now enforces the post-closure conditions directly:
      - no release-facing truth surface may omit the `full compared cohort`
        status once achieved
      - once `full_compared_cohort == true` and retained backlog is `0`, every
        shipped family row in `comparison_coverage` must remain fully compared
        with no executable-only or harvested-but-not-compared remainder
    - This remained a release-process and generated-check change, not a
      product feature sweep.

56. Reassess the `Rework` families after full compared-cohort closure.
    - Complete.
    - The governance appendix now records the post-full-compared reassessment
      explicitly:
      - no family is reclassified in this pass
      - implementation-planning attention is now narrowed to the top of the
        reordered `Rework` shortlist rather than the entire `Rework` surface
      - plotting remains the default first candidate, with remote retrieval as
        the strongest alternative and protein-property rework as the next
        fallback
    - This remains a governance/planning outcome, not a silent implementation
      expansion.

57. Decide whether plotting rework should become the first post-v1.x family implementation program.
    - Complete.
    - Decision:
      - plotting is the chosen first post-v1.x family implementation-program
        candidate
    - Basis:
      - validated plot contracts
      - R rendering ownership
      - governed producers (`charge`, `pepwindow`, `wordcount`)
      - lower architectural ambiguity than the other remaining `Rework`
        families
    - This remains a planning decision, not permission to silently widen the
      plotting surface.

58. Decide whether remote retrieval rework should become the next alternative to plotting.
    - Complete.
    - Decision:
      - plotting remains the first post-v1.x implementation-program candidate
      - remote retrieval is the explicit next alternative if plotting-first is
        later blocked
    - Basis:
      - provider-backed seams
      - mocked compared evidence
      - governed release and docs path
      - strongest remaining operational model after plotting
    - This remains a planning decision, not permission to silently widen the
      retrieval surface.

59. Add a generated “next family recommendation” report if roadmap pressure becomes ambiguous again.
    - Complete without adding a new report.
    - The existing generated surface is already sufficient for the current
      state:
      - `cohort_health` records release-truth health and the absence of
        retained backlog pressure
      - `comparison_coverage` records zero executable-only and zero
        harvested-but-not-compared remainder across all shipped families
      - `full_compared_cohort`, `harvest_coverage`, and
        `retained_backlog_closure` close the remaining ambiguity directly
    - If ambiguity reappears in a future cycle, add a dedicated recommendation
      artefact then rather than pre-creating one without real pressure.

60. Extend this roadmap again from generated truth after Task `59`.
    - Complete.
    - This rollover remains derived from:
      - cohort validation
      - governance alignment
      - cohort health
      - comparison coverage
      - retained backlog closure
      - full compared cohort
      - harvest coverage
    - Planning continues from generated truth rather than memory or
      preference.

## Subsequent Tier Task Map

This next extension is derived from the current generated truth surface:

- shipped methods: `96`
- compared evidence: `96`
- executable evidence: `0`
- harvested legacy provenance present: `96`
- retained backlog: `0`
- full compared cohort: `true`
- harvest coverage complete: `true`
- retained backlog closed: `true`
- `gapped_method_count`: `0`
- weakest evidence family signal is now cleanly closed:
  - `weak_evidence_method_count: 0`
  - `weakest_evidence_family: null`

The retained implementation and shipped-evidence backlog is now closed. The
next tier should therefore move from evidence creation to:
- summary-semantic cleanup
- release-gate hardening
- post-closure documentation truth maintenance
- disciplined preparation for the first actual `Rework` implementation program

61. Re-evaluate `gapped_method_count` semantics now that the shipped cohort is fully compared and fully harvested.
    - Complete.
    - Resolution:
      - `gapped_method_count` now counts only blocking cohort gaps
      - non-blocking `validation_report_gap` notes remain visible per tool but
        no longer inflate the top-line summary
    - Current generated result:
      - `gapped_method_count: 0`
      - `charge` and `pepwindow` still surface visible non-blocking plotting
        notes without being misrepresented as top-line cohort gaps

62. Re-evaluate `weakest_evidence_family` semantics when `weak_evidence_method_count == 0`.
    - Complete.
    - Resolution:
      - `weakest_evidence_family` now becomes `null` when no shipped family is
        below compared evidence
      - the cohort-health gate no longer fabricates a weak-evidence signal or
        recommendation in the zero-burden state
    - Current generated result:
      - `weak_evidence_method_count: 0`
      - `weakest_evidence_family: null`
      - `signals: 0`
      - `recommendations: 0`

63. Add a generated “summary semantics” cleanup pass if Task `61` or `62` reveals stale field meanings.
    - Complete without adding a separate new artefact.
    - Resolution:
      - the needed cleanup happened directly in the existing governed report
        surfaces from Tasks `61` and `62`
      - `gapped_method_count` now reflects blocking cohort gaps only
      - `weakest_evidence_family` now becomes `null` in the zero-burden state
    - The current generated summaries are now internally coherent enough that a
      separate summary-semantics report would be redundant at this stage.

64. Tighten the release gate to require stable post-closure summary semantics.
    - Complete.
    - `release_metadata.py truth-check` now fails if the fully closed
      post-closure state regresses into confusing summary semantics:
      - `gapped_method_count` must remain `0`
      - `weak_evidence_method_count: 0` must imply
        `weakest_evidence_family: null`
      - zero-burden weak-evidence signals and recommendations must remain absent
    - The release-process and release-facing docs now state those invariants
      explicitly so the gate is enforcing checked repository truth rather than
      hidden policy.

65. Reassess the post-`1.0.0` release narrative now that full compared and full harvest are achieved.
    - Complete.
    - The release-facing narrative now describes the current state directly:
      - shipped-method evidence debt is closed for the governed cohort
      - remaining limitations are scope boundaries, release cutover, and future
        `Rework` programs rather than missing compared/harvested evidence
      - the cohort-health gate is described as zero-burden rather than as an
        active weak-evidence reprioritization surface
    - This remained documentation truth maintenance, not marketing expansion.

66. Add a generated “post-closure evidence invariants” report if summary drift remains hard to audit across multiple artefacts.
    - Complete without adding a new report.
    - The current generated surface is already sufficient to express the
      post-closure invariants cleanly:
      - cohort validation
      - cohort health
      - comparison coverage
      - full compared cohort
      - harvest coverage
      - retained backlog closure
    - `release_metadata.py truth-check` now cross-validates those artefacts
      directly, so an additional invariants report would currently duplicate
      checked repository truth rather than clarify it.

67. Decide whether the first actual `Rework` implementation program remains plotting after the summary-semantic cleanup.
    - Complete.
    - Reconfirmation:
      - plotting remains the first actual `Rework` implementation-program
        candidate
      - remote retrieval remains the explicit fallback
    - Basis:
      - `gapped_method_count: 0`
      - `weakest_evidence_family: null`
      - `weak_evidence_method_count: 0`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
    - Nothing in Tasks `61` through `66` changed the plotting-first decision
      materially.

68. If plotting remains first, generate a dedicated plotting rework sub-roadmap before code changes begin.
    - Complete.
    - Dedicated plotting rework sub-roadmap is now recorded in:
      - `docs/governance/appendices/family_to_tool_mapping_reference.md`
    - Bounded Phase 1 method subset:
      - `hmoment`
      - `octanol`
      - `pepinfo`
    - Explicitly out of initial scope:
      - dotplot-style methods
      - diagram/layout methods
      - pretty-display / presentation-heavy methods
      - trace / specialized laboratory-plot methods
    - Program guardrails now recorded:
      - plot-contract evidence remains table-first and contract-validated
      - Rust continues to own computation and typed contract emission only
      - `emboss-r` continues to own rendering and presentation ergonomics
      - broad plotting rollout should not begin until that bounded Phase 1
        path is accepted as the first rework program

69. If plotting is later blocked, generate the equivalent remote-retrieval rework sub-roadmap instead of switching informally.
    - Complete.
    - Dedicated remote-retrieval fallback sub-roadmap is now recorded in:
      - `docs/governance/appendices/family_to_tool_mapping_reference.md`
    - Bounded fallback Phase 1 method subset:
      - `seqretsetall`
      - `seqretsplit`
      - `infoassembly`
    - Explicitly out of initial scope:
      - `assemblyget`
      - `whichdb`
      - `entret`
      - broad provider-parity or multi-source search behavior
    - Program guardrails now recorded:
      - preserve the explicit fallback ordering after plotting
      - keep provider seams mocked or managed for deterministic validation
      - require compared evidence on normalized returned data or metadata
      - do not widen into hidden live-network dependence or informal database
        compatibility claims

70. Reassess whether protein-property rework still remains the third candidate after the first-program sub-roadmap is drafted.
    - Complete.
    - Reassessment outcome:
      - protein-property rework still remains the third candidate
      - plotting remains the first implementation-program candidate
      - remote retrieval remains the explicit fallback second
    - Basis:
      - plotting now has a bounded Phase 1 sub-roadmap
      - remote retrieval now has a bounded fallback Phase 1 sub-roadmap
      - protein-property still has a strong scientific substrate, but it does
        not displace either of the two more explicitly prepared programs
    - Recorded in:
      - `docs/governance/appendices/family_to_tool_mapping_reference.md`
    - This remains a planning check only, not a promotion.

71. Audit `make release-generated-check` for any remaining post-closure ordering hazards or incidental churn.
    - Complete.
    - Audit result:
      - the older acceptance-anchor regeneration-order issue did not reproduce
        in this pass
      - the broad refresh path is still not clean enough for steady-state use
        because it introduces deterministic EOF-only churn across a narrow set
        of generated tool pages
    - Current observed churn set:
      - `aaindexextract`
      - `biosed`
      - `iep`
      - `infobase`
      - `inforesidue`
      - `makenucseq`
      - `makeprotseq`
      - `megamerger`
      - `merger`
      - `msbar`
      - `oddcomp`
      - `pepdigest`
      - `shuffleseq`
      - `sizeseq`
      - `trimest`
      - `vectorstrip`
    - Follow-on:
      - treat the remaining churn as release-process debt in Task `72`, not as
        tolerated folklore

72. If release-generated ordering hazards remain, fix them as release-process debt rather than tolerating them as folklore.
    - Complete.
    - Fix implemented:
      - generated tool-page Markdown is now normalized to exactly one trailing
        newline at emission time
      - the checked-in generated tool pages were refreshed to match that
        canonical form
    - Result:
      - `make release-generated-check` no longer reproduces the earlier
        deterministic EOF-only churn in steady-state use
      - no manual restoration step is needed for the generated tool-page
        subset audited in Task `71`

73. Reassess whether any remaining release-facing generated reports are redundant after post-closure cleanup.
    - Complete without removing any generated report.
    - Reassessment outcome:
      - no current release-facing generated artefact is redundant enough to
        consolidate safely
      - several reports now sit at steady-state `0` or `yes`, but each still
        carries a distinct checked role:
        - `cohort_validation`: per-method evidence and visible-gap truth
        - `governance_alignment`: governance mapping and retained-vs-rework
          reconciliation
        - `cohort_health`: reprioritization and release-truth-drift signaling
        - `comparison_coverage`: family-level compared-coverage summary
        - `full_compared_cohort`: all-shipped-method compared-evidence gate
        - `harvest_coverage`: harvested-provenance exceptions gate
        - `retained_backlog_closure`: retained-backlog closure gate
    - Resolution:
      - keep the current generated report surface intact
      - only revisit consolidation if one report's unique checked invariant is
        later absorbed elsewhere rather than merely reaching a steady-state
        satisfied condition

74. If the first `Rework` program is approved, capture its acceptance criteria as governed roadmap truth before implementation starts.
    - Complete.
    - The plotting-first `Rework` program now has explicit governed acceptance
      criteria recorded in:
      - `docs/governance/appendices/family_to_tool_mapping_reference.md`
    - Recorded entry criteria:
      - plotting must remain the explicitly chosen first `Rework` program
      - bounded Phase 1 scope must remain limited to `hmoment`, `octanol`, and
        `pepinfo`
      - the current zero-burden release-truth state must remain intact
      - the work must stay within analytical-table plus typed plot-contract
        boundaries without moving renderer/layout logic into Rust
    - Recorded exit criteria:
      - all three bounded Phase 1 methods must be shipped
      - each must have governed docs, validation metadata, canonical
        plot-contract fixtures, and compared evidence for both table and
        contract outputs
      - no silent broadening of the plotting family may be implied from Phase
        1 completion alone

75. Extend this roadmap again after Task `74`.
    - Complete.
    - This rollover now replaces the stale placeholder next-tier block with a
      fresh plan derived from the current zero-burden generated truth and the
      now-explicit plotting-program acceptance criteria.
    - The updated extension remains grounded in:
      - cohort validation
      - governance alignment
      - cohort health
      - comparison coverage
      - retained backlog closure
      - full compared cohort
      - harvest coverage
      - plotting-first rework governance
    - Preserve the same repository rules:
      - honest documentation
      - scoped changes
      - commit-and-push completion
      - no unrelated code churn

## Next Tier Task Map

This next extension is derived from the current generated truth surface:

- shipped methods: `96`
- compared evidence: `96`
- executable evidence: `0`
- harvested legacy provenance present: `96`
- retained backlog: `0`
- full compared cohort: `true`
- harvest coverage complete: `true`
- retained backlog closed: `true`
- `gapped_method_count`: `0`
- `weak_evidence_method_count`: `0`
- `weakest_evidence_family`: `null`

The remaining work is now centered on disciplined transition from a fully
closed retained/evidence cohort into the first actual `Rework`
implementation-program cycle. The near-term priorities are:
- preserving the current zero-burden release-truth state
- turning the plotting-first governance choice into an implementation-ready
  operating plan
- keeping the retrieval fallback equally explicit so the repository can switch
  cleanly if plotting scope breaks down
- avoiding silent scope creep while the first rework program is being prepared

76. Add a generated zero-burden release-state report if the existing summary set still feels too distributed during plotting-program prep.
    - Complete without adding a new report.
    - Reassessment outcome:
      - the current checked report set already covers the zero-burden state
        cleanly enough for plotting-program preparation
      - no additional generated surface is needed just to restate a set of
        already-explicit satisfied conditions
    - Current zero-burden state already remains visible through:
      - `cohort_validation`: `gapped_method_count == 0`
      - `cohort_health`: zero retained-backlog and weak-evidence pressure
      - `full_compared_cohort`: `full_compared_cohort == true`
      - `harvest_coverage`: `harvest_coverage_complete == true`
      - `retained_backlog_closure`: `retained_backlog_closed == true`
    - If a later cycle reveals a genuinely missing checked invariant, add a
      dedicated report then rather than creating one preemptively now.

77. Reassess whether the two non-blocking plotting `validation_report_gap` notes should stay visible as-is or move into a more precise report category.
    - Complete.
    - Reassessment outcome:
      - the two plotting notes should remain visible
      - they should not remain in the long term under the overly generic
        `validation_report_gap` label
    - Reason:
      - `charge` and `pepwindow` already have compared evidence and do not
        contribute to `gapped_method_count`
      - the remaining note is narrower: the examples have harvested provenance
        and committed canonical outputs, but they do not yet carry a separate
        explicit legacy-reference artefact
    - Resolution:
      - keep the notes visible now
      - treat the report-category refinement as Task `78`, where cohort,
        health, release docs, and truth-check semantics can be updated
        coherently in one pass instead of as a one-off field rename

78. If Task `77` changes visible gap semantics, update cohort, health, release docs, and truth-check rules coherently in one pass.
    - Complete.
    - Cohort/report-surface refinement implemented:
      - the non-blocking plotting notes for `charge` and `pepwindow` now use
        the more precise visible gap code
        `missing_explicit_legacy_reference`
      - they remain non-blocking and therefore still do not contribute to
        `gapped_method_count`
    - Coherent cross-surface updates landed in one pass:
      - cohort report typing and visible-gap rendering
      - generated shipped cohort JSON and Markdown
      - repository docs and release-facing docs
      - `release_metadata.py truth-check` markers for the release-facing
        narrative
    - Resolution:
      - the note remains visible because it still communicates real provenance
        nuance
      - the note is now categorized precisely enough that it no longer reads
        like a generic unresolved validation deficit

79. Re-ran an end-to-end release-process audit after the Task `77`/`78`
    decision.
    - Result:
      - `make release-generated-check` completed cleanly on a clean tree
      - `make docs` remained clean
      - `truth-check` still matched the post-closure narrative

80. Closed this release-process debt checkpoint after the Task `79` audit.
    - Result:
      - no new deterministic churn was exposed
      - no further generator normalization or ordering fix was required before
        rework planning

81. Added a dedicated implementation-readiness checklist for the plotting-first
    program.
    - The existing acceptance criteria were sufficient on scope but still left
      operational ambiguity before method-order sequencing.
    - The checklist is now recorded in:
      - `docs/governance/appendices/family_to_tool_mapping_reference.md`
    - No new generated report was needed because this is governance/planning
      clarification rather than a new checked truth surface.

82. Translated the plotting Phase 1 acceptance criteria into an explicit
    implementation sequence.
    - The governed execution order is now:
      1. `hmoment`
      2. `octanol`
      3. `pepinfo`
    - The sequence is recorded in:
      - `docs/governance/appendices/family_to_tool_mapping_reference.md`
    - It remains method-associated and evidence-aware:
      - `hmoment` first as the narrowest single-series seam extension
      - `octanol` second as a second single-series analytical profile
      - `pepinfo` third because it is the first likely multi-series contract
        expansion

83. Reassessed whether `hmoment` is still the best first plotting method after
    the implementation sequence was written down.
    - Result:
      - `hmoment` remains the explicit lead method
    - Reason:
      - it is still the narrowest single-series extension of the current
        governed plotting seam
      - `octanol` remains better as the second analytical-profile check
      - `pepinfo` still remains the first likely multi-series contract
        expansion and so should not lead Phase 1
    - The explicit reassessment is now recorded in:
      - `docs/governance/appendices/family_to_tool_mapping_reference.md`

84. If `hmoment` remains first, capture its method-level acceptance criteria before code changes begin.
    - Captured in:
      - `docs/governance/appendices/family_to_tool_mapping_reference.md`
    - The governed method-level acceptance criteria now cover:
      - analytical output expectations
      - typed contract expectations
      - fixture/evidence expectations
      - explicit non-goals
    - `hmoment` is now ready to serve as the first code-bearing plotting task
      boundary without informal widening.

85. Closed this bounded reprioritization checkpoint without reordering.
    - Task `83` confirmed that `hmoment` remains the best first plotting
      method.
    - So no reordered Phase 1 sequence was needed, and no within-family
      reprioritization was recorded.

86. Reconfirm the remote-retrieval fallback plan after the plotting implementation sequence exists.
    - Reconfirmed in:
      - `docs/governance/appendices/family_to_tool_mapping_reference.md`
    - Result:
      - the fallback still has a bounded Phase 1:
        - `seqretsetall`
        - `seqretsplit`
        - `infoassembly`
      - it remains ready for a clean switch if plotting becomes
        architecturally noisy
    - No reordering or widening was needed.

87. Reassess whether protein-property rework still remains the third candidate after the plotting implementation sequence is fixed.
    - Reconfirmed in:
      - `docs/governance/appendices/family_to_tool_mapping_reference.md`
    - Result:
      - protein-property rework still remains the third candidate
    - Reason:
      - plotting now has the strongest implementation-readiness detail
      - remote retrieval remains the clearest prepared fallback
      - protein-property retains a credible substrate but still lacks the same
        immediate start-boundary detail as the two higher-ranked programs
    - This remains a planning checkpoint, not a promotion.

88. If plotting still remains first and `hmoment` is still the lead method, capture the exact start conditions for the first implementation patch.
    - Captured in:
      - `docs/governance/appendices/family_to_tool_mapping_reference.md`
    - The exact start conditions now make the first code-bearing plotting task
      unambiguous before it is mapped.
    - They explicitly require:
      - plotting-first / retrieval-second / protein-property-third ordering to
        remain intact
      - the bounded `hmoment` -> `octanol` -> `pepinfo` order to remain intact
      - the current zero-burden release-truth state to remain intact
      - the first patch to stay limited to `hmoment` plus the smallest support
        needed for computation, contract emission, docs, fixtures, and
        compared evidence

89. Add the next code-bearing rework task tier to this roadmap only after the plotting-first start conditions are explicit enough to prevent informal widening.
    - Added the first bounded code-bearing plotting tier below.
    - It is intentionally limited to `hmoment` and the smallest support needed
      to ship a full governed slice without widening into `octanol` or
      `pepinfo`.
    - The tier is split so each task can close honestly with implementation,
      docs, fixtures, evidence, and release-truth updates.

90. Extended the roadmap again after Task `89`.
    - The next tier now begins from the current zero-burden truth state:
      - shipped methods: `96`
      - compared evidence: `96`
      - harvested legacy provenance present: `96`
      - retained backlog: `0`
      - full compared cohort: `true`
      - harvest coverage complete: `true`
      - retained backlog closed: `true`
    - The next mapped work stays anchored to:
      - cohort validation
      - governance alignment
      - cohort health
      - comparison coverage
      - retained backlog closure
      - full compared cohort
      - harvest coverage
      - release-process audit results
      - plotting-program readiness decisions
    - Preserve the same repository rules:
      - honest documentation
      - scoped changes
      - commit-and-push completion
      - no unrelated code churn

91. Implemented the bounded analytical core for `hmoment`.
    - Landed in:
      - `crates/emboss-core/src/protein_hydrophobic_moment.rs`
      - `crates/emboss-core/src/lib.rs`
    - Result:
      - method-associated Rust computation only
      - deterministic sliding-window hydrophobic-moment profile output
      - no registry/service exposure yet
    - Constraints preserved:
      - protein input only
      - single-series analytical profile only
      - no renderer-coupled logic

92. Add the typed `hmoment` plot-contract path and minimal narrow helper
    support.
    - Landed in:
      - `crates/emboss-tools/src/protein_plots/hmoment.rs`
      - `crates/emboss-tools/src/protein_plots/mod.rs`
      - `crates/emboss-tools/tests/fixtures/hmoment_protein.fasta`
    - Result:
      - typed single-series line-plot contract for the staged `hmoment` path
      - minimal narrow support kept inside the method-associated module
      - focused tests for profile execution, plot shape, and error mapping
    - Constraints preserved:
      - no broad plotting framework
      - no widening into `octanol` or `pepinfo`
      - no registry or service exposure yet

93. Expose `hmoment` through the governed shipped surface.
    - Implemented:
      - `crates/emboss-tools/src/protein_plots/mod.rs`
      - `crates/emboss-tools/src/lib.rs`
      - `crates/emboss-service/src/service.rs`
      - `crates/emboss-cli/src/app.rs`
      - `docs/autodoc/tools/hmoment.json`
      - `docs/generated/tools/hmoment.md`
      - `docs/generated/validation/hmoment.validation.json`
    - Result:
      - `hmoment` is now present in the governed shipped registry and routed
        through the shared service/CLI surface
      - the method now ships with executable evidence and harvested legacy
        provenance
      - the branch intentionally leaves the zero-burden evidence state here:
        - shipped methods: `97`
        - compared evidence: `96`
        - executable evidence: `1`
        - `full_compared_cohort: false`
        - `release_truth_current: true`
        - `python3 scripts/release_metadata.py truth-check`: still expected to
          fail until Task `94` restores full-compared status
    - Constraints preserved:
      - changes stayed method-associated
      - no widening into `octanol` or `pepinfo`

94. Add canonical `hmoment` fixtures and compared evidence.
    - Complete.
    - Landed:
      - committed analytical-output fixture:
        - `crates/emboss-testkit/tests/fixtures/acceptance_anchors/hmoment_hmoment_profile_example.tsv`
      - committed canonical plot-contract fixture:
        - `crates/emboss-tools/tests/fixtures/hmoment_plot_contract.json`
      - governed autodoc update:
        - `docs/autodoc/tools/hmoment.json`
      - acceptance-anchor closure:
        - `crates/emboss-testkit/src/anchor.rs`
    - Outcome:
      - `hmoment` now carries compared evidence for both analytical-table and
        typed plot-contract outputs
      - the shipped cohort returns to the zero-burden evidence state:
        - shipped methods: `97`
        - compared evidence: `97`
        - executable evidence: `0`
        - harvested legacy provenance present: `97`
        - `full_compared_cohort: true`
        - `release_truth_current: true`
    - Constraints preserved:
      - no executable-only closeout
      - no special-case release-truth exceptions

95. Reassess the shipped `hmoment` slice before starting `octanol`.
    - Complete.
    - Reassessment outcome:
      - the seam stayed narrow
      - release-truth stayed clean
      - no contract sprawl or renderer-coupled pressure emerged
    - Recorded explicitly in:
      - `docs/governance/appendices/family_to_tool_mapping_reference.md`
    - Basis:
      - `hmoment` shipped as one bounded analytical helper plus one
        method-associated tool implementation
      - no widening into `octanol`, `pepinfo`, or broader plotting-family
        members was needed
      - compared evidence now closes both analytical-table and typed
        plot-contract outputs
      - the governed release-truth surface remains green:
        - shipped methods: `97`
        - compared evidence: `97`
        - executable evidence: `0`
        - harvested legacy provenance present: `97`
        - `full_compared_cohort: true`
        - `release_truth_current: true`
    - Decision:
      - plotting-first remains valid
      - the repository may proceed to the bounded `octanol` planning tier
      - the retrieval fallback does not need to activate here

96. If `hmoment` passes the post-ship reassessment, map the bounded `octanol`
    implementation tier explicitly before writing code for it.
    - Complete.
    - Mapped bounded `octanol` tier:
      - Task `97`: capture `octanol` method-level acceptance criteria
      - Task `98`: capture exact start conditions for the first `octanol`
        implementation patch
      - Task `99`: implement the bounded analytical core for `octanol`
      - Task `100`: add the typed `octanol` plot-contract emission path
      - Task `101`: expose `octanol` through the governed shipped surface
      - Task `102`: add canonical `octanol` fixtures and compared evidence
      - Task `103`: re-run the full release-truth surface after shipping
        `octanol`
      - Task `104`: reassess the shipped `octanol` slice before starting
        `pepinfo`
    - Constraints preserved:
      - preserve the same no-widening rule used for `hmoment`
      - keep `octanol` single-series unless implementation proves otherwise
      - do not widen into `pepinfo` or broader contract taxonomy while mapping
        the second plotting method

97. Capture `octanol` method-level acceptance criteria before code changes
    begin.
    - Complete.
    - Recorded explicitly in:
      - `docs/governance/appendices/family_to_tool_mapping_reference.md`
    - Captured criteria now require:
      - bounded protein-sequence input only
      - deterministic Rust-computed `octanol` analytical profile
      - stable table-first analytical output
      - deterministic single-series line-plot contract from the same
        computation path
      - governed autodoc, generated docs/validation metadata, canonical
        fixtures, and compared evidence for both table and contract outputs
    - Constraints preserved:
      - same method-associated, single-series, no-widening rules used for
        `hmoment`
      - the distinct `octanol` analytical model is recorded honestly rather
        than treated as a cosmetic `pepwindow` variant

98. Capture the exact start conditions for the first `octanol`
    implementation patch.
    - Complete.
    - Recorded explicitly in:
      - `docs/governance/appendices/family_to_tool_mapping_reference.md`
    - The start gate now requires:
      - plotting-first ordering to remain intact
      - `hmoment` to have already passed explicit post-ship reassessment
      - the current zero-burden release-truth state to remain intact
      - the patch to stay limited to `octanol` plus the smallest support
        needed for deterministic computation, typed single-series contract
        emission, and governed docs/validation plumbing
      - the patch to land as a full governed slice rather than a half-start
    - Constraints preserved:
      - do not widen into `pepinfo`
      - do not introduce broader plot-contract taxonomies or generalized
        plotting-framework work without stopping to reassess

99. Implement the bounded analytical core for `octanol`.
    - Complete.
    - Landed as a method-associated core module in `emboss-core`:
      - deterministic sliding-window White-Wimley interface-minus-octanol
        profile computation
      - explicit bounded error types for non-protein input, invalid window,
        invalid step, short input, and unsupported residues
      - stable one-based window coordinates plus a single analytical series
        field for the bounded v1 result
    - Intentionally not included in this task:
      - no plot-contract emission yet
      - no service or CLI exposure yet
      - no governed docs or validation artefacts yet

100. Add the typed `octanol` plot-contract emission path.
    - Complete.
    - Landed as a staged private `octanol` tool path in `emboss-tools`:
      - typed single-series line-plot contract emitted from the same bounded
        White-Wimley analytical computation path
      - focused staged tests for one-record execution, invalid record count,
        validation-code mapping, and plot-shape invariants
      - one focused protein fixture for the staged path
    - Constraints preserved:
      - no governed shipped-surface exposure yet
      - no autodoc or generated validation/docs yet
      - no canonical committed contract fixture or compared evidence yet

101. Expose `octanol` through the governed shipped surface.
    - Complete.
    - Landed the governed shipped-surface exposure:
      - tool descriptor and governed registry inclusion
      - service routing, parameter parsing, result shaping, and plot-contract
        file emission support
      - CLI tool-path parsing coverage
      - governed autodoc contract plus generated docs/validation/report
        refresh
    - This task intentionally stops at the executable-evidence interim:
      - `octanol` is shipped and harvested
      - canonical committed contract fixtures and compared evidence remain for
        Task `102`

102. Add canonical `octanol` fixtures and compared evidence.
    - Completed. Committed analytical-output and plot-contract fixtures now
      exist for the bounded `octanol` example.
    - Completed. Compared evidence now covers both analytical and plot
      surfaces, returning the shipped cohort to the full-compared state.

103. Re-run the full release-truth surface after shipping `octanol`.
    - Complete.
    - Confirmation:
      - `PYTHON=.venv-docs/bin/python make release-generated-check` now passes
        cleanly after the shipped `octanol` slice
      - the full-compared and harvest-complete gates remained green throughout
        the rerun
      - no special-case exceptions or post-ship release-truth carve-outs were
        introduced
    - Current generated state remains:
      - shipped methods: `98`
      - compared evidence: `98`
      - executable evidence: `0`
      - harvested legacy provenance present: `98`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `release_truth_current: true`

104. Reassess the shipped `octanol` slice before starting `pepinfo`.
    - Complete.
    - Reassessment result:
      - the shipped `octanol` seam stayed narrow
      - no renderer-coupled pressure emerged
      - no broad plotting-framework pressure emerged
      - no retrieval-fallback activation is justified at this boundary
    - Basis:
      - `octanol` shipped as one bounded analytical helper plus one
        method-associated plotting tool path
      - the contract seam stayed single-series and table-derived
      - the evidence path closed fully for both analytical and contract
        outputs
      - the governed release-truth surface remained green:
        - shipped methods: `98`
        - compared evidence: `98`
        - executable evidence: `0`
        - harvested legacy provenance present: `98`
        - `full_compared_cohort: true`
        - `harvest_coverage_complete: true`
        - `release_truth_current: true`
    - Consequence:
      - plotting-first remains valid
      - the repository may proceed to the bounded `pepinfo` planning gate
        without reopening higher-level family selection yet

105. Reconfirm the remote-retrieval fallback again after the second shipped
    plotting method exists.
    - Complete.
    - Reconfirmation result:
      - the remote-retrieval fallback remains ready after the second shipped
        plotting slice
      - `octanol` did not widen the plotting seam far enough to activate the
        fallback
    - Basis:
      - the shipped plotting slices remained method-associated, single-series,
        and fully evidence-closed
      - the fallback still has a distinct bounded Phase 1:
        - `seqretsetall`
        - `seqretsplit`
        - `infoassembly`
      - the fallback still preserves a different operational seam:
        - explicit provider-aware orchestration
        - deterministic mocked-provider or managed-asset validation
        - compared evidence on normalized returned sequence or metadata outputs
      - the governed release-truth surface remained fully green while the
        fallback stayed dormant:
        - shipped methods: `98`
        - compared evidence: `98`
        - harvested legacy provenance present: `98`
        - `full_compared_cohort: true`
        - `harvest_coverage_complete: true`
        - `release_truth_current: true`
    - Consequence:
      - retrieval remains the explicit next alternative if plotting later
        becomes noisy
      - no family-order change is justified at this checkpoint

106. Reassess whether protein-property rework still remains the third
    candidate after two shipped plotting slices exist.
    - Complete.
    - Reassessment result:
      - protein-property rework still remains the third candidate
      - no shortlist change is justified after two shipped plotting slices
    - Basis:
      - plotting now has two shipped bounded slices with no contract sprawl:
        - `hmoment`
        - `octanol`
      - remote retrieval remains the clearest prepared fallback if plotting
        later becomes noisy
      - protein-property rework still has a credible analytical substrate, but
        it still lacks the same immediate implementation-readiness detail now
        recorded for the two higher-ranked programs
      - the governed release-truth surface remained fully green:
        - shipped methods: `98`
        - compared evidence: `98`
        - harvested legacy provenance present: `98`
        - `full_compared_cohort: true`
        - `harvest_coverage_complete: true`
        - `release_truth_current: true`
    - Shortlist remains:
      1. plotting
      2. remote retrieval
      3. protein-property rework
    - This remains a planning checkpoint only and does not promote
      protein-property work.

107. Add the next plotting or retrieval implementation tier to this roadmap
    only after the post-`octanol` reassessment is explicit enough to prevent
    informal widening.
    - Complete.
    - Resolution:
      - the next tier is now mapped explicitly as the bounded `pepinfo`
        plotting tier rather than as a generic “continue plotting” step
      - the retrieval fallback remains documented as the explicit next
        alternative if `pepinfo` forces contract sprawl or renderer-coupled
        pressure
    - The bounded `pepinfo` tier is now:
      1. capture `pepinfo` method-level acceptance criteria
      2. capture exact start conditions for the first `pepinfo`
         implementation patch
      3. implement the bounded analytical core
      4. add the typed plot-contract emission path
      5. expose `pepinfo` through the governed shipped surface
      6. add canonical fixtures and compared evidence
      7. re-run the full release-truth surface after shipping `pepinfo`
      8. reassess the shipped `pepinfo` slice before any broader plotting
         expansion is mapped
    - Guardrails remain explicit:
      - method-associated implementation only
      - no silent widening into a generic plotting framework
      - no Rust-side rendering behavior
      - pause and reassess if `pepinfo` forces materially broader
        plot-contract taxonomy

108. Extend this roadmap again after Task `107`.
    - Complete.
    - The roadmap is now extended from the current generated truth and the
      bounded `pepinfo` tier that Task `107` mapped.
    - Current generated state recorded into this extension:
      - shipped methods: `98`
      - compared evidence: `98`
      - executable evidence: `0`
      - harvested legacy provenance present: `98`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `gapped_method_count: 0`
      - `weakest_evidence_family: null`
      - `release_truth_current: true`
    - Next mapped tier:
      - `109`: capture `pepinfo` method-level acceptance criteria
      - `110`: capture exact start conditions for the first `pepinfo`
        implementation patch
      - `111`: implement the bounded analytical core
      - `112`: add the typed plot-contract emission path
      - `113`: expose `pepinfo` through the governed shipped surface
        - Status: complete
        - Result: `pepinfo` is now shipped through the governed registry,
          service, CLI, and autodoc surface.
      - `114`: add canonical fixtures and compared evidence
        - Status: complete
        - Result: `pepinfo` now has checked-in analytical and plot-contract
          fixtures plus compared acceptance evidence, restoring the governed
          shipped cohort to full compared/full harvest zero-burden state at
          `99` shipped methods.
      - `115`: re-run the full release-truth surface after shipping `pepinfo`
      - `116`: reassess the shipped `pepinfo` slice before any broader
        plotting expansion is mapped
      - `117`: reconfirm the remote-retrieval fallback if `pepinfo` widens the
        seam more than `hmoment` and `octanol` did
      - `118`: reassess whether protein-property still remains the third
        candidate after the full bounded plotting Phase 1 exists
      - `119`: decide explicitly whether bounded plotting Phase 1 is complete
        enough to continue plotting, or whether planning should switch to the
        retrieval fallback instead
      - `120`: if `pepinfo` passes its reassessment, map the next bounded
        post-Phase-1 plotting or retrieval gate explicitly before any further
        code starts
      - `121`: if `pepinfo` fails its reassessment, map the bounded retrieval
        fallback implementation tier explicitly before any further code starts
      - `122`: extend this roadmap again after Task `120` or `121`, using the
        generated truth plus the observed `pepinfo` seam behavior rather than
        the pre-`pepinfo` assumptions

109. Capture `pepinfo` method-level acceptance criteria.
    - Complete.
    - Governed `pepinfo` acceptance criteria are now recorded explicitly in the
      plotting-governance appendix.
    - The criteria preserve the same bounded program shape while making the
      first likely multi-series pressure point explicit:
      - bounded protein-sequence input only
      - deterministic Rust-computed analytical output
      - stable table-first analytical surface
      - typed plot-contract output derived from the same computation path
      - compared evidence required for both table and contract outputs
    - The criteria also make the non-goals explicit:
      - no Rust-side rendering
      - no silent widening into a generic plotting framework
      - no broader contract taxonomy unless `pepinfo` itself forces a real
        reassessment

110. Capture exact start conditions for the first `pepinfo` implementation patch.
    - Complete.
    - Governed `pepinfo` patch start conditions are now recorded explicitly in
      the plotting-governance appendix.
    - The start gate now requires:
      - the current shortlist to remain intact:
        1. plotting
        2. remote retrieval
        3. protein-property rework
      - the bounded plotting order to remain intact:
        1. `hmoment`
        2. `octanol`
        3. `pepinfo`
      - the current zero-burden release-truth state to remain intact
      - the patch to stay limited to `pepinfo` plus the smallest support
        needed for deterministic analytical computation, typed contract
        emission, and governed docs/validation plumbing
      - the patch to land as a full governed slice rather than a half-start
    - The same guardrails remain explicit:
      - no Rust-side rendering
      - no silent widening into a generic plotting framework
      - no broader contract taxonomy unless `pepinfo` itself forces a real
        reassessment

111. Implement the bounded analytical core.
    - Complete.
    - The bounded `pepinfo` analytical core now exists in `emboss-core` as a
      method-associated sliding-window multi-property protein profile.
    - Implemented bounded analytical surface:
      - one stable window row per emitted window
      - deterministic governed residue-property rollups:
        - `mean_hydropathy`
        - `mean_residue_mass`
        - `charged_fraction`
        - `polar_fraction`
      - bounded validation errors for:
        - non-protein input
        - unsupported residues
        - invalid window
        - invalid step
        - sequence shorter than the window
    - This task intentionally stops at the analytical core:
      - no plot-contract emission yet
      - no registry or shipped-surface exposure yet
      - no governed autodoc or compared evidence yet

112. Add the typed plot-contract emission path.
    - Complete.
    - The staged typed `pepinfo` plot-contract path now exists in
      `emboss-tools` as a private method-associated plotting module.
    - Implemented staged surface:
      - one-record-only staged execution path over the Task `111` analytical
        core
      - deterministic table-derived typed line-plot contract
      - four explicit staged series:
        - `mean_hydropathy`
        - `mean_residue_mass`
        - `charged_fraction`
        - `polar_fraction`
      - governed provenance:
        - tool: `pepinfo`
        - method: `protein_pepinfo_profile`
        - source artifact: `table:pepinfo-profile`
    - This task intentionally stops before the shipped surface:
      - no registry or CLI exposure yet
      - no governed autodoc or generated validation/docs yet
      - no canonical fixture or compared evidence yet

113. Expose `pepinfo` through the governed shipped surface.
    - Complete.
    - `pepinfo` is now shipped through the governed registry, service, CLI,
      autodoc, generated docs, and validation-stub surface.
    - This task intentionally stopped at the interim shipped state:
      - shipped with executable evidence
      - harvested legacy provenance present
      - canonical fixtures and compared evidence still pending in Task `114`

114. Add canonical fixtures and compared evidence.
    - Complete.
    - `pepinfo` now has checked-in analytical and multi-series plot-contract
      fixtures plus compared acceptance evidence.
    - The governed shipped cohort returned to zero-burden release-truth state:
      - shipped methods: `99`
      - compared evidence: `99`
      - executable evidence: `0`
      - harvested legacy provenance present: `99`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `release_truth_current: true`

115. Re-run the full release-truth surface after shipping `pepinfo`.
    - Complete.
    - The full governed release-generated and release-truth surface was rerun
      after the bounded `pepinfo` slice closed.
    - Observed result:
      - `PYTHON=.venv-docs/bin/python make release-generated-check` passed
      - `python3 scripts/release_metadata.py truth-check` passed
      - `PYTHON=.venv-docs/bin/python make docs` passed
      - `git diff --check` passed
    - The generated state remained fully green:
      - shipped methods: `99`
      - compared evidence: `99`
      - executable evidence: `0`
      - harvested legacy provenance present: `99`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

116. Reassess the shipped `pepinfo` slice before any broader plotting expansion is mapped.
    - Reassess whether the `pepinfo` seam stayed method-associated and
      renderer-agnostic enough to justify further bounded plotting work.
    - If `pepinfo` forces materially broader contract taxonomy, pause and do
      not map broader plotting work by inertia.

117. Reconfirm the remote-retrieval fallback if `pepinfo` widens the seam more than `hmoment` and `octanol` did.
    - If `pepinfo` broadens the plotting seam beyond the bounded Phase 1
      assumptions, explicitly restate whether the retrieval fallback should now
      become the next active implementation program.

118. Reassess whether protein-property still remains the third candidate after the full bounded plotting Phase 1 exists.
    - Once all three bounded plotting methods exist, check whether the current
      shortlist still holds:
      1. plotting
      2. remote retrieval
      3. protein-property rework

119. Decide explicitly whether bounded plotting Phase 1 is complete enough to continue plotting, or whether planning should switch to the retrieval fallback instead.
    - Make the decision explicitly from observed seam behavior rather than from
      the pre-`pepinfo` assumptions.

120. If `pepinfo` passes its reassessment, map the next bounded post-Phase-1 plotting or retrieval gate explicitly before any further code starts.
    - Do not continue directly into more plotting implementation without first
      mapping the next bounded tier from the observed `pepinfo` seam.

121. If `pepinfo` fails its reassessment, map the bounded retrieval fallback implementation tier explicitly before any further code starts.
    - If the plotting seam becomes too broad, switch planning cleanly to the
      prepared retrieval fallback rather than widening plotting informally.

122. Extend this roadmap again after Task `120` or `121`, using the generated truth plus the observed `pepinfo` seam behavior rather than the pre-`pepinfo` assumptions.
    - Rebase the next planning tier on the actual post-`pepinfo` state.
