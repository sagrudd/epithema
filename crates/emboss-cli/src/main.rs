//! Command-line entry point for EMBOSS-RS.

mod app;
mod commands;
mod error;
mod output;

use std::process::ExitCode;

use app::CliApp;

fn main() -> ExitCode {
    CliApp::run()
}
