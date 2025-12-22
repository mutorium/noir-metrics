use assert_cmd::cargo::cargo_bin_cmd;
use serde_json::Value;
use std::path::PathBuf;

#[test]
fn cli_json_output_snapshot() {
    let fixture = PathBuf::from("tests/fixtures/project_metrics");

    let mut cmd = cargo_bin_cmd!("noir-metrics");
    cmd.arg(&fixture).arg("--json");

    let assert = cmd.assert().success();

    let stdout = String::from_utf8(assert.get_output().stdout.clone()).expect("stdout is utf-8");

    let mut v: Value = serde_json::from_str(&stdout).expect("stdout is valid JSON");

    // Ensure tool.version stays correct, but don't bake it into the snapshot (it changes every release).
    let version = v["tool"]["version"]
        .as_str()
        .expect("tool.version should be a string");
    assert_eq!(version, env!("CARGO_PKG_VERSION"));
    v["tool"]["version"] = Value::String("<VERSION>".to_string());

    // project_root is absolute because Project::from_root canonicalizes.
    let project_root = v["project_root"]
        .as_str()
        .expect("project_root should be a string");
    assert!(
        project_root.ends_with("tests/fixtures/project_metrics"),
        "unexpected project_root: {project_root}"
    );
    v["project_root"] = Value::String("tests/fixtures/project_metrics".to_string());

    insta::assert_json_snapshot!(v);
}
