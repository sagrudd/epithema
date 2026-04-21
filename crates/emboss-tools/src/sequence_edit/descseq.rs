//! `descseq` implementation.

use emboss_core::{SequenceMetadata, SequenceRecord, SequenceTopology};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `descseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DescseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
}

/// Stable summary row for one sequence record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DescseqRow {
    /// Stable 1-based row ordinal in source order.
    pub ordinal: usize,
    /// Stable accession or primary identifier.
    pub identifier: String,
    /// Optional display name.
    pub display_name: Option<String>,
    /// Optional free-text description.
    pub description: Option<String>,
    /// Sequence length in residues.
    pub length: usize,
    /// Molecule kind label.
    pub molecule: String,
    /// Alphabet classification label.
    pub alphabet: String,
    /// Count of attached features.
    pub feature_count: usize,
    /// Optional source label.
    pub source: Option<String>,
    /// Optional organism label.
    pub organism: Option<String>,
    /// Optional topology label.
    pub topology: Option<String>,
}

/// Structured `descseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DescseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Ordered record summaries.
    pub rows: Vec<DescseqRow>,
}

/// Returns `descseq` help text.
#[must_use]
pub fn descseq_help() -> &'static str {
    "Usage: emboss-rs descseq <input>\n\nReport a stable tabular description summary for one or more sequence records. v1 reports source-order rows with identifier, display name, description, length, molecule, alphabet, feature count, and selected metadata when available."
}

/// Executes `descseq`.
pub fn run_descseq(params: DescseqParams) -> Result<DescseqOutcome, ToolExecutionError> {
    let rows = load_sequence_records(&params.input)?
        .into_iter()
        .enumerate()
        .map(|(index, record)| summarize_record(index + 1, &record))
        .collect();

    Ok(DescseqOutcome {
        input: params.input,
        rows,
    })
}

fn summarize_record(ordinal: usize, record: &SequenceRecord) -> DescseqRow {
    DescseqRow {
        ordinal,
        identifier: record.identifier().accession().to_owned(),
        display_name: record.identifier().display_name().map(ToOwned::to_owned),
        description: record.metadata().description.clone(),
        length: record.len(),
        molecule: record.molecule().to_string(),
        alphabet: record.alphabet().to_string(),
        feature_count: record.features().len(),
        source: record.metadata().source.clone(),
        organism: record.metadata().organism.clone(),
        topology: topology_label(record.metadata()),
    }
}

fn topology_label(metadata: &SequenceMetadata) -> Option<String> {
    metadata.topology.map(|topology| match topology {
        SequenceTopology::Linear => "linear".to_owned(),
        SequenceTopology::Circular => "circular".to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use emboss_core::{Alphabet, MoleculeKind, SequenceIdentifier, SequenceRecord};

    use super::{DescseqParams, run_descseq};
    use crate::sequence_stream::SequenceInput;

    #[test]
    fn summarizes_plain_record_metadata() {
        let fixture = SequenceInput::new(
            "/Users/stephen/Projects/emboss-rs/crates/emboss-tools/tests/fixtures/three_records.fasta",
        );

        let outcome = run_descseq(DescseqParams { input: fixture }).expect("descseq should run");
        assert_eq!(outcome.rows.len(), 3);
        assert_eq!(outcome.rows[0].ordinal, 1);
        assert_eq!(outcome.rows[0].identifier, "alpha");
        assert_eq!(
            outcome.rows[0].description.as_deref(),
            Some("first example")
        );
        assert_eq!(outcome.rows[0].length, 4);
        assert_eq!(outcome.rows[0].molecule, "dna");
        assert_eq!(outcome.rows[0].alphabet, "DNA alphabet");
        assert_eq!(outcome.rows[0].feature_count, 0);
    }

    #[test]
    fn summarizes_annotation_aware_metadata() {
        let fixture = SequenceInput::new(
            "/Users/stephen/Projects/emboss-rs/crates/emboss-tools/tests/fixtures/annotated_feature.gbk",
        );

        let outcome = run_descseq(DescseqParams { input: fixture }).expect("descseq should run");
        assert_eq!(outcome.rows.len(), 1);
        assert_eq!(outcome.rows[0].identifier, "FEAT1");
        assert_eq!(
            outcome.rows[0].description.as_deref(),
            Some("Example annotated sequence.")
        );
        assert_eq!(outcome.rows[0].feature_count, 2);
        assert_eq!(
            outcome.rows[0].source.as_deref(),
            Some("Synthetic construct")
        );
        assert_eq!(
            outcome.rows[0].organism.as_deref(),
            Some("Synthetic construct")
        );
        assert_eq!(outcome.rows[0].topology, None);
    }

    #[test]
    fn includes_display_name_when_present() {
        let record = SequenceRecord::with_alphabet(
            SequenceIdentifier::new("seq1")
                .expect("identifier")
                .with_display_name("Example One"),
            MoleculeKind::Unknown,
            Alphabet::Text,
            "ACGT",
        )
        .expect("record");

        let row = super::summarize_record(1, &record);
        assert_eq!(row.display_name.as_deref(), Some("Example One"));
        assert_eq!(row.description, None);
        assert_eq!(row.topology, None);
    }
}
