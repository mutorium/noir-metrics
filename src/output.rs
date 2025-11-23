use crate::analysis::project::MetricsReport;
use anyhow::Result;
use serde_json;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Print a human-readable summary to stdout.
pub fn print_human_summary(report: &MetricsReport) -> Result<()> {
    println!("Project: {}", report.project_root.display());
    println!("Files: {}", report.totals.files);
    println!(
        "Lines: total={}, code={}, comments={}, blanks={}, test={}, non-test={}, test_functions={}, test_code={:.2}%",
        report.totals.total_lines,
        report.totals.code_lines,
        report.totals.comment_lines,
        report.totals.blank_lines,
        report.totals.test_lines,
        report.totals.non_test_lines,
        report.totals.test_functions,
        report.totals.test_code_percentage,
    );
    println!(
        "Functions: total={}, pub={}, non-test={}, files_with_main={}, TODOs={}",
        report.totals.functions,
        report.totals.pub_functions,
        report.totals.non_test_functions,
        report.totals.files_with_main,
        report.totals.todo_count,
    );
    println!();

    println!("Per-file metrics:");
    for file in &report.files {
        println!(
            "- {} (total={}, code={}, comments={}, blanks={}, tests={}, non-test={}, test_functions={}, fns={}, pub_fns={}, todos={}, is_test_file={})",
            file.path.display(),
            file.total_lines,
            file.code_lines,
            file.comment_lines,
            file.blank_lines,
            file.test_lines,
            file.non_test_lines,
            file.test_functions,
            file.functions,
            file.pub_functions,
            file.todo_count,
            file.is_test_file,
        );
    }

    Ok(())
}

/// Write the metrics report as pretty JSON to either stdout or a file.
pub fn write_json(report: &MetricsReport, output: Option<&Path>) -> Result<()> {
    match output {
        Some(path) => {
            let file = File::create(path)?;
            serde_json::to_writer_pretty(file, report)?;
        }
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            serde_json::to_writer_pretty(&mut handle, report)?;
            writeln!(handle)?;
        }
    }

    Ok(())
}
