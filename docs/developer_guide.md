# Developer Guide

## Build

```bash
cargo build --workspace --verbose
```

## Test

```bash
cargo test --workspace --verbose
```

## Quick Manual Checks

```bash
cargo run -p dustfmt -- --check crates/dustfmt/tests/fixtures/sample.ds
cargo run -p dustfmt -- crates/dustfmt/tests/fixtures/sample.ds
```

(Adjust file paths to local fixtures that exist.)

## Suggested Workflow for Formatter Changes

1. update `src/lexer.rs` or `src/format.rs`
2. run tests
3. add/update tests in `crates/dustfmt/tests/format.rs`
4. update docs in `dustfmt/docs`

## Priority Fix Areas

- align line-comment behavior with intended preservation (`//` fidelity)
- expand parser coverage to match formatter keyword surface
- improve CLI diagnostics for file-system failures
