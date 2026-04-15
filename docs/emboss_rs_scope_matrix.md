# EMBOSS tool scope matrix for `emboss-rs`

This matrix classifies the **256 core EMBOSS applications** listed in the official EMBOSS applications index, plus **`complex`** as an explicit extra row because you have already said it must be retained.

Decision meanings:

- **Retain** — keep as a first-class capability in `emboss-rs`
- **Rework** — keep the user need, but redesign substantially because methods, data sources, UX, or rendering should change
- **Omit** — do not carry this forward as a user-facing command in the initial reboot
- **Add** — new capability proposed beyond the historical core catalog

Current recommendation totals for the historical/core list:

- Retain: **90**
- Rework: **102**
- Omit: **65**

Scope notes:

1. This table is based on the EMBOSS core applications index, which documents the main EMBOSS applications and notes that EMBASSY applications are documented separately.
2. `complex` is included explicitly because you stated it must be retained, even though it is not in the core applications index page.
3. The HMM family is handled in the **Additions beyond the historical core** section because the old HMM wrapper commands lived outside the core index and should be rebooted as a modern capability area rather than preserved as wrapper compatibility.
4. Ontology/EDAM/GO command groups are recommended for initial omission as user-facing commands, while leaving room for internal metadata tagging later.
5. Retrieval/database/server-oriented commands are frequently marked **Rework** where the user need remains but the old transport and server model should be replaced with modern ENA/SRA/RefSeq/taxonomy/variation integrations.

## Historical/core tools

| Tool | Description | Decision | Rationale |
|---|---|---|---|
| `aaindexextract` | Extract amino acid property data from AAINDEX | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `abiview` | Display the trace in an ABI sequencer file | **Rework** | Underlying biological need remains, but methods and datasets have advanced enough that a substantial modernization is justified. |
| `acdc` | Test an application ACD file | **Omit** | Legacy ACD developer/test plumbing; not needed in a Rust-native tool-definition model. |
| `acdpretty` | Correctly reformat an application ACD file | **Omit** | Legacy ACD developer/test plumbing; not needed in a Rust-native tool-definition model. |
| `acdtable` | Generate an HTML table of parameters from an application ACD file | **Omit** | Legacy ACD developer/test plumbing; not needed in a Rust-native tool-definition model. |
| `acdtrace` | Trace processing of an application ACD file (for testing) | **Omit** | Legacy ACD developer/test plumbing; not needed in a Rust-native tool-definition model. |
| `acdvalid` | Validate an application ACD file | **Omit** | Legacy ACD developer/test plumbing; not needed in a Rust-native tool-definition model. |
| `aligncopy` | Read and write alignments | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `aligncopypair` | Read and write pairs from alignments | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `antigenic` | Find antigenic sites in proteins | **Rework** | Retain the problem domain, but rework heavily because the user need persists while implementation, data sources, or UX should change substantially. |
| `assemblyget` | Get assembly of sequence reads | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `backtranambig` | Back-translate a protein sequence to ambiguous nucleotide sequence | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `backtranseq` | Back-translate a protein sequence to a nucleotide sequence | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `banana` | Plot bending and curvature data for B-DNA | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `biosed` | Replace or delete sequence sections | **Retain** | Simple but useful sequence editing/transformation capability; keep as a first-class part of the toolkit. |
| `btwisted` | Calculate the twisting in a B-DNA sequence | **Rework** | Retain the problem domain, but rework heavily because the user need persists while implementation, data sources, or UX should change substantially. |
| `cachedas` | Generate server cache file for DAS servers or for the DAS registry | **Omit** | Legacy remote-server cache generation for DAS/Dbfetch/EB-eye/Ensembl; replace with modern provider adapters rather than preserving as commands. |
| `cachedbfetch` | Generate server cache file for Dbfetch/WSDbfetch data sources | **Omit** | Legacy remote-server cache generation for DAS/Dbfetch/EB-eye/Ensembl; replace with modern provider adapters rather than preserving as commands. |
| `cacheebeyesearch` | Generate server cache file for EB-eye search domains | **Omit** | Legacy remote-server cache generation for DAS/Dbfetch/EB-eye/Ensembl; replace with modern provider adapters rather than preserving as commands. |
| `cacheensembl` | Generate server cache file for an Ensembl server | **Omit** | Legacy remote-server cache generation for DAS/Dbfetch/EB-eye/Ensembl; replace with modern provider adapters rather than preserving as commands. |
| `cai` | Calculate codon adaptation index | **Retain** | Still-useful descriptive/statistical analysis with straightforward modernization inside the Rust core. |
| `chaos` | Draw a chaos game representation plot for a nucleotide sequence | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `charge` | Draw a protein charge plot | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `checktrans` | Report STOP codons and ORF statistics of a protein | **Retain** | Still-useful descriptive/statistical analysis with straightforward modernization inside the Rust core. |
| `chips` | Calculate Nc codon usage statistic | **Retain** | Still-useful descriptive/statistical analysis with straightforward modernization inside the Rust core. |
| `cirdna` | Draw circular map of DNA constructs | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `codcmp` | Codon usage table comparison | **Retain** | Still-useful descriptive/statistical analysis with straightforward modernization inside the Rust core. |
| `codcopy` | Copy and reformat a codon usage table | **Retain** | Still-useful descriptive/statistical analysis with straightforward modernization inside the Rust core. |
| `coderet` | Extract CDS, mRNA and translations from feature tables | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `compseq` | Calculate the composition of unique words in sequences | **Retain** | Core search/comparison utility worth preserving as part of a broad EMBOSS-style toolkit. |
| `cons` | Create a consensus sequence from a multiple alignment | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `consambig` | Create an ambiguous consensus sequence from a multiple alignment | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `cpgplot` | Identify and plot CpG islands in nucleotide sequence(s) | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `cpgreport` | Identify and report CpG-rich regions in nucleotide sequence(s) | **Rework** | Keep the biological task, but refresh algorithms/data sources/output models and route any graphics through R. |
| `cusp` | Create a codon usage table from nucleotide sequence(s) | **Retain** | Still-useful descriptive/statistical analysis with straightforward modernization inside the Rust core. |
| `cutgextract` | Extract codon usage tables from CUTG database | **Rework** | Retain the problem domain, but rework heavily because the user need persists while implementation, data sources, or UX should change substantially. |
| `cutseq` | Remove a section from a sequence | **Retain** | Simple but useful sequence editing/transformation capability; keep as a first-class part of the toolkit. |
| `dan` | Calculate nucleic acid melting temperature | **Retain** | Still-useful descriptive/statistical analysis with straightforward modernization inside the Rust core. |
| `dbiblast` | Index a BLAST database | **Omit** | Legacy database indexing/admin plumbing for EMBOSS-era local/remote database architecture; not a user-facing scientific priority. |
| `dbifasta` | Index a fasta file database | **Omit** | Legacy database indexing/admin plumbing for EMBOSS-era local/remote database architecture; not a user-facing scientific priority. |
| `dbiflat` | Index a flat file database | **Omit** | Legacy database indexing/admin plumbing for EMBOSS-era local/remote database architecture; not a user-facing scientific priority. |
| `dbigcg` | Index a GCG formatted database | **Omit** | Legacy database indexing/admin plumbing for EMBOSS-era local/remote database architecture; not a user-facing scientific priority. |
| `dbtell` | Display information about a public database | **Omit** | Legacy database indexing/admin plumbing for EMBOSS-era local/remote database architecture; not a user-facing scientific priority. |
| `dbxcompress` | Compress an uncompressed dbx index | **Omit** | Legacy database indexing/admin plumbing for EMBOSS-era local/remote database architecture; not a user-facing scientific priority. |
| `dbxedam` | Index the EDAM ontology using b+tree indices | **Omit** | Legacy database indexing/admin plumbing for EMBOSS-era local/remote database architecture; not a user-facing scientific priority. |
| `dbxfasta` | Index a fasta file database using b+tree indices | **Omit** | Legacy database indexing/admin plumbing for EMBOSS-era local/remote database architecture; not a user-facing scientific priority. |
| `dbxflat` | Index a flat file database using b+tree indices | **Omit** | Legacy database indexing/admin plumbing for EMBOSS-era local/remote database architecture; not a user-facing scientific priority. |
| `dbxgcg` | Index a GCG formatted database using b+tree indices | **Omit** | Legacy database indexing/admin plumbing for EMBOSS-era local/remote database architecture; not a user-facing scientific priority. |
| `dbxobo` | Index an obo ontology using b+tree indices | **Omit** | Legacy database indexing/admin plumbing for EMBOSS-era local/remote database architecture; not a user-facing scientific priority. |
| `dbxreport` | Validate index and report internals for dbx databases | **Omit** | Legacy database indexing/admin plumbing for EMBOSS-era local/remote database architecture; not a user-facing scientific priority. |
| `dbxresource` | Index a data resource catalogue using b+tree indices | **Omit** | Legacy database indexing/admin plumbing for EMBOSS-era local/remote database architecture; not a user-facing scientific priority. |
| `dbxstat` | Dump statistics for dbx databases | **Omit** | Legacy database indexing/admin plumbing for EMBOSS-era local/remote database architecture; not a user-facing scientific priority. |
| `dbxtax` | Index NCBI taxonomy using b+tree indices | **Omit** | Legacy database indexing/admin plumbing for EMBOSS-era local/remote database architecture; not a user-facing scientific priority. |
| `dbxuncompress` | Uncompress a compressed dbx index | **Omit** | Legacy database indexing/admin plumbing for EMBOSS-era local/remote database architecture; not a user-facing scientific priority. |
| `degapseq` | Remove non-alphabetic (e.g. gap) characters from sequences | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `density` | Draw a nucleic acid density plot | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `descseq` | Alter the name or description of a sequence | **Retain** | Simple but useful sequence editing/transformation capability; keep as a first-class part of the toolkit. |
| `diffseq` | Compare and report features of two similar sequences | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `distmat` | Create a distance matrix from a multiple sequence alignment | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `dotmatcher` | Draw a threshold dotplot of two sequences | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `dotpath` | Draw a non-overlapping wordmatch dotplot of two sequences | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `dottup` | Display a wordmatch dotplot of two sequences | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `dreg` | Regular expression search of nucleotide sequence(s) | **Retain** | Core search/comparison utility worth preserving as part of a broad EMBOSS-style toolkit. |
| `drfinddata` | Find public databases by data type | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `drfindformat` | Find public databases by format | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `drfindid` | Find public databases by identifier | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `drfindresource` | Find public databases by resource | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `drget` | Get data resource entries | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `drtext` | Get data resource entries complete text | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `edamdef` | Find EDAM ontology terms by definition | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `edamhasinput` | Find EDAM ontology terms by has_input relation | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `edamhasoutput` | Find EDAM ontology terms by has_output relation | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `edamisformat` | Find EDAM ontology terms by is_format_of relation | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `edamisid` | Find EDAM ontology terms by is_identifier_of relation | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `edamname` | Find EDAM ontology terms by name | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `edialign` | Local multiple alignment of sequences | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `einverted` | Find inverted repeats in nucleotide sequences | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `embossdata` | Find and retrieve EMBOSS data files | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `embossupdate` | Checks for more recent updates to EMBOSS | **Omit** | Superseded by built-in CLI help/version/update mechanics in a unified `emboss-rs` binary. |
| `embossversion` | Report the current EMBOSS version number | **Omit** | Superseded by built-in CLI help/version/update mechanics in a unified `emboss-rs` binary. |
| `emma` | Multiple sequence alignment (ClustalW wrapper) | **Rework** | Retain the capability, but modernize the wrapper/database model and decouple from EMBOSS-era external tool assumptions. |
| `emowse` | Search protein sequences by digest fragment molecular weight | **Omit** | Niche/dated workflow with limited modern demand relative to maintenance cost. |
| `entret` | Retrieve sequence entries from flatfile databases and files | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `epestfind` | Find PEST motifs as potential proteolytic cleavage sites | **Rework** | Underlying biological need remains, but methods and datasets have advanced enough that a substantial modernization is justified. |
| `eprimer3` | Pick PCR primers and hybridization oligos | **Rework** | Retain the capability, but modernize the wrapper/database model and decouple from EMBOSS-era external tool assumptions. |
| `eprimer32` | Pick PCR primers and hybridization oligos | **Omit** | Legacy wrapper tied to an old Primer3 executable naming convention; retain primer-design capability elsewhere instead. |
| `equicktandem` | Find tandem repeats in nucleotide sequences | **Rework** | Underlying biological need remains, but methods and datasets have advanced enough that a substantial modernization is justified. |
| `est2genome` | Align EST sequences to genomic DNA sequence | **Rework** | Underlying biological need remains, but methods and datasets have advanced enough that a substantial modernization is justified. |
| `etandem` | Find tandem repeats in a nucleotide sequence | **Rework** | Underlying biological need remains, but methods and datasets have advanced enough that a substantial modernization is justified. |
| `extractalign` | Extract regions from a sequence alignment | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `extractfeat` | Extract features from sequence(s) | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `extractseq` | Extract regions from a sequence | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `featcopy` | Read and write a feature table | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `featmerge` | Merge two overlapping feature tables | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `featreport` | Read and write a feature table | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `feattext` | Return a feature table original text | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `findkm` | Calculate and plot enzyme reaction data | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `freak` | Generate residue/base frequency table or plot | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `fuzznuc` | Search for patterns in nucleotide sequences | **Retain** | Core search/comparison utility worth preserving as part of a broad EMBOSS-style toolkit. |
| `fuzzpro` | Search for patterns in protein sequences | **Retain** | Core search/comparison utility worth preserving as part of a broad EMBOSS-style toolkit. |
| `fuzztran` | Search for patterns in protein sequences (translated) | **Retain** | Core search/comparison utility worth preserving as part of a broad EMBOSS-style toolkit. |
| `garnier` | Predict protein secondary structure using GOR method | **Rework** | Underlying biological need remains, but methods and datasets have advanced enough that a substantial modernization is justified. |
| `geecee` | Calculate fractional GC content of nucleic acid sequences | **Retain** | Still-useful descriptive/statistical analysis with straightforward modernization inside the Rust core. |
| `getorf` | Find and extract open reading frames (ORFs) | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `godef` | Find GO ontology terms by definition | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `goname` | Find GO ontology terms by name | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `helixturnhelix` | Identify nucleic acid-binding motifs in protein sequences | **Rework** | Retain the problem domain, but rework heavily because the user need persists while implementation, data sources, or UX should change substantially. |
| `hmoment` | Calculate and plot hydrophobic moment for protein sequence(s) | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `iep` | Calculate the isoelectric point of proteins | **Retain** | Still-useful descriptive/statistical analysis with straightforward modernization inside the Rust core. |
| `infoalign` | Display basic information about a multiple sequence alignment | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `infoassembly` | Display information about assemblies | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `infobase` | Return information on a given nucleotide base | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `inforesidue` | Return information on a given amino acid residue | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `infoseq` | Display basic information about sequences | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `isochore` | Plot isochores in DNA sequences | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `jaspextract` | Extract data from JASPAR | **Rework** | Retain the capability, but modernize the wrapper/database model and decouple from EMBOSS-era external tool assumptions. |
| `jaspscan` | Scan DNA sequences for transcription factors | **Rework** | Retain the capability, but modernize the wrapper/database model and decouple from EMBOSS-era external tool assumptions. |
| `lindna` | Draw linear maps of DNA constructs | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `listor` | Write a list file of the logical OR of two sets of sequences | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `makenucseq` | Create random nucleotide sequences | **Retain** | Simple but useful sequence editing/transformation capability; keep as a first-class part of the toolkit. |
| `makeprotseq` | Create random protein sequences | **Retain** | Simple but useful sequence editing/transformation capability; keep as a first-class part of the toolkit. |
| `marscan` | Find matrix/scaffold recognition (MRS) signatures in DNA sequences | **Rework** | Underlying biological need remains, but methods and datasets have advanced enough that a substantial modernization is justified. |
| `maskambignuc` | Mask all ambiguity characters in nucleotide sequences with N | **Retain** | Simple but useful sequence editing/transformation capability; keep as a first-class part of the toolkit. |
| `maskambigprot` | Mask all ambiguity characters in protein sequences with X | **Retain** | Simple but useful sequence editing/transformation capability; keep as a first-class part of the toolkit. |
| `maskfeat` | Write a sequence with masked features | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `maskseq` | Write a sequence with masked regions | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `matcher` | Waterman-Eggert local alignment of two sequences | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `megamerger` | Merge two large overlapping DNA sequences | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `merger` | Merge two overlapping sequences | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `msbar` | Mutate a sequence | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `mwcontam` | Find weights common to multiple molecular weights files | **Omit** | Niche/dated workflow with limited modern demand relative to maintenance cost. |
| `mwfilter` | Filter noisy data from molecular weights file | **Omit** | Niche/dated workflow with limited modern demand relative to maintenance cost. |
| `needle` | Needleman-Wunsch global alignment of two sequences | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `needleall` | Many-to-many pairwise alignments of two sequence sets | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `newcpgreport` | Identify CpG islands in nucleotide sequence(s) | **Rework** | Keep the biological task, but refresh algorithms/data sources/output models and route any graphics through R. |
| `newcpgseek` | Identify and report CpG-rich regions in nucleotide sequence(s) | **Rework** | Keep the biological task, but refresh algorithms/data sources/output models and route any graphics through R. |
| `newseq` | Create a sequence file from a typed-in sequence | **Retain** | Simple but useful sequence editing/transformation capability; keep as a first-class part of the toolkit. |
| `nohtml` | Remove mark-up (e.g. HTML tags) from an ASCII text file | **Omit** | Generic text-cleaning utility outside the core scientific scope. |
| `noreturn` | Remove carriage return from ASCII files | **Omit** | Generic text-cleaning utility outside the core scientific scope. |
| `nospace` | Remove whitespace from an ASCII text file | **Omit** | Generic text-cleaning utility outside the core scientific scope. |
| `notab` | Replace tabs with spaces in an ASCII text file | **Omit** | Generic text-cleaning utility outside the core scientific scope. |
| `notseq` | Write to file a subset of an input stream of sequences | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `nthseq` | Write to file a single sequence from an input stream of sequences | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `nthseqset` | Read and write (return) one set of sequences from many | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `octanol` | Draw a White-Wimley protein hydropathy plot | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `oddcomp` | Identify proteins with specified sequence word composition | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `ontocount` | Count ontology term(s) | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `ontoget` | Get ontology term(s) | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `ontogetcommon` | Get common ancestor for terms | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `ontogetdown` | Get ontology term(s) by parent id | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `ontogetobsolete` | Get ontology ontology terms | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `ontogetroot` | Get ontology root terms by child identifier | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `ontogetsibs` | Get ontology term(s) by id with common parent | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `ontogetup` | Get ontology term(s) by id of child | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `ontoisobsolete` | Report whether an ontology term id is obsolete | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `ontotext` | Get ontology term(s) original full text | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `palindrome` | Find inverted repeats in nucleotide sequence(s) | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `pasteseq` | Insert one sequence into another | **Retain** | Simple but useful sequence editing/transformation capability; keep as a first-class part of the toolkit. |
| `patmatdb` | Search protein sequences with a sequence motif | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `patmatmotifs` | Scan a protein sequence with motifs from the PROSITE database | **Rework** | Retain the capability, but modernize the wrapper/database model and decouple from EMBOSS-era external tool assumptions. |
| `pepcoil` | Predict coiled coil regions in protein sequences | **Rework** | Underlying biological need remains, but methods and datasets have advanced enough that a substantial modernization is justified. |
| `pepdigest` | Report on protein proteolytic enzyme or reagent cleavage sites | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `pepinfo` | Plot amino acid properties of a protein sequence in parallel | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `pepnet` | Draw a helical net for a protein sequence | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `pepstats` | Calculate statistics of protein properties | **Retain** | Still-useful descriptive/statistical analysis with straightforward modernization inside the Rust core. |
| `pepwheel` | Draw a helical wheel diagram for a protein sequence | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `pepwindow` | Draw a hydropathy plot for a protein sequence | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `pepwindowall` | Draw Kyte-Doolittle hydropathy plot for a protein alignment | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `plotcon` | Plot conservation of a sequence alignment | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `plotorf` | Plot potential open reading frames in a nucleotide sequence | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `polydot` | Draw dotplots for all-against-all comparison of a sequence set | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `preg` | Regular expression search of protein sequence(s) | **Retain** | Core search/comparison utility worth preserving as part of a broad EMBOSS-style toolkit. |
| `prettyplot` | Draw a sequence alignment with pretty formatting | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `prettyseq` | Write a nucleotide sequence and its translation to file | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `primersearch` | Search DNA sequences for matches with primer pairs | **Rework** | Retain the problem domain, but rework heavily because the user need persists while implementation, data sources, or UX should change substantially. |
| `printsextract` | Extract data from PRINTS database for use by pscan | **Rework** | Retain the capability, but modernize the wrapper/database model and decouple from EMBOSS-era external tool assumptions. |
| `profit` | Scan one or more sequences with a simple frequency matrix | **Rework** | Underlying biological need remains, but methods and datasets have advanced enough that a substantial modernization is justified. |
| `prophecy` | Create frequency matrix or profile from a multiple alignment | **Rework** | Underlying biological need remains, but methods and datasets have advanced enough that a substantial modernization is justified. |
| `prophet` | Scan one or more sequences with a Gribskov or Henikoff profile | **Rework** | Underlying biological need remains, but methods and datasets have advanced enough that a substantial modernization is justified. |
| `prosextract` | Process the PROSITE motif database for use by patmatmotifs | **Rework** | Retain the capability, but modernize the wrapper/database model and decouple from EMBOSS-era external tool assumptions. |
| `pscan` | Scan protein sequence(s) with fingerprints from the PRINTS database | **Rework** | Retain the capability, but modernize the wrapper/database model and decouple from EMBOSS-era external tool assumptions. |
| `psiphi` | Calculates phi and psi torsion angles from protein coordinates | **Rework** | Underlying biological need remains, but methods and datasets have advanced enough that a substantial modernization is justified. |
| `rebaseextract` | Process the REBASE database for use by restriction enzyme applications | **Rework** | Retain the capability, but modernize the wrapper/database model and decouple from EMBOSS-era external tool assumptions. |
| `recoder` | Find restriction sites to remove (mutate) with no translation change | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `redata` | Retrieve information from REBASE restriction enzyme database | **Rework** | Retain the capability, but modernize the wrapper/database model and decouple from EMBOSS-era external tool assumptions. |
| `refseqget` | Get reference sequence | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `remap` | Display restriction enzyme binding sites in a nucleotide sequence | **Rework** | Keep the biological task, but refresh algorithms/data sources/output models and route any graphics through R. |
| `restover` | Find restriction enzymes producing a specific overhang | **Rework** | Keep the biological task, but refresh algorithms/data sources/output models and route any graphics through R. |
| `restrict` | Report restriction enzyme cleavage sites in a nucleotide sequence | **Rework** | Keep the biological task, but refresh algorithms/data sources/output models and route any graphics through R. |
| `revseq` | Reverse and complement a nucleotide sequence | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `seealso` | Find programs with similar function to a specified program | **Rework** | Useful capability remains, but the user experience should be redesigned for a single `emboss-rs` interface and modern documentation/search. |
| `seqcount` | Read and count sequences | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `seqmatchall` | All-against-all word comparison of a sequence set | **Retain** | Core search/comparison utility worth preserving as part of a broad EMBOSS-style toolkit. |
| `seqret` | Read and write (return) sequences | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `seqretsetall` | Read and write (return) many sets of sequences | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `seqretsplit` | Read sequences and write them to individual files | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `seqxref` | Retrieve all database cross-references for a sequence entry | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `seqxrefget` | Retrieve all cross-referenced data for a sequence entry | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `servertell` | Display information about a public server | **Omit** | Old public-server discovery/inspection model should be retired in favor of explicit modern provider integrations. |
| `showalign` | Display a multiple sequence alignment in pretty format | **Rework** | Useful capability remains, but the user experience should be redesigned for a single `emboss-rs` interface and modern documentation/search. |
| `showdb` | Display information on configured databases | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `showfeat` | Display features of a sequence in pretty format | **Rework** | Useful capability remains, but the user experience should be redesigned for a single `emboss-rs` interface and modern documentation/search. |
| `showorf` | Display a nucleotide sequence and translation in pretty format | **Rework** | Useful capability remains, but the user experience should be redesigned for a single `emboss-rs` interface and modern documentation/search. |
| `showpep` | Display protein sequences with features in pretty format | **Rework** | Useful capability remains, but the user experience should be redesigned for a single `emboss-rs` interface and modern documentation/search. |
| `showseq` | Display sequences with features in pretty format | **Rework** | Useful capability remains, but the user experience should be redesigned for a single `emboss-rs` interface and modern documentation/search. |
| `showserver` | Display information on configured servers | **Omit** | Old public-server discovery/inspection model should be retired in favor of explicit modern provider integrations. |
| `shuffleseq` | Shuffle a set of sequences maintaining composition | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `sigcleave` | Report on signal cleavage sites in a protein sequence | **Rework** | Underlying biological need remains, but methods and datasets have advanced enough that a substantial modernization is justified. |
| `silent` | Find restriction sites to insert (mutate) with no translation change | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `sirna` | Find siRNA duplexes in mRNA | **Rework** | Underlying biological need remains, but methods and datasets have advanced enough that a substantial modernization is justified. |
| `sixpack` | Display a DNA sequence with 6-frame translation and ORFs | **Rework** | Useful capability remains, but the user experience should be redesigned for a single `emboss-rs` interface and modern documentation/search. |
| `sizeseq` | Sort sequences by size | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `skipredundant` | Remove redundant sequences from an input set | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `skipseq` | Read and write (return) sequences, skipping first few | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `splitsource` | Split sequence(s) into original source sequences | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `splitter` | Split sequence(s) into smaller sequences | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `stretcher` | Needleman-Wunsch rapid global alignment of two sequences | **Rework** | Useful capability remains, but the user experience should be redesigned for a single `emboss-rs` interface and modern documentation/search. |
| `stssearch` | Search a DNA database for matches with a set of STS primers | **Omit** | Niche/dated workflow with limited modern demand relative to maintenance cost. |
| `supermatcher` | Calculate approximate local pair-wise alignments of larger sequences | **Rework** | Useful capability remains, but the user experience should be redesigned for a single `emboss-rs` interface and modern documentation/search. |
| `syco` | Draw synonymous codon usage statistic plot for a nucleotide sequence | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `taxget` | Get taxon(s) | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `taxgetdown` | Get descendants of taxon(s) | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `taxgetrank` | Get parents of taxon(s) | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `taxgetspecies` | Get all species under taxon(s) | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `taxgetup` | Get parents of taxon(s) | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `tcode` | Identify protein-coding regions using Fickett TESTCODE statistic | **Rework** | Underlying biological need remains, but methods and datasets have advanced enough that a substantial modernization is justified. |
| `textget` | Get text data entries | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `textsearch` | Search the textual description of sequence(s) | **Rework** | Retain the problem domain, but rework heavily because the user need persists while implementation, data sources, or UX should change substantially. |
| `tfextract` | Process TRANSFAC transcription factor database for use by tfscan | **Rework** | Retain the capability, but modernize the wrapper/database model and decouple from EMBOSS-era external tool assumptions. |
| `tfm` | Display full documentation for an application | **Omit** | Superseded by built-in CLI help/version/update mechanics in a unified `emboss-rs` binary. |
| `tfscan` | Identify transcription factor binding sites in DNA sequences | **Rework** | Retain the capability, but modernize the wrapper/database model and decouple from EMBOSS-era external tool assumptions. |
| `tmap` | Predict and plot transmembrane segments in protein sequences | **Rework** | Plotting/visualization should move to the R companion package, with Rust emitting plot-ready data and R handling rendering. |
| `tranalign` | Generate an alignment of nucleic coding regions from aligned proteins | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `transeq` | Translate nucleic acid sequences | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `trimest` | Remove poly-A tails from nucleotide sequences | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `trimseq` | Remove unwanted characters from start and end of sequence(s) | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `trimspace` | Remove extra whitespace from an ASCII text file | **Omit** | Generic text-cleaning utility outside the core scientific scope. |
| `twofeat` | Find neighbouring pairs of features in sequence(s) | **Retain** | Recommended to retain as a core EMBOSS-style analytical or transformation command in the reboot. |
| `union` | Concatenate multiple sequences into a single sequence | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `urlget` | Get URLs of data resources | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `variationget` | Get sequence variations | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `vectorstrip` | Remove vectors from the ends of nucleotide sequence(s) | **Retain** | Simple but useful sequence editing/transformation capability; keep as a first-class part of the toolkit. |
| `water` | Smith-Waterman local alignment of sequences | **Retain** | Core sequence/alignment/transformation capability that still has clear day-to-day value with no need to omit conceptually. |
| `whichdb` | Search all sequence databases for an entry and retrieve it | **Rework** | Retain the user need, but redesign around modern accession/provider-aware retrieval and metadata services (ENA/SRA/RefSeq/taxonomy/variation). |
| `wobble` | Plot third base position variability in a nucleotide sequence | **Rework** | Retain the problem domain, but rework heavily because the user need persists while implementation, data sources, or UX should change substantially. |
| `wordcount` | Count and extract unique words in molecular sequence(s) | **Retain** | Core search/comparison utility worth preserving as part of a broad EMBOSS-style toolkit. |
| `wordfinder` | Match large sequences against one or more other sequences | **Retain** | Core search/comparison utility worth preserving as part of a broad EMBOSS-style toolkit. |
| `wordmatch` | Find regions of identity (exact matches) of two sequences | **Retain** | Core search/comparison utility worth preserving as part of a broad EMBOSS-style toolkit. |
| `wossdata` | Find programs by EDAM data | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `wossinput` | Find programs by EDAM input data | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `wossname` | Find programs by keywords in their short description | **Rework** | Useful capability remains, but the user experience should be redesigned for a single `emboss-rs` interface and modern documentation/search. |
| `wossoperation` | Find programs by EDAM operation | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `wossoutput` | Find programs by EDAM output data | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `wossparam` | Find programs by EDAM parameter | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `wosstopic` | Find programs by EDAM topic | **Omit** | Ontology-driven user command group deferred from initial scope; keep extension path internally but omit user-facing commands for now. |
| `yank` | Add a sequence reference (a full USA) to a list file | **Omit** | Initial omission recommended because the legacy command surface is obsolete, plumbing-heavy, or outside the reboot’s scientific core. |
| `complex` | Complexity / low-complexity analysis tool (EMBASSY; explicit user retain) | **Retain** | Explicit user-required retain; preserve as part of the reboot even though it is outside the core EMBOSS app index. |

## Additions beyond the historical core

| Tool / domain | Decision | Rationale |
|---|---|---|
| `ena_get` | **Add** | Accession-first retrieval of public ENA records and sequence/metadata via modern APIs, including EMBL/FASTA/XML record types and file reports. |
| `ena_fetch_runs` | **Add** | Bulk retrieval of public ENA run data, assemblies, and associated files using ENA Browser/Portal APIs plus FTP/Aspera-aware download orchestration. |
| `sra_fetch_runs` | **Add** | Bulk SRA run download and resume-aware retrieval workflow centered on modern SRA Toolkit patterns (`prefetch` then `fasterq-dump`). |
| `sra_fetch_original` | **Add** | Access original submitted files for SRA accessions where available, reflecting current SRA cloud/object retrieval workflows. |
| `hmmbuild / hmmsearch / hmmscan / hmmalign` | **Add** | Modern profile-HMM capability domain should be present in the reboot, but as contemporary HMM methods rather than legacy EMBOSS/EMBASSY wrapper compatibility. |
| `jackhmmer / nhmmer / nhmmscan` | **Add** | Extend HMM support into iterative protein search and nucleotide-profile workflows that reflect current HMMER practice. |

## Working interpretation of the policy behind the table

- Prefer **Retain** for durable, broadly useful sequence-analysis, alignment, transformation, format, ORF, feature, restriction, composition, and comparison commands.
- Prefer **Rework** where there is still real user demand but the underlying methods, external resources, archive interfaces, rendering model, or UX should be substantially modernized.
- Prefer **Omit** for:
  - ACD/developer plumbing
  - legacy remote server/cache/index administration
  - ontology command groups deferred from initial scope
  - generic text-cleaning utilities outside the bioinformatics core
  - small pockets of dated or low-value specialist workflow
- Use **Add** for modern capabilities that EMBOSS did not cover well enough, especially contemporary archive retrieval and rebooted HMM support.

