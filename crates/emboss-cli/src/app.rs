//! Top-level CLI application orchestration for `emboss-rs`.

use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{CommandFactory, Parser, Subcommand};
use emboss_service::{EmbossService, InvocationRequest, ServiceRegistry, ToolName};
use emboss_tools::governed_tool_descriptors;

use crate::commands;
use crate::error::CliError;
use crate::output;

/// CLI application wrapper for the governed `emboss-rs` binary.
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
                    eprintln!("emboss-rs: {error}");
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

    fn run_tool(service: &EmbossService, arguments: Vec<OsString>) -> Result<(), CliError> {
        let mut arguments = arguments.into_iter();
        let tool = arguments
            .next()
            .and_then(|value| value.into_string().ok())
            .ok_or_else(CliError::missing_tool_name)?;
        let tool_arguments = arguments
            .map(|value| {
                value.into_string().map_err(|_| {
                    CliError::from(
                        emboss_diagnostics::PlatformError::new(
                            emboss_diagnostics::ErrorCategory::Validation,
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

fn build_service() -> EmbossService {
    let mut registry = ServiceRegistry::new();
    for descriptor in governed_tool_descriptors() {
        registry
            .register(*descriptor)
            .expect("built-in tool registration should succeed");
    }
    EmbossService::new(registry)
}

#[derive(Debug, Parser)]
#[command(
    name = "emboss-rs",
    version,
    about = "Governed EMBOSS reboot command surface in Rust.",
    long_about = "EMBOSS-RS is a governed reboot of EMBOSS with a single binary surface. Use `emboss-rs <tool>` for tool execution, `emboss-rs list` for governed tool discovery, and `emboss-rs autodoc` to validate autodoc contracts, emit generated documentation pages, and derive validation evidence stubs.",
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
    /// Invoke a governed EMBOSS-RS tool through the shared service layer.
    Tool(Vec<OsString>),
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::Cli;

    #[test]
    fn parses_list_command() {
        let cli = Cli::try_parse_from(["emboss-rs", "list"]).expect("list should parse");
        assert!(format!("{cli:?}").contains("List"));
    }

    #[test]
    fn parses_autodoc_command() {
        let cli = Cli::try_parse_from(["emboss-rs", "autodoc", "example.json"])
            .expect("autodoc should parse");
        assert!(format!("{cli:?}").contains("Autodoc"));
        assert!(format!("{cli:?}").contains("example.json"));
    }

    #[test]
    fn parses_autodoc_generation_flags() {
        let cli = Cli::try_parse_from([
            "emboss-rs",
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
        let cli = Cli::try_parse_from(["emboss-rs", "needle"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("needle"));
    }

    #[test]
    fn routes_seqret_to_tool_path() {
        let cli = Cli::try_parse_from(["emboss-rs", "seqret", "ena:AB000263"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("seqret"));
    }

    #[test]
    fn routes_refseqget_to_tool_path() {
        let cli = Cli::try_parse_from(["emboss-rs", "refseqget", "ncbi:protein:NP_000537.3"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("refseqget"));
    }

    #[test]
    fn routes_runinfo_to_tool_path() {
        let cli = Cli::try_parse_from(["emboss-rs", "runinfo", "ena:ERR123456"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("runinfo"));
    }

    #[test]
    fn routes_runget_to_tool_path() {
        let cli = Cli::try_parse_from(["emboss-rs", "runget", "ena:ERR123456"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("runget"));
    }

    #[test]
    fn routes_charge_to_tool_path() {
        let cli =
            Cli::try_parse_from(["emboss-rs", "charge", "example.faa"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("charge"));
    }

    #[test]
    fn routes_density_to_tool_path() {
        let cli = Cli::try_parse_from(["emboss-rs", "density", "example.fna"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("density"));
    }

    #[test]
    fn routes_wobble_to_tool_path() {
        let cli =
            Cli::try_parse_from(["emboss-rs", "wobble", "example.fna"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("wobble"));
    }

    #[test]
    fn routes_isochore_to_tool_path() {
        let cli = Cli::try_parse_from(["emboss-rs", "isochore", "example.fna"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("isochore"));
    }

    #[test]
    fn routes_hmoment_to_tool_path() {
        let cli = Cli::try_parse_from(["emboss-rs", "hmoment", "example.faa"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("hmoment"));
    }

    #[test]
    fn routes_octanol_to_tool_path() {
        let cli = Cli::try_parse_from(["emboss-rs", "octanol", "example.faa"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("octanol"));
    }

    #[test]
    fn routes_pepinfo_to_tool_path() {
        let cli = Cli::try_parse_from(["emboss-rs", "pepinfo", "example.faa"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("pepinfo"));
    }
}
