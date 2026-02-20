# Testing

Source: `crates/dustfmt/tests/format.rs`

## Current Tests

- `simple_forge`: basic brace normalization.
- `preserves_line_comments`: verifies comment output behavior.
- `invalid_syntax_reports_error`: validates parse-error propagation.

## Test Characteristics

- tests call public `dustfmt::format_source`
- newline normalization helper avoids CRLF/LF mismatch noise

## Important Observation

The comment preservation test currently expects transformed comment text (`"/ comment"`) rather than canonical `"// comment"`, reflecting current lexer behavior rather than intended spec-level formatting.

## Running Tests

From `dustfmt/`:

```bash
cargo test --workspace --verbose
```
