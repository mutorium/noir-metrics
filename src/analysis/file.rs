use anyhow::Result;
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

/// Metrics computed for a single `.nr` file.
#[derive(Debug, Clone, Serialize)]
pub struct FileMetrics {
    /// Path to the file, relative to the project root
    pub path: PathBuf,

    /// Heuristic: is this file considered a "test" file?
    pub is_test_file: bool,

    /// Total number of lines in the file (including blank and comment lines).
    pub total_lines: usize,

    /// Lines that are empty or only whitespace.
    pub blank_lines: usize,

    /// Lines that are comments:
    /// - starting with `//` after trimming, or
    /// - inside `/* ... */` block comments.
    pub comment_lines: usize,

    /// Lines that are considered code (everything that's not blank or comment).
    pub code_lines: usize,

    /// Number of functions annotated with `#[test...]` (including #[test(should_fail)] variants).
    pub test_functions: usize,

    /// Number of code lines inside `#[test]` functions.
    pub test_lines: usize,

    /// Number of code lines outside tests: code_lines - test_lines.
    pub non_test_lines: usize,

    /// Total number of functions (`fn` and `pub fn`) in this file.
    pub functions: usize,

    /// Number of `pub fn` (public functions) in this file.
    pub pub_functions: usize,

    /// Number of non-test functions (i.e. functions that are not tests).
    pub non_test_functions: usize,

    /// Does this file define a `main` function?
    pub has_main: bool,

    /// Number of TODO/FIXME markers in comment lines.
    pub todo_count: usize,
}

/// Analyze a single `.nr` file and compute basic line metrics.
pub fn analyze_file(path: &Path, project_root: &Path) -> Result<FileMetrics> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut total_lines = 0usize;
    let mut blank_lines = 0usize;
    let mut comment_lines = 0usize;
    let mut code_lines = 0usize;

    let mut test_functions = 0usize;
    let mut test_lines = 0usize;
    let mut non_test_lines = 0usize;

    let mut functions = 0usize;
    let mut pub_functions = 0usize;
    let mut non_test_functions = 0usize;
    let mut has_main = false;
    let mut todo_count = 0usize;

    let mut pending_test_attr = false;
    let mut inside_test = false;
    let mut brace_depth: i32 = 0;
    let mut in_block_comment = false;

    for line_result in reader.lines() {
        let line = line_result?;
        total_lines += 1;

        let trimmed = line.trim();

        if in_block_comment {
            comment_lines += 1;

            if line_has_todo(trimmed) {
                todo_count += 1;
            }

            if trimmed.contains("*/") {
                in_block_comment = false;
            }
            continue;
        }

        if trimmed.starts_with("/*") {
            comment_lines += 1;

            if line_has_todo(trimmed) {
                todo_count += 1;
            }

            if !trimmed.contains("*/") {
                in_block_comment = true;
            }
            continue;
        }

        let mut is_test_attr_line = false;

        if trimmed.starts_with("#[test") {
            pending_test_attr = true;
            is_test_attr_line = true;
        }

        let is_fn_line = trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ");
        let is_pub_fn = trimmed.starts_with("pub fn ");

        if is_fn_line {
            functions += 1;
            if is_pub_fn {
                pub_functions += 1;
            }

            if pending_test_attr {
                test_functions += 1;
                inside_test = true;
                pending_test_attr = false;
                brace_depth = 0;
            } else {
                non_test_functions += 1;
            }

            if trimmed.starts_with("fn main(") || trimmed.starts_with("pub fn main(") {
                has_main = true;
            }
        }

        if trimmed.is_empty() {
            blank_lines += 1;
        } else if trimmed.starts_with("//") {
            comment_lines += 1;

            if line_has_todo(trimmed) {
                todo_count += 1;
            }
        } else {
            code_lines += 1;

            if inside_test || is_test_attr_line {
                test_lines += 1;
            } else {
                non_test_lines += 1;
            }
        }

        let braces_delta = count_braces(&line);
        brace_depth += braces_delta;

        if inside_test && brace_depth == 0 {
            inside_test = false;
        }
    }

    let rel_path = path
        .strip_prefix(project_root)
        .unwrap_or(path)
        .to_path_buf();

    let is_test_file = is_test_file(&rel_path);

    Ok(FileMetrics {
        path: rel_path,
        is_test_file,
        total_lines,
        blank_lines,
        comment_lines,
        code_lines,
        test_functions,
        test_lines,
        non_test_lines,
        functions,
        pub_functions,
        non_test_functions,
        has_main,
        todo_count,
    })
}

/// Count the net number of braces on a line: `{` as +1, `}` as -1.
fn count_braces(line: &str) -> i32 {
    let mut delta = 0i32;

    for ch in line.chars() {
        match ch {
            '{' => delta += 1,
            '}' => delta -= 1,
            _ => {}
        }
    }

    delta
}

/// Check if a string contains todo or fixme
fn line_has_todo(s: &str) -> bool {
    let lower = s.to_lowercase();
    lower.contains("todo") || lower.contains("fixme")
}

/// Heuristic to decide if a file is a "test file".
///
/// Rules:
/// - If any path component is exactly "tests" or "test" return true.
/// - If the file name ends with `_test.nr`, return true.
fn is_test_file(rel_path: &Path) -> bool {
    if rel_path
        .components()
        .any(|c| matches!(c.as_os_str().to_str(), Some("tests" | "test")))
    {
        return true;
    }

    if let Some(file_name) = rel_path.file_name().and_then(|s| s.to_str())
        && file_name.ends_with("_test.nr")
    {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn counts_lines_and_tests_exactly() {
        let project_root = PathBuf::from("tests/fixtures/file_metrics");
        let path = project_root.join("src/metrics.nr");

        let metrics = analyze_file(&path, &project_root).expect("analyze_file should succeed");

        assert_eq!(metrics.total_lines, 23, "total_lines");
        assert_eq!(metrics.blank_lines, 3, "blank_lines");
        assert_eq!(metrics.comment_lines, 7, "comment_lines");
        assert_eq!(metrics.code_lines, 13, "code_lines");

        assert_eq!(metrics.test_functions, 2, "test_functions");
        assert_eq!(metrics.test_lines, 8, "test_lines");
        assert_eq!(metrics.non_test_lines, 5, "non_test_lines");

        assert_eq!(metrics.functions, 3, "functions");
        assert_eq!(metrics.pub_functions, 0, "pub_functions");
        assert_eq!(metrics.non_test_functions, 1, "non_test_functions");
        assert!(metrics.has_main, "has_main should be true");

        assert_eq!(metrics.todo_count, 3, "todo_count");

        assert_eq!(
            metrics.code_lines,
            metrics.test_lines + metrics.non_test_lines,
            "code_lines should equal test_lines + non_test_lines",
        );
    }
}
