//! Command-line entry point for the initial EMBOSS-RS workspace skeleton.

use std::env;
use std::process::ExitCode;

use emboss_service::ServiceRuntime;

fn main() -> ExitCode {
    let runtime = ServiceRuntime::new();
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
            eprintln!(
                "emboss-rs: '{command}' is not implemented yet.\n{}",
                runtime.status_line()
            );
            ExitCode::from(2)
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
