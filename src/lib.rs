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
use std::path::Path;

pub use crate::analysis::file::FileMetrics;
pub use crate::analysis::project::{MetricsReport, ProjectTotals};
pub use crate::project::Project as NoirProject;

/// Analyze a Noir project at the given root path.
///
/// This is the main entry point for *library* users.
pub fn analyze_path(root: &Path) -> Result<MetricsReport> {
    let project = Project::from_root(root.to_path_buf())?;
    analyze_project(&project)
}

/// Entry point used by the binary.
///
/// Parses CLI args, calls `analyze_path`, and then either prints a human
/// summary or writes JSON (and optionally saves it to a file).
pub fn run() -> Result<()> {
    let args = Cli::parse();

    if args.verbose {
        eprintln!("noir-metrics");
        eprintln!("  project_root: {}", args.project_root.display());
        eprintln!("  json: {}", args.json);
        eprintln!(
            "  output: {}",
            args.output
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "<stdout>".to_string())
        );
    }

    let report = analyze_path(&args.project_root)?;

    if args.json {
        write_json(&report, args.output.as_deref())?;
    } else {
        print_human_summary(&report)?;
    }

    Ok(())
}
