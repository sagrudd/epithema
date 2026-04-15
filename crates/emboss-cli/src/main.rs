//! Command-line entry point for the initial EMBOSS-RS workspace skeleton.

use std::env;
use std::process::ExitCode;

use emboss_service::{InvocationRequest, ServiceError, ServiceRuntime, ToolName};

fn main() -> ExitCode {
    let runtime = ServiceRuntime::empty();
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        None | Some("-h") | Some("--help") => {
            print_help(&runtime);
            ExitCode::SUCCESS
        }
        Some("-V") | Some("--version") => {
            println!("emboss-rs {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Some(command) => {
            let tool = match ToolName::new(command) {
                Ok(tool) => tool,
                Err(error) => {
                    eprintln!("emboss-rs: {error}");
                    return ExitCode::from(2);
                }
            };

            let request = InvocationRequest::new(runtime.default_context(), tool);

            match runtime.invoke(request) {
                Ok(response) => {
                    eprintln!(
                        "emboss-rs: '{}' is registered but not implemented yet.\n{}",
                        response.tool,
                        runtime.status_line()
                    );
                    ExitCode::from(2)
                }
                Err(ServiceError::UnknownTool { tool }) => {
                    eprintln!(
                        "emboss-rs: unknown tool '{tool}'.\n{}",
                        runtime.status_line()
                    );
                    ExitCode::from(2)
                }
                Err(error) => {
                    eprintln!("emboss-rs: {error}");
                    ExitCode::from(2)
                }
            }
        }
    }
}

fn print_help(runtime: &ServiceRuntime) {
    println!("EMBOSS-RS workspace skeleton");
    println!();
    println!("Usage:");
    println!("  emboss-rs <tool> ...");
    println!("  emboss-rs --help");
    println!("  emboss-rs --version");
    println!();
    println!("{}", runtime.status_line());
    println!("Plotting is delegated to the sister emboss-r project.");
}
