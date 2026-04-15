//! Output helpers for the `emboss-rs` command surface.

use emboss_service::{EmbossService, InvocationResponse};

/// Prints the current governed tool catalogue.
pub fn print_tool_list(service: &EmbossService) {
    println!("EMBOSS-RS governed tool catalogue");
    println!("{}", service.status_line());

    if service.descriptors().is_empty() {
        println!("No governed tools are registered yet.");
        return;
    }

    for descriptor in service.descriptors() {
        println!("{:<16} {}", descriptor.name, descriptor.summary);
    }
}

/// Prints the current placeholder response for a known but unimplemented tool.
pub fn print_unimplemented_tool(response: &InvocationResponse, service: &EmbossService) {
    println!(
        "Tool '{}' is governed but not implemented yet.",
        response.tool
    );
    println!("{}", response.descriptor.summary);
    println!("{}", service.status_line());
}
