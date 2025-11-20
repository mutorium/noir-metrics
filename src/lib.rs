mod analysis;
mod cli;
mod project;

use crate::analysis::file::analyze_file;
use crate::cli::Cli;
use crate::project::Project;
use anyhow::Result;
use clap::Parser;

/// Entry point used by the binary.
pub fn run() -> Result<()> {
    let args = Cli::parse();

    let project = Project::from_root(args.project_root.clone())?;

    if args.verbose {
        eprintln!("noir-metrics");
        eprintln!("  project_root: {}", project.root.display());
        eprintln!("  manifest: {}", project.manifest_path.display());
    }

    let nr_files = project.nr_files()?;

    if args.verbose {
        eprintln!("Found {} .nr files", nr_files.len());
    }

    // Temporary stub: print each .nr file relative to the project root.
    for path in nr_files {
        let metrics = analyze_file(&path, &project.root)?;

        println!("{}", metrics.path.display());

        if args.verbose {
            eprintln!(
                " -> toatl={}, code={}, comments={}, blanks={}, is_test_file={}",
                metrics.total_lines,
                metrics.code_lines,
                metrics.comment_lines,
                metrics.blank_lines,
                metrics.is_test_file,
            )
        }
    }

    Ok(())
}
