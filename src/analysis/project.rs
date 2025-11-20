use crate::analysis::file::{FileMetrics, analyze_file};
use crate::project::Project;
use anyhow::Result;
use serde::Serialize;
use std::path::PathBuf;

/// Aggregated metrics for a whole Noir project.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ProjectTotals {
    /// Number of `.nr` files in the project.
    pub files: usize,

    /// Total number of lines across all `.nr` files.
    pub total_lines: usize,

    /// Total blank lines across all `.nr` files.
    pub blank_lines: usize,

    /// Total comment lines across all `.nr` files.
    pub comment_lines: usize,

    /// Total code lines across all `.nr` files.
    pub code_lines: usize,

    /// Total number of `#[test]` functions across all files.
    pub test_functions: usize,

    /// Total code lines inside `#[test]` functions.
    pub test_lines: usize,

    /// Total code lines outside `#[test]` functions.
    pub non_test_lines: usize,

    /// Percentage of code lines that are test lines (0.0 if there is no code).
    pub test_code_percentage: f64,
}

/// Full metrics report for a project (for JSON & internal use).
#[derive(Debug, Clone, Serialize)]
pub struct MetricsReport {
    /// Absolute path to the project root.
    pub project_root: PathBuf,

    /// Aggregated totals over all `.nr` files in the project.
    pub totals: ProjectTotals,

    /// Per-file metrics for each discovered `.nr` file.
    pub files: Vec<FileMetrics>,
}

/// Analyze a project: collect per-file metrics and aggregate totals.
pub fn analyze_project(project: &Project) -> Result<MetricsReport> {
    let nr_files = project.nr_files()?;

    let mut files_metrics = Vec::new();
    for path in &nr_files {
        let metrics = analyze_file(path, &project.root)?;
        files_metrics.push(metrics);
    }

    let totals = compute_totals(&files_metrics);

    Ok(MetricsReport {
        project_root: project.root.clone(),
        totals,
        files: files_metrics,
    })
}

/// Compute project-level totals from per-file metrics
fn compute_totals(files: &[FileMetrics]) -> ProjectTotals {
    let mut totals = ProjectTotals::default();

    totals.files = files.len();

    for fm in files {
        totals.total_lines += fm.total_lines;
        totals.blank_lines += fm.blank_lines;
        totals.comment_lines += fm.comment_lines;
        totals.code_lines += fm.code_lines;
        totals.test_functions += fm.test_functions;
        totals.test_lines += fm.test_lines;
        totals.non_test_lines += fm.non_test_lines;
    }

    totals.test_code_percentage = if totals.code_lines == 0 {
        0.0
    } else {
        (totals.test_lines as f64 / totals.code_lines as f64) * 100.0
    };

    totals
}
