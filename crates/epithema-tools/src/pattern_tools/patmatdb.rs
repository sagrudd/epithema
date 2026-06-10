//! `patmatdb` implementation.

use std::fs;
use std::path::PathBuf;

use epithema_core::ProteinPattern;
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `patmatdb`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatmatdbParams {
    /// Local protein input path.
    pub input: SequenceInput,
    /// Local TSV motif database path.
    pub database: PathBuf,
}

/// One loaded motif entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatmatdbMotif {
    /// Stable motif identifier.
    pub motif_id: String,
    /// Parsed protein motif.
    pub pattern: ProteinPattern,
    /// Optional free-text description.
    pub description: Option<String>,
}

/// One motif database hit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatmatdbHit {
    /// Source record identifier.
    pub record_id: String,
    /// Matched motif identifier.
    pub motif_id: String,
    /// Matched motif pattern text.
    pub pattern: String,
    /// Optional motif description.
    pub description: Option<String>,
    /// Zero-based inclusive start.
    pub start: usize,
    /// Zero-based half-open end.
    pub end: usize,
    /// Matched normalized input slice.
    pub matched: String,
}

/// Structured `patmatdb` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatmatdbOutcome {
    /// Source protein input.
    pub input: SequenceInput,
    /// Source motif database path.
    pub database: PathBuf,
    /// Loaded motifs.
    pub motifs: Vec<PatmatdbMotif>,
    /// Stable ordered hits.
    pub hits: Vec<PatmatdbHit>,
}

/// Returns `patmatdb` help text.
#[must_use]
pub fn patmatdb_help() -> &'static str {
    "Usage: epithema patmatdb <protein-input> <motif-db.tsv>\n\nSearch protein sequence records with a local TSV motif database. v1 expects one motif per line as <motif-id><TAB><pattern><TAB><optional-description>, where pattern syntax matches fuzzpro exact residues plus X wildcard. Overlapping matches are reported with 1-based inclusive coordinates."
}

/// Executes `patmatdb`.
pub fn run_patmatdb(params: PatmatdbParams) -> Result<PatmatdbOutcome, ToolExecutionError> {
    let motifs = load_motif_database(&params.database)?;
    let mut hits = Vec::new();

    for record in load_sequence_records(&params.input)? {
        validate_protein_record("patmatdb", &record)?;
        for motif in &motifs {
            hits.extend(
                motif
                    .pattern
                    .scan(record.residues())
                    .into_iter()
                    .map(|hit| PatmatdbHit {
                        record_id: record.identifier().accession().to_owned(),
                        motif_id: motif.motif_id.clone(),
                        pattern: motif.pattern.raw().to_owned(),
                        description: motif.description.clone(),
                        start: hit.start(),
                        end: hit.end(),
                        matched: hit.matched().to_owned(),
                    }),
            );
        }
    }

    Ok(PatmatdbOutcome {
        input: params.input,
        database: params.database,
        motifs,
        hits,
    })
}

fn load_motif_database(path: &PathBuf) -> Result<Vec<PatmatdbMotif>, ToolExecutionError> {
    let contents = fs::read_to_string(path).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Invocation,
            format!(
                "failed to read patmatdb motif database '{}': {error}",
                path.display()
            ),
        )
        .with_code("tools.patmatdb.database.read_failed")
    })?;

    let mut motifs = Vec::new();
    for (line_number, line) in contents.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let mut fields = trimmed.splitn(3, '\t');
        let motif_id = fields.next().unwrap_or_default().trim();
        let pattern = fields.next().unwrap_or_default().trim();
        let description = fields
            .next()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned);

        if motif_id.is_empty() || pattern.is_empty() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!(
                    "patmatdb motif database line {} must contain at least motif id and pattern",
                    line_number + 1
                ),
            )
            .with_code("tools.patmatdb.database.invalid_line")
            .with_detail(trimmed.to_owned()));
        }

        let pattern = ProteinPattern::parse(pattern).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Validation,
                format!(
                    "invalid patmatdb motif pattern on line {}: {error}",
                    line_number + 1
                ),
            )
            .with_code("tools.patmatdb.database.invalid_pattern")
            .with_detail(trimmed.to_owned())
        })?;

        motifs.push(PatmatdbMotif {
            motif_id: motif_id.to_owned(),
            pattern,
            description,
        });
    }

    if motifs.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "patmatdb motif database must contain at least one motif entry",
        )
        .with_code("tools.patmatdb.database.empty"));
    }

    Ok(motifs)
}

fn validate_protein_record(
    tool: &str,
    record: &epithema_core::SequenceRecord,
) -> Result<(), ToolExecutionError> {
    if record.molecule().is_nucleotide() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} expects protein input but '{}' was classified as {}",
                record.identifier().accession(),
                record.molecule()
            ),
        )
        .with_code(format!("tools.{tool}.input.not_protein")));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{PatmatdbParams, run_patmatdb};
    use crate::sequence_stream::SequenceInput;
    use std::fs;

    fn write_temp_file(name: &str, suffix: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "epithema-patmatdb-{name}-{}-{}.{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("main"),
            suffix
        ));
        fs::write(&path, contents).expect("temporary fixture should be written");
        path
    }

    #[test]
    fn reports_hits_from_multiple_motifs() {
        let sequence = write_temp_file("sequence", "fasta", ">protA\nMAMKLS\n");
        let database = write_temp_file(
            "database",
            "tsv",
            "motif_a\tMAM\tleading motif\nmotif_b\tKLS\tterminal motif\n",
        );

        let outcome = run_patmatdb(PatmatdbParams {
            input: SequenceInput::new(sequence.clone()),
            database: database.clone(),
        })
        .expect("patmatdb should succeed");

        fs::remove_file(sequence).ok();
        fs::remove_file(database).ok();

        assert_eq!(outcome.hits.len(), 2);
        assert_eq!(outcome.hits[0].motif_id, "motif_a");
        assert_eq!(outcome.hits[1].motif_id, "motif_b");
    }
}
