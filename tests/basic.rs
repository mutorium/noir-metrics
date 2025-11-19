use assert_cmd::cargo::cargo_bin_cmd;
use std::path::PathBuf;

#[test]
fn lists_nr_files_for_simple_fixture() {
    let fixture_root = PathBuf::from("tests/fixtures/simple_noir");

    let mut cmd = cargo_bin_cmd!("noir-metrics");

    cmd.arg(&fixture_root);

    let output = cmd.assert().success().get_output().stdout.clone();

    let stdout = String::from_utf8_lossy(&output);

    assert!(
        stdout.contains("src/main.nr"),
        "expected output to contain src/main.nr, got: {stdout:?}"
    );
}
