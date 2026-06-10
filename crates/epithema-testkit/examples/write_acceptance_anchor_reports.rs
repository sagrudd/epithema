//! Writes executed-and-compared validation reports for the acceptance anchors.

use std::path::PathBuf;

use epithema_testkit::write_acceptance_anchor_reports;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let mut output_dir: Option<PathBuf> = None;

    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--output-dir" => {
                let path = args.next().ok_or("--output-dir requires a path argument")?;
                output_dir = Some(PathBuf::from(path));
            }
            other => return Err(format!("unknown argument '{other}'").into()),
        }
    }

    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?;
    let output_dir = output_dir.unwrap_or_else(|| repo_root.join("docs/generated/validation"));
    let written = write_acceptance_anchor_reports(&repo_root, &output_dir)?;

    for path in written {
        println!("{}", path.display());
    }

    Ok(())
}
