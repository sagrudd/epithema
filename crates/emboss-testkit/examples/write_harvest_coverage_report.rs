//! Writes the harvest-coverage exceptions report as JSON and/or Markdown.

use std::path::PathBuf;

use emboss_testkit::{
    derive_harvest_coverage_report, render_harvest_coverage_markdown,
    write_harvest_coverage_markdown, write_harvest_coverage_report_json,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let mut json_output: Option<PathBuf> = None;
    let mut markdown_output: Option<PathBuf> = None;

    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--json" => {
                let path = args.next().ok_or("--json requires a path argument")?;
                json_output = Some(PathBuf::from(path));
            }
            "--markdown" => {
                let path = args.next().ok_or("--markdown requires a path argument")?;
                markdown_output = Some(PathBuf::from(path));
            }
            other => return Err(format!("unknown argument '{other}'").into()),
        }
    }

    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?;
    let report = derive_harvest_coverage_report(&repo_root)?;

    if let Some(path) = json_output {
        write_harvest_coverage_report_json(&report, path)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&report)?);
    }

    if let Some(path) = markdown_output {
        write_harvest_coverage_markdown(&report, path)?;
    } else {
        eprintln!("{}", render_harvest_coverage_markdown(&report));
    }

    Ok(())
}
