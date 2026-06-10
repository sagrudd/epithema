//! Top-level CLI application orchestration for `epithema`.

use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{CommandFactory, Parser, Subcommand};
use epithema_service::{EpithemaService, InvocationRequest, ServiceRegistry, ToolName};
use epithema_tools::governed_tool_descriptors;

use crate::commands;
use crate::error::CliError;
use crate::output;

/// CLI application wrapper for the governed `epithema` binary.
pub struct CliApp;

impl CliApp {
    /// Parses arguments, executes the requested command, and returns the final
    /// process exit code.
    #[must_use]
    pub fn run() -> ExitCode {
        match Cli::try_parse() {
            Ok(cli) => match Self::run_parsed(cli) {
                Ok(()) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("epithema: {error}");
                    error.exit_code()
                }
            },
            Err(error) => error.exit(),
        }
    }

    fn run_parsed(cli: Cli) -> Result<(), CliError> {
        let service = build_service();

        match cli.command {
            None => {
                print!("{}", Cli::command().render_long_help());
                Ok(())
            }
            Some(Command::List) => {
                commands::run_list(&service);
                Ok(())
            }
            Some(Command::Autodoc {
                input,
                emit_docs,
                docs_output_dir,
                emit_validation_stub,
                validation_output_path,
            }) => commands::run_autodoc(
                &service,
                &input,
                emit_docs,
                docs_output_dir.as_deref(),
                emit_validation_stub,
                validation_output_path.as_deref(),
            )
            .map(|_| ())
            .map_err(CliError::from),
            Some(Command::Tool(arguments)) => Self::run_tool(&service, arguments),
        }
    }

    fn run_tool(service: &EpithemaService, arguments: Vec<OsString>) -> Result<(), CliError> {
        let mut arguments = arguments.into_iter();
        let tool = arguments
            .next()
            .and_then(|value| value.into_string().ok())
            .ok_or_else(CliError::missing_tool_name)?;
        let tool_arguments = arguments
            .map(|value| {
                value.into_string().map_err(|_| {
                    CliError::from(
                        epithema_diagnostics::PlatformError::new(
                            epithema_diagnostics::ErrorCategory::Validation,
                            "tool arguments must be valid UTF-8",
                        )
                        .with_code("cli.tool.arguments.non_utf8"),
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let tool = ToolName::new(tool).map_err(CliError::from)?;
        let request =
            InvocationRequest::new(service.default_context(), tool).with_arguments(tool_arguments);
        let response = service.invoke(request).map_err(CliError::from)?;

        output::print_tool_response(&response, service);
        Ok(())
    }
}

fn build_service() -> EpithemaService {
    let mut registry = ServiceRegistry::new();
    for descriptor in governed_tool_descriptors() {
        registry
            .register(*descriptor)
            .expect("built-in tool registration should succeed");
    }
    EpithemaService::new(registry)
}

#[derive(Debug, Parser)]
#[command(
    name = "epithema",
    version,
    about = "Governed EMBOSS reboot command surface in Rust.",
    long_about = "Epithema is a governed reboot of EMBOSS with a single binary surface. Use `epithema <tool>` for tool execution, `epithema list` for governed tool discovery, and `epithema autodoc` to validate autodoc contracts, emit generated documentation pages, and derive validation evidence stubs.",
    arg_required_else_help = false,
    disable_help_subcommand = true
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List currently governed tools known to the shared service layer.
    List,
    /// Load and validate an autodoc JSON contract, optionally emitting generated docs pages.
    Autodoc {
        /// Path to the autodoc JSON document to load and validate.
        input: PathBuf,
        /// Emit generated Markdown pages under the docs tree.
        #[arg(long)]
        emit_docs: bool,
        /// Override the output directory used for generated Markdown pages.
        #[arg(long)]
        docs_output_dir: Option<PathBuf>,
        /// Emit a structured validation evidence stub as JSON.
        #[arg(long)]
        emit_validation_stub: bool,
        /// Override the output path used for the validation evidence JSON.
        #[arg(long)]
        validation_output_path: Option<PathBuf>,
    },
    #[command(external_subcommand)]
    /// Invoke a governed Epithema tool through the shared service layer.
    Tool(Vec<OsString>),
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::Cli;

    #[test]
    fn parses_list_command() {
        let cli = Cli::try_parse_from(["epithema", "list"]).expect("list should parse");
        assert!(format!("{cli:?}").contains("List"));
    }

    #[test]
    fn parses_autodoc_command() {
        let cli = Cli::try_parse_from(["epithema", "autodoc", "example.json"])
            .expect("autodoc should parse");
        assert!(format!("{cli:?}").contains("Autodoc"));
        assert!(format!("{cli:?}").contains("example.json"));
    }

    #[test]
    fn parses_autodoc_generation_flags() {
        let cli = Cli::try_parse_from([
            "epithema",
            "autodoc",
            "example.json",
            "--emit-docs",
            "--docs-output-dir",
            "docs/generated",
            "--emit-validation-stub",
            "--validation-output-path",
            "docs/generated/validation/example.validation.json",
        ])
        .expect("autodoc generation flags should parse");
        assert!(format!("{cli:?}").contains("emit_docs: true"));
        assert!(format!("{cli:?}").contains("emit_validation_stub: true"));
        assert!(format!("{cli:?}").contains("docs/generated"));
    }

    #[test]
    fn routes_unknown_subcommand_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "needle"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("needle"));
    }

    #[test]
    fn routes_seqret_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "seqret", "ena:AB000263"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("seqret"));
    }

    #[test]
    fn routes_seqretsetall_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "seqretsetall", "a.fasta", "b.fasta"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("seqretsetall"));
    }

    #[test]
    fn routes_seqretsplit_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "seqretsplit", "a.fasta"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("seqretsplit"));
    }

    #[test]
    fn routes_refseqget_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "refseqget", "ncbi:protein:NP_000537.3"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("refseqget"));
    }

    #[test]
    fn routes_assemblyget_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "assemblyget", "ena:ERR123456"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("assemblyget"));
    }

    #[test]
    fn routes_whichdb_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "whichdb", "ena:AB000263"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("whichdb"));
    }

    #[test]
    fn routes_runinfo_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "runinfo", "ena:ERR123456"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("runinfo"));
    }

    #[test]
    fn routes_infoassembly_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "infoassembly", "ena:ERR123456"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("infoassembly"));
    }

    #[test]
    fn routes_runget_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "runget", "ena:ERR123456"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("runget"));
    }

    #[test]
    fn routes_charge_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "charge", "example.faa"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("charge"));
    }

    #[test]
    fn routes_density_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "density", "example.fna"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("density"));
    }

    #[test]
    fn routes_banana_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "banana", "example.fna"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("banana"));
    }

    #[test]
    fn routes_wobble_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "wobble", "example.fna"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("wobble"));
    }

    #[test]
    fn routes_isochore_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "isochore", "example.fna"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("isochore"));
    }

    #[test]
    fn routes_syco_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "syco", "example.fna"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("syco"));
    }

    #[test]
    fn routes_hmoment_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "hmoment", "example.faa"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("hmoment"));
    }

    #[test]
    fn routes_octanol_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "octanol", "example.faa"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("octanol"));
    }

    #[test]
    fn routes_pepinfo_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "pepinfo", "example.faa"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("pepinfo"));
    }

    #[test]
    fn routes_psiphi_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "psiphi", "example.pdb"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("psiphi"));
    }

    #[test]
    fn routes_primersearch_to_tool_path() {
        let cli = Cli::try_parse_from([
            "epithema",
            "primersearch",
            "targets.fasta",
            "primer_pairs.tsv",
        ])
        .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("primersearch"));
    }

    #[test]
    fn routes_eprimer3_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "eprimer3", "targets.fasta"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("eprimer3"));
    }

    #[test]
    fn routes_sirna_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "sirna", "targets.fasta"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("sirna"));
    }

    #[test]
    fn routes_wossname_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "wossname", "pairwise align"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("wossname"));
    }

    #[test]
    fn routes_seealso_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "seealso", "needle"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("seealso"));
    }
}
