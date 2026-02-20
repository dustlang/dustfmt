# CLI Reference

Source: `crates/dustfmt/src/main.rs`

## Invocation

```text
dustfmt [--check|-c] [files...]
```

## Flags

- `--check`: check-only mode
- `-c`: short alias for check-only mode

No other special flags are handled in current implementation.

## Modes

## File Mode

When one or more files are provided:

- normal mode: each file is formatted and rewritten in place
- check mode: formatting differences produce an error and exit code `1`

## STDIN Mode

When no files are provided:

- reads entire stdin
- normal mode: writes formatted text to stdout
- check mode: emits message and exits `1` when formatted output differs

## Exit Behavior

- `0`: successful formatting/check with no differences (in check mode)
- `1`: check failed, parse/lex/format error, file read/write error, or explicit process error path

## Diagnostics

Representative stderr output patterns:

- `format error: <details>`
- `<file>: format error: <details>`
- `check failed: input differs from formatted output`
- `check failed: <file> differs from formatted output`
