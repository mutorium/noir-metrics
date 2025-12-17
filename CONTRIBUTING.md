# Contributing

Thanks for considering contributing!

## Development setup

- Install Rust (stable).
- Ensure `nargo` is installed and on your `PATH` (required for running `noir-metrics` on Noir projects).

## Quality gates

Before opening a PR, please run:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

## Tests and snapshots

- Prefer unit tests for pure functions.
- Prefer **insta snapshots** when output stability matters (CLI/report/JSON).
  - If a snapshot change is intentional: `cargo insta accept`.
- Integration tests should avoid depending on a real Noir toolchain when possible.

## Mutation testing (optional but encouraged)

We use `cargo-mutants` to harden the test suite:

```bash
cargo mutants
```

This is not required for every PR, but it should be run periodically and before releases.

## Commit style

Conventional commits are welcome, e.g.:

- `docs: add changelog`
- `test: add cli snapshot coverage`
- `feat: add baseline diff`
- `refactor: simplify report rendering`
