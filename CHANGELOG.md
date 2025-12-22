# Changelog

All notable changes to this project will be documented in this file.

We aim to follow Semantic Versioning. While <1.0, we try to keep minor releases backwards compatible when reasonable.

## [Unreleased]

### Added
- —

### Changed
- —

### Fixed
- —


## [0.2.0] - 2025-12-22

### Added
- `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `SECURITY.md`.
- `CHANGELOG.md`.
- GitHub Actions CI workflow (fmt, clippy, test).
- `.gitignore` entry to ignore `cargo-mutants` artifacts (`/mutants.out*`).
- Snapshot testing with `insta` to stabilize metrics expectations.
- Cargo-mutants configuration to exclude an equivalent mutant.
- Integration test that snapshots CLI `--json` output.
- Integration test for CLI `--json --output` file writing.
- Integration test that validates human-readable CLI output structure.

### Changed
- Expanded and adjusted test fixtures to cover additional metrics paths (e.g., `pub fn`, block TODOs).
- Improved Rustdoc: added crate-level and API documentation for core types and behavior.
- Documented CLI output format via `--format (human|json)`; `--json` remains a hidden alias.

### Fixed
- Clippy warnings in analysis and output modules (no functional changes intended).
- Strengthened tests guided by `cargo-mutants` (all non-equivalent mutants now caught).
- Deterministic ordering of discovered `.nr` files for stable output/tests.


## [0.1.0] - 2025-11-25
- Initial release.
