//! Writes the canonical cross-surface fixture catalogue to stdout.

use epithema_testkit::cross_surface::CrossSurfaceFixtureCatalog;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let catalog = CrossSurfaceFixtureCatalog::curated()?;
    println!("{}", serde_json::to_string_pretty(&catalog)?);
    Ok(())
}
