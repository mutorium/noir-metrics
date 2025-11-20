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

    ///Heuristic: is this file considered a "test" file?
    pub is_test_file: bool,

    /// Total number of lines in the file (including blank and comment lines).
    pub total_lines: usize,

    /// Lines that are empty or only whitespace.
    pub blank_lines: usize,

    /// Lines that are comments (starting with `//` after trimming).
    pub comment_lines: usize,

    /// Lines that are considered code:
    /// total_lines - blank_lines - comment_lines
    pub code_lines: usize,

    /// Number of functions annotated with `#[test]`.
    pub test_functions: usize,

    /// Number of code lines inside `#[test]` functions.
    pub test_lines: usize,

    /// Number of code lines outside tests: code_lines - test_lines.
    pub non_test_lines: usize,
}

/// Analyze a single `.nr` file and compute basic line metrics.
///
/// Current implementation:
/// - We count total / blank / comment / code lines
/// - Test-related fields are stubbed (0) and will be implemented later.
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

    let mut pending_test_attr = false;
    let mut inside_test = false;
    let mut brace_depth: i32 = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        total_lines += 1;

        let trimmed = line.trim();

        let mut is_test_attr_line = false;

        if trimmed.starts_with("#[test") {
            pending_test_attr = true;
            is_test_attr_line = true;
        }

        if pending_test_attr && (trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ")) {
            test_functions += 1;
            inside_test = true;
            pending_test_attr = false;
            brace_depth = 0;
        }

        if trimmed.is_empty() {
            blank_lines += 1;
        } else if trimmed.starts_with("//") {
            comment_lines += 1;
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

    if let Some(file_name) = rel_path.file_name().and_then(|s| s.to_str()) {
        if file_name.ends_with("_test.nr") {
            return true;
        }
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

        assert_eq!(metrics.total_lines, 19, "total_lines");
        assert_eq!(metrics.blank_lines, 3, "blank_lines");
        assert_eq!(metrics.comment_lines, 3, "comment_lines");
        assert_eq!(metrics.code_lines, 13, "code_lines");

        assert_eq!(metrics.test_functions, 2, "test_functions");
        assert_eq!(metrics.test_lines, 8, "test_lines");
        assert_eq!(metrics.non_test_lines, 5, "non_test_lines");

        assert_eq!(
            metrics.code_lines,
            metrics.test_lines + metrics.non_test_lines,
            "code_lines should equal test_lines + non_test_lines",
        );
    }
}
