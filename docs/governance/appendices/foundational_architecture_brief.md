# Epithema Foundational Architecture Brief

Sponsored work for Mnemosyne Biosciences Ltd

Status: supporting architecture appendix

Date: 2026-04-15

Primary repositories:

- `epithema`
- `epithemaR`

Canonical governance context:
[Epithema Governance Manual](../epithema_governance_manual.md)


1. EXECUTIVE SUMMARY

Epithema is a full-platform reboot of the historic EMBOSS collection of bioinformatics tools. This work is sponsored by Mnemosyne Biosciences Ltd.

The reboot is concept-compatible with EMBOSS rather than strictly CLI-compatible. The objective is to preserve and modernize the scientific utility, breadth, and reputation of EMBOSS while replacing its legacy implementation model with a contemporary Rust architecture, a first-class R interface, and a modern validation and documentation pipeline.

The platform has two first-class user surfaces:

1) Rust / CLI surface via the single binary:
   epithema

2) R surface via the sister package:
   epithemaR

The R package is not merely a plotting adapter. It has two equal responsibilities:

1) it is the exclusive plotting backend for graphical tools
2) it exposes the full set of epithema methods to R users through Rust integration

The system must support the historic breadth of EMBOSS, including some historically fringe or non-production tools, while allowing permanent omission of tools that are biologically obsolete, redundant, or not defensible to maintain.

The defining acceptance criterion is historical scientific reproducibility through executable documentation. Historic EMBOSS vignettes, example runs, datasets, and expected outputs must be harvested into an automated validation and documentation framework that compares legacy EMBOSS and Epithema behavior on canonical datasets. This is a critical requirement and a central proof obligation of the project.


2. PROJECT MANDATE

Epithema must satisfy all of the following:

- reimplement EMBOSS as a modern Rust platform
- expose all tool functionality behind a single binary:
  epithema <tool> ...
- avoid standalone legacy tool executables such as needle or water
- treat R as a first-class citizen
- use R as the only plotting backend
- establish a shared execution and service layer so the CLI, R, and future API all execute against the same scientific core
- prioritize classic bioinformatics input and output formats
- aggressively exploit parallel execution where appropriate
- scaffold the full tool catalog early, even where some implementations arrive incrementally
- make all implemented tools visible as ordinary commands
- support Linux first, with containers as the supported path on macOS and Windows
- validate the system using historic EMBOSS examples, outputs, and vignettes
- use the resulting validation corpus as the basis for auto-documentation and acceptance reporting


3. DECISION RECORD FROM REQUIREMENTS DISCOVERY

The following decisions are established:

1) Compatibility target
   Concept-compatible only. The reboot preserves functional intent and domain capability rather than historical CLI behavior or output formatting quirks.

2) First release strategy
   Full catalog scaffolding first. The registry, command model, service layer, and documentation framework should cover the full intended catalog early, even where implementations are partial.

3) Tool definition strategy
   Rust code only. Tool definitions, validation, defaults, help text, and execution wiring live in Rust source rather than external metadata files.

4) Algorithm strategy
   Aggressive modernization is allowed, provided practical analytical results remain sound and acceptance evidence is preserved.

5) Naming strategy
   Historic EMBOSS tool identities are preserved conceptually, but all tools are exposed behind the epithema binary.

6) CLI strategy
   Single binary only. No standalone executable shims such as needle, water, or seqret.

7) Plot strategy
   R is the only plotting backend.

8) Architecture strategy
   Shared service layer from the outset so CLI and future API are peers.

9) Output strategy
   Priority goes to classic bioinformatics formats rather than structured JSON-first outputs.

10) Catalog coverage strategy
    Target near-total recovery of EMBOSS except for genuine dead ends.

11) Dependency strategy
    Pragmatic use of the Rust ecosystem.

12) Performance strategy
    Aggressive parallelism from the outset.

13) Visibility strategy
    Any implemented tool is visible as a normal command.

14) Deployment strategy
    Linux first, with Linux containers used to support macOS and Windows users.

15) Acceptance strategy
    Legacy EMBOSS vignettes, example runs, and example outputs are mandatory acceptance artifacts. Legacy EMBOSS and Epithema must be compared on those datasets. This corpus must also drive auto-documentation.

16) R platform strategy
    R is a first-class user interface and integration target, not just a rendering helper. The sister repository epithemaR will expose the full method catalog to R users through Rust integration.


4. SPONSORSHIP CONTEXT

This reboot is sponsored by Mnemosyne Biosciences Ltd and is intended to support use of EMBOSS-derived capability within the Mneion software ecosystem while also re-establishing EMBOSS as a modern, maintainable, extensible computational platform.

This architecture brief therefore serves both product and engineering purposes:

- recovery and modernization of the EMBOSS computational estate
- long-term maintainability and extension
- first-class CLI and R user access
- future API enablement
- defensible scientific validation and auditability
- integration readiness for downstream Mnemosyne Biosciences Ltd systems


5. SCOPE

5.1 In scope

- full-platform reboot of EMBOSS in Rust
- companion R package for plotting and full method access
- unified command runtime behind epithema
- internal service layer for future API exposure
- legacy vignette ingestion and automated example reproduction
- broad catalog scaffolding across nearly all EMBOSS tools
- performance-oriented parallel execution model
- Linux-native builds and Linux container delivery

5.2 Out of scope

- strict compatibility with historical EMBOSS CLI behavior
- preservation of ACD as the primary interface definition system
- native macOS-first or Windows-first delivery
- secondary plotting backends in Rust or Python
- guarantee that every historic EMBOSS tool will survive unchanged
- forcing the R interface to mimic CLI syntax
- JSON-first redesign of all tool outputs in the first delivery phase


6. DESIGN PRINCIPLES

6.1 Concept over legacy syntax

The reboot preserves the scientific and functional intent of EMBOSS, not the historical command-line quirks.

6.2 One computational core, multiple front ends

Scientific logic must not be duplicated across CLI, R, and future API surfaces. All front ends dispatch through the same shared core.

6.3 R as a peer, not a wrapper

The epithemaR package must expose the full method set to R users and own the rendering of graphical outputs. It is a primary client surface.

6.4 Scientific contract through examples

Each tool should be defined not only by its code, but also by its example datasets, canonical invocations, expected outputs, comparison rules, and generated documentation.

6.5 Performance is a product feature

Parallelism and throughput are part of the value proposition, not late-stage optimization work.

6.6 Rust-native definitions

Tool definitions, validation rules, help text, and execution wiring must live in Rust source rather than in a replacement metadata language.

6.7 Traceability

Every significant tool implementation should be traceable to legacy EMBOSS behavior, documentation, or justification for divergence.

6.8 Non-fatal rendering dependency

Lack of R installation must not cause fatal failure for non-graphical workflows. Graphical workflows must fail gracefully with clear guidance.


7. PRODUCT SURFACES

7.1 CLI surface

Canonical invocation model:

  epithema <tool> [arguments...]

Examples:

  epithema needle ...
  epithema water ...
  epithema seqret ...
  epithema complex ...

Rules:

- epithema is the only executable surface
- no standalone command shims
- all implemented tools are listed as normal commands
- command discovery and help are provided centrally
- user experience should be modern, consistent, and scriptable

7.2 R surface

Canonical model:

- idiomatic R functions
- Rust-backed computation
- R-native plot creation
- no requirement to preserve CLI semantics

Intended characteristics:

- friendly R function signatures
- support for vectors and data frames where biologically appropriate
- clear error propagation
- plot objects and rendered graphics native to R workflows
- complete exposure of the method catalog over time

7.3 Future API surface

The architecture must allow an upcoming API sprint to expose the method catalog programmatically without re-architecting the scientific core. The future API is not in scope for immediate delivery, but readiness for it is mandatory.


8. REPOSITORY TOPOLOGY

8.1 epithema

Primary Rust repository.

Responsibilities:

- scientific core algorithms
- tool registry
- CLI
- shared execution and service layer
- format parsing and writing
- validation harness
- benchmark harness
- documentation generation tooling
- R-facing Rust bridge

8.2 epithemaR

Primary R repository.

Responsibilities:

- plotting and rendering
- R user interface
- Rust integration into R
- R-native wrappers over the method catalog
- R-side documentation and vignettes where needed

8.3 Relationship between repositories

A submodule relationship or equivalent coupling strategy may be introduced by CODEX. This is an implementation detail rather than a product-level requirement, provided that:

- compatibility between versions is explicit
- builds are reproducible
- interfaces are stable and testable
- the separation of responsibilities remains clear


9. LOGICAL ARCHITECTURE

The platform consists of the following layers:

1) user interfaces
   - CLI via epithema
   - R via epithemaR

2) shared service and execution layer
   - tool dispatch
   - parameter normalization
   - resource management
   - concurrency control
   - diagnostics
   - front-end-agnostic invocation model

3) scientific core
   - sequence algorithms
   - alignment
   - motifs
   - statistics
   - transforms
   - analysis kernels

4) IO and format layer
   - FASTA
   - FASTQ
   - EMBL
   - GenBank
   - GFF
   - Newick
   - Stockholm
   - associated domain formats

5) plot contract layer
   - plot-ready typed objects emitted by Rust tools

6) R rendering backend
   - rendering implementation in epithemaR


10. RECOMMENDED RUST WORKSPACE STRUCTURE

The exact crate layout may evolve, but the following separation of concerns is recommended:

epithema/
  Cargo.toml
  Makefile
  crates/
    epithema-core/
    epithema-io/
    epithema-tools/
    epithema-service/
    epithema-cli/
    epithema-plot-contract/
    epithema-r-bridge/
    epithema-testkit/
    epithema-docgen/
    epithema-fixtures/

10.1 epithema-core

Contains shared scientific primitives and domain abstractions, including:

- sequence models
- alphabets
- scoring schemes
- alignment kernels
- motif primitives
- coordinate logic
- statistical helpers
- biology-focused utility abstractions

No CLI-specific logic. No R-specific logic.

10.2 epithema-io

Contains classic bioinformatics format parsing and writing, including:

- FASTA
- FASTQ
- EMBL
- GenBank
- GFF
- Newick
- Stockholm
- tabular domain formats
- stream-oriented IO
- compressed input support where appropriate

10.3 epithema-tools

Contains the tool registry and tool implementations.

Responsibilities:

- one Rust module per tool or tool family
- typed argument definitions
- per-tool validation
- execution wiring into the service layer
- help text and examples in Rust source
- optional plot contract emission

This crate replaces the historical ACD-driven application definition model.

10.4 epithema-service

Contains the shared execution runtime.

Responsibilities:

- dispatch orchestration
- execution context
- parameter normalization
- concurrency management
- resource management
- cancellation and timeout support where needed
- shared diagnostics and error models
- front-end-agnostic invocation interface

10.5 epithema-cli

Contains the epithema binary.

Responsibilities:

- top-level argument capture
- help generation
- terminal-facing error presentation
- exit code behavior
- dispatch into epithema-service

Scientific logic must not live here.

10.6 epithema-plot-contract

Contains plot-ready data structures and metadata emitted by scientific tools for R rendering.

Responsibilities:

- plot payload definitions
- axis and label metadata
- legend and annotation structures
- tool-specific plot schemas
- serialization or FFI-safe transport models as needed

No rendering implementation lives here.

10.7 epithema-r-bridge

Contains the Rust side of the R integration.

Responsibilities:

- exported Rust entry points callable from R
- conversion between Rust and R types
- bridge-level stability layer
- vectorization-aware interfaces where appropriate
- efficient movement of large scientific data

10.8 epithema-testkit

Contains the acceptance and regression harness.

Responsibilities:

- execution of legacy EMBOSS
- execution of Epithema
- output comparison utilities
- tolerance policies for intentional divergence
- performance measurement harnesses
- fixture orchestration

10.9 epithema-docgen

Contains the executable documentation system.

Responsibilities:

- ingestion of vignette and example definitions
- generation of run specifications
- rendering of documentation pages and reports
- embedding of inputs, commands, outputs, plots, and benchmark summaries

10.10 epithema-fixtures

Contains curated datasets and comparison assets.

Responsibilities:

- legacy example datasets
- expected outputs
- edge case corpora
- performance corpora
- documentation sample assets


11. TOOL MODEL

11.1 Tool definition strategy

Tool definitions live directly in Rust. Each tool should include:

- canonical name
- brief summary
- parameter definitions
- input requirements
- output contract
- validation rules
- execution binding
- example metadata
- optional plotting metadata
- acceptance-test references

11.2 Tool registry

A central registry must support:

- CLI discovery
- help generation
- R wrapper generation or exposure
- future API reflection
- validation coverage tracking
- documentation generation

11.3 Catalog coverage policy

The target is near-total historical coverage, including fringe or historically non-production tools, except for clear dead ends such as:

- biologically obsolete tools
- redundant tools with no practical recovery value
- tools whose maintenance burden cannot be justified

Any omission should be explicit and documented.

11.4 Tool implementation states

The first release may include different depths of implementation, but the catalog should be scaffolded broadly. Where a tool is present and executable, it should appear as a normal command.


12. EXECUTION MODEL

12.1 Shared invocation contract

All front ends must call into a shared service interface. The precise internal Rust API may evolve, but conceptually each tool must be invocable through the same service layer.

12.2 Concurrency model

Parallelism is a first-order platform requirement.

The service layer should provide:

- shared task scheduling or equivalent execution primitives
- centralized concurrency controls
- predictable resource usage
- support for streaming and chunked workloads
- tool-level opt-in or policy-driven parallel execution
- scalable support for batch operations

12.3 Likely early parallel targets

- batch sequence transforms
- scanning and search workloads
- motif scanning
- alignment farms
- large-format conversions
- statistical routines over many records

12.4 Determinism

Parallel execution must not undermine reproducibility. Policies are required for:

- stable output ordering where semantically meaningful
- numeric tolerance handling
- seed management for stochastic components if any
- explicit documentation of any nondeterministic modes


13. INPUT AND OUTPUT FORMAT STRATEGY

13.1 Priority

The platform prioritizes classic bioinformatics formats over JSON-first output contracts.

Primary targets include:

- FASTA
- FASTQ
- EMBL
- GenBank
- GFF
- Newick
- Stockholm
- common tabular biology outputs

13.2 Internal type safety

Although JSON is not a first-release output priority, internal representations should remain strongly typed to support future API exposure cleanly.

13.3 Streaming and scale

The IO layer should support:

- streaming reads and writes where possible
- memory-aware handling of large datasets
- conventional stdin and stdout behavior where appropriate
- compression support where practical


14. R INTEGRATION ARCHITECTURE

14.1 Role of epithemaR

epithemaR has two equal responsibilities:

1) render all graphical outputs
2) expose the complete Epithema method surface to R users

14.2 Binding philosophy

The R API does not need CLI parity. It should be idiomatic for R users and should take advantage of R conventions where appropriate.

Design goals:

- natural function signatures
- support for vectorized use where sensible
- native R object integration
- data frame friendly interfaces where helpful
- clear warnings and errors
- plot outputs native to R workflows

14.3 Bridge requirements

The Rust to R bridge must support:

- robust type conversion
- efficient movement of large data structures
- stable version contracts between epithema and epithemaR
- clear propagation of domain errors into R
- optional batching and concurrency-friendly entry points

14.4 Plotting contract

Tools that produce plots must emit plot-ready structures from Rust. The R package consumes those structures and renders them.

This preserves:

- one scientific implementation of each analysis
- one rendering implementation
- clean separation between computation and presentation

14.5 Failure semantics around R availability

For CLI usage:

- lack of R must not be fatal for non-graphical workflows
- if a plotting workflow is requested and R is unavailable, the user must receive:
  - a clear warning
  - a statement that plotting could not be performed
  - instructions for installing the required R components

For R usage:

- the Rust bridge must fail clearly if the Rust components are missing, incompatible, or not correctly installed


15. CLI ARCHITECTURE

15.1 Canonical command shape

  epithema <tool> [tool-arguments...]

15.2 Administrative commands

Additional top-level administrative commands may be introduced, such as:

  epithema help
  epithema list
  epithema version
  epithema doc <tool>
  epithema example <tool>
  epithema validate <tool>

These are platform-level conveniences and are not attempts to reproduce historical EMBOSS behavior.

15.3 Responsibilities of the CLI

- argument capture
- help and usage presentation
- terminal-oriented diagnostics
- exit status behavior
- dispatch into the service layer

Scientific logic must remain outside the CLI layer.


16. VALIDATION AND ACCEPTANCE STRATEGY

16.1 Core principle

The only convincing proof that Epithema is correct is that it reproduces the intended scientific behavior demonstrated by historic EMBOSS tools on their published example datasets and example outputs, while clearly documenting any justified differences.

16.2 Mandatory acceptance inputs

Acceptance material must be constructed from:

- historic EMBOSS vignettes
- example runs
- example datasets
- expected outputs
- tool help and usage examples where relevant

16.3 Validation modes

A) Legacy comparison
   Run original EMBOSS and compare against Epithema on canonical examples.

B) Scientific validity
   Confirm biological correctness on trusted datasets, including cases where Epithema intentionally improves on the original implementation.

C) Performance comparison
   Measure legacy EMBOSS and Epithema on selected datasets for speed and scalability.

D) Documentation generation
   Use the same executions to generate user-facing documentation and acceptance reports.

16.4 Per-tool validation contract

Each tool should ultimately have:

- fixture set
- historical example mapping
- canonical epithema invocation
- expected output or explicit comparison rule
- comparison against legacy EMBOSS where possible
- benchmark scenario
- generated documentation artifact

16.5 Comparison policies

Not all outputs compare by simple byte equality. The framework must support:

- exact comparison
- normalized text comparison
- sorted comparison where order is semantically irrelevant
- numeric tolerance comparison
- structural comparison for graphical or summary outputs
- explicit divergence annotations and justifications

16.6 R validation

For tools exposed in epithemaR, the validation framework should eventually compare:

- legacy EMBOSS vs epithema CLI
- epithema CLI vs shared Rust service layer
- shared Rust service layer vs epithemaR function calls
- plot payload vs rendered plot expectations where appropriate

16.7 Acceptance reporting

The acceptance system must produce reports that clearly demonstrate:

- what historical example was used
- how EMBOSS was run
- how Epithema was run
- whether results matched, diverged acceptably, or failed
- any benchmark outcomes
- links to generated documentation artifacts

This reporting is central to demonstrating correctness.


17. DOCUMENTATION STRATEGY

17.1 Executable documentation

Documentation must be generated from real runs, not hand-written illustrative commands alone.

Each documented tool page should ideally include:

- tool purpose
- parameter summary
- example datasets
- canonical epithema command
- resulting outputs
- performance notes where available
- plot outputs where relevant
- comparison notes against legacy EMBOSS
- R usage example where applicable

17.2 Documentation sources

Documentation should be assembled from:

- Rust tool definitions
- fixture metadata
- validation outputs
- vignette mappings
- R examples where relevant

17.3 Documentation goals

- serve users
- serve maintainers
- serve scientific auditability
- serve acceptance proof for sponsored work


18. DEPENDENCY AND THIRD-PARTY POLICY

18.1 Rust dependency stance

Use the Rust ecosystem pragmatically where it materially improves correctness, performance, or maintainability.

18.2 Preference order

- well-maintained crates
- clear licensing
- active maintenance
- strong test culture
- compatibility with long-term project stability

18.3 Risk management

Critical dependencies should be pinned and reviewed. Where an upstream crate becomes unstable or unsuitable, replacement planning should be feasible without architectural disruption.


19. BUILD, PACKAGING, AND DELIVERY

19.1 Build system

The implementation should be entirely Rust-based with a Makefile coordinating common developer and CI tasks.

Likely build responsibilities:

- Rust workspace build
- validation runs
- documentation generation
- benchmark execution
- packaging steps
- container image build

19.2 Primary supported runtime target

Linux is the primary supported native runtime target.

19.3 Secondary platform strategy

macOS and Windows users are supported through Linux container delivery rather than native first-class builds in the initial strategy.

19.4 Packaging goals

- reproducible builds
- CI-generated artifacts
- container images for end users and integration environments
- clear version pairing between epithema and epithemaR


20. NON-FUNCTIONAL REQUIREMENTS

20.1 Maintainability

The codebase must support long-term maintenance and extension across a broad tool catalog.

20.2 Performance

The platform must materially improve on legacy EMBOSS performance where possible and demonstrate that through benchmark evidence.

20.3 Reliability

Errors must be clear, bounded, and testable. Data corruption and silent misbehavior are unacceptable.

20.4 Reproducibility

Runs used for validation and documentation must be reproducible.

20.5 Observability

The system should support diagnostics sufficient for debugging tool failures, validation failures, and integration issues.

20.6 Extensibility

The architecture should make it straightforward to add new tools, new acceptance fixtures, and future API exposure.


21. RISKS AND MITIGATIONS

21.1 Risk
The EMBOSS catalog is large and uneven in quality and usage.

Mitigation
Adopt full catalog scaffolding with explicit prioritization, tool-by-tool acceptance mapping, and documented omission criteria for genuine dead ends.

21.2 Risk
Legacy documentation and example outputs may be incomplete or inconsistent for some tools.

Mitigation
Preserve all recoverable example material, supplement with curated reference datasets, and record justification whenever historical evidence is ambiguous.

21.3 Risk
R bridge design may become brittle if treated as an afterthought.

Mitigation
Design the bridge and plot contract as first-class architecture from the beginning.

21.4 Risk
Aggressive parallelization may compromise reproducibility.

Mitigation
Build deterministic policies into the execution layer and define explicit comparison tolerances.

21.5 Risk
The platform may drift away from EMBOSS scientific intent under aggressive modernization.

Mitigation
Ground acceptance in historical examples and explicit divergence documentation.

21.6 Risk
Broad scope may delay user-visible wins.

Mitigation
Separate catalog scaffolding from implementation depth, and use the executable documentation framework to show incremental progress credibly.


22. GOVERNANCE IMPLICATIONS

This architecture implies the need for explicit governance over:

- tool inclusion and omission decisions
- divergence from legacy EMBOSS behavior
- fixture curation
- version compatibility between epithema and epithemaR
- validation thresholds
- documentation publication standards

Because this work is sponsored by Mnemosyne Biosciences Ltd, governance should also support auditable demonstration that the sponsored objectives have been satisfied.


23. SUCCESS CRITERIA

The reboot should be considered architecturally successful when the following are true:

- epithema provides a unified Rust command surface for the EMBOSS catalog
- epithemaR provides first-class R access to the method catalog and all plotting functionality
- scientific logic is shared across CLI, R, and future API surfaces
- the platform demonstrates strong performance and parallel execution characteristics
- the historic EMBOSS example corpus has been converted into executable validation and documentation assets
- acceptance reports can demonstrate, tool by tool, that Epithema matches or acceptably improves upon legacy EMBOSS behavior
- Linux-native and containerized delivery are reproducible and support downstream integration needs


24. IMMEDIATE NEXT ARCHITECTURAL DELIVERABLES

The next documents that should be produced from this brief are:

1) a crate-by-crate technical specification for epithema
2) an R integration specification for epithemaR
3) a tool registry design and trait model
4) a validation and executable-documentation specification
5) a migration and prioritization plan for harvesting legacy EMBOSS examples
6) a container and build orchestration specification
7) a sponsor-facing acceptance reporting template for Mnemosyne Biosciences Ltd


25. CLOSING STATEMENT

Epithema is not merely a port of legacy EMBOSS. It is a scientific platform reboot.

Its essential promise is that the breadth and trustworthiness historically associated with EMBOSS will be recovered and modernized through:

- Rust implementation
- a single coherent CLI
- first-class R support
- R-based plotting
- aggressive performance engineering
- and acceptance evidence grounded in the historical EMBOSS corpus

This architecture is the foundation for that reboot and for the sponsored delivery obligations to Mnemosyne Biosciences Ltd.
