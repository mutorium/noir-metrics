# noir-metrics

Source code metrics for [Noir](https://noir-lang.org/) projects.

`noir-metrics` scans a Nargo project (looks for `Nargo.toml`), walks all `.nr` files, and computes metrics that are useful for Noir developers, auditors, and tooling.

> **Status:** This project is still under active development. The core API and JSON schema may evolve. Expect breaking changes before `1.0.0`.

It is designed to be:

- a **CLI tool** you can run in CI or locally,
- a **library** you can embed,
- a potential building block for future integrations like `nargo metrics`.

---

## Features

Current metrics (per file and project-level totals):

- Basic line stats:
  - `total_lines`, `blank_lines`, `comment_lines`, `code_lines`
- Test-related:
  - `test_functions` (functions annotated with `#[test...]`)
  - `test_lines` vs `non_test_lines`
  - heuristic `is_test_file` flag
- Function surface:
  - total `functions`, `pub_functions`, `non_test_functions`
  - `has_main` and `files_with_main`
- Inline documentation:
  - `todo_count` (TODO/FIXME markers in comments or code)

The report is exposed both as:

- a **human-readable summary** (default), and  
- a **machine-readable JSON** document with a versioned schema.

---

## Installation

### From crates.io

```bash
cargo install noir-metrics
```

This will install a `noir-metrics` binary into your Cargo bin directory.

### From source

Clone the repository and install from the local checkout:

```bash
git clone https://github.com/mutorium/noir-metrics.git
cd noir-metrics
cargo install --path .
```

---

## CLI usage

Run `noir-metrics` in a Nargo project (directory containing `Nargo.toml`):

```bash
# Human-readable summary (default)
noir-metrics .

# JSON output to stdout
noir-metrics . --json

# JSON output to stdout and also save to a file
noir-metrics . --json --output metrics.json
```

Available flags:

- `PROJECT_ROOT` (positional): path to the Noir project (default: `.`)
- `--json`: output JSON instead of a human-readable summary
- `--output <PATH>`: also write JSON to the given file
- `-v, --verbose`: print additional debug info to stderr

Example (verbose JSON run):

```bash
noir-metrics . --json --output metrics.json --verbose
```

---

## JSON output

When run with `--json`, `noir-metrics` emits a JSON document of the form:

```json
{
  "tool": {
    "name": "noir-metrics",
    "version": "0.1.0",
    "schema_version": 1
  },
  "project_root": "path/to/project",
  "totals": {
    "files": 2,
    "total_lines": 42,
    "blank_lines": 5,
    "comment_lines": 10,
    "code_lines": 27,
    "test_functions": 3,
    "test_lines": 12,
    "non_test_lines": 15,
    "functions": 5,
    "pub_functions": 1,
    "non_test_functions": 2,
    "todo_count": 1,
    "files_with_main": 1,
    "test_code_percentage": 44.44
  },
  "files": [
    {
      "path": "src/main.nr",
      "is_test_file": false,
      "total_lines": 20,
      "blank_lines": 2,
      "comment_lines": 3,
      "code_lines": 15,
      "test_functions": 1,
      "test_lines": 5,
      "non_test_lines": 10,
      "functions": 2,
      "pub_functions": 0,
      "non_test_functions": 1,
      "has_main": true,
      "todo_count": 0
    }
    // ...
  ]
}
```

> **Schema version:** The `tool.schema_version` field is also available as the Rust constant `JSON_SCHEMA_VERSION` and is incremented when breaking changes are made to the JSON layout. New fields may be added without bumping the schema version.

---

## Library usage (embedding in other tools)

You can use `noir-metrics` as a library from other Rust crates.

Add it as a dependency:

```toml
[dependencies]
noir-metrics = "0.1"
```

Then call the library API:

```rust
use noir_metrics::{analyze_path, MetricsReport};
use std::path::Path;

fn run_metrics(root: &Path) -> anyhow::Result<MetricsReport> {
    let report = analyze_path(root)?;
    println!("Total code lines: {}", report.totals.code_lines);
    Ok(report)
}
```

Core exported types:

- `analyze_path(&Path) -> Result<MetricsReport>`
- `MetricsReport` (project_root, totals, per-file metrics)
- `ProjectTotals`
- `FileMetrics`
- `NoirProject` (re-export of the internal `Project` type)
- `JSON_SCHEMA_VERSION`

---

## Heuristics and limitations

This is a line-based analyzer with Noir-aware heuristics:

- Comments:
  - `//` line comments and `/* ... */` block comments are counted as comment lines.
- Tests:
  - Functions annotated with `#[test]`, `#[test(should_fail)]`, or other `#[test(...)]` forms are treated as tests.
  - Test code is counted between the function’s opening brace and the point where the brace depth returns to zero.
- Test files:
  - A file is considered a “test file” if:
    - any path component is `tests` or `test`, or
    - the file name ends with `_test.nr`.

This tool does not parse Noir’s full AST (yet), so complex edge cases may not be classified perfectly. The goal is to provide useful, cheap metrics that are good enough for:

- understanding project size and test coverage at a glance,
- feeding into higher-level tooling such as mutation testing.

---

## Roadmap

Planned / possible future work:

- Control-flow and complexity metrics:
  - counts of `if`, `for`/`while` loops, `match`
- More Noir-specific metrics:
  - constraint and assert density
  - better distinction between public entrypoints and helpers
- Configuration:
  - include/exclude patterns (e.g. `--exclude target`)
- Deeper integration:
  - potential `nargo metrics` subcommand built on this crate

---

## License

MIT

See [LICENSE](LICENSE).
