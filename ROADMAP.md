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
    - Prefer tightening existing report schemas over adding parallel one-off
      prose explanations.
    - If a field no longer communicates useful truth, rename or replace it
      through governed generated outputs.

64. Tighten the release gate to require stable post-closure summary semantics.
    - Once Tasks `61` through `63` are resolved, make the release truth check
      fail if release-facing docs or generated summaries drift back into
      numerically confusing post-closure states.

65. Reassess the post-`1.0.0` release narrative now that full compared and full harvest are achieved.
    - Refresh the draft release notes and RC readiness framing so they describe
      the current evidence posture directly rather than as a near-term
      milestone.
    - Keep this as documentation truth maintenance, not marketing expansion.

66. Add a generated “post-closure evidence invariants” report if summary drift remains hard to audit across multiple artefacts.
    - Only add this if the current set of generated reports is no longer enough
      to explain the post-closure state cleanly.
    - Do not invent a new artefact if existing reports can be clarified instead.

67. Decide whether the first actual `Rework` implementation program remains plotting after the summary-semantic cleanup.
    - Reconfirm the Task `57` decision against the cleaned generated surface.
    - Explicitly verify that nothing in Tasks `61` through `66` changes the
      plotting-first decision materially.

68. If plotting remains first, generate a dedicated plotting rework sub-roadmap before code changes begin.
    - The sub-roadmap should define:
      - bounded initial method subset
      - plot-contract evidence model
      - R-rendering handoff constraints
      - release-risk framing
    - Do not start broad plotting implementation before this exists.

69. If plotting is later blocked, generate the equivalent remote-retrieval rework sub-roadmap instead of switching informally.
    - Preserve the explicit fallback ordering already recorded in governance.
    - Require the same bounded scope and evidence framing as the plotting path.

70. Reassess whether protein-property rework still remains the third candidate after the first-program sub-roadmap is drafted.
    - This should remain a planning check, not an implicit promotion.

71. Audit `make release-generated-check` for any remaining post-closure ordering hazards or incidental churn.
    - Earlier roadmap work observed regeneration-order issues and EOF-only
      churn during broad refreshes.
    - Determine whether the current release-generated path is clean enough for
      the post-closure steady state.

72. If release-generated ordering hazards remain, fix them as release-process debt rather than tolerating them as folklore.
    - Prefer deterministic generation order and clean-tree stability over
      manual restoration steps.

73. Reassess whether any remaining release-facing generated reports are redundant after post-closure cleanup.
    - If two artefacts express the same truth with no distinct release value,
      consider consolidating them.
    - Do not remove reports that still carry unique governance or release-gate
      meaning.

74. If the first `Rework` program is approved, capture its acceptance criteria as governed roadmap truth before implementation starts.
    - The first post-retained implementation program should have explicit entry
      and exit criteria, not just a family label.

75. Extend this roadmap again after Task `74`.
    - Continue to derive priorities from:
      - cohort validation
      - governance alignment
      - cohort health
      - comparison coverage
      - retained backlog closure
      - full compared cohort
      - harvest coverage
      - any new post-closure summary reports
    - Preserve the same repository rules:
      - honest documentation
      - scoped changes
      - commit-and-push completion
      - no unrelated code churn

## Next Tier Task Map

This next extension remains derived from the current generated truth surface:

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

The remaining work is no longer about shipping retained methods or deepening
evidence. It is now about:
- making the generated summaries semantically crisp in the post-closure state
- eliminating lingering release-generation folklore
- ensuring the release-truth surface is as deterministic as the code/evidence
  surface
- preparing the first actual `Rework` implementation program without starting
  it prematurely

76. Preserve the resolved `gapped_method_count` semantics against future drift.
    - The field now represents blocking cohort gaps only and currently reports
      `0`.
    - If future non-blocking validation-report notes reappear, they should stay
      in the per-tool visible gap surface rather than inflating the top-line
      cohort summary again.

77. Preserve the resolved zero-burden `weakest_evidence_family` semantics against future drift.
    - When `weak_evidence_method_count == 0`, the field should remain `null`
      and should not trigger weak-evidence signals or recommendations.
    - If a future informational family ranking is needed, add it explicitly as
      a separate concept rather than overloading the weakness field again.

78. If Tasks `76` or `77` require schema changes, update the generated report families coherently rather than one field at a time.
    - Keep cohort, health, comparison, and release-facing docs aligned in one
      governed summary-semantics pass.
    - Avoid introducing parallel compatibility shims unless they are strictly
      necessary for repository consumers.

79. Tighten `release_metadata.py truth-check` to enforce the post-closure summary semantics chosen in Tasks `76` through `78`.
    - The release gate should fail if release-facing docs or generated reports
      drift back to semantically confusing post-closure summary states.

80. Reassess the draft `v1.0.0` release narrative once the summary-semantics pass lands.
    - Make sure RC readiness and release notes describe:
      - full compared cohort
      - full harvest coverage
      - zero retained backlog
      - post-closure summary meanings
    - Keep this as release-truth maintenance, not marketing embellishment.

81. Audit `make release-generated-check` end to end in the fully closed evidence state.
    - Confirm whether the historical ordering hazards and incidental EOF churn
      are still present.
    - Treat this as process validation, not as permission to edit unrelated
      generated artefacts casually.

82. If the release-generated path still has ordering hazards, fix them as deterministic release-process debt.
    - Prefer stable generation order and clean-tree invariants over manual
      restoration habits.
    - Do not leave “known but tolerated” post-closure churn undocumented.

83. Reassess the generated report set for redundancy after summary cleanup and release-generated stabilization.
    - Identify whether any pair of generated artefacts now express the same
      truth with no distinct governance or release-gate value.
    - Consolidate only if the truth surface becomes clearer, not merely
      smaller.

84. If a consolidation candidate exists, perform a governed report-surface simplification pass.
    - Keep links, docs index membership, and release-truth checks aligned.
    - Preserve unique governance and release meanings even if two reports look
      superficially similar.

85. Reconfirm that plotting remains the first actual `Rework` implementation-program candidate after Tasks `76` through `84`.
    - The confirmation should explicitly test whether any summary/process
      cleanup changed the ordering rationale materially.

86. If plotting still remains first, generate a dedicated plotting rework sub-roadmap with bounded initial scope.
    - The sub-roadmap should define:
      - first method slice
      - contract/evidence expectations
      - R handoff boundaries
      - release and migration risks
    - Do not begin broad plotting implementation before this exists.

87. If plotting is blocked during sub-roadmap generation, generate the equivalent remote-retrieval rework sub-roadmap instead.
    - Preserve the already documented fallback ordering.
    - Require the same bounded entry criteria and evidence framing.

88. Reassess whether protein-property rework remains the third candidate after the first-program sub-roadmap exists.
    - This remains a planning checkpoint, not a silent promotion.

89. Capture the acceptance criteria for the first approved `Rework` program as governed roadmap truth before code changes begin.
    - Entry and exit criteria should be explicit enough that future work can be
      judged against them without relying on conversational memory.

90. Extend this roadmap again after Task `89`.
    - Continue to derive priorities from:
      - cohort validation
      - governance alignment
      - cohort health
      - comparison coverage
      - retained backlog closure
      - full compared cohort
      - harvest coverage
      - any post-closure summary-semantics outputs
      - any stabilized release-generated findings
    - Preserve the same repository rules:
      - honest documentation
      - scoped changes
      - commit-and-push completion
      - no unrelated code churn
