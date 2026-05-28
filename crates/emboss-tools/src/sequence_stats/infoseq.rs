//! `infoseq` implementation.

use emboss_core::{Alphabet, GcSummary, MoleculeKind, SequenceRecord};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `infoseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InfoseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
}

/// Stable summary row for one sequence record.
#[derive(Clone, Debug, PartialEq)]
pub struct InfoseqRow {
    /// Stable 1-based record ordinal.
    pub ordinal: usize,
    /// Stable accession or primary identifier.
    pub identifier: String,
    /// Optional display name.
    pub display_name: Option<String>,
    /// Sequence length in residues.
    pub length: usize,
    /// Stable molecule kind label.
    pub molecule: String,
    /// Stable alphabet label.
    pub alphabet: String,
    /// Optional GC percentage for nucleotide-like inputs.
    pub gc_percent: Option<f64>,
    /// Feature count where annotations are present.
    pub feature_count: usize,
    /// Optional description text.
    pub description: Option<String>,
    /// Optional organism metadata.
    pub organism: Option<String>,
}

/// Structured `infoseq` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct InfoseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Ordered rows in source order.
    pub rows: Vec<InfoseqRow>,
}

/// Returns `infoseq` help text.
#[must_use]
pub fn infoseq_help() -> &'static str {
    "Usage: emboss-rs infoseq <input>\n\nReport stable basic sequence information for one or more records. v1 emits one source-order row per record with identifier, optional display name, length, molecule, alphabet, optional GC percentage for nucleotide-like inputs, feature count, and selected metadata."
}

/// Executes `infoseq`.
pub fn run_infoseq(params: InfoseqParams) -> Result<InfoseqOutcome, ToolExecutionError> {
    let rows = load_sequence_records(&params.input)?
        .into_iter()
        .enumerate()
        .map(|(index, record)| summarize_record(index + 1, &record))
        .collect();

    Ok(InfoseqOutcome {
        input: params.input,
        rows,
    })
}

fn summarize_record(ordinal: usize, record: &SequenceRecord) -> InfoseqRow {
    InfoseqRow {
        ordinal,
        identifier: record.identifier().accession().to_owned(),
        display_name: record.identifier().display_name().map(ToOwned::to_owned),
        length: record.len(),
        molecule: record.molecule().to_string(),
        alphabet: record.alphabet().to_string(),
        gc_percent: nucleotide_gc_percent(record),
        feature_count: record.features().len(),
        description: record.metadata().description.clone(),
        organism: record.metadata().organism.clone(),
    }
}

fn nucleotide_gc_percent(record: &SequenceRecord) -> Option<f64> {
    if record.molecule().is_protein()
        || (!record.molecule().is_nucleotide() && !looks_like_nucleotide_record(record))
    {
        return None;
    }

    let gc = GcSummary::from_sequence(record.residues());
    (gc.canonical_symbols > 0).then_some(gc.gc_percent())
}

fn looks_like_nucleotide_record(record: &SequenceRecord) -> bool {
    Alphabet::Dna
        .validate(MoleculeKind::Dna, record.residues())
        .is_ok()
        || Alphabet::Rna
            .validate(MoleculeKind::Rna, record.residues())
            .is_ok()
}

#[cfg(test)]
mod tests {
    use emboss_core::{MoleculeKind, SequenceIdentifier, SequenceMetadata, SequenceTopology};

    use super::{InfoseqParams, run_infoseq, summarize_record};
    use crate::sequence_stream::SequenceInput;

    #[test]
    fn reports_basic_plain_sequence_information() {
        let outcome = run_infoseq(InfoseqParams {
            input: SequenceInput::new(
                "/Users/stephen/Projects/emboss-rs/crates/emboss-tools/tests/fixtures/three_records.fasta",
            ),
        })
        .expect("infoseq should execute");

        assert_eq!(outcome.rows.len(), 3);
        assert_eq!(outcome.rows[0].identifier, "alpha");
        assert_eq!(outcome.rows[0].length, 4);
        assert_eq!(outcome.rows[0].molecule, "dna");
        assert_eq!(outcome.rows[0].gc_percent, Some(50.0));
        assert_eq!(
            outcome.rows[0].description.as_deref(),
            Some("first example")
        );
    }

    #[test]
    fn reports_annotation_aware_metadata() {
        let outcome = run_infoseq(InfoseqParams {
            input: SequenceInput::new(
                "/Users/stephen/Projects/emboss-rs/crates/emboss-tools/tests/fixtures/annotated_feature.gbk",
            ),
        })
        .expect("infoseq should execute");

        assert_eq!(outcome.rows.len(), 1);
        assert_eq!(outcome.rows[0].identifier, "FEAT1");
        assert_eq!(outcome.rows[0].feature_count, 2);
        assert_eq!(
            outcome.rows[0].organism.as_deref(),
            Some("Synthetic construct")
        );
    }

    #[test]
    fn omits_gc_for_protein_records() {
        let record = emboss_core::SequenceRecord::new(
            SequenceIdentifier::new("pep1").expect("identifier"),
            MoleculeKind::Protein,
            "MSTN",
        )
        .expect("record")
        .with_metadata(
            SequenceMetadata::new()
                .with_description("protein")
                .with_topology(SequenceTopology::Linear),
        );

        let row = summarize_record(1, &record);
        assert_eq!(row.gc_percent, None);
        assert_eq!(row.description.as_deref(), Some("protein"));
    }
}
