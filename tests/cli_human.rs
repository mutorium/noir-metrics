use assert_cmd::cargo::cargo_bin_cmd;
use std::path::PathBuf;

#[test]
fn cli_human_output_contains_summary() {
    let fixture = PathBuf::from("tests/fixtures/project_metrics");

    let mut cmd = cargo_bin_cmd!("noir-metrics");
    cmd.arg(&fixture);

    let output = cmd.assert().success().get_output().stdout.clone();
    let stdout = String::from_utf8_lossy(&output);

    // Summary section headers
    assert!(stdout.contains("Project:"), "stdout: {stdout}");
    assert!(stdout.contains("Files:"), "stdout: {stdout}");
    assert!(stdout.contains("Lines: total="), "stdout: {stdout}");
    assert!(stdout.contains("Functions: total="), "stdout: {stdout}");

    // Per-file section header
    assert!(stdout.contains("Per-file metrics:"), "stdout: {stdout}");

    // Per-file entries present
    assert!(stdout.contains("- src/main.nr"), "stdout: {stdout}");
    assert!(stdout.contains("- src/main2.nr"), "stdout: {stdout}");
    assert!(stdout.contains("- src/pub_todo.nr"), "stdout: {stdout}");

    // Aggregated values rendered in the summary line
    assert!(stdout.contains("TODOs=1"), "stdout: {stdout}");
    assert!(stdout.contains("pub_fns=1"), "stdout: {stdout}");
}
