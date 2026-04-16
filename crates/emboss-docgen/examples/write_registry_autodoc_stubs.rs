//! Writes deterministic committed autodoc stub inputs for the exposed tool registry.

use std::path::PathBuf;

use emboss_docgen::{DEFAULT_AUTODOC_STUBS_ROOT, write_stub_catalog};

fn main() {
    let output_root = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_AUTODOC_STUBS_ROOT));

    let paths = write_stub_catalog(&output_root).expect("stub catalog should be written");
    for path in paths {
        println!("{}", path.display());
    }
}
