//! Command handlers for the `emboss-rs` CLI.

use emboss_service::EmbossService;

use crate::output;

/// Runs the `list` command against the shared service registry.
pub fn run_list(service: &EmbossService) {
    output::print_tool_list(service);
}
