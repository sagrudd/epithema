# Epithema Tool-Family Governance Policy

**Project:** Epithema reboot  
**Sponsor:** Mnemosyne Biosciences Ltd  
**Date:** 2026-04-15  
**Status:** Governance policy derived from the raw scope matrix

Canonical governance context:
[Epithema Governance Manual](../epithema_governance_manual.md)

---

## 1. Purpose

This document converts the per-tool EMBOSS scope matrix into a governance policy that can be applied consistently across the `epithema` and `epithemaR` projects.

It defines named scope buckets, the criteria for placing a tool family into each bucket, default family assignments, exception handling, and promotion rules for future releases.

This policy is intended to answer four practical questions:

1. What must be preserved as part of the EMBOSS reboot?
2. What should be preserved only after substantial redesign?
3. What should be deferred without being declared obsolete?
4. What should be excluded as legacy implementation baggage rather than user value?

A fifth bucket, **Strategic Add**, is included because some high-value capability areas should be introduced even where they were absent, weakly implemented, or wrapper-driven in historic EMBOSS.

---

## 2. Decision Buckets

### 2.1 Core Retain

**Definition**

Capabilities that remain directly useful to day-to-day bioinformatics work and can be carried forward as first-class parts of `epithema` without requiring conceptual reinvention.

**Typical characteristics**

- clear continuing user value
- computationally bounded and well understood
- not dependent on obsolete server or database plumbing
- not primarily a wrapper around external legacy infrastructure
- can be modernized internally without changing what the tool fundamentally is

**Default implementation expectation**

- included in the reboot roadmap
- exposed as ordinary commands in `epithema`
- exposed through the R package over time
- covered by fixture-driven validation

---

### 2.2 Modernize / Rework

**Definition**

Capabilities where the user need clearly remains, but the historic EMBOSS implementation model should not be preserved because the field, data sources, algorithms, UX expectations, or rendering model have changed substantially.

**Typical characteristics**

- strong user need remains
- original implementation reflects old assumptions or obsolete dependencies
- modern providers, APIs, archives, databases, or methods exist
- substantial value can be added by redesigning the capability

**Default implementation expectation**

- capability remains in scope
- command identity may change internally even if the conceptual function remains recognizable
- algorithms, data source integrations, and outputs may be modernized aggressively
- comparison with legacy EMBOSS is still required where meaningful

---

### 2.3 Defer

**Definition**

Capabilities that are not core to the initial reboot, but are not judged obsolete or permanently undesirable. These are postponed rather than rejected.

**Typical characteristics**

- lower immediate user demand
- meaningful maintenance cost relative to near-term value
- niche or metadata-heavy workflows
- useful mainly in specialized environments
- better added after the core platform, validation corpus, and user surfaces are stable

**Default implementation expectation**

- not part of the initial delivery commitment
- architecture should avoid blocking later reintroduction
- may remain represented internally in documentation, metadata, or roadmap notes

---

### 2.4 Exclude Permanently

**Definition**

Commands or families whose main purpose was to support the historic EMBOSS implementation architecture rather than enduring scientific workflows.

**Typical characteristics**

- ACD-specific developer plumbing
- server/cache/registry/index administration for EMBOSS-era remote infrastructure
- command surfaces that exist only to expose obsolete transport or wrapper internals
- little independent scientific value
- no credible user demand beyond compatibility nostalgia

**Default implementation expectation**

- not carried forward as user-facing commands
- not part of the reboot commitment
- replacement is allowed only if a genuine user problem reappears in a modern form

---

### 2.5 Strategic Add

**Definition**

Capabilities that should be introduced or rebooted because they address clear current user need, even if the historic EMBOSS implementation was missing, weak, wrapper-driven, or technically outdated.

**Typical characteristics**

- strong modern utility
- high leverage for scientific workflows
- aligns with the reboot’s modernization goals
- naturally supported by the shared Rust service layer and the first-class R surface

**Default implementation expectation**

- explicitly planned as part of the modern platform
- not blocked by historic command compatibility concerns
- validated against external reference behavior, datasets, or contemporary tools where legacy EMBOSS comparisons are weak or absent

---

## 3. Global Policy Rules

### 3.1 Concept compatibility overrides CLI compatibility

Tools are classified according to the biological or workflow need they satisfy, not according to whether the historical CLI should be preserved.

### 3.2 A user need outranks a legacy implementation

If the user need persists but the historic plumbing is obsolete, the family belongs in **Modernize / Rework**, not **Exclude Permanently**.

### 3.3 Historic implementation baggage is not protected

ACD machinery, server registries, remote-cache builders, and legacy database administration commands are not retained merely because they existed in EMBOSS.

### 3.4 Plot-producing tools are not automatically deprecated

If a tool’s graphical output is still useful, the capability remains in scope, but rendering must move to the R package. That places most such tools in **Modernize / Rework**, not **Exclude Permanently**.

### 3.5 Deferred is not obsolete

A deferred family is deliberately left out of the initial delivery but remains eligible for later promotion if user demand or integration value justifies it.

### 3.6 Sponsor and product priorities may override family defaults

Mnemosyne Biosciences Ltd product requirements, validation obligations, or platform integration needs may elevate specific tools even if the broader family default would suggest another bucket.

### 3.7 Explicit exception: `complex`

`complex` is designated **Core Retain** irrespective of broader family heuristics.

---

## 4. Family-Level Bucket Assignments

The following assignments are the default policy for tool families. Individual tools may be overridden by explicit decision.

| Tool family | Default bucket | Policy |
|---|---|---|
| Basic sequence IO and conversion | Core Retain | Keep as foundational capability: read, write, extract, reformat, select, split, join, reverse, translate, back-translate, and related transformations. |
| Sequence editing and manipulation | Core Retain | Preserve simple, direct sequence editing and transformation utilities as first-class commands. |
| Alignment read/write and post-processing | Core Retain | Keep alignment transformation, consensus, copy, and extraction operations. |
| Core sequence statistics and composition | Core Retain | Preserve established descriptive analyses such as composition, codon usage, peptide statistics, and related summary measures. |
| Simple motif, pattern, and regular-expression search | Core Retain | Keep lightweight, bounded search capabilities that remain broadly useful. |
| ORF and translation-adjacent utilities | Core Retain | Keep practical sequence interpretation utilities where the biological need remains stable. |
| Restriction-enzyme design and analysis | Modernize / Rework | Keep the user need, but modernize databases, reporting, and visualization; avoid old wrapper and data-prep assumptions. |
| Primer and assay-oriented search | Modernize / Rework | Preserve the problem domain, but redesign around current expectations for primer validation, reporting, and scale. |
| Plotting and visualization tools | Modernize / Rework | Retain graphical capability where useful, but move rendering to `epithemaR`; Rust emits plot-ready data only. |
| Remote retrieval and archive acquisition | Modernize / Rework | Retain accession-driven biological retrieval, but replace EMBOSS-era server/database plumbing with provider integrations such as ENA and SRA. |
| External database preparation helpers | Modernize / Rework | Keep only where an enduring user need exists; redesign around modern resource preparation rather than EMBOSS-specific preprocessors. |
| Legacy prediction methods with enduring scientific value | Modernize / Rework | Preserve domains such as motif/profile scanning or region prediction where user need remains, but upgrade algorithms and reference data aggressively. |
| Protein property and structural-summary utilities | Modernize / Rework | Keep where still useful, but modernize methods, outputs, and any graphics. |
| Command discovery and help/navigation | Modernize / Rework | Replace historic per-command discovery helpers with modern unified `epithema` discovery, docs, and search. |
| HMM and probabilistic homology workflows | Strategic Add | Reintroduce as a modern capability domain rather than preserving old wrapper semantics. |
| Modern archive-scale raw data ingestion | Strategic Add | Add ENA/SRA run and study acquisition, including NGS-oriented workflows absent from the original EMBOSS worldview. |
| Ontology command group | Defer | Omit as a user-facing group initially, but do not classify ontology support itself as obsolete. |
| Specialized metadata and semantic lookup utilities | Defer | Defer unless demanded by downstream API, documentation, or interoperability needs. |
| ACD developer tooling | Exclude Permanently | Omit all commands whose purpose is to validate, trace, format, or inspect ACD files. |
| EMBOSS-era server/cache/registry plumbing | Exclude Permanently | Omit cache generators, server interrogators, and remote registry builders tied to obsolete infrastructure. |
| EMBOSS local database indexing administration | Exclude Permanently | Omit index-construction and database-admin commands whose value was mainly to support the old EMBOSS database layer. |
| Wrapper-only compatibility commands | Exclude Permanently | Omit commands that existed only to expose external tool quirks through EMBOSS conventions. |

---

## 5. Family Notes and Rationale

### 5.1 Core Retain families

These families form the identity backbone of the reboot. They represent the broad, useful, everyday “toolbox” character that made EMBOSS valuable in the first place.

Representative examples include:

- sequence conversion and extraction
- translation and back-translation
- sequence copying, slicing, and editing
- alignment copying and consensus operations
- codon usage and composition summaries
- peptide and nucleotide statistics
- regular-expression and motif search
- direct utility tools such as `complex`

These tools should generally remain lightweight, scriptable, and easy to compose.

### 5.2 Modernize / Rework families

These families should remain part of the scientific product, but only after a strong redesign.

The major rule here is:

retain the workflow problem, discard the historic implementation assumptions.

Representative examples include:

- graphical plots now rendered through R
- single-sequence and archive retrieval now routed through contemporary providers
- profile and motif workflows updated to modern resources
- restriction, primer, and assay utilities with refreshed data sources and reporting
- prediction-oriented methods where the scientific need remains but old algorithms are no longer sufficient

### 5.3 Defer families

The purpose of this bucket is scope discipline.

A capability should land here when it is plausible, potentially useful, and architecturally compatible, but not essential to proving the reboot.

The ontology group is the clearest current example:

- not central to the primary analytical reboot
- still relevant to metadata and interoperability in principle
- better deferred than declared obsolete

### 5.4 Exclude Permanently families

These families exist mainly because EMBOSS had a very specific internal architecture involving ACD, remote server registries, local indexing helpers, and assorted infrastructure commands.

Those commands do not define EMBOSS’s scientific value. They define its implementation history.

That history should not constrain the reboot.

### 5.5 Strategic Add families

The reboot should be better than legacy EMBOSS where current bioinformatics practice clearly demands it.

The strongest examples today are:

- modern ENA/SRA data acquisition, including NGS-oriented download workflows
- modern HMM capability as a rebooted domain rather than an old wrapper layer

This bucket exists so the project is not trapped into only recovering the past.

#### 5.5.1 Milestone: Governed NGS dataset acquisition

Epithema should add a governed NGS dataset acquisition milestone for public ENA
and SRA datasets. This milestone is a Strategic Add, not part of the
coordinated `1.0.0` release scope.

The milestone introduces two user-facing methods:

- `ngslist <accession>` lists FASTQ, BAM/CRAM, FAST5, POD5, SRA, and other
  provider-reported dataset assets associated with a study, sample, experiment,
  or run accession.
- `ngsget <accession>` materializes the selected assets, downloading generated
  FASTQ by default and including raw/submitted data only when requested with an
  explicit `--raw` flag.

The milestone must extend the existing governed archive-provider seam rather
than adding ad hoc downloader logic. It must normalize study, sample,
experiment, and run queries to a run-level manifest; preserve ENA and SRA
provider provenance; verify downloaded assets when checksums or byte counts are
available; and write a standard provenance JSON document suitable for later
object-store ingestion.

SRA-specific implementation may use the SRA Toolkit execution model where that
is the scientifically appropriate route. In that case Epithema should treat the
toolkit invocation as a governed conversion step: the downloaded SRA archive,
the extracted FASTQ files, the tool version, the container image when used, and
all verification outcomes must be recorded in provenance.

---

## 6. Promotion and Demotion Rules

### 6.1 Promotion from Defer to Modernize / Rework

A deferred family should be promoted when one or more of the following become true:

- repeated user demand is demonstrated
- it materially improves downstream API or R usability
- it meaningfully improves scientific interoperability or metadata quality
- the implementation cost falls because enabling infrastructure already exists

### 6.2 Promotion from Modernize / Rework to Core Retain

A reworked family may be treated operationally as core once:

- the redesign has stabilized
- validation coverage is in place
- user workflows are clearly established
- maintenance burden is predictable

### 6.3 Demotion from Modernize / Rework to Defer

A family may be demoted temporarily if:

- the implementation cost is too high relative to current program goals
- enabling data sources are unstable or poor quality
- validation evidence is too weak to make a defensible release commitment

### 6.4 Demotion to Exclude Permanently

This should be rare and should require explicit documentation. It is justified only when:

- the user need is effectively gone
- the tool exists only because of historic EMBOSS infrastructure
- a better modern solution exists elsewhere and there is no product reason to duplicate it
- the capability would create disproportionate maintenance cost for little scientific value

---

## 7. Validation Expectations by Bucket

### 7.1 Core Retain

Validation must include:

- legacy EMBOSS comparison where applicable
- fixture-driven reproducibility checks
- documentation examples
- R exposure over time where appropriate

### 7.2 Modernize / Rework

Validation must include:

- legacy comparison where meaningful
- explicit divergence notes where algorithms or providers have changed
- modern dataset tests
- acceptance evidence showing why the redesign is scientifically superior or at least equivalent in practical use

### 7.3 Defer

Validation is not required for initial release, but the architecture should make later validation feasible.

### 7.4 Exclude Permanently

No user-facing validation obligation. Only rationale documentation is required.

### 7.5 Strategic Add

Validation must rely on modern reference behavior, provider expectations, or contemporary gold-standard tooling when legacy EMBOSS comparisons are absent or not relevant.

---

## 8. Governance Workflow

For each tool family or individual tool considered during planning, the maintainers should record:

1. bucket assignment
2. short rationale
3. whether there is a historical EMBOSS comparison target
4. whether there is a modern external comparison target
5. whether R exposure is required
6. whether plotting is involved
7. whether the decision is a default family assignment or an explicit override

All overrides should be recorded in the planning registry.

---

## 9. Immediate Policy Decisions for Epithema

The following immediate decisions are established under this policy:

- `complex` is Core Retain
- ontology command groups are Defer
- EMBOSS-era remote server, cache, and registry plumbing is Exclude Permanently
- ACD developer tooling is Exclude Permanently
- sequence and archive retrieval is Modernize / Rework
- plot-producing capabilities are Modernize / Rework, with rendering in `epithemaR`
- modern ENA/SRA raw-data ingestion is Strategic Add
- HMM capability is Strategic Add as a rebooted modern domain

---

## 10. Practical Interpretation

This policy means the reboot should preserve EMBOSS as a broad scientific toolbox, but not as a museum of its internal architecture.

What users valued most should remain.
What users still need but now need differently should be reworked.
What is merely old plumbing should be left behind.
What modern bioinformatics now requires should be added deliberately.

That is the governance boundary for the Epithema reboot.
