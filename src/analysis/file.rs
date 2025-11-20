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

    for line_result in reader.lines() {
        let line = line_result?;
        total_lines += 1;

        let trimmed = line.trim();

        if trimmed.is_empty() {
            blank_lines += 1;
        } else if trimmed.starts_with("//") {
            comment_lines += 1;
        } else {
            // todo: detect test logic
        }
    }

    let code_lines = total_lines.saturating_sub(blank_lines + comment_lines);

    let rel_path = path
        .strip_prefix(project_root)
        .unwrap_or(path)
        .to_path_buf();

    let is_test_file = is_test_file(&rel_path);

    // todo: implement test discovery
    let test_functions = 0;
    let test_lines = 0;
    let non_test_lines = code_lines;

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
