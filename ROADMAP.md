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
    - Complete.
    - The shipped `pepinfo` slice has now passed explicit reassessment in the
      plotting-governance appendix.
    - The observed seam stayed bounded enough to justify further planning
      without switching families:
      - `pepinfo` remained method-associated and renderer-agnostic
      - the first multi-series contract stayed table-derived and bounded to the
        method rather than widening into a generic plotting framework
      - no broader plotting-family members were implicitly started
      - the release-truth surface remained fully green at `99` shipped methods
    - Result:
      - no retrieval-fallback activation is justified at this checkpoint
      - the repository may proceed to the next explicit post-Phase-1 planning
        decision rather than pausing bounded plotting by default

117. Reconfirm the remote-retrieval fallback if `pepinfo` widens the seam more than `hmoment` and `octanol` did.
    - Complete.
    - The remote-retrieval fallback has now been explicitly reconfirmed after
      the shipped `pepinfo` reassessment.
    - Result:
      - the fallback remains ready but inactive
      - no activation is justified because bounded plotting Phase 1 still
        stayed method-associated, table-derived, and renderer-agnostic even
        after the first multi-series method shipped
      - the repository should preserve the same family ordering:
        1. plotting
        2. remote retrieval
        3. protein-property rework
    - The fallback itself remains unchanged and ready for a clean switch if
      later plotting work crosses the bounded Phase 1 seam into real contract
      sprawl or renderer-coupled pressure.

118. Reassess whether protein-property still remains the third candidate after the full bounded plotting Phase 1 exists.
    - Complete.
    - Protein-property rework has now been explicitly reassessed after the full
      bounded plotting Phase 1 exists.
    - Result:
      - the shortlist still holds:
        1. plotting
        2. remote retrieval
        3. protein-property rework
      - plotting now has three shipped bounded methods, including the first
        multi-series case, without forcing framework sprawl
      - remote retrieval still remains the clearest prepared fallback
      - protein-property still has a credible substrate, but it does not yet
        have the same immediate implementation-readiness detail as the two
        higher-ranked programs
    - This remains a planning checkpoint only and does not promote
      protein-property work.

119. Decide explicitly whether bounded plotting Phase 1 is complete enough to continue plotting, or whether planning should switch to the retrieval fallback instead.
    - Complete.
    - The post-Phase-1 family decision has now been made explicitly in the
      plotting-governance appendix.
    - Result:
      - continue plotting-family planning as the active path
      - keep remote retrieval as the explicit prepared fallback
      - do not switch families at this checkpoint
    - Basis:
      - all three bounded plotting methods stayed method-associated and
        renderer-agnostic
      - `pepinfo` introduced the first multi-series case without forcing a
        generic plotting framework or broader contract taxonomy
      - the release-truth surface remained fully green at `99` shipped methods
    - So the next planning action should be to map the next bounded
      post-Phase-1 gate explicitly rather than switching implementation
      families by inertia.

120. If `pepinfo` passes its reassessment, map the next bounded post-Phase-1 plotting or retrieval gate explicitly before any further code starts.
    - Complete.
    - The next bounded post-Phase-1 gate is now mapped explicitly in the
      plotting-governance appendix.
    - Result:
      - the next gate stays inside plotting-family planning rather than
        switching families immediately
      - it is a bounded Phase 2 candidate-selection and seam-compatibility
        gate for the remaining plotting-family methods
    - The mapped gate requires the repository to:
      - inventory the remaining plotting-family methods not already covered by
        the preexisting governed seam or the bounded Phase 1 slice
      - classify the remaining methods by seam pressure
      - decide whether at least one remaining method still fits the proven
        method-associated, table-derived, renderer-agnostic seam
      - either map exactly one bounded next-method plotting tier or switch
        cleanly to the already-prepared retrieval fallback planning path
    - So no further plotting code should start until that bounded Phase 2 gate
      is handled explicitly.

121. If `pepinfo` fails its reassessment, map the bounded retrieval fallback implementation tier explicitly before any further code starts.
    - Complete as not triggered.
    - `pepinfo` passed its reassessment, so this branch was not activated.
    - Result:
      - no bounded retrieval-fallback implementation tier was mapped here
      - the repository remains on the `120` path:
        - bounded Phase 2 plotting candidate-selection and seam-compatibility
          gating
      - the prepared retrieval fallback remains documented and ready, but it
        was not promoted to the active planning path
    - This closes the conditional branch honestly instead of inventing a
      fallback implementation start that the governed decision did not choose.

122. Extend this roadmap again after Task `120` or `121`, using the generated truth plus the observed `pepinfo` seam behavior rather than the pre-`pepinfo` assumptions.
    - Complete.
    - The roadmap is now rebased on the actual post-`pepinfo` state rather
      than on the pre-`pepinfo` assumptions.
    - Current governed truth carried forward into the next tier:
      - shipped methods: `99`
      - compared evidence: `99`
      - executable evidence: `0`
      - harvested legacy provenance present: `99`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `gapped_method_count: 0`
      - `weakest_evidence_family: null`
      - `release_truth_current: true`
    - Current planning consequence carried forward:
      - bounded plotting Phase 1 passed explicit reassessment
      - the repository remains on the plotting path
      - the next gate is bounded Phase 2 plotting candidate selection, not
        direct implementation

## Next Tier Task Map

123. Inventory the remaining plotting-family methods not already covered by the preexisting governed seam or the bounded Phase 1 slice.
    - Complete.
    - The explicit Phase 2 plotting-family candidate pool is now recorded in
      the plotting-governance appendix.
    - The inventory preserves the seam distinction honestly:
      - governed seam precedents:
        - `charge`
        - `pepwindow`
        - `wordcount`
      - bounded Phase 1 shipped plotting-family slice:
        - `hmoment`
        - `octanol`
        - `pepinfo`
      - important note:
        - `wordcount` is part of the governed plotting seam, but it is not
          itself a member of the plotting-family mapping
    - The actual remaining plotting-family candidate pool is therefore the
      remaining `23` plotting-family methods:
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
    - This task is inventory only. It does not yet classify seam pressure or
      choose the Phase 2 candidate.

124. Classify the remaining plotting-family methods by seam pressure.
    - Complete.
    - The explicit Phase 2 plotting-family pool is now classified in the
      plotting-governance appendix as bounded seam-pressure buckets.
    - Classification result:
      - likely seam-compatible:
        - `banana`
        - `cpgplot`
        - `density`
        - `isochore`
        - `syco`
        - `wobble`
      - requires broader contract taxonomy:
        - `chaos`
        - `pepwindowall`
        - `plotcon`
      - dotplot-style or comparative-matrix heavy:
        - `dotmatcher`
        - `dotpath`
        - `dottup`
        - `polydot`
      - diagram/layout or presentation-heavy:
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
    - This remains governance-only classification. It does not yet choose the
      Phase 2 candidate or imply that every seam-compatible method is equally
      suitable.

125. Reassess whether any remaining plotting-family method still fits the proven bounded seam closely enough to justify Phase 2.
    - Complete.
    - The actual Phase 2 pass/fail decision has now been made from the
      classified remainder set.
    - Result:
      - yes, plotting still has bounded Phase 2 candidates that fit the proven
        seam closely enough to justify another bounded plotting tier
      - the no-candidate branch is not taken
      - retrieval fallback remains ready, but it is not activated here
    - The current viable Phase 2 pool is:
      - `banana`
      - `cpgplot`
      - `density`
      - `isochore`
      - `syco`
      - `wobble`
    - So the repository should remain on the plotting-family path and proceed
      to the next bounded decision: choose exactly one Phase 2 candidate from
      this viable pool.

126. If at least one plotting-family method remains seam-compatible, choose exactly one bounded Phase 2 candidate.
    - Complete.
    - Exactly one bounded Phase 2 plotting candidate has now been chosen in
      the plotting-governance appendix:
      - `density`
    - Selection basis:
      - it appears to be the closest extension of the proven seam as a likely
        single-series nucleotide analytical profile
      - it looks more naturally table-first and typed-contract-friendly than
        the more event- or region-oriented candidates such as `cpgplot` or
        `isochore`
      - it avoids the extra coding-sequence or codon-usage specificity likely
        to arise in `syco` or `wobble`
      - it does not immediately signal the broader contract-taxonomy pressure
        associated with methods like `chaos`, `pepwindowall`, or `plotcon`
    - The non-selected viable methods remain viable but inactive:
      - `banana`
      - `cpgplot`
      - `isochore`
      - `syco`
      - `wobble`
    - So the next bounded planning step is to capture `density`-specific
      acceptance criteria and exact patch start conditions before any code
      starts.

127. If no plotting-family method remains seam-compatible, promote the retrieval fallback from “ready” to the active next planning program.
    - Complete as not triggered.
    - The no-candidate branch was not activated because Task `125` confirmed
      that bounded Phase 2 plotting remains viable and Task `126` selected
      `density` as the single bounded next-method candidate.
    - Result:
      - retrieval fallback remains documented, prepared, and inactive
      - it was not promoted to the active planning path here
      - the repository remains on the plotting continuation branch
    - This closes the conditional branch honestly instead of inventing a
      fallback promotion that the governed decision did not choose.

128. If a plotting Phase 2 candidate exists, capture that method’s bounded acceptance criteria before any code starts.
    - Complete.
    - Governed method-level acceptance criteria for the selected Phase 2
      candidate `density` are now recorded explicitly in the
      plotting-governance appendix.
    - The criteria keep the bounded Phase 2 shape parallel to the earlier
      method-level planning pattern used for `hmoment`, `octanol`, and
      `pepinfo`.
    - Recorded expectations:
      - bounded nucleotide-sequence analytical profile
      - stable table-first analytical output
      - deterministic typed plot contract from the same computation path
      - canonical analytical and plot-contract fixtures
      - compared evidence for both table and contract outputs
    - Recorded non-goals:
      - no Rust-side figure rendering
      - no generic plotting-framework widening
      - no dotplot, matrix, circular-map, or pretty-display behavior
      - no broader contract taxonomy unless `density` itself forces a real
        reassessment

129. If a plotting Phase 2 candidate exists, capture exact start conditions for its first implementation patch.
    - Complete.
    - Governed patch-start conditions for the selected Phase 2 candidate
      `density` are now recorded explicitly in the plotting-governance
      appendix.
    - The start gate preserves the same no-widening rules used in bounded
      Phase 1 and now requires:
      - plotting-first ordering to remain intact
      - bounded plotting Phase 1 reassessment to remain passed
      - the Phase 2 viability gate to remain passed with `density` as the
        single selected candidate
      - the zero-burden release-truth surface to remain intact
      - the patch to stay limited to `density` plus the smallest support
        needed for deterministic computation, typed contract emission, and
        governed docs/validation plumbing
      - the patch to land as a full governed slice rather than a half-start
    - The same guardrails remain explicit:
      - no Rust-side rendering
      - no generalized plotting framework
      - no broader contract taxonomy unless `density` itself forces a real
        reassessment

130. If a plotting Phase 2 candidate exists, map the full bounded implementation tier for that one method before writing code.
    - Complete.
    - The full bounded implementation tier for the selected Phase 2 candidate
      `density` is now mapped explicitly in the plotting-governance appendix.
    - The mapped tier is:
      1. implement the bounded analytical core
      2. add the typed plot-contract emission path
      3. expose `density` through the governed shipped surface
      4. add canonical analytical and plot-contract fixtures plus compared
         evidence
      5. re-run the full release-truth surface after shipping `density`
      6. reassess the shipped `density` slice before any further Phase 2
         plotting continuation is mapped
    - The mapped tier preserves the same bounded architectural rules:
      - method-associated implementation only
      - table-first analytical output
      - typed contract output from the same computation path
      - no Rust-side rendering
      - no generic plotting-framework widening
      - no broader contract taxonomy unless `density` itself forces a real
        reassessment

131. If the no-candidate branch is chosen, map the bounded retrieval fallback implementation tier explicitly.
    - Complete as an untriggered conditional branch.
    - The no-candidate branch was not chosen because Task `125` confirmed that
      bounded Phase 2 plotting remains viable, and Task `126` selected
      `density` as the single active Phase 2 candidate.
    - No bounded retrieval fallback implementation tier was mapped in this
      task.
    - The already-governed retrieval fallback shortlist remains documented and
      prepared, but inactive:
      - `seqretsetall`
      - `seqretsplit`
      - `infoassembly`

132. If the no-candidate branch is chosen, choose exactly one retrieval fallback lead method.
    - Complete as an untriggered conditional branch.
    - No retrieval fallback lead method was chosen in this task because the
      no-candidate branch remains inactive:
      - bounded Phase 2 plotting remains viable
      - `density` remains the single selected active Phase 2 candidate
    - The retrieval path therefore remains documented and prepared, but not
      promoted to the active planning branch.
    - If fallback activation ever becomes necessary later, the retrieval path
      should still be kept bounded and method-associated in the same way the
      plotting path was bounded.

133. Reconfirm that protein-property still remains the third candidate after the Phase 2 candidate-selection gate resolves.
    - Complete.
    - The Phase 2 candidate-selection gate has now resolved on the plotting
      branch:
      - bounded Phase 2 plotting remains viable
      - `density` is the single selected active continuation candidate
      - retrieval fallback remains documented, prepared, and inactive
    - Protein-property rework still remains the third candidate in the
      shortlist because:
      1. plotting now has the most immediate next-step readiness through the
         selected bounded `density` tier
      2. remote retrieval remains the clearest prepared fallback if plotting
         later fails a boundedness check
      3. protein-property still has a credible substrate, but it still lacks
         the same immediate implementation-readiness detail as the two
         higher-ranked programs

134. Reconfirm that the release-truth surface still remains zero-burden after the Phase 2 candidate-selection decision.
    - Complete.
    - The release-truth surface remains zero-burden after the Phase 2
      candidate-selection decision resolved on the plotting branch.
    - No new report was needed because the existing generated truth surface
      already expresses this state cleanly:
      - shipped cohort summary still reports `99` compared methods, `0`
        executable-only methods, and `0` blocking gaps
      - cohort health still reports:
        - `release_truth_current: true`
        - `weak_evidence_method_count: 0`
        - `weakest_evidence_family: null`
        - `retained_backlog_count: 0`
      - full compared cohort still reports `below_compared_method_count: 0`
      - harvest coverage still reports `harvest_exception_count: 0`
      - retained backlog closure still reports
        `retained_backlog_closed: true`
    - This remains a documentation/reporting truth check only, not a prompt
      for another generated report.

135. If the repository stays on plotting, capture the explicit stop conditions that would finally force fallback activation.
    - Complete.
    - The explicit fallback-activation stop conditions are now recorded in the
      plotting-governance appendix for the bounded `density` tier and any later
      plotting continuation gate.
    - Continued plotting should now stop and the prepared retrieval fallback
      should activate if any of the following becomes true:
      1. the method cannot stay table-first with a typed contract derived from
         the same computation path
      2. the method requires Rust-side rendering, layout, styling, or other
         presentation policy
      3. the method cannot remain method-associated and instead demands a
         generalized plotting framework before one bounded shipped slice closes
      4. the method forces a broader non-local plot-contract taxonomy
      5. the bounded slice cannot close as a full governed shipment with
         fixtures, compared evidence, and a green release-truth surface
      6. after `density`, no equally bounded next plotting candidate remains
    - This replaces the earlier generic “contract sprawl” wording with an
      explicit activation gate.

136. Extend the roadmap again after the Phase 2 candidate-selection branch is resolved.
    - Complete.
    - The roadmap has now been extended from the actual resolved branch rather
      than the earlier placeholder split.
    - Current governed truth at this extension point:
      - shipped methods: `99`
      - compared evidence: `99`
      - executable evidence: `0`
      - harvested legacy provenance present: `99`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `gapped_method_count: 0`
      - `weakest_evidence_family: null`
      - `release_truth_current: true`
    - The resolved planning state is now explicit:
      - bounded plotting Phase 1 passed
      - the Phase 2 seam-compatibility gate passed
      - `density` is the single active bounded continuation candidate
      - retrieval fallback remains documented and prepared, but inactive unless
        the explicit stop conditions are triggered
    - The next mapped tier is now:

137. Implement the bounded analytical core for `density`.
    - Complete.
    - Added the bounded analytical core in `crates/emboss-core` as a
      method-associated sliding-window nucleotide-density profile.
    - The core remains table-first and bounded:
      - nucleotide-sequence input only
      - explicit window and step controls
      - one row per emitted window
      - stable fractions for:
        - `a`
        - `c`
        - `g`
        - `t/u`
        - `at`
        - `gc`
      - explicit honesty columns for:
        - canonical symbols
        - ambiguous symbols
        - ignored gap symbols
    - The landed scope stays intentionally narrow:
      - no typed plot-contract emission yet
      - no governed shipped-surface exposure yet
      - no renderer-coupled logic

138. Add the typed plot-contract emission path for `density`.
    - Complete.
    - Added the staged typed plot-contract path in `crates/emboss-tools` as a
      separate nucleotide plotting module over the bounded core.
    - The staged `density` path remains bounded:
      - exactly one nucleotide record
      - analytical table from the same computation path as Task `137`
      - deterministic single-series line contract
      - no Rust-side rendering behavior
      - no generalized plotting framework
    - The bounded v1 contract uses the analytically derived `gc_fraction` line
      as the emitted series while preserving the richer base-fraction table in
      the analytical output.
    - The landed scope stays intentionally narrow:
      - no governed shipped-surface exposure yet
      - no autodoc or generated validation/docs yet
      - no canonical fixture or compared evidence yet

139. Complete. `density` is now exposed through the governed shipped surface.
    The staged Phase 2 nucleotide plotting path is now wired through the
    governed registry, service, CLI, autodoc, generated tool page, and
    validation stub. This leaves the repository in the intended interim state
    for this task boundary: shipped methods `100`, compared evidence `99`,
    executable evidence `1`, harvest coverage complete, and full-compared
    cohort temporarily false until the canonical fixtures and compared
    acceptance evidence land in Task 140.
    - Wire the bounded method through registry, service, CLI, and governed
      autodoc.
    - Accept the temporary shipped-plus-executable-evidence interim only until
      the compared-evidence follow-on closes.

140. Complete. Add canonical analytical and plot-contract fixtures plus compared evidence for `density`.
    The `density` slice now carries a checked-in analytical fixture, a
    canonical GC-density plot-contract fixture, and acceptance-anchor coverage
    for both surfaces. The governed report surface is back to the zero-burden
    state: shipped methods `100`, compared evidence `100`, executable evidence
    `0`, harvested legacy provenance present `100`, full-compared cohort
    `true`, and release-truth current `true`.

141. Complete. Re-run the full release-truth surface after shipping `density`.
    The broad post-ship gate stayed clean. `make release-generated-check`,
    `python3 scripts/release_metadata.py truth-check`, `make docs`, and
    `git diff --check` all passed after the `density` slice closed. The
    observed governed state remained fully green: shipped methods `100`,
    compared evidence `100`, executable evidence `0`, harvested legacy
    provenance present `100`, full-compared cohort `true`, harvest coverage
    complete `true`, retained-backlog closed `true`, and release-truth current
    `true`.

142. Complete. Reassess the shipped `density` slice before any further plotting continuation is mapped.
    The `density` slice stayed inside the bounded seam. It shipped as one
    method-associated nucleotide analytical helper plus one method-associated
    plotting path, kept the table-first surface honest with richer base-density
    rows, and emitted only a bounded single-series GC-density contract rather
    than forcing broader taxonomy or renderer-coupled pressure. No fallback-
    activation stop condition tripped, and the governed release-truth surface
    remained fully green: shipped methods `100`, compared evidence `100`,
    executable evidence `0`, harvested legacy provenance present `100`,
    full-compared cohort `true`, harvest coverage complete `true`,
    retained-backlog closed `true`, and release-truth current `true`.

143. Complete. Close the conditional retrieval-activation branch honestly.
    No fallback-activation stop condition was crossed by the shipped `density`
    slice, so the prepared retrieval path was not promoted here. The
    repository remains on the plotting continuation branch, while the governed
    retrieval fallback remains documented, prepared, and inactive.

144. Complete. Close the untriggered retrieval-lead branch honestly.
    No bounded retrieval lead method was chosen here because the retrieval
    fallback was not activated. The governed shortlist remains documented,
    prepared, and inactive:
      - `seqretsetall`
      - `seqretsplit`
      - `infoassembly`

145. Complete. Inventory the remaining plotting-family Phase 2 pool after removing `density`.
    The post-`density` remainder set is now explicit rather than inherited
    from the older pre-shipment inventory. With `density` removed, the active
    plotting-family remainder pool is the remaining `22` methods:
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

146. Complete. Classify the post-`density` plotting remainder by seam pressure again.
    The rebased remainder pool now splits more narrowly than the earlier
    pre-`density` view. After an actual bounded Phase 2 shipment exists, the
    strongest remaining seam-compatible candidates are:
      - `banana`
      - `isochore`
      - `syco`
      - `wobble`
    `cpgplot` remains plausible but now sits in a more conditional
    region-oriented bucket, while the broader-taxonomy, dotplot, diagram, and
    specialized-trace buckets remain outside the currently proven bounded seam.

147. Complete. Decide whether another bounded Phase 2 plotting candidate still exists.
    Yes. Another bounded plotting continuation candidate still exists after
    the `density` shipment gate resolves. The strongest current next-candidate
    pool remains non-empty:
      - `banana`
      - `isochore`
      - `syco`
      - `wobble`
    So the no-candidate and fallback-activation branches are not taken here,
    and the repository remains on the plotting path for the next bounded
    selection step.

148. Complete. Reconfirm that protein-property still remains the third candidate after the `density` shipment gate resolves.
    The shortlist does not change after the `density` shipment gate:
      1. plotting
      2. remote retrieval
      3. protein-property rework
    Plotting remains the active path because the bounded seam survived a Phase
    2 shipment and still has a non-empty next-candidate pool, retrieval
    remains the clearest prepared fallback, and protein-property still lacks
    the same immediate bounded continuation detail as the two higher-ranked
    programs.

149. Complete. Reconfirm that the release-truth surface still remains zero-burden after the `density` shipment gate resolves.
    The existing generated surface remains sufficient and fully green, so no
    new report was needed. The current checked state remains:
      - shipped cohort: `100` compared, `0` executable-only, `0` blocking gaps
      - full compared cohort: `100/100`, `0` below compared
      - harvest coverage: `100` harvested, `0` exceptions
      - retained backlog closure: `0` retained backlog, closed `true`
      - cohort health: `weak_evidence_method_count 0`,
        `weakest_evidence_family null`, `release_truth_current true`

150. Complete. Extend the roadmap again after the `density` shipment gate resolves.
    The roadmap is now rebased on the actual observed branch:
      - plotting remained the active path
      - retrieval fallback remained documented, prepared, and inactive
      - the strongest next-candidate pool narrowed to:
        - `banana`
        - `isochore`
        - `syco`
        - `wobble`
    The current governed truth at this extension point remains:
      - shipped methods: `100`
      - compared evidence: `100`
      - executable evidence: `0`
      - harvested legacy provenance present: `100`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `gapped_method_count: 0`
      - `weakest_evidence_family: null`
      - `release_truth_current: true`

151. Complete. Choose exactly one bounded post-`density` plotting continuation candidate from the narrowed strong pool.
    The selected next bounded plotting candidate is:
      - `wobble`
    The selection basis is:
      - `wobble` looks like the closest remaining extension of the currently
        proven seam as a likely single-series nucleotide analytical profile
      - it appears more naturally table-first and typed-contract-friendly than
        the more region-oriented `isochore`
      - it appears narrower and easier to keep method-associated than
        `banana`, which signals a heavier biophysical model surface, or `syco`,
        which signals a broader synonymous-codon-usage interpretation surface
      - it does not immediately imply the broader contract-taxonomy or
        presentation pressure already excluded elsewhere in the plotting family
    The non-selected strong candidates remain viable but inactive:
      - `banana`
      - `isochore`
      - `syco`
    So the next bounded planning step is to capture `wobble`-specific
    acceptance criteria and exact patch start conditions before any code
    starts.

152. Complete. If no credible bounded continuation candidate remains after the selection review, activate the prepared retrieval fallback path explicitly.
    This conditional branch was not triggered. After the bounded selection
    review:
      - the narrowed plotting pool did not collapse
      - `wobble` was selected as the active next bounded continuation
        candidate
    So the repository does not activate the retrieval fallback here. The
    prepared retrieval shortlist remains documented, ready, and inactive.

153. Complete. If plotting remains active, close the untriggered retrieval-activation branch honestly.
    Plotting remains active after the bounded selection review, so this branch
    is now closed explicitly rather than left implied. The repository should:
      - keep the retrieval shortlist documented and prepared
      - not promote retrieval by inertia
      - continue on the active `wobble` plotting branch

154. Complete. Capture method-level acceptance criteria for the selected next plotting candidate.
    `wobble` now has explicit method-level acceptance criteria recorded in the
    plotting-governance appendix. Those criteria now make four things explicit
    before code starts:
      - analytical expectations
      - typed-contract expectations
      - fixture and evidence expectations
      - explicit non-goals
    The governed shape keeps `wobble` bounded as a coding-sequence analytical
    profile with a table-first output and a deterministic typed plot contract
    derived from the same computation path.

155. Complete. Capture exact patch start conditions for the selected next plotting candidate.
    `wobble` now has explicit start conditions recorded in the
    plotting-governance appendix. The start gate now requires:
      - plotting-first ordering to remain intact
      - the post-`density` continuation gate to remain passed with `wobble`
        as the single selected candidate
      - the current zero-burden release-truth state to remain intact
      - the first patch to stay limited to `wobble` plus the smallest support
        needed for deterministic computation, typed contract emission, and
        governed docs/validation plumbing
      - the patch to land as a full governed slice rather than a half-start

156. Complete. Map the full bounded implementation tier for the selected next plotting candidate.
    The bounded `wobble` implementation tier is now explicit:
      1. implement the bounded analytical core
      2. add the typed plot-contract emission path
      3. expose `wobble` through the governed shipped surface
      4. add canonical analytical and plot-contract fixtures plus compared
         evidence
      5. re-run the full release-truth surface after shipping `wobble`
      6. reassess the shipped `wobble` slice before any further continuation
         is mapped
    The same bounded constraints remain explicit:
      - method-associated implementation only
      - table-first analytical output
      - typed contract output from the same computation path
      - no Rust-side rendering
      - no generic plotting-framework widening

157. Complete. If the selected next plotting candidate widens into region-track, threshold-call, or broader contract-taxonomy pressure before code starts, pause and reconsider the family path.
    The pre-code seam-pressure stop conditions are now explicit for `wobble`.
    The repository should pause and reopen planning if, before code starts:
      - `wobble` cannot remain table-first with a typed contract derived from
        the same computation path
      - `wobble` requires Rust-side rendering, layout, styling, or other
        presentation policy
      - `wobble` cannot remain method-associated and instead demands a
        generalized plotting framework
      - `wobble` forces region-track, threshold-call, or broader
        plot-contract-taxonomy pressure that is not clearly local to the
        method
    This keeps the branch honest and prevents force-fitting `wobble` through a
    seam it no longer actually matches.

158. Complete. If the selected next plotting candidate stays bounded through planning, implement its bounded analytical core.
    The bounded analytical core for `wobble` now exists in the Rust core
    layer as a method-associated third-base-position variability profile.
    The shipped core stays table-first and coding-sequence-specific:
      - codon-windowed analytical rows
      - explicit wobble-base composition columns
      - a single derived `wobble_variability` metric for later plotting
    This task intentionally stops at the analytical core:
      - no plot-contract emission yet
      - no shipped-surface exposure yet
      - no fixtures or compared evidence yet

159. Complete. Add the typed plot-contract emission path for the selected next plotting candidate.
    The staged `wobble` plotting path now exists in the tools layer as a
    table-derived single-series line contract over the bounded analytical
    core. The emitted contract stays method-associated and renderer-agnostic:
      - tool: `wobble`
      - method: `nucleotide_wobble_profile`
      - source artifact: `table:wobble-profile`
      - emitted series: `wobble_variability`
    This task intentionally stops short of shipping the governed surface:
      - no registry or CLI exposure yet
      - no governed autodoc or generated validation/docs yet
      - no canonical fixtures or compared evidence yet

160. Complete. Expose the selected next plotting candidate through the governed shipped surface.
    `wobble` is now wired through the governed shipped surface:
      - tool descriptor and registry
      - shared service invocation path
      - CLI tool routing
      - governed autodoc contract
      - generated docs and validation stub
    This task intentionally stops at the same interim state used by earlier
    shipment steps:
      - shipped plus executable evidence
      - not yet restored to full compared state
      - canonical compared fixtures remain the next task

161. Complete. Add canonical analytical and plot-contract fixtures plus compared evidence for the selected next plotting candidate.
    `wobble` now has canonical checked-in analytical and plot-contract fixtures
    wired into the acceptance-anchor harness. The governed reports are restored
    to the full compared/full harvest state after the temporary shipped-plus-
    executable-only interim:
      - shipped methods: `101`
      - compared evidence: `101`
      - executable evidence: `0`
      - harvested legacy provenance present: `101`
      - `full_compared_cohort: true`
      - `release_truth_current: true`

162. Complete. Re-run the full release-truth surface after shipping the selected next plotting candidate.
    The broad post-`wobble` release-truth rerun completed cleanly:
      - `PYTHON=.venv-docs/bin/python make release-generated-check`
      - `python3 scripts/release_metadata.py truth-check`
      - `PYTHON=.venv-docs/bin/python make docs`
      - `git diff --check`
    The governed release surface remains fully green after the `wobble`
    evidence closure:
      - shipped methods: `101`
      - compared evidence: `101`
      - executable evidence: `0`
      - harvested legacy provenance present: `101`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

163. Complete.
    - Reassessed the shipped `wobble` slice before any further continuation was
      mapped.
    - The seam remained bounded: `wobble` stayed method-associated, the
      analytical surface stayed table-first and coding-sequence-specific, the
      richer third-position composition remained in the analytical table, and
      the emitted contract stayed bounded to a single derived
      `wobble_variability` series.
    - No renderer-coupled pressure emerged, no generalized
      plotting-framework pressure emerged, and no fallback-activation stop
      condition tripped.
    - The governed release-truth surface remained fully green after `wobble`
      closed:
      - shipped methods: `101`
      - compared evidence: `101`
      - executable evidence: `0`
      - harvested legacy provenance present: `101`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`
    - Retrieval fallback remains documented and prepared, but inactive.

164. Complete.
    - Extended the roadmap from the actual post-`wobble` branch outcome rather
      than the unused fallback branch.
    - The current governed truth at this extension point remains:
      - shipped methods: `101`
      - compared evidence: `101`
      - executable evidence: `0`
      - harvested legacy provenance present: `101`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `gapped_method_count: 0`
      - `weakest_evidence_family: null`
      - `release_truth_current: true`
    - Bounded plotting continuation remains the active branch after `wobble`.
      The prepared retrieval fallback remains documented and ready, but inactive
      unless the narrowed plotting pool fails honest seam review.
    - The remaining narrowed plotting continuation pool is now:
      - `banana`
      - `isochore`
      - `syco`
    - The next mapped tier is now Tasks `165` through `180`, centered on:
      - inventorying and reclassifying the post-`wobble` plotting remainder
      - deciding whether another bounded plotting continuation candidate still
        exists
      - either activating the prepared retrieval fallback if no credible
        bounded candidate remains, or choosing exactly one next plotting method
      - capturing acceptance criteria, exact start conditions, and explicit stop
        conditions for the chosen method
      - mapping and shipping one full bounded method slice
      - rerunning release truth and reassessing the seam before any further
        continuation is mapped

165. Complete. Inventory the remaining plotting-family continuation pool after the shipped `wobble` slice.
    - Rebased the active continuation pool on the actual post-`wobble` state
      rather than the pre-`wobble` shortlist assumptions.
    - The plotting-family remainder pool now excludes the preexisting governed
      seam precedents plus the bounded shipped continuations:
      - seam precedents:
        - `charge`
        - `pepwindow`
      - bounded shipped plotting-family continuations:
        - `hmoment`
        - `octanol`
        - `pepinfo`
        - `density`
        - `wobble`
    - The remaining plotting-family continuation pool is therefore the
      remaining `21` methods:
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
    - This task is inventory only. It does not yet reclassify seam pressure or
      choose the next bounded continuation candidate.

166. Complete. Reclassify the post-`wobble` plotting remainder by seam pressure.
    - Reclassified the post-`wobble` remainder against the now-proven bounded
      seam rather than the earlier post-`density` shortlist.
    - The strongest remaining seam-compatible continuation candidates are now:
      - `banana`
      - `isochore`
      - `syco`
    - `cpgplot` remains plausible, but only as a more conditional
      region-oriented candidate.
    - The broader buckets remain outside the currently proven bounded seam:
      - requires broader contract taxonomy:
        - `chaos`
        - `pepwindowall`
        - `plotcon`
      - dotplot-style or comparative-matrix heavy:
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
    - This remains governance-only reclassification. It narrows the next
      candidate pool, but it does not yet choose the next bounded continuation
      candidate.

167. Complete. Decide explicitly whether another bounded plotting continuation candidate still exists after `wobble`.
    - Closed the post-`wobble` pass/fail gate explicitly.
    - The decision is affirmative: another bounded plotting continuation
      candidate still exists after the `wobble` shipment gate.
    - The current viable post-`wobble` continuation pool is:
      - `banana`
      - `isochore`
      - `syco`
    - So the no-candidate branch is not taken here.
    - Retrieval fallback remains documented, prepared, and inactive, but it is
      not activated at this checkpoint.
    - The repository remains on the plotting continuation path and can move to
      the next bounded selection step.

167. Decide explicitly whether another bounded plotting continuation candidate still exists after `wobble`.
    - Either keep the plotting branch active with a non-empty narrowed pool, or
      trigger the no-candidate branch honestly.

168. Complete. If no credible bounded continuation candidate remains after the post-`wobble` review, activate the prepared retrieval fallback path explicitly.
    - Closed this as an untriggered conditional branch.
    - The post-`wobble` viability gate stayed affirmative, so the narrowed pool
      did not fail honest seam review.
    - The repository therefore does not activate the prepared retrieval
      fallback at this checkpoint.
    - Retrieval fallback remains documented, prepared, and inactive while the
      repository stays on the plotting continuation branch.

169. Complete. If plotting remains viable, choose exactly one next bounded plotting continuation candidate.
    - Chose the single next bounded plotting continuation candidate from the
      narrowed remaining pool.
    - The selected candidate is:
      - `isochore`
    - Selection basis:
      - `isochore` looks like the narrowest remaining extension of the proven
        bounded nucleotide plotting seam as an analytical, table-first profile
        with a likely single derived continuation line
      - it appears less likely than `banana` to force a heavier biophysical
        model surface
      - it appears less likely than `syco` to force codon-usage-specific
        structure or coding-sequence-only seam pressure
    - The non-selected viable methods remain documented but inactive:
      - `banana`
      - `syco`
    - So the next bounded planning step is to capture `isochore`-specific
      acceptance criteria and exact patch start conditions before any code
      starts.

170. Complete. If plotting remains viable, close the untriggered retrieval-activation branch explicitly.
    - Closed the retrieval-activation branch explicitly after selecting
      `isochore`.
    - Retrieval fallback remains documented, prepared, and inactive.
    - The repository does not promote retrieval by inertia while the active
      bounded plotting continuation branch remains credible.

171. Complete. Capture explicit method-level acceptance criteria for the selected next plotting candidate.
    - Captured explicit `isochore` method-level acceptance criteria in the
      governance appendix before code starts.
    - Recorded:
      - bounded nucleotide analytical surface
      - single-series-unless-forced typed contract expectations
      - canonical fixture and compared-evidence obligations
      - explicit method-local non-goals and reassessment pressure

172. Complete. Capture the exact start conditions for the first implementation patch of the selected next plotting candidate.
    - Captured the explicit `isochore` start gate in the governance appendix.
    - Recorded:
      - current shortlist and active plotting-branch requirements
      - post-`wobble` continuation viability with `isochore` as the selected
        candidate
      - zero-burden release-truth requirements
      - patch-scope and full-governed-slice constraints before code starts

173. Complete. Map the full bounded implementation tier for the selected next plotting candidate.
    - Mapped the explicit bounded `isochore` implementation tier in the
      governance appendix.
    - Recorded the full method-associated slice:
      1. analytical core
      2. typed plot-contract emission path
      3. governed shipped-surface exposure
      4. canonical fixtures and compared evidence
      5. post-ship release-truth rerun
      6. post-ship reassessment

174. Complete. Capture the explicit seam-pressure stop conditions for the selected next plotting candidate.
    - Captured the explicit `isochore` seam-pressure stop conditions in the
      governance appendix.
    - Recorded the pre-code pause-and-reassess triggers needed to prevent
      informal widening beyond the bounded plotting seam.

175. Implement the bounded analytical core for the selected next plotting candidate.
    - Added the bounded `isochore` analytical core in `emboss-core`.
    - Kept the implementation table-first, method-associated, and renderer-
      agnostic with explicit GC/AT window rows and bounded isochore-band
      classification.

176. Add the typed plot-contract emission path for the selected next plotting candidate.
    - Added the staged typed `isochore` contract path over the bounded
      analytical core.
    - Kept the emitted contract method-associated, deterministic, and bounded
      to a single derived GC-percent line without widening into a generic
      plotting framework.

177. Expose the selected next plotting candidate through the governed shipped surface.
    - Wired `isochore` through the governed shipped surface.
    - Added registry, service, CLI, governed autodoc, and generated
      docs/validation support while keeping the state explicitly executable-only
      until compared evidence closes.

178. Add canonical analytical and plot-contract fixtures plus compared evidence for the selected next plotting candidate.
    - Added canonical analytical and plot-contract fixtures plus compared
      evidence for `isochore`.
    - Restored the shipped cohort from the temporary executable-only shipment
      state back to full compared/full-harvest truth.

179. Re-run the full release-truth surface after shipping the selected next plotting candidate.
    - Reconfirmed `make release-generated-check`, `truth-check`, generated
      docs, and zero-burden summary semantics after the `isochore` slice
      closed.
    - Observed result:
      - `release-generated-check`: passed
      - `truth-check`: passed
      - docs build: passed
      - `git diff --check`: passed

180. Reassess the shipped selected next plotting slice before any further continuation is mapped.
    - Recorded the explicit post-ship `isochore` reassessment in the
      governance appendix.
    - Observed result:
      - the shipped `isochore` slice stayed bounded, method-associated, and
        table-first
      - the emitted contract stayed bounded to a single derived GC-percent
        series
      - no fallback-activation stop condition tripped
      - the release-truth surface remained fully green
    - Conclusion:
      - bounded plotting still remains credible after `isochore`
      - the prepared retrieval fallback remains documented and ready, but
        inactive at this checkpoint

181. Complete. Extend the roadmap from the actual post-`isochore` branch outcome.
    - Recorded the current governed truth explicitly:
      - shipped methods: `102`
      - compared evidence: `102`
      - executable evidence: `0`
      - harvested legacy provenance present: `102`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `gapped_method_count: 0`
      - `weakest_evidence_family: null`
      - `release_truth_current: true`
    - Recorded the actual post-`isochore` planning consequence:
      - bounded plotting still remains credible
      - the prepared retrieval fallback remains ready but inactive
      - the narrowed active plotting continuation pool is now:
        - `banana`
        - `syco`
    - Added the next mapped tier as Tasks `182` through `195`, centered on:
      - inventorying and reclassifying the post-`isochore` plotting remainder
      - deciding whether another bounded plotting continuation candidate still
        exists
      - either activating the prepared retrieval fallback if the narrowed pool
        fails honest seam review, or choosing exactly one next candidate from:
        - `banana`
        - `syco`
      - capturing acceptance criteria, exact start conditions, and explicit
        stop conditions for the chosen method
      - mapping and shipping one full bounded method slice
      - rerunning release truth and reassessing the seam before any further
        continuation is mapped

182. Inventory the post-`isochore` plotting-family remainder against the current active bounded seam.
    - Rebased the plotting-family remainder onto the actual post-`isochore`
      state rather than any earlier pre-`isochore` shortlist.
    - Recorded the seam exclusions explicitly:
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
    - Recorded the actual post-`isochore` plotting-family remainder pool as
      the remaining `20` methods:
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

183. Reclassify the post-`isochore` plotting remainder by seam pressure.
    - Reclassified the post-`isochore` remainder against the now-proven seam.
    - Recorded the strongest remaining seam-compatible continuation candidates:
      - `banana`
      - `syco`
    - Recorded `cpgplot` as the only still-plausible but more conditional
      region-oriented candidate.
    - Recorded the heavier out-of-seam buckets explicitly:
      - broader contract-taxonomy:
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
      - specialized laboratory-trace or kinetic:
        - `abiview`
        - `findkm`

184. Decide whether another bounded plotting continuation candidate still exists after `isochore`.
    - Closed the pass/fail gate explicitly after the post-`isochore`
      seam-pressure reclassification.
    - Decision:
      - yes, another bounded plotting continuation candidate still exists
        after `isochore`
    - Recorded the current viable post-`isochore` continuation pool:
      - `banana`
      - `syco`
    - Conclusion:
      - the no-candidate branch is not taken
      - the repository remains on the plotting continuation path
      - the prepared retrieval fallback remains documented, ready, and
        inactive at this checkpoint

185. If the post-`isochore` plotting pool fails honest seam review, activate the prepared retrieval fallback branch explicitly.
    - Closed this conditional branch honestly as untriggered.
    - Basis:
      - the post-`isochore` viability gate remained affirmative
      - the narrowed plotting continuation pool is still:
        - `banana`
        - `syco`
    - Conclusion:
      - the repository does not activate the prepared retrieval fallback at
        this checkpoint
      - retrieval remains documented, ready, and inactive while the active
        branch stays on bounded plotting continuation

186. If the post-`isochore` plotting pool remains viable, choose exactly one next bounded continuation candidate.
    - Restricted the active selection to the narrowed viable pool:
      - `banana`
      - `syco`
    - Selected:
      - `banana`
    - Selection basis:
      - `banana` looks like the closest remaining extension of the proven
        bounded nucleotide plotting seam as an analytical, table-first profile
        with a likely single derived continuation line
      - it appears less likely than `syco` to force codon-usage-specific
        structure or coding-sequence-only seam pressure
      - it still appears compatible with the established renderer-agnostic
        typed contract seam
    - Kept the non-selected viable method documented but inactive:
      - `syco`

187. Close the untriggered branch explicitly after the post-`isochore` candidate decision.
    - Closed the opposite branch explicitly after selecting `banana`.
    - Recorded:
      - the plotting-continuation branch remains active
      - retrieval fallback remains documented, prepared, and inactive
      - the non-selected viable method `syco` remains documented but inactive
    - Conclusion:
      - no retrieval implementation tier or retrieval lead-method choice is
        mapped at this checkpoint
      - the next bounded planning step remains `banana`-specific method
        acceptance criteria and patch start conditions

188. Capture explicit method-level acceptance criteria for the selected next bounded continuation candidate.
    - Captured explicit `banana` method-level acceptance criteria in the
      governance appendix.
    - Recorded:
      - bounded nucleotide analytical surface
      - table-first output expectations with enough explicit columns to
        reconstruct the plotted continuation line
      - typed contract expectations, staying single-series unless the method
        itself proves otherwise
      - canonical fixture and compared-evidence obligations
      - explicit method-local non-goals preserving renderer ownership in
        `emboss-r` and blocking generic plotting-framework widening

189. Capture the exact start conditions for the first implementation patch of the selected next bounded continuation candidate.
    - Captured the explicit `banana` patch-start gate in the governance
      appendix.
    - Recorded:
      - the current shortlist and active plotting-branch requirements
      - post-`isochore` continuation viability with `banana` as the selected
        candidate
      - zero-burden release-truth requirements
      - patch-scope and full-governed-slice constraints before code starts

190. Map the full bounded implementation tier for the selected next bounded continuation candidate.
    - Mapped the explicit bounded `banana` implementation tier in the
      governance appendix.
    - Recorded the full method-associated slice:
      1. analytical core
      2. typed plot-contract emission path
      3. governed shipped-surface exposure
      4. canonical fixtures and compared evidence
      5. post-ship release-truth rerun
      6. post-ship reassessment

191. Capture the explicit seam-pressure stop conditions for the selected next bounded continuation candidate.
    - Captured the explicit seam-pressure stop conditions for `banana` in the
      governance appendix.
    - Recorded the pre-code pause-and-reassess triggers:
      1. failure to remain table-first and same-path contract-derived
      2. Rust-side rendering or presentation-policy pressure
      3. generalized plotting-framework pressure
      4. curvature-track, region-call, threshold-call, or broader
         plot-contract taxonomy pressure not clearly local to the method

192. Implement the bounded analytical core for the selected next bounded continuation candidate.
    - Implemented the bounded analytical core for `banana` in
      `crates/emboss-core`.
    - Recorded the bounded core shape:
      - per-base analytical rows
      - local bend and curvature columns
      - edge-aware undefined positions where the historical model does not
        yield a defined value
      - canonical DNA-like residue validation with `U` treated as `T`
    - Kept the implementation method-associated, table-first, and renderer-
      agnostic.

193. Add the typed plot-contract emission path for the selected next bounded continuation candidate.
    - Added the staged typed `banana` plot-contract path in
      `crates/emboss-tools/src/nucleotide_plots`.
    - Kept the emitted contract method-associated and deterministic:
      - tool: `banana`
      - method: `nucleotide_banana_profile`
      - source artifact: `table:banana-profile`
    - Kept the staged surface bounded to the smallest honest plotting shape:
      - the analytical table retains both local bend and curvature columns
      - the emitted plot contract is a single curvature continuation line
        over positions where curvature is defined

194. Expose the selected next bounded continuation candidate through the governed shipped surface.
    - Completed the governed shipped-surface exposure for `banana`.
    - Added registry, service, CLI, governed autodoc, and generated
      docs/validation support while keeping the temporary evidence state
      explicit until compared evidence closes.
    - The current interim state is now:
      - shipped methods: `103`
      - compared evidence: `102`
      - executable evidence: `1`
      - harvested legacy provenance present: `103`
      - `full_compared_cohort: false`
      - `harvest_coverage_complete: true`

195. Add canonical analytical and plot-contract fixtures plus compared evidence for the selected next bounded continuation candidate.
    - Closed the `banana` evidence slice by adding the canonical analytical
      fixture and canonical plot-contract fixture.
    - Wired `banana` into the acceptance-anchor harness and regenerated the
      governed validation/report surface from the temporary executable-only
      state back to full compared/full-harvest truth.
    - The current post-closure state is now:
      - shipped methods: `103`
      - compared evidence: `103`
      - executable evidence: `0`
      - harvested legacy provenance present: `103`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `release_truth_current: true`

196. Re-run the full release-truth surface after shipping the selected next bounded continuation candidate.
    - Used this as the post-`banana` release-truth checkpoint and recorded the
      observed clean state.
    - Re-ran the currently decisive release-truth gates for the shipped
      repository surface:
      - `python3 scripts/release_metadata.py truth-check`: passed
      - docs build: passed
      - `git diff --check`: passed
    - The governed state remained fully green:
      - shipped methods: `103`
      - compared evidence: `103`
      - executable evidence: `0`
      - harvested legacy provenance present: `103`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

197. Reassess the shipped slice for the selected next bounded continuation candidate before any further continuation is mapped.
    - Recorded the post-ship `banana` reassessment explicitly in the
      governance appendix and roadmap.
    - The result is affirmative:
      - the shipped `banana` slice stayed bounded
      - the slice remained method-associated and table-first
      - the richer bendability surface remained in the analytical table
      - the emitted contract stayed bounded to a single derived curvature
        continuation line
      - no fallback-activation stop condition tripped
    - The governed release-truth surface remained fully green:
      - shipped methods: `103`
      - compared evidence: `103`
      - executable evidence: `0`
      - harvested legacy provenance present: `103`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

198. Complete. Extend the roadmap from the actual post-`banana` branch outcome.
    - Recorded the current governed truth explicitly:
      - shipped methods: `103`
      - compared evidence: `103`
      - executable evidence: `0`
      - harvested legacy provenance present: `103`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `gapped_method_count: 0`
      - `weakest_evidence_family: null`
      - `release_truth_current: true`
    - Recorded the actual post-`banana` planning consequence:
      - bounded plotting still remains credible
      - the prepared retrieval fallback remains ready but inactive
      - the narrowed active plotting continuation pool is now:
        - `syco`
    - Added the next mapped tier as Tasks `199` through `212`, centered on:
      - rebasing the plotting-family remainder onto the actual post-`banana`
        state
      - deciding whether the last narrowed plotting candidate still passes an
        honest seam review
      - either activating the prepared retrieval fallback if the remaining
        plotting continuation pool fails that review, or selecting `syco` as
        the final bounded plotting continuation candidate
      - capturing method-level acceptance criteria, exact patch start
        conditions, and explicit stop conditions for `syco` if that branch
        remains active
      - mapping and shipping one full bounded `syco` slice through analytical
        core, typed contract path, governed shipped exposure, and compared
        evidence closure
      - rerunning release truth and reassessing the seam again before any
        further continuation is mapped

199. Rebase the plotting-family remainder onto the actual post-`banana` state.
    - Rebased the plotting-family remainder inventory onto the actual
      post-`banana` shipped state rather than the older post-`isochore`
      shortlist.
    - The plotting-family remainder now excludes the governed seam precedents
      plus every bounded shipped plotting continuation:
      - seam precedents:
        - `charge`
        - `pepwindow`
      - bounded shipped plotting-family continuations:
        - `hmoment`
        - `octanol`
        - `pepinfo`
        - `density`
        - `wobble`
        - `isochore`
        - `banana`
    - The remaining continuation pool is therefore the remaining `19`
      plotting-family methods:
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
    - This task is inventory only. It does not yet reclassify seam pressure or
      decide whether the final narrowed continuation candidate still passes an
      honest seam review.

200. Reclassify the post-`banana` plotting remainder by seam pressure.
    - Reclassified the post-`banana` plotting remainder by seam pressure in
      light of the actual shipped plotting seam rather than the earlier
      post-`isochore` shortlist.
    - The strongest remaining seam-compatible continuation candidate is now:
      - `syco`
    - `cpgplot` remains the only plausibly arguable adjacent case, but only as
      a more conditional region-oriented candidate rather than as part of the
      current narrowed active continuation pool.
    - The heavier buckets remain outside the currently proven bounded seam:
      - broader contract taxonomy:
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
    - This remains governance-only reclassification. It narrows the final
      continuation candidate pool, but it does not yet decide whether `syco`
      still passes an honest seam review strongly enough to remain active.

201. Decide whether the last narrowed plotting candidate still passes an honest seam review.
    - Closed the post-`banana` pass/fail gate explicitly.
    - The decision is affirmative: a final bounded plotting continuation
      candidate still remains credible after `banana`.
    - The current viable final continuation candidate is:
      - `syco`
    - So the no-candidate branch is not taken here.
    - The prepared retrieval fallback remains documented, ready, and inactive.
    - The repository therefore stays on the plotting continuation branch for
      the next bounded selection step, which is now simply whether to activate
      `syco` as the final active candidate.

202. Close the untriggered fallback branch after the final-candidate viability decision.
    - Closed the opposite branch explicitly after the affirmative final-
      candidate viability decision.
    - The recorded outcome is:
      - the plotting-continuation branch remains active
      - the prepared retrieval fallback remains documented, ready, and
        inactive
      - retrieval is not promoted by inertia while `syco` remains the final
        bounded active continuation candidate
    - So no retrieval implementation tier or retrieval lead-method choice is
      mapped at this checkpoint.

203. Activate the final bounded plotting continuation candidate.
    - Activated `syco` as the single final bounded plotting continuation
      candidate.
    - Recorded the basis explicitly:
      - `syco` is now the only remaining seam-compatible continuation
        candidate in the narrowed post-`banana` pool
      - the prepared retrieval fallback remains documented, ready, and
        inactive because the no-candidate branch was not taken
      - no other plotting-family method remains comparably bounded without
        heavier contract, presentation, or comparative-matrix pressure
    - The next bounded planning step is therefore `syco`-specific:
      - method-level acceptance criteria
      - exact patch start conditions
      - explicit seam stop conditions

204. Capture explicit method-level acceptance criteria for the final bounded plotting continuation candidate.
    - Captured explicit `syco` method-level acceptance criteria in the
      governance appendix and roadmap.
    - The recorded governed criteria make `syco` concrete before code starts:
      - bounded coding nucleotide analytical surface
      - stable table-first output with enough explicit columns to reconstruct
        the plotted continuation line from the same computation path
      - typed contract expectations that remain single-series unless the
        analytical needs of `syco` itself prove a broader local structure is
        unavoidable
      - canonical analytical and plot-contract fixtures
      - compared evidence for both table and contract outputs
    - The same non-goals remain explicit:
      - no Rust-side figure rendering
      - no silent widening into a generic plotting framework
      - no broader contract taxonomy unless `syco` itself forces a real local
        reassessment

205. Capture the exact start conditions for the first implementation patch of the final bounded plotting continuation candidate.
    - Captured the explicit `syco` patch-start gate in the governance appendix
      and roadmap.
    - Recorded:
      - the current shortlist and active plotting-branch requirements
      - post-`banana` continuation viability with `syco` as the selected final
        active candidate
      - the zero-burden release-truth surface requirement
      - patch-scope and full-governed-slice constraints before code starts

206. Map the full bounded implementation tier for the final bounded plotting continuation candidate.
    - Mapped the full bounded `syco` implementation tier in the governance
      appendix and roadmap.
    - Recorded the complete method-associated slice before code starts:
      1. analytical core
      2. typed plot-contract emission path
      3. governed shipped-surface exposure
      4. canonical fixtures and compared evidence
      5. post-ship release-truth rerun
      6. post-ship reassessment

207. Capture the explicit seam-pressure stop conditions for the final bounded plotting continuation candidate.
    - Captured the explicit `syco` seam-pressure stop conditions in the
      governance appendix and roadmap.
    - Recorded the pause-and-reassess triggers before code starts so the final
      bounded plotting seam is not stretched informally.

208. Implement the bounded analytical core for the final bounded plotting continuation candidate.
    - Added the bounded `syco` analytical core in `emboss-core` and exported
      it through the crate root.
    - Landed a method-associated coding-sequence profile over codon windows
      using a bounded reference-profile synonymous preference score.
    - Kept the implementation table-first, coding-sequence-specific, and
      renderer-agnostic.

209. Add the typed plot-contract emission path for the final bounded plotting continuation candidate.
    - Added the staged typed `syco` plot-contract path in `emboss-tools`,
      wired it through the nucleotide plotting module, and added a focused
      coding-sequence fixture.
    - Landed a deterministic single-series `syco_score` line contract derived
      from the same analytical computation path without widening into a generic
      plotting framework.

210. Expose the final bounded plotting continuation candidate through the governed shipped surface.
    - Wired `syco` through the registry, service, CLI, governed autodoc, and
      generated docs/validation stub.
    - Refreshed the governed cohort/report surfaces to the honest interim
      state:
      - shipped methods: `104`
      - compared evidence: `103`
      - executable evidence: `1`
      - harvested legacy provenance present: `104`
      - `full_compared_cohort: false`
      - `release_truth_current: true`

211. Add canonical analytical and plot-contract fixtures plus compared evidence for the final bounded plotting continuation candidate.
    - Added the canonical analytical fixture and canonical plot-contract
      fixture for `syco`.
    - Wired `syco` into the acceptance-anchor cohort and regenerated the
      governed report surface back to the full-compared release-truth state:
      - shipped methods: `104`
      - compared evidence: `104`
      - executable evidence: `0`
      - harvested legacy provenance present: `104`
      - `full_compared_cohort: true`
      - `release_truth_current: true`

212. Re-run the full release-truth surface after shipping the final bounded plotting continuation candidate.
    - Re-ran the post-`syco` release-truth surface and recorded the observed
      result.
    - The broad release gate stayed clean after the `syco` evidence closure:
      - `truth-check`: passed
      - docs build: passed
      - `git diff --check`: passed
    - The governed state remained fully green:
      - shipped methods: `104`
      - compared evidence: `104`
      - executable evidence: `0`
      - harvested legacy provenance present: `104`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

213. Reassess the shipped final bounded plotting continuation candidate before any further continuation is mapped.
    - Recorded the post-ship `syco` reassessment explicitly in the governance
      appendix and roadmap.
    - The result is affirmative:
      - the shipped `syco` slice stayed bounded and method-associated
      - the analytical surface stayed coding-sequence-specific and table-first
      - the richer codon-window scoring surface remained in the analytical
        table
      - the emitted contract stayed bounded to a single derived
        `syco_score` series
      - no fallback-activation stop condition tripped
    - The governed release-truth surface remained fully green:
      - shipped methods: `104`
      - compared evidence: `104`
      - executable evidence: `0`
      - harvested legacy provenance present: `104`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

214. Extend the roadmap from the actual post-`syco` branch outcome rather than from the earlier plotting-first assumptions.
    - Complete.
    - The roadmap now records the current governed truth explicitly:
      - shipped methods: `104`
      - compared evidence: `104`
      - executable evidence: `0`
      - harvested legacy provenance present: `104`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `gapped_method_count: 0`
      - `weakest_evidence_family: null`
      - `release_truth_current: true`
    - The extension also makes the planning consequence explicit:
      - the bounded plotting continuation program has now closed through its
        final selected candidate
      - no comparably bounded plotting continuation candidate remains active
      - the prepared remote-retrieval fallback becomes the next planning
        program by explicit branch resolution rather than by inertia
    - The next mapped tier is now Tasks `215` through `228`, centered on:
      - confirming that the plotting continuation family is now exhausted as a
        bounded active branch
      - activating the prepared remote-retrieval fallback as the next active
        planning program
      - reconfirming that protein-property rework still remains the third
        candidate after the bounded plotting program closes
      - choosing exactly one bounded retrieval lead method from:
        - `seqretsetall`
        - `seqretsplit`
        - `infoassembly`
      - capturing acceptance criteria, exact patch start conditions, and
        explicit provider/seam stop conditions for the chosen retrieval method
      - mapping and shipping one full bounded retrieval slice
      - rerunning release truth and reassessing the shipped retrieval slice
        before any further continuation is mapped

215. Confirm explicitly that the bounded plotting continuation family is now exhausted as an active branch.
    - Complete.
    - Recorded the branch-resolution outcome explicitly in the governance
      appendix and roadmap.
    - The conclusion is:
      - the bounded plotting continuation program has now closed through its
        final selected candidate
      - no comparably bounded plotting continuation candidate remains active
      - continued plotting should no longer be treated as the default next
        planning branch
      - the prepared remote-retrieval fallback should now become the active
        next planning program by explicit branch resolution
    - Basis:
      - bounded plotting Phase 1 closed and passed reassessment:
        - `hmoment`
        - `octanol`
        - `pepinfo`
      - bounded continuation slices also closed and passed reassessment:
        - `density`
        - `wobble`
        - `isochore`
        - `banana`
        - `syco`
      - the governed release-truth surface remained fully green:
        - shipped methods: `104`
        - compared evidence: `104`
        - executable evidence: `0`
        - harvested legacy provenance present: `104`
        - `full_compared_cohort: true`
        - `harvest_coverage_complete: true`
        - `retained_backlog_closed: true`
        - `release_truth_current: true`

216. Activate the prepared remote-retrieval fallback as the next active planning program.
    - Complete.
    - Recorded the explicit activation in the governance appendix and roadmap.
    - The result is:
      - plotting is no longer treated as an active continuation branch
      - remote retrieval now becomes the next active planning program by
        explicit branch resolution
      - protein-property rework remains third and is not promoted by this
        step
    - The active bounded retrieval planning subset remains:
      - `seqretsetall`
      - `seqretsplit`
      - `infoassembly`
    - This activation does not widen retrieval scope beyond the already
      governed fallback posture:
      - no broad provider-parity claims
      - no implicit live-network validation
      - no automatic promotion of broader historical retrieval members such
        as `assemblyget`, `whichdb`, or `entret`

217. Reconfirm that protein-property rework still remains the third candidate after retrieval activation.
    - Complete.
    - Recorded the shortlist reassessment explicitly in the governance
      appendix and roadmap.
    - The conclusion remains:
      1. completed bounded plotting continuation program
      2. active remote-retrieval planning program
      3. protein-property rework
    - Basis:
      - retrieval now has the strongest active bounded plan after plotting
        closure:
        - `seqretsetall`
        - `seqretsplit`
        - `infoassembly`
      - retrieval preserves a clearer operational seam at this checkpoint:
        - explicit provider-aware orchestration
        - deterministic mocked-provider or managed-asset validation
        - compared evidence on normalized returned sequence or metadata
          outputs
      - protein-property rework still has a credible scientific substrate,
        but it still lacks the same immediate bounded lead-method activation
        detail now attached to the active retrieval branch

218. Choose exactly one bounded retrieval lead method from `seqretsetall`, `seqretsplit`, and `infoassembly`.
    - Complete.
    - Recorded the retrieval lead-method decision explicitly in the
      governance appendix and roadmap.
    - The selected lead method is:
      - `seqretsetall`
    - Selection basis:
      - it is the narrowest extension of the already-shipped governed
        retrieval slice because it stays closest to the existing normalized
        sequence-return path in `seqret`
      - it appears easier to keep deterministic under mocked-provider or
        managed-asset validation than a split-output filesystem-oriented lead
        path
      - it avoids starting retrieval Phase 1 with the broader assembly
        metadata shape and provider-surface questions that would come with
        `infoassembly`
    - The non-selected bounded retrieval candidates remain documented but
      inactive:
      - `seqretsplit`
      - `infoassembly`

219. Capture explicit method-level acceptance criteria for `seqretsetall`.
    - Complete.
    - Recorded the governed method-level acceptance criteria explicitly in the
      governance appendix and roadmap.
    - The criteria now make `seqretsetall` concrete before code starts:
      - bounded provider-aware many-set retrieval/write workflow only
      - deterministic normalized sequence-return output built on the same
        governed retrieval substrate as `seqret`
      - stable output surface with explicit source and grouping behavior
        defined by the same computation path
      - canonical managed-asset or mocked-provider fixtures
      - compared evidence required on normalized returned sequence sets, not
        just orchestration intent
    - The non-goals remain explicit:
      - no hidden live-network validation
      - no broad provider-parity claims
      - no generic retrieval-family widening merely because `seqretsetall`
        ships

220. Capture the exact start conditions for the first `seqretsetall` implementation patch.
    - Complete.
    - Recorded the exact patch start conditions explicitly in the governance
      appendix and roadmap.
    - The start gate now requires:
      - the current shortlist to remain intact:
        1. completed bounded plotting continuation program
        2. active remote-retrieval planning program
        3. protein-property rework
      - the bounded retrieval planning subset to remain limited to:
        - `seqretsetall`
        - `seqretsplit`
        - `infoassembly`
      - `seqretsetall` to remain the single selected bounded retrieval lead
        method
      - the zero-burden release-truth surface to remain intact
      - the first patch to stay limited to `seqretsetall` plus the smallest
        support needed for deterministic provider-aware orchestration,
        normalized sequence-return output, and governed docs/validation
        plumbing
      - the patch to land as a full governed slice rather than a half-start
    - The same guardrails remain explicit:
      - no hidden live-network dependencies
      - no broad provider-parity claims
      - no retrieval-family widening beyond the selected lead-method slice

221. Map the full bounded `seqretsetall` implementation tier.
    - Complete.
    - Recorded the full bounded implementation tier explicitly in the
      governance appendix and roadmap.
    - The bounded `seqretsetall` tier is now explicit:
      1. implement the bounded provider-aware orchestration and normalized
         many-set return core
      2. expose the governed output surface for deterministic many-set
         retrieval/write behavior
      3. expose `seqretsetall` through the governed shipped surface
      4. add canonical managed-asset or mocked-provider fixtures plus compared
         evidence
      5. re-run the full release-truth surface after shipping `seqretsetall`
      6. reassess the shipped `seqretsetall` slice before any further
         retrieval continuation is mapped

222. Capture the explicit provider/seam stop conditions for `seqretsetall`.
    - Complete.
    - Recorded the explicit provider/seam stop conditions in the governance
      appendix and roadmap.
    - The repository should now pause and reopen planning before code starts
      if:
      1. `seqretsetall` cannot remain deterministic under mocked-provider or
         managed-asset validation
      2. `seqretsetall` requires hidden live-network dependencies, implicit
         provider fallback chains, or unclear provider precedence
      3. `seqretsetall` cannot remain a bounded extension of the normalized
         `seqret` return path and instead demands broader retrieval-family
         orchestration before one shipped slice closes
      4. `seqretsetall` forces broad filesystem-policy, batching-policy, or
         provider-parity claims that are not clearly local to the method

223. Implement the bounded provider-aware orchestration and normalized many-set return core for `seqretsetall`.
    - Complete.
    - Added the bounded `seqretsetall` execution core in
      `crates/emboss-tools/src/retrieval_tools/seqretsetall.rs` and exported
      it through `crates/emboss-tools/src/retrieval_tools/mod.rs`.
    - Added the private multi-input service seam in
      `crates/emboss-service/src/service.rs` so the bounded retrieval lead can
      resolve ordered local and provider-backed inputs through the same
      governed substrate already used by `seqret`.
    - What landed in the core:
      - ordered resolved input-set handling over `SeqretSource`
      - deterministic partition-preserving normalized record sets
      - total-record accounting across all input sets
      - bounded validation for:
        - too few input sets
        - empty resolved input sets
      - focused service-seam tests for:
        - two ordered local inputs
        - mixed local plus provider-qualified input through a mocked client
    - This task intentionally stopped at the bounded core:
      - no governed result-surface exposure yet
      - no registry or CLI exposure yet
      - no docs/autodoc/generated validation yet
      - no canonical fixtures or compared evidence yet

224. Expose the governed output surface for deterministic many-set retrieval/write behavior.
    - Complete.
    - Added the front-end-neutral `seqretsetall` result surface in
      `crates/emboss-service/src/service.rs` and the bounded descriptor entry
      in `crates/emboss-tools/src/retrieval_tools/mod.rs`.
    - What landed in the governed surface:
      - a bounded `invoke_seqretsetall_with_client(...)` service path
      - `MethodResult`/`InvocationResponse` emission through
        `ResultPayload::SequencePartitions`
      - deterministic summary lines and output artifact metadata for
        partitioned normalized FASTA output
      - focused service tests for:
        - local ordered partitioned payloads
        - mixed local and mocked-provider partitioned payloads
    - This task intentionally stopped short of shipment:
      - no registry exposure yet
      - no CLI routing yet
      - no generated docs or validation stubs yet
      - no canonical fixtures or compared evidence yet

225. Expose `seqretsetall` through the governed shipped surface.
    - Complete.
    - `seqretsetall` is now exposed through the governed registry, service,
      CLI, and autodoc surface.
    - What landed:
      - registry exposure through the governed tool-descriptor list
      - shipped service dispatch through `Service::invoke(...)`
      - CLI routing coverage for `emboss-rs seqretsetall`
      - governed autodoc, generated docs, and generated validation stub for
        the bounded many-set retrieval surface
    - Current governed state at this task boundary is intentionally interim:
      - shipped methods: `105`
      - compared evidence: `104`
      - executable evidence: `1`
      - harvested legacy provenance present: `105`
      - `full_compared_cohort: false`
      - `release_truth_current: true`
    - This is the expected executable-only state before Task `226` adds
      canonical fixtures and compared evidence for `seqretsetall`.

226. Complete. Add canonical fixtures and compared evidence for `seqretsetall`.
    The `seqretsetall` slice now carries a checked-in partitioned-output
    acceptance fixture and acceptance-anchor coverage for the bounded local
    many-set retrieval path. The governed report surface is back to the
    zero-burden state: shipped methods `105`, compared evidence `105`,
    executable evidence `0`, harvested legacy provenance present `105`,
    full-compared cohort `true`, harvest coverage complete `true`, and
    release-truth current `true`.

227. Complete. Re-run the full release-truth surface after shipping
    `seqretsetall`.
    The broad post-ship gate stayed clean after the `seqretsetall` evidence
    closure. `make release-generated-check` passed, `python3
    scripts/release_metadata.py truth-check` passed, the docs build passed,
    and `git diff --check` passed. The governed release-truth surface remained
    fully green: shipped methods `105`, compared evidence `105`, executable
    evidence `0`, harvested legacy provenance present `105`,
    full-compared cohort `true`, harvest coverage complete `true`,
    retained backlog closed `true`, and release-truth current `true`.

228. Complete. Reassess the shipped `seqretsetall` slice before any further
    retrieval continuation is mapped.
    The shipped `seqretsetall` slice stayed bounded, provider-aware, and
    deterministic. It remained a local extension of the normalized `seqret`
    return path rather than forcing broader retrieval-family orchestration,
    hidden live-network behavior, or non-local batching policy. The governed
    release-truth surface remained fully green: shipped methods `105`,
    compared evidence `105`, executable evidence `0`, harvested legacy
    provenance present `105`, full-compared cohort `true`, harvest coverage
    complete `true`, retained backlog closed `true`, and release-truth current
    `true`.

229. Complete. Extend the roadmap from the actual post-`seqretsetall` branch
    outcome.
    The roadmap now records the current governed truth explicitly:
      - shipped methods: `105`
      - compared evidence: `105`
      - executable evidence: `0`
      - harvested legacy provenance present: `105`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `gapped_method_count: 0`
      - `weakest_evidence_family: null`
      - `release_truth_current: true`
    - The extension also makes the branch consequence explicit:
      - the first bounded retrieval continuation slice closed cleanly through
        `seqretsetall`
      - retrieval remains the active planning program, but it now has to be
        re-based onto the remaining bounded candidate pool instead of assuming
        automatic continuation
      - protein-property rework remains the reserve third program and should
        not be promoted unless the narrowed retrieval continuation branch fails
        honest seam review
    - The next mapped tier is now Tasks `230` through `243`, centered on:
      - rebasing the remaining bounded retrieval continuation pool onto the
        actual post-`seqretsetall` state
      - deciding whether another bounded retrieval continuation candidate still
        exists after the first shipped slice
      - either promoting protein-property rework if the narrowed retrieval
        continuation branch fails honest seam review, or selecting exactly one
        next bounded retrieval candidate from:
        - `seqretsplit`
        - `infoassembly`
      - capturing acceptance criteria, exact patch start conditions, and
        explicit provider/seam stop conditions for the chosen retrieval method
      - mapping and shipping one full bounded retrieval slice
      - rerunning release truth and reassessing the shipped retrieval slice
        before any further continuation is mapped

230. Complete. Rebase the bounded retrieval continuation pool onto the actual
    post-`seqretsetall` state.
    - Recorded the narrowed retrieval continuation pool explicitly in the
      governance appendix and roadmap.
    - The first bounded retrieval continuation slice is now treated as closed:
      - shipped bounded retrieval slice:
        - `seqretsetall`
    - The remaining bounded retrieval continuation pool is now:
      - `seqretsplit`
      - `infoassembly`
    - This checkpoint is inventory only.
    - It does not yet decide whether both remaining methods still pass honest
      seam review strongly enough to remain active continuation candidates.

231. Complete. Decide whether another bounded retrieval continuation candidate
    still exists after the first shipped slice.
    - Recorded the post-`seqretsetall` retrieval viability decision explicitly
      in the governance appendix and roadmap.
    - The decision is affirmative:
      - another bounded retrieval continuation candidate still exists after
        `seqretsetall`
      - the no-candidate branch is not taken here
      - retrieval therefore remains the active planning program
    - The current viable bounded continuation pool remains:
      - `seqretsplit`
      - `infoassembly`
    - Protein-property rework remains the reserve third program and should not
      be promoted at this checkpoint.

232. Complete. Close the untriggered protein-property promotion branch after
    the affirmative retrieval viability decision.
    - Recorded the explicit branch closeout in the governance appendix and
      roadmap.
    - The resulting branch consequence is:
      - retrieval continuation remains the active planning program
      - protein-property rework remains documented as the reserve third program
      - protein-property is not promoted by inertia while bounded retrieval
        continuation still has viable candidates
    - The active viable bounded continuation pool remains:
      - `seqretsplit`
      - `infoassembly`

233. Complete. Choose exactly one next bounded retrieval continuation
    candidate from the narrowed viable pool.
    - Recorded the retrieval lead-method decision explicitly in the governance
      appendix and roadmap.
    - The selected bounded continuation candidate is:
      - `seqretsplit`
    - Selection basis:
      - it is the narrowest remaining extension of the already-shipped
        governed retrieval slice because it stays closest to normalized
        sequence-return behavior while only adding deterministic split-output
        partitioning
      - it appears easier to keep deterministic under mocked-provider or
        managed-asset validation than `infoassembly`
      - it avoids starting the next retrieval slice with the broader assembly
        metadata shape and provider-surface questions that `infoassembly`
        would introduce
    - The non-selected bounded continuation candidate remains documented but
      inactive:
      - `infoassembly`

234. Complete. Capture explicit method-level acceptance criteria for
    `seqretsplit`.
    - Recorded the governed method-level acceptance criteria explicitly in the
      governance appendix and roadmap.
    - The criteria now make `seqretsplit` concrete before code starts:
      - bounded provider-aware split-output retrieval workflow only
      - deterministic normalized sequence-return output built on the same
        governed retrieval substrate as `seqret` and `seqretsetall`
      - stable partitioned output surface with explicit file-naming and
        grouping behavior defined by the same computation path
      - canonical managed-asset or mocked-provider fixtures
      - compared evidence required on normalized split-output sequence sets,
        not just orchestration intent
    - The non-goals remain explicit:
      - no hidden live-network validation
      - no broad provider-parity claims
      - no implicit promotion of `infoassembly`
      - no generic retrieval-family widening merely because `seqretsplit`
        ships

235. Complete. Capture the exact start conditions for the first
    `seqretsplit` implementation patch.
    - Recorded the exact patch start conditions explicitly in the governance
      appendix and roadmap.
    - The start gate now requires:
      - the current shortlist to remain intact:
        1. completed bounded plotting continuation program
        2. active remote-retrieval planning program
        3. protein-property rework
      - the bounded retrieval continuation pool to remain limited to:
        - `seqretsplit`
        - `infoassembly`
      - `seqretsplit` to remain the single selected bounded retrieval
        continuation lead method
      - the zero-burden release-truth surface to remain intact
      - the first patch to stay limited to `seqretsplit` plus the smallest
        support needed for deterministic provider-aware split-output
        orchestration, normalized sequence-return output, and governed
        docs/validation plumbing
      - the patch to land as a full governed slice rather than a half-start
    - The same guardrails remain explicit:
      - no hidden live-network dependencies
      - no broad provider-parity claims
      - no implicit widening into `infoassembly`
      - no retrieval-family widening beyond the selected lead-method slice

236. Complete. Map the full bounded `seqretsplit` implementation tier.
    - Recorded the full bounded implementation tier explicitly in the
      governance appendix and roadmap.
    - The bounded `seqretsplit` tier is now explicit:
      1. implement the bounded provider-aware split-output orchestration and
         normalized return core
      2. expose the governed output surface for deterministic split-output
         retrieval behavior
      3. expose `seqretsplit` through the governed shipped surface
      4. add canonical managed-asset or mocked-provider fixtures plus compared
         evidence
      5. re-run the full release-truth surface after shipping `seqretsplit`
      6. reassess the shipped `seqretsplit` slice before any further retrieval
         continuation is mapped

237. Complete. Capture the explicit provider/seam stop conditions for
    `seqretsplit`.
    - Recorded the explicit provider/seam stop conditions in the governance
      appendix and roadmap.
    - The recorded pause-and-reassess triggers are:
      1. `seqretsplit` cannot remain deterministic under mocked-provider or
         managed-asset validation
      2. `seqretsplit` requires hidden live-network dependencies, implicit
         provider fallback chains, or unclear provider precedence
      3. `seqretsplit` cannot remain a bounded extension of normalized
         sequence-return behavior and instead demands broader filesystem-policy
         orchestration before one shipped slice closes
      4. `seqretsplit` forces broad filename-policy, directory-policy,
         batching-policy, or provider-parity claims that are not clearly local
         to the method

238. Complete. Implement the bounded provider-aware split-output orchestration
    and normalized return core for `seqretsplit`.
    - Added a method-associated `seqretsplit` core in
      `crates/emboss-tools/src/retrieval_tools/seqretsplit.rs`.
    - The landed bounded core now provides:
      - deterministic per-record split-output projections over one resolved
        `SeqretSource`
      - explicit file-name derivation from the same normalized computation path
      - bounded validation for empty resolved record sets
    - Added the private service seam that resolves one local or provider-backed
      input into the bounded `seqretsplit` core.
    - Focused validation covers:
      - local split-output naming over a real fixture
      - provider-backed split-output naming through a mocked client
    - This task intentionally stops at the bounded core:
      - no governed output-surface exposure yet
      - no registry or CLI exposure yet
      - no docs/autodoc/generated validation yet
      - no canonical fixtures or compared evidence yet

239. Complete. Expose the governed output surface for deterministic
    split-output retrieval behavior.
    - Added the bounded `seqretsplit` descriptor entry in
      `crates/emboss-tools/src/retrieval_tools/mod.rs`.
    - Added a bounded `invoke_seqretsplit_with_client(...)` service path in
      `crates/emboss-service/src/service.rs`.
    - The governed result surface now emits:
      - `ResultPayload::SequencePartitions` with one normalized record per
        deterministic split-output partition
      - explicit summary lines describing input source, split count, partition
        policy, and deterministic file naming
      - artifact metadata for the partitioned output plus each deterministic
        output file name
    - Focused validation covers:
      - local split-output partition payloads
      - provider-backed split-output partition payloads through a mocked client
    - This task intentionally stops short of shipment:
      - no governed registry exposure yet
      - no CLI routing yet
      - no generated docs or validation stubs yet
      - no canonical fixtures or compared evidence yet

240. Complete. Expose `seqretsplit` through the governed shipped surface.
    - Wired the bounded `seqretsplit` retrieval path through the governed
      registry, service dispatch, CLI, autodoc, generated tool page, and
      validation stub.
    - Refreshed the generated cohort and release-report surface to the
      intended interim state for this boundary:
      - shipped methods `106`
      - compared evidence `105`
      - executable evidence `1`
      - harvested legacy provenance present `106`
      - full-compared cohort temporarily `false`
      - harvest coverage complete remains `true`
    - This task intentionally stops at shipped plus executable evidence:
      - canonical fixtures and compared evidence still belong to Task `241`
      - the full release-truth gate is expected to fail until that closure
        lands

241. Complete. Close the `seqretsplit` compared-evidence slice.
    - Added the canonical bounded split-partition acceptance anchor for
      `seqretsplit` and refreshed the autodoc contract to reflect the real
      compared-evidence posture.
    - Restored the generated cohort/report surface from the temporary
      executable-only state back to full release truth:
      - shipped methods `106`
      - compared evidence `106`
      - executable evidence `0`
      - harvested legacy provenance present `106`
      - full-compared cohort `true`
      - harvest coverage complete `true`
    - This closes the bounded `seqretsplit` retrieval slice at compared
      evidence without widening retrieval-family scope beyond the selected
      method.

242. Complete. Re-run the full release-truth surface after shipping
    `seqretsplit`.
    - The broad post-ship gate stayed clean after the `seqretsplit` evidence
      closure.
    - `make release-generated-check` passed.
    - `python3 scripts/release_metadata.py truth-check` passed.
    - The docs build passed.
    - `git diff --check` passed.
    - The governed release-truth surface remained fully green:
      - shipped methods `106`
      - compared evidence `106`
      - executable evidence `0`
      - harvested legacy provenance present `106`
      - full-compared cohort `true`
      - harvest coverage complete `true`
      - retained backlog closed `true`
      - release-truth current `true`

243. Complete. Reassess the shipped `seqretsplit` slice before any further
    retrieval continuation is mapped.
    - Recorded the post-ship `seqretsplit` reassessment in the governance
      appendix and roadmap.
    - The result is affirmative:
      - the shipped `seqretsplit` slice stayed bounded, provider-aware, and
        deterministic
      - it remained a local extension of normalized sequence-return behavior
        rather than forcing broader retrieval-family orchestration
      - it did not require hidden live-network behavior, implicit provider
        fallback chains, or unclear provider precedence
      - it did not force broad filename-policy, directory-policy,
        batching-policy, or provider-parity claims beyond the local method
        slice
    - The governed release-truth surface remained fully green:
      - shipped methods `106`
      - compared evidence `106`
      - executable evidence `0`
      - harvested legacy provenance present `106`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

244. Complete. Extend the roadmap from the actual post-`seqretsplit` branch
    outcome.
    - The roadmap now records the current governed truth explicitly:
      - shipped methods: `106`
      - compared evidence: `106`
      - executable evidence: `0`
      - harvested legacy provenance present: `106`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `gapped_method_count: 0`
      - `weakest_evidence_family: null`
      - `release_truth_current: true`
    - The extension also makes the branch consequence explicit:
      - the second bounded retrieval continuation slice closed cleanly through
        `seqretsplit`
      - retrieval remains the active planning program, but it now has to be
        re-based onto the final narrowed candidate pool instead of assuming
        automatic continuation
      - protein-property rework remains the reserve third program and should
        not be promoted unless the final narrowed retrieval branch fails
        honest seam review
    - The next mapped tier is now Tasks `245` through `258`, centered on:
      - rebasing the remaining bounded retrieval continuation pool onto the
        actual post-`seqretsplit` state
      - deciding whether another bounded retrieval continuation candidate
        still exists after the second shipped slice
      - either promoting protein-property rework if the final narrowed
        retrieval continuation branch fails honest seam review, or selecting
        exactly one next bounded retrieval candidate:
        - `infoassembly`
      - capturing acceptance criteria, exact patch start conditions, and
        explicit provider/seam stop conditions for the chosen retrieval
        method
      - mapping and shipping one full bounded retrieval slice
      - rerunning release truth and reassessing the shipped retrieval slice
        before any further continuation is mapped

245. Complete. Rebase the bounded retrieval continuation pool onto the actual
    post-`seqretsplit` state.
    - Recorded the narrowed retrieval continuation pool explicitly in the
      governance appendix and roadmap.
    - The second bounded retrieval continuation slice is now treated as
      closed:
      - shipped bounded retrieval slices:
        - `seqretsetall`
        - `seqretsplit`
    - The remaining bounded retrieval continuation pool is now:
      - `infoassembly`
    - This checkpoint is inventory only.
    - It does not yet decide whether the final remaining method still passes
      honest seam review strongly enough to remain an active continuation
      candidate.

246. Complete. Decide whether another bounded retrieval continuation
    candidate still exists after the second shipped slice.
    - Recorded the post-`seqretsplit` retrieval viability decision explicitly
      in the governance appendix and roadmap.
    - The decision is affirmative:
      - another bounded retrieval continuation candidate still exists after
        `seqretsplit`
      - the no-candidate branch is not taken here
      - retrieval therefore remains the active planning program
    - The current viable bounded continuation pool remains:
      - `infoassembly`
    - Protein-property rework remains the reserve third program and should
      not be promoted at this checkpoint.

247. Complete. Close the untriggered protein-property promotion branch after
    the affirmative retrieval viability decision.
    - Recorded the explicit branch closeout in the governance appendix and
      roadmap.
    - The resulting branch consequence is:
      - retrieval continuation remains the active planning program
      - protein-property rework remains documented as the reserve third
        program
      - protein-property is not promoted by inertia while bounded retrieval
        continuation still has a viable candidate
    - The active viable bounded continuation pool remains:
      - `infoassembly`

248. Complete. Choose the single final bounded retrieval continuation
    candidate.
    - Recorded the retrieval lead-candidate activation explicitly in the
      governance appendix and roadmap.
    - The selected bounded continuation candidate is:
      - `infoassembly`
    - Recorded basis:
      - `infoassembly` is now the only remaining bounded continuation
        candidate in the narrowed post-`seqretsplit` pool
      - the prepared protein-property reserve branch remains documented but
        inactive because the no-candidate branch was not taken
      - no other retrieval-family method remains comparably bounded without
        widening provider-surface, orchestration, or scope claims beyond the
        already-governed continuation plan

249. Complete. Capture explicit method-level acceptance criteria for
    `infoassembly`.
    - Recorded the governed method-level acceptance criteria explicitly in the
      governance appendix and roadmap.
    - The criteria now make `infoassembly` concrete before code starts:
      - bounded provider-aware assembly metadata retrieval workflow only
      - deterministic normalized assembly metadata output built from the same
        governed provider-resolution and execution path
      - stable metadata-first output surface with explicit identifiers,
        provider source, and assembly fields defined by the same computation
        path
      - canonical managed-asset or mocked-provider fixtures
      - compared evidence required on normalized returned assembly metadata,
        not just orchestration intent
    - The non-goals remain explicit:
      - no hidden live-network validation
      - no broad provider-parity claims
      - no implicit widening into broader retrieval-family members such as
        `assemblyget`, `whichdb`, or `entret`
      - no generic retrieval-family widening merely because `infoassembly`
        ships

250. Complete. Capture the exact start conditions for the first
    `infoassembly` implementation patch.
    - Recorded the exact patch start conditions explicitly in the governance
      appendix and roadmap.
    - The start gate now requires:
      - the current shortlist to remain intact:
        1. completed bounded plotting continuation program
        2. active remote-retrieval planning program
        3. protein-property rework
      - the bounded retrieval continuation pool to remain limited to:
        - `infoassembly`
      - `infoassembly` to remain the single selected bounded retrieval
        continuation candidate
      - the zero-burden release-truth surface to remain intact
      - the first patch to stay limited to `infoassembly` plus the smallest
        support needed for deterministic provider-aware metadata retrieval,
        normalized assembly metadata output, and governed docs/validation
        plumbing
      - the patch to land as a full governed slice rather than a half-start
    - The same guardrails remain explicit:
      - no hidden live-network dependencies
      - no broad provider-parity claims
      - no implicit widening into broader retrieval-family members such as
        `assemblyget`, `whichdb`, or `entret`
      - no retrieval-family widening beyond the selected lead-method slice

251. Complete. Map the full bounded `infoassembly` implementation tier.
    - Recorded the full bounded implementation tier explicitly in the
      governance appendix and roadmap before code starts.
    - The bounded `infoassembly` tier is now explicit:
      1. implement the bounded provider-aware assembly metadata retrieval and
         normalized metadata return core
      2. expose the governed output surface for deterministic assembly
         metadata retrieval behavior
      3. expose `infoassembly` through the governed shipped surface
      4. add canonical managed-asset or mocked-provider fixtures plus
         compared evidence
      5. re-run the full release-truth surface after shipping `infoassembly`
      6. reassess the shipped `infoassembly` slice before any further
         retrieval continuation is mapped
    - The bounded tier preserves the same architectural rules:
      - provider-aware but deterministic execution
      - metadata-first normalized output
      - method-local scope only
      - no hidden live-network validation
      - no broad provider-parity claims
      - no retrieval-family widening unless `infoassembly` itself forces a
        real reassessment

252. Complete. Capture the explicit provider/seam stop conditions for
    `infoassembly`.
    - Recorded the explicit provider/seam stop conditions in the governance
      appendix and roadmap before code starts.
    - The repository should pause and reassess before implementation if:
      1. `infoassembly` cannot remain deterministic under mocked-provider or
         managed-asset validation
      2. `infoassembly` requires hidden live-network dependencies, implicit
         provider fallback chains, or unclear provider precedence
      3. `infoassembly` cannot remain a bounded provider-aware metadata-first
         slice and instead demands broader retrieval-family orchestration
         before one shipped slice closes
      4. `infoassembly` forces broad assembly-schema, provider-parity, or
         archive-scale acquisition claims that are not clearly local to the
         method

253. Complete. Implement the bounded `infoassembly` core.
    - Added the bounded `infoassembly` analytical core in
      `crates/emboss-tools/src/archive_tools/infoassembly.rs` and exported it
      through `crates/emboss-tools/src/archive_tools/mod.rs`.
    - Added the private `infoassembly` service seam in
      `crates/emboss-service/src/service.rs`.
    - What landed in the bounded core:
      - a method-associated `run_infoassembly(...)` path
      - deterministic assembly-first projection over normalized archive
        metadata
      - derived `assembly_accession` selection from linked study/project
        identifiers
      - explicit file-count and total-size summary fields from the same
        provider-backed metadata path
      - bounded validation for missing provider labels, missing accessions,
        missing object-class labels, missing route labels, and missing linked
        study/project identifiers
      - focused private service-seam tests for mocked ENA and SRA archive
        metadata
    - This task intentionally stopped at the bounded core:
      - no governed result-surface exposure yet
      - no registry or CLI exposure yet
      - no docs/autodoc/generated validation yet
      - no canonical fixtures or compared evidence yet

254. Complete. Expose the bounded `infoassembly` result surface.
    - Added the bounded `infoassembly` descriptor entry in
      `crates/emboss-tools/src/archive_tools/mod.rs`.
    - Added the bounded `invoke_infoassembly_with_client(...)` service path in
      `crates/emboss-service/src/service.rs`.
    - What landed in the governed surface:
      - deterministic `ResultPayload::TableReport` emission for bounded
        assembly-first metadata rows
      - explicit summary lines for provider, accession, object class,
        assembly identifier, file count, total bytes, and route
      - focused service tests for mocked ENA and SRA assembly-first metadata
        payloads
    - This task intentionally stopped short of shipment:
      - no governed registry exposure yet
      - no CLI routing yet
      - no generated docs or validation stubs yet
      - no canonical fixtures or compared evidence yet

255. Complete. Expose `infoassembly` through the governed shipped surface.
    - `infoassembly` is now exposed through the governed registry, service,
      and CLI.
    - Added the governed autodoc contract in
      `docs/autodoc/tools/infoassembly.json`.
    - Added a managed-asset mocked-provider autodoc case for the bounded
      shipment example in
      `crates/emboss-testkit/tests/fixtures/autodoc/infoassembly_ena_err123456_case.md`.
    - Refreshed the generated docs and validation/report surface to the honest
      interim executable-only state.
    - What landed in this shipment step:
      - governed registry exposure for `infoassembly`
      - governed service dispatch exposure for `emboss-rs infoassembly`
      - CLI routing coverage for `emboss-rs infoassembly`
      - generated tool page and validation stub for `infoassembly`
    - Interim governed state after this task:
      - shipped methods `107`
      - compared evidence `106`
      - executable evidence `1`
      - harvested legacy provenance present `106`
      - `full_compared_cohort: false`
      - `harvest_coverage_complete: false`
      - `release_truth_current: false`
    - This task intentionally stops at shipped plus executable evidence:
      - compared evidence for `infoassembly` still belongs to Task `256`
      - harvested legacy provenance for `infoassembly` still belongs to Task
        `256`
      - `python3 scripts/release_metadata.py truth-check` is expected to fail
        at this boundary because the shipped cohort is no longer fully
        compared or fully harvested

256. Complete. Close the `infoassembly` compared-evidence and harvested-provenance slice.
    - Added the bounded `infoassembly` acceptance-anchor fixture in
      `crates/emboss-testkit/tests/fixtures/acceptance_anchors/infoassembly_normalize_ena_assembly_metadata.tsv`.
    - Wired `infoassembly` into the acceptance-anchor harness through the
      mocked-provider execution path in `crates/emboss-testkit/src/anchor.rs`.
    - Updated the governed autodoc contract in
      `docs/autodoc/tools/infoassembly.json` to record the bounded
      compared-and-harvested example honestly.
    - Refreshed the generated docs and validation/report surface back to the
      fully green state.
    - Current governed state after this task:
      - shipped methods `107`
      - compared evidence `107`
      - executable evidence `0`
      - harvested legacy provenance present `107`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `release_truth_current: true`
    - This closes the bounded `infoassembly` retrieval slice at compared
      evidence and harvested legacy provenance.

257. Complete. Re-run the full release-truth surface after shipping `infoassembly`.
    - Re-ran the broad release-truth checkpoint after the bounded
      `infoassembly` evidence closure.
    - Observed result:
      - `PYTHON=.venv-docs/bin/python make release-generated-check`: passed
      - `python3 scripts/release_metadata.py truth-check`: passed
      - `PYTHON=.venv-docs/bin/python make docs`: passed
      - `git diff --check`: passed
    - Governed release-truth state remained fully green:
      - shipped methods `107`
      - compared evidence `107`
      - executable evidence `0`
      - harvested legacy provenance present `107`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

258. Complete. Reassess the shipped `infoassembly` slice before any further retrieval continuation is mapped.
    - Recorded the post-ship `infoassembly` reassessment in the governance
      appendix and roadmap.
    - The result is affirmative:
      - the shipped `infoassembly` slice stayed bounded, provider-aware, and
        metadata-first
      - it remained a local extension of the governed retrieval program rather
        than forcing broader retrieval-family orchestration
      - it did not require hidden live-network behavior, implicit provider
        fallback chains, or non-local provider-parity or assembly-schema claims
    - The governed release-truth surface remained fully green:
      - shipped methods `107`
      - compared evidence `107`
      - executable evidence `0`
      - harvested legacy provenance present `107`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

259. Complete. Extend the roadmap from the actual post-`infoassembly` branch outcome.
    - Recorded the current governed truth explicitly:
      - shipped methods `107`
      - compared evidence `107`
      - executable evidence `0`
      - harvested legacy provenance present `107`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `gapped_method_count: 0`
      - `weakest_evidence_family: null`
      - `release_truth_current: true`
    - Recorded the branch consequence explicitly:
      - the bounded retrieval continuation program has now closed through its
        final selected candidate
      - no comparably bounded retrieval continuation candidate remains active
      - protein-property rework now becomes the next active planning program
        by explicit branch resolution rather than by inertia
      - restriction-analysis rework remains the reserve next program and
        should not be promoted unless the bounded protein-property branch fails
        honest seam review
    - The next mapped tier is now Tasks `260` through `273`, centered on:
      - confirming retrieval continuation is exhausted as an active bounded
        branch
      - activating the protein-property rework program explicitly
      - confirming the reserve next-program ordering behind it
      - activating the single remaining bounded protein-property candidate:
        - `psiphi`
      - capturing acceptance criteria, exact patch start conditions, and
        explicit coordinate/seam stop conditions
      - mapping and shipping one full bounded `psiphi` slice
      - rerunning release truth and reassessing the shipped `psiphi` slice
        before any further continuation is mapped

260. Confirm that the bounded retrieval continuation program has now closed through its final selected candidate.
    - Recorded explicitly that the bounded retrieval continuation program
      closed cleanly through:
      - `seqretsetall`
      - `seqretsplit`
      - `infoassembly`
    - Recorded explicitly that no comparably bounded retrieval continuation
      candidate remains active after the shipped `infoassembly`
      reassessment.
    - Recorded explicitly that retrieval continuation should no longer be
      treated as the default next planning branch.
    - The governed release-truth surface remained fully green:
      - shipped methods `107`
      - compared evidence `107`
      - executable evidence `0`
      - harvested legacy provenance present `107`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

261. Activate protein-property rework as the next active planning program by explicit branch resolution.
    - Recorded explicitly that plotting and bounded retrieval continuation are
      no longer the active next-planning branches.
    - Promoted protein-property rework from reserve status to the active next
      planning program by explicit branch resolution.
    - Recorded explicitly that restriction-analysis rework remains behind the
      active protein-property branch and is not promoted by this step.
    - The active bounded protein-property planning subset is now narrowed to:
      - `psiphi`
    - This activation does not widen protein-property scope beyond the already
      governed bounded branch posture:
      - no broad structural-biology family activation
      - no implicit promotion of already-shipped `iep` or `pepdigest`
      - no early revival of omitted molecular-weight utilities

262. Reconfirm which reserve program now sits behind active protein-property rework.
    - Recorded explicitly that restriction-analysis rework remains the next
      reserve program after protein-property activation.
    - Recorded explicitly that the reserve restriction-analysis subset remains:
      - `recoder`
      - `silent`
    - Do not promote that reserve program by inertia while the bounded
      protein-property branch is still active.
    - This remains an ordering checkpoint only:
      - no `psiphi` code work starts here
      - no restriction-analysis implementation tier is activated here

263. Activate the single bounded protein-property lead candidate explicitly.
    - Recorded explicitly that `psiphi` is now the single bounded lead
      candidate for the active protein-property rework program.
    - Recorded explicitly why the family narrows to that candidate:
      - `iep` is already shipped and fully evidenced
      - `pepdigest` is already shipped and fully evidenced
      - `psiphi` is the only remaining governed protein-property rework
        member that is neither already shipped nor explicitly omitted
    - Recorded explicitly that no broader protein-property candidate pool
      remains active at this checkpoint.

264. Capture explicit `psiphi` method-level acceptance criteria.
    - Captured explicit governed `psiphi` acceptance criteria in the roadmap
      and governance appendix.
    - The criteria now make `psiphi` concrete before code starts:
      - bounded protein-coordinate analytical surface only
      - stable table-first output with enough explicit columns to reconstruct
        per-residue phi/psi reporting from the same computation path
      - deterministic typed result surface derived directly from the same
        coordinate-processing path
      - canonical analytical fixtures and compared evidence on normalized
        torsion-angle rows
      - explicit honesty about residue eligibility, chain continuity, and
        coordinate-model limitations
    - The same non-goals remain explicit:
      - no Ramachandran plotting or renderer-coupled figure logic
      - no broad structural-analysis family activation
      - no implicit widening into general structure parsing or modeling
      - no family-wide continuation claim merely because `psiphi` ships

265. Capture the exact start conditions for the first `psiphi` implementation patch.
    - Captured the exact governed patch start conditions for `psiphi` in the
      roadmap and governance appendix.
    - The start gate now requires:
      - the current active-program ordering to remain intact:
        1. protein-property rework
        2. restriction-analysis rework
      - `psiphi` to remain the single bounded lead candidate for the active
        protein-property branch
      - the zero-burden release-truth surface to remain intact
      - the first patch to stay limited to `psiphi` plus the smallest support
        needed for deterministic coordinate-derived computation, typed
        result-surface emission, and governed docs/validation plumbing
      - the patch to land as a full bounded slice rather than a half-start
    - The same guardrails remain explicit:
      - no Ramachandran plotting
      - no broad structural-analysis family activation
      - no generalized coordinate-processing framework
      - no promotion of reserve restriction-analysis work while the bounded
        `psiphi` branch remains active

266. Complete. Map the full bounded `psiphi` implementation tier.
    - Mapped the full bounded `psiphi` implementation tier explicitly in the
      roadmap and governance appendix before code starts.
    - The bounded `psiphi` tier is now explicit:
      1. implement the bounded protein-coordinate analytical core
      2. expose the deterministic typed result surface for normalized
         per-residue torsion-angle reporting
      3. expose `psiphi` through the governed shipped surface
      4. add canonical analytical fixtures plus compared evidence on
         normalized torsion-angle rows
      5. re-run the full release-truth surface after shipping `psiphi`
      6. reassess the shipped `psiphi` slice before any further continuation
         is mapped
    - The bounded tier preserves the same architectural rules:
      - protein-coordinate, table-first scope only
      - deterministic typed result output from the same computation path
      - method-local implementation only
      - no Ramachandran plotting
      - no generalized coordinate-processing framework
      - no structural-analysis family widening unless `psiphi` itself forces
        a real reassessment

267. Complete. Capture explicit coordinate/seam stop conditions for `psiphi`.
    - Captured the explicit coordinate/seam stop conditions for `psiphi` in
      the roadmap and governance appendix.
    - The repository should now pause and reassess before implementation if:
      1. `psiphi` cannot remain table-first with a deterministic typed result
         surface derived from the same coordinate-processing path
      2. `psiphi` requires Ramachandran plotting, renderer-coupled figure
         logic, or other presentation-policy behavior
      3. `psiphi` cannot remain method-associated and instead demands a
         generalized coordinate-processing or structural-analysis framework
         before one shipped slice closes
      4. `psiphi` forces broad chain-model normalization, missing-atom
         imputation, alternate-conformer policy, or comparative
         structure-analysis claims that are not clearly local to the method

268. Complete. Implement the bounded `psiphi` analytical core.
    - Added the bounded method-associated `psiphi` analytical core in
      `emboss-core` and exported it through the core library surface.
    - What landed in the bounded core:
      - a deterministic `protein_psiphi_profile(...)` computation path over
        PDB `ATOM` coordinate text
      - stable per-residue analytical rows with:
        - chain identifier
        - residue name
        - residue number
        - insertion code
        - backbone `N`/`CA`/`C` presence flags
        - previous/next continuity flags
        - bounded `phi_degrees`
        - bounded `psi_degrees`
      - explicit bounded v1 behavior:
        - only backbone `ATOM` records are retained
        - only blank or `A` alternate locations are considered eligible
        - continuity is limited to same-chain, sequential,
          insertion-code-free residues
        - torsions remain absent rather than inferred when continuity or
          required backbone atoms are missing
      - focused bounded validation for:
        - no retained backbone atoms
        - invalid residue-number parsing
        - invalid coordinate-field parsing
    - This task intentionally stops at the analytical core:
      - no governed result-surface exposure yet
      - no registry or CLI exposure yet
      - no autodoc or generated validation surface yet
      - no canonical fixtures or compared evidence yet

269. Complete. Expose the bounded `psiphi` result surface.
    - Added the bounded `psiphi` result surface in the method-local tools and
      service seams without exposing the method through the shipped registry
      yet.
    - What landed in the governed surface:
      - a method-local `protein_coordinates::psiphi` tool path over the Task
        `268` analytical core
      - a bounded local coordinate-input wrapper and deterministic coordinate
        file loader
      - a public service `invoke_psiphi(...)` seam that emits
        `ResultPayload::TableReport`
      - stable per-residue torsion rows with chain, residue identity,
        continuity flags, backbone presence flags, and bounded
        `phi_degrees`/`psi_degrees` values
      - focused result-surface tests for:
        - local coordinate execution
        - provider-backed input rejection
        - backbone-free input rejection
    - This task intentionally stopped short of shipment:
      - no governed registry exposure yet
      - no CLI routing yet
      - no autodoc or generated validation surface yet
      - no canonical fixtures or compared evidence yet

270. Complete. Expose `psiphi` through the governed shipped surface.
    - Wired `psiphi` through the governed registry, service, CLI, curated
      autodoc, and generated shipped/docs surface in the honest interim
      executable-only state.
    - Current governed interim state after this task:
      - shipped methods: `108`
      - compared evidence: `107`
      - executable evidence: `1`
      - harvested legacy provenance present: `107`
      - `full_compared_cohort: false`
      - `harvest_coverage_complete: false`
      - `release_truth_current: true` in cohort health
    - The expected `truth-check` gate remains red at this boundary because
      `psiphi` is still below compared evidence and harvested legacy
      provenance capture until Task 271 closes that slice.

271. Close the `psiphi` compared-evidence and harvested-provenance slice.
    - Complete. Added canonical bounded fixtures, harvested provenance, and
      compared evidence for the shipped `psiphi` slice.
    - Wired `psiphi` into the acceptance-anchor harness with a checked-in
      torsion-angle table fixture and harvested legacy reference.
    - Restored the generated/report surface to the fully green state:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `release_truth_current: true`

272. Re-run the full release-truth surface after shipping `psiphi`.
    - Complete. Re-ran the governed release-truth surface after the bounded
      `psiphi` evidence closure and recorded the observed clean state.
    - `python3 scripts/release_metadata.py truth-check` passed.
    - `make docs` passed.
    - `git diff --check` passed.
    - The observed governed state remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

273. Reassess the shipped `psiphi` slice before any further continuation is mapped.
    - Complete. Recorded the post-ship `psiphi` reassessment explicitly after
      the compared-evidence closure and release-truth rerun.
    - The reassessment result is affirmative:
      - the shipped `psiphi` slice stayed bounded, method-associated, and
        scientifically honest
      - it remained a protein-coordinate, table-first analytical surface
        rather than forcing Ramachandran plotting or renderer-coupled figure
        logic
      - it kept the normalized per-residue torsion-angle rows and typed
        result surface derived from the same coordinate-processing path
      - it did not force broad chain-model normalization, missing-atom
        imputation, alternate-conformer policy, or comparative
        structure-analysis claims beyond the local method slice
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

274. Extend the roadmap from the actual post-`psiphi` branch outcome.
    - Recorded the current governed truth explicitly:
      - shipped methods `108`
      - compared evidence `108`
      - executable evidence `0`
      - harvested legacy provenance present `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `gapped_method_count: 0`
      - `weakest_evidence_family: null`
      - `release_truth_current: true`
    - Recorded the branch consequence explicitly:
      - the bounded protein-property rework program has now closed through its
        single selected candidate
      - no comparably bounded protein-property rework candidate remains active
      - restriction-analysis rework now becomes the next active planning
        program by explicit branch resolution rather than by inertia
      - the next reserve-program ordering should be revisited only after the
        bounded restriction-analysis branch is assessed, rather than being
        implied early from stale backlog assumptions
    - The next mapped tier is now Tasks `275` through `288`, centered on:
      - confirming protein-property rework is exhausted as an active bounded
        branch
      - activating restriction-analysis rework explicitly
      - deciding whether the narrowed bounded restriction-analysis branch still
        passes honest seam review strongly enough to remain active
      - either revisiting reserve-program ordering if that narrowed branch
        fails honest seam review, or selecting exactly one bounded
        restriction-analysis lead candidate from:
        - `recoder`
        - `silent`
      - capturing method-level acceptance criteria, exact patch start
        conditions, and explicit recoding/seam stop conditions for the chosen
        method
      - mapping and shipping one full bounded restriction-analysis slice
      - rerunning release truth and reassessing the shipped
        restriction-analysis slice before any further continuation is mapped

275. Confirm that the bounded protein-property rework program has now closed through its single selected candidate.
    - Recorded explicitly that the bounded protein-property rework program
      closed cleanly through:
      - `psiphi`
    - Recorded explicitly why that closeout is now honest:
      - `iep` is already shipped and fully evidenced
      - `pepdigest` is already shipped and fully evidenced
      - `psiphi` has now shipped, closed compared evidence and harvested
        provenance, rerun release truth, and passed post-ship reassessment
    - Recorded explicitly that no comparably bounded protein-property rework
      candidate remains active after the shipped `psiphi` reassessment.
    - Recorded explicitly that protein-property rework should no longer be
      treated as the default next planning branch.
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

276. Activate restriction-analysis rework as the next active planning program by explicit branch resolution.
    - Recorded explicitly that plotting, bounded retrieval continuation, and
      protein-property rework are no longer the active next-planning branches.
    - Promoted restriction-analysis rework from reserve status to the active
      next planning program by explicit branch resolution.
    - Recorded explicitly that the active bounded restriction-analysis
      planning subset is now narrowed to:
      - `recoder`
      - `silent`
    - This activation does not widen restriction-analysis scope beyond the
      already governed bounded branch posture:
      - no broad enzyme-catalog or provider-parity claims
      - no implicit promotion of omitted broader restriction workflows
      - no early activation of post-restriction reserve-program ordering
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

277. Decide whether the narrowed bounded restriction-analysis branch still passes honest seam review strongly enough to remain active.
    - Recorded the post-activation seam-review decision explicitly for the
      narrowed bounded restriction-analysis pool.
    - The decision is affirmative:
      - a bounded restriction-analysis continuation candidate still exists
      - the no-candidate branch is not taken here
      - restriction-analysis rework therefore remains the active planning
        program
    - The current viable bounded continuation pool remains:
      - `recoder`
      - `silent`
    - The broader family remains outside the active bounded branch at this
      checkpoint:
      - database extraction and provider-heavy surfaces such as
        `rebaseextract` and `redata`
      - reporting or visualization-heavy surfaces such as `remap`
      - broader enzyme-scan workflows such as `restrict` and `restover`
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

278. Close the untriggered reserve-ordering revisit branch explicitly after the affirmative restriction-analysis seam review.
    - Recorded explicitly that the narrowed bounded restriction-analysis
      branch stayed affirmative at the seam-review gate.
    - Recorded explicitly that the reserve-ordering revisit branch is not
      taken at this checkpoint.
    - The resulting branch consequence is:
      - restriction-analysis rework remains the active planning program
      - reserve-program ordering behind it remains deferred and inactive
      - the active bounded continuation pool remains:
        - `recoder`
        - `silent`
    - The broader family is still not activated by inertia:
      - no promotion of `rebaseextract` or `redata`
      - no promotion of `remap`, `restrict`, or `restover`
      - no reserve-program reshuffle while the bounded restriction-analysis
        branch still has viable candidates
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

279. Reconcile the bounded restriction-analysis lead-candidate assumption against the actual shipped state.
    - Complete. Closed this checkpoint as an honesty correction rather than a
      lead-method selection.
    - Recorded explicitly that the previously assumed bounded candidate pool
      is already fully shipped and evidenced in the actual repository state:
      - `recoder`
      - `silent`
    - Recorded explicitly that no new lead candidate needs to be selected at
      this checkpoint because:
      - both methods already appear in the governed shipped registry
      - both methods already carry compared evidence
      - both methods already carry harvested legacy provenance
    - The resulting planning consequence is:
      - the bounded restriction-analysis continuation plan written in Task
        `274` is stale relative to current repository truth
      - further restriction-analysis planning must be rebased on the actual
        post-`recoder` and post-`silent` shipped state before any new
        continuation tier is mapped
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

280. Extend the roadmap from the actual post-restriction shipped state.
    - Recorded the current governed truth explicitly:
      - shipped methods `108`
      - compared evidence `108`
      - executable evidence `0`
      - harvested legacy provenance present `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `gapped_method_count: 0`
      - `weakest_evidence_family: null`
      - `release_truth_current: true`
    - Recorded the branch consequence explicitly:
      - the previously assumed bounded restriction-analysis continuation tier
        is already closed in the actual shipped state because `recoder` and
        `silent` are both already shipped and fully evidenced
      - no comparably bounded restriction-analysis continuation candidate
        remains active
      - the roadmap is back in an ambiguity-resolution state rather than on a
        preselected implementation branch
      - the next planning cycle must therefore be derived from current
        governance and generated truth rather than from stale reserve-order
        assumptions
    - The next mapped tier is now Tasks `281` through `294`, centered on:
      - explicitly closing the bounded restriction-analysis continuation branch
      - confirming the broader restriction-analysis family remains inactive
        unless a new bounded local slice is justified
      - inventorying the remaining unshipped rework and add families from the
        current governance appendix
      - generating an explicit next-program recommendation from current
        shipped-state truth rather than from inherited family order
      - choosing one next planning program only after that recommendation is
        made explicit
      - capturing bounded acceptance criteria, start conditions, and stop
        conditions for the newly selected program before any code starts

281. Confirm that the bounded restriction-analysis continuation branch is now closed in the actual shipped state.
    - Recorded explicitly that the bounded restriction-analysis continuation
      branch is now closed in the actual shipped state because its retained
      bounded members are already shipped:
      - `recoder`
      - `silent`
    - Recorded explicitly that no comparably bounded restriction-analysis
      continuation candidate remains active after the post-shipment
      reconciliation step.
    - Recorded explicitly that restriction-analysis continuation should no
      longer be treated as an active implementation branch.
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

282. Confirm that the broader restriction-analysis family remains inactive unless a new bounded local slice is justified explicitly.
    - Recorded explicitly that closing the bounded restriction-analysis
      continuation branch does not activate the broader family by inertia.
    - Recorded explicitly that the broader family remains inactive at this
      checkpoint:
      - `rebaseextract`
      - `redata`
      - `remap`
      - `restrict`
      - `restover`
    - Recorded explicitly why those methods remain inactive:
      - database-extraction and provider-surface pressure remains unbounded
      - reporting and visualization surfaces remain broader than the closed
        local recoding seam
      - broader enzyme-scan workflows remain outside the already-shipped
        bounded edit-design primitives
    - Recorded explicitly that any future activation would require a new
      bounded local justification rather than inherited family momentum.
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

283. Inventory the remaining wholly unshipped `Rework` and `Add` families from the current governance appendix.
    - Recorded explicitly that the already-activated `Rework` families now
      have shipped representation and therefore are not part of the remaining
      wholly unshipped family inventory:
      - plotting and visualization tools
      - remote retrieval and archive acquisition
      - protein property and structural-summary utilities
      - restriction-enzyme design and analysis
    - Recorded explicitly that the remaining wholly unshipped `Rework`
      families are:
      - primer and assay-oriented search
      - external database preparation helpers
      - legacy prediction methods with enduring scientific value
      - command discovery and help-navigation
    - Recorded explicitly that the remaining wholly unshipped `Add` families
      are:
      - HMM and probabilistic homology workflows
      - modern archive-scale raw data ingestion
    - Recorded explicitly that these six families now form the live inventory
      for the next-program recommendation step, rather than any stale
      inherited family order.
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

284. Generate an explicit next-program recommendation from current shipped-state truth rather than inherited family order.
    - Recorded explicitly that the current generated surfaces remain neutral:
      - no reprioritization signals were generated
      - no recommendation changes are required automatically
    - Recorded explicitly that the repository therefore needs a manual
      recommendation from the remaining live inventory rather than a
      report-driven reorder.
    - Recorded explicitly that the recommended next active planning program is:
      - primer and assay-oriented search
    - Recorded explicitly why this family leads the remaining inventory:
      - it is the most bounded remaining user-facing `Rework` family
      - it has only three surviving `Rework` methods after existing `Omit`
        decisions:
        - `eprimer3`
        - `primersearch`
        - `sirna`
      - it does not depend on broad provider-ingestion seams or strategic-add
        platform claims
      - it appears materially less open-ended than legacy prediction or
        archive-scale `Add` programs
    - Recorded explicitly that the remaining family order behind it is:
      1. primer and assay-oriented search
      2. command discovery and help-navigation
      3. external database preparation helpers
      4. legacy prediction methods with enduring scientific value
      5. HMM and probabilistic homology workflows
      6. modern archive-scale raw data ingestion
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

285. Activate the recommended next planning program explicitly rather than leaving it as an advisory note only.
    - Recorded explicitly that primer and assay-oriented search is now the
      active planning program by branch resolution rather than by inherited
      family order.
    - Recorded explicitly that the currently live bounded family subset is:
      - `eprimer3`
      - `primersearch`
      - `sirna`
    - Recorded explicitly that the remaining families stay inactive behind the
      newly activated program:
      - command discovery and help-navigation
      - external database preparation helpers
      - legacy prediction methods with enduring scientific value
      - HMM and probabilistic homology workflows
      - modern archive-scale raw data ingestion
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

286. Decide whether the narrowed primer and assay-oriented search branch still passes honest seam review strongly enough to remain active.
    - Recorded explicitly that the decision is affirmative.
    - Recorded explicitly that a bounded continuation candidate still exists
      inside the active primer family.
    - Recorded explicitly that the viable bounded continuation pool remains:
      - `eprimer3`
      - `primersearch`
      - `sirna`
    - Recorded explicitly that the omitted family members do not re-enter the
      active branch:
      - `eprimer32`
      - `stssearch`
    - Recorded explicitly that the family therefore remains active for the
      next bounded lead-method selection step rather than forcing an immediate
      return to reserve-program ordering.
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

287. Choose exactly one bounded lead candidate from the active primer-family pool.
    - Recorded explicitly that the selected bounded lead candidate is:
      - `primersearch`
    - Recorded explicitly why `primersearch` leads the active pool:
      - it is the narrowest remaining search-first surface in the family
      - it appears easier to keep deterministic than primer-design or
        siRNA-selection workflows
      - it avoids immediately forcing optimization-heavy scoring or broader
        biological-design policy claims
    - Recorded explicitly that the non-selected bounded family members remain
      documented but inactive:
      - `eprimer3`
      - `sirna`
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

288. Capture explicit `primersearch` method-level acceptance criteria before code starts.
    - Recorded explicitly that the governed criteria now make
      `primersearch` concrete before implementation:
      - bounded primer-pair sequence-search workflow only
      - deterministic table-first output derived from the same matching path
      - explicit reporting of primer-pair identity, strand/orientation, and
        matched interval coordinates from the same computation path
      - canonical analytical fixtures and compared evidence on normalized
        primer-hit rows
      - honest handling of mismatch policy, orientation rules, ambiguity
        handling, and pair-completion rules
    - Recorded explicitly that the non-goals remain:
      - no primer-design optimization
      - no broad assay-scoring or thermodynamic ranking framework
      - no widening into `eprimer3` or `sirna`
      - no family-wide continuation claim merely because `primersearch`
        ships
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

289. Capture the exact start conditions for the first `primersearch` implementation patch.
    - Recorded explicitly that the start gate now requires:
      - primer and assay-oriented search to remain the active planning
        program
      - `primersearch` to remain the single selected bounded lead candidate
      - the active bounded family subset to remain limited to:
        - `primersearch`
        - `eprimer3`
        - `sirna`
      - the zero-burden release-truth surface to remain intact
      - the first patch to stay limited to `primersearch` plus the smallest
        support needed for deterministic primer-pair matching, normalized
        table-first reporting, and governed docs/validation plumbing
      - the patch to land as a full bounded slice rather than a half-start
    - Recorded explicitly that the same guardrails remain:
      - no primer-design optimization
      - no assay-ranking or thermodynamic scoring framework
      - no widening into `eprimer3` or `sirna`
      - no family-wide continuation claim merely because one bounded
        `primersearch` slice ships
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

290. Map the full bounded `primersearch` implementation tier.
    - Recorded explicitly that the bounded `primersearch` tier is now:
      1. implement the bounded primer-pair matching analytical core
      2. expose the deterministic typed result surface for normalized
         primer-hit reporting
      3. expose `primersearch` through the governed shipped surface
      4. add canonical analytical fixtures plus compared evidence on
         normalized primer-hit rows
      5. re-run the full release-truth surface after shipping
         `primersearch`
      6. reassess the shipped `primersearch` slice before any further
         primer-family continuation is mapped
    - Recorded explicitly that the same bounded rules remain:
      - primer-pair search scope only
      - deterministic table-first reporting from the same matching path
      - method-local implementation only
      - no primer-design optimization
      - no assay-ranking or thermodynamic scoring framework
      - no primer-family widening unless `primersearch` itself forces a real
        reassessment
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

291. Capture the explicit seam-pressure stop conditions for `primersearch`.
    - Recorded explicitly that the repository should pause and reassess
      before implementation if:
      1. `primersearch` cannot remain table-first with a deterministic typed
         result surface derived from the same primer-matching path
      2. `primersearch` requires primer-design optimization, assay-ranking,
         thermodynamic scoring policy, or other non-local biological-design
         behavior
      3. `primersearch` cannot remain method-associated and instead demands a
         generalized primer-analysis framework before one shipped slice closes
      4. `primersearch` forces broad ambiguity-resolution policy, mismatch
         search taxonomy, or pair-completion semantics that are not clearly
         local to the method
    - The governed release-truth surface remained fully green:
      - shipped methods: `108`
      - compared evidence: `108`
      - executable evidence: `0`
      - harvested legacy provenance present: `108`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`

292. Implement the bounded primer-pair matching analytical core for `primersearch`.
    - Added the bounded `primersearch` analytical core in `emboss-core` and
      exported it through the crate root.
    - Landed a method-associated deterministic profile over one nucleotide
      target record plus a stable ordered primer-pair set.
    - Kept the analytical surface table-first and bounded to complete pair
      hits only.
    - Landed explicit bounded v1 behavior:
      - exact or IUPAC-ambiguous primer-pattern matching only
      - deterministic forward and reverse orientation detection on the same
        input strand
      - one-based inclusive left-primer, right-primer, and amplicon
        coordinates from the same matching path
      - no primer-design optimization, assay-ranking, or mismatch scoring
    - Added focused bounded validation for:
      - non-nucleotide input
      - empty primer-pair sets
      - empty primer-pair names
      - invalid primer-pattern symbols

293. Expose the deterministic typed result surface for normalized `primersearch` hit reporting.
    - Added the bounded `primersearch` result surface in `emboss-tools` and
      a direct public `EmbossService::invoke_primersearch(...)` seam in
      `emboss-service`.
    - Landed a method-local local-input wrapper for:
      - one nucleotide target input
      - one tab-delimited primer-pair file
    - Kept the surface bounded and not yet shipped:
      - no governed registry exposure yet
      - no CLI routing yet
      - no generated docs or validation stubs yet
      - no canonical fixtures or compared evidence yet
    - Landed deterministic normalized table rows with:
      - target record identifier
      - primer-pair name
      - strand/orientation
      - left-primer coordinates
      - right-primer coordinates
      - amplicon coordinates and length
      - matched left and right primer slices
    - Added focused bounded validation for:
      - provider-backed sequence inputs not supported by this local seam
      - invalid primer-pair file rows

294. Expose `primersearch` through the governed shipped surface.
    - Wired `primersearch` through the governed tool registry, governed
      service dispatch/help surface, and CLI parsing path.
    - Added the curated autodoc contract for `primersearch` plus the
      generated shipped docs and validation stub.
    - Refreshed the generated cohort/report surface to the honest interim
      shipped state:
      - shipped methods: `109`
      - compared evidence: `108`
      - executable evidence: `1`
      - harvested legacy provenance present: `109`
      - `full_compared_cohort: false`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`
    - Recorded explicitly that this is an executable-only shipment boundary:
      - the governed local `primersearch` seam is now shipped
      - the canonical compared analytical fixture is still pending
      - the full-compared release gate does not go green again until the next
        evidence-closing task
      - the release-readiness truth surface was refreshed to the new
        `109`-method interim state so the generated cohort and release docs
        agree even though the compared-evidence gate remains amber

295. Close the `primersearch` compared-evidence slice and restore the fully green release gate.
    - Added the canonical compared analytical fixture for `primersearch` and
      wired it into the acceptance-anchor harness.
    - Updated the governed autodoc prose so the shipped state now describes
      canonical compared evidence rather than the interim executable-only
      checkpoint.
    - Refreshed the generated validation/report surface back to the fully
      green state:
      - shipped methods: `109`
      - compared evidence: `109`
      - executable evidence: `0`
      - harvested legacy provenance present: `109`
      - `full_compared_cohort: true`
      - `harvest_coverage_complete: true`
      - `retained_backlog_closed: true`
      - `release_truth_current: true`
    - Restored the release-facing documentation to the actual current state:
      - no shipped methods remain below compared evidence
      - no weak-evidence family remains active in the generated cohort
      - the bounded `primersearch` slice is now fully evidenced rather than
        executable-only
