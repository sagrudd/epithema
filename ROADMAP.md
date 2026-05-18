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
