mod analysis;
mod cli;
mod output;
mod project;

use crate::analysis::project::analyze_project;
use crate::cli::Cli;
use crate::output::{print_human_summary, write_json};
use crate::project::Project;
use anyhow::Result;
use clap::Parser;

/// Entry point used by the binary.
pub fn run() -> Result<()> {
    let args = Cli::parse();

    let project = Project::from_root(args.project_root.clone())?;

    if args.verbose {
        eprintln!("noir-metrics (verbose)");
        eprintln!("  project_root: {}", project.root.display());
        eprintln!("  manifest: {}", project.manifest_path.display());
    }

    let report = analyze_project(&project)?;

    if args.verbose {
        eprintln!("  analyzed .nr files: {}", report.totals.files);

        if args.json {
            if let Some(ref out) = args.output {
                eprintln!("  mode: JSON -> {}", out.display());
            } else {
                eprintln!("  mode: JSON -> stdout");
            }
        } else {
            eprintln!("  mode: human summary -> stdout");
        }
    }

    if args.json {
        // Always print JSON to stdout
        write_json(&report, None)?;

        // If an output file is specified, also write JSON there
        if let Some(path) = args.output.as_deref() {
            write_json(&report, Some(path))?;
        }
    } else {
        print_human_summary(&report)?;
    }

    Ok(())
}
