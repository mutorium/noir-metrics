//! Source code metrics for Noir (Nargo) projects.
//!
//! `noir-metrics` scans a project directory containing a `Nargo.toml`, walks all `.nr` files,
//! and computes per-file and project-level metrics.
//!
//! # CLI
//!
//! ```text
//! noir-metrics .
//! noir-metrics . --json
//! noir-metrics . --json --output metrics.json
//! ```
//!
//! # Library
//!
//! ```no_run
//! use std::path::Path;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let report = noir_metrics::analyze_path(Path::new("."))?;
//!     println!("Total code lines: {}", report.totals.code_lines);
//!     Ok(())
//! }
//! ```
//!
//! # JSON schema versioning
//!
//! JSON output includes a `tool.schema_version` field. The current schema version is
//! [`JSON_SCHEMA_VERSION`].

mod analysis;
mod cli;
mod output;
mod project;

use crate::analysis::project::analyze_project;
use crate::cli::{Cli, OutputFormat};
use crate::output::{print_human_summary, write_json};
use crate::project::Project;
use anyhow::{Result, bail};
use clap::Parser;
use std::path::Path;

pub use crate::analysis::file::FileMetrics;
pub use crate::analysis::project::{MetricsReport, ProjectTotals};

/// Noir project handle (re-export of the internal [`project::Project`] type).
pub use crate::project::Project as NoirProject;

/// JSON schema version for the noir-metrics report format.
///
/// Bump this when making breaking changes to the JSON layout.
pub const JSON_SCHEMA_VERSION: u32 = 1;

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

    let format = match (args.format, args.json) {
        (Some(f), false) => f,
        (None, true) => OutputFormat::Json,
        (Some(_), true) => bail!("flags --format and --json cannot be used together"),
        (None, false) => OutputFormat::Human,
    };

    if args.output.is_some() && !matches!(format, OutputFormat::Json) {
        bail!("--output requires JSON output (use --format json)");
    }

    if args.verbose {
        eprintln!("noir-metrics");
        eprintln!("  project_root: {}", args.project_root.display());
        eprintln!("  format: {:?}", format);
        eprintln!(
            "  output: {}",
            args.output
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "<stdout>".to_string())
        );
    }

    let report = analyze_path(&args.project_root)?;

    match format {
        OutputFormat::Json => write_json(&report, args.output.as_deref())?,
        OutputFormat::Human => print_human_summary(&report)?,
    }

    Ok(())
}
