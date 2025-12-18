use crate::JSON_SCHEMA_VERSION;
use crate::analysis::project::MetricsReport;
use anyhow::Result;
use serde::Serialize;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Metadata about this tool and the JSON schema version.
#[derive(Debug, Serialize)]
struct ToolMeta {
    name: &'static str,
    version: &'static str,
    schema_version: u32,
}

/// JSON representation of a metrics report including tool metadata.
#[derive(Debug, Serialize)]
struct JsonReport<'a> {
    tool: ToolMeta,
    #[serde(flatten)]
    report: &'a MetricsReport,
}

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
///
/// The JSON includes a `tool` block with name, version, and schema_version.
pub fn write_json(report: &MetricsReport, output: Option<&Path>) -> Result<()> {
    let meta = ToolMeta {
        name: "noir-metrics",
        version: env!("CARGO_PKG_VERSION"),
        schema_version: JSON_SCHEMA_VERSION,
    };

    let wrapper = JsonReport { tool: meta, report };

    match output {
        Some(path) => {
            let file = File::create(path)?;
            serde_json::to_writer_pretty(file, &wrapper)?;
        }
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            serde_json::to_writer_pretty(&mut handle, &wrapper)?;
            writeln!(handle)?; // newline at the end
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::write_json;
    use crate::analysis::file::FileMetrics;
    use crate::analysis::project::{MetricsReport, ProjectTotals};
    use std::path::PathBuf;

    #[test]
    fn write_json_writes_a_file() {
        // Build a minimal report
        let report = MetricsReport {
            project_root: PathBuf::from("tests/fixtures/simple_noir"),
            totals: ProjectTotals {
                files: 1,
                total_lines: 1,
                code_lines: 1,
                non_test_lines: 1,
                functions: 1,
                non_test_functions: 1,
                files_with_main: 1,
                ..Default::default()
            },
            files: vec![FileMetrics {
                path: PathBuf::from("src/main.nr"),
                is_test_file: false,
                total_lines: 1,
                blank_lines: 0,
                comment_lines: 0,
                code_lines: 1,
                test_functions: 0,
                test_lines: 0,
                non_test_lines: 1,
                functions: 1,
                pub_functions: 0,
                non_test_functions: 1,
                has_main: true,
                todo_count: 0,
            }],
        };

        // Write to a unique temp file.
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let out_path = std::env::temp_dir().join(format!("noir_metrics_write_json_{unique}.json"));

        // If something already exists (unlikely), remove it.
        let _ = std::fs::remove_file(&out_path);

        write_json(&report, Some(&out_path)).expect("write_json should succeed");

        let s = std::fs::read_to_string(&out_path).expect("expected output json file to exist");

        // Key checks: if the mutant turns write_json into Ok(()), these will fail.
        assert!(
            s.contains("\"tool\""),
            "expected JSON to contain tool metadata"
        );
        assert!(
            s.contains("\"schema_version\""),
            "expected JSON to contain schema_version"
        );
        assert!(
            s.contains("\"project_root\""),
            "expected JSON to contain project_root"
        );

        let _ = std::fs::remove_file(&out_path);
    }
}
