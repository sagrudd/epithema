//! Top-level CLI application orchestration for `emboss-rs`.

use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{CommandFactory, Parser, Subcommand};
use emboss_service::{EmbossService, InvocationRequest, ServiceRegistry, ToolName};

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
            }) => commands::run_autodoc(&service, &input, emit_docs, docs_output_dir.as_deref())
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

        if arguments.next().is_some() {
            return Err(CliError::tool_arguments_not_implemented(tool));
        }

        let tool = ToolName::new(tool).map_err(CliError::from)?;
        let request = InvocationRequest::new(service.default_context(), tool);
        let response = service.invoke(request).map_err(CliError::from)?;

        output::print_unimplemented_tool(&response, service);
        Ok(())
    }
}

fn build_service() -> EmbossService {
    EmbossService::new(ServiceRegistry::new())
}

#[derive(Debug, Parser)]
#[command(
    name = "emboss-rs",
    version,
    about = "Governed EMBOSS reboot command surface in Rust.",
    long_about = "EMBOSS-RS is a governed reboot of EMBOSS with a single binary surface. Use `emboss-rs <tool>` for tool execution, `emboss-rs list` for governed tool discovery, and `emboss-rs autodoc` to validate autodoc contracts and emit generated documentation pages.",
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
        ])
        .expect("autodoc generation flags should parse");
        assert!(format!("{cli:?}").contains("emit_docs: true"));
        assert!(format!("{cli:?}").contains("docs/generated"));
    }

    #[test]
    fn routes_unknown_subcommand_to_tool_path() {
        let cli = Cli::try_parse_from(["emboss-rs", "needle"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("needle"));
    }
}
