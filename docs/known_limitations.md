# Known Limitations

## Parser Coverage vs Keyword Surface

`src/lexer.rs` recognizes many keywords, but embedded parser coverage is narrower. Formatting can fail on valid-looking inputs when parser support is incomplete.

## Comment Tokenization Quirk

Current formatter lexer comment path consumes one `/` before line-comment capture, and existing tests encode this behavior. This can produce non-ideal comment rendering.

## Formatting Is Token-Heuristic, Not Full AST Pretty-Print

`format_tokens` applies local token spacing/newline rules; it does not perform deep AST-aware layout decisions.

## Limited CLI Feature Set

Only `--check`/`-c` is implemented. There are no configuration files, width options, or selective rule toggles in current code.

## Error Handling Style

Several file I/O paths use `expect(...)`, resulting in immediate process termination on file read/write failures rather than structured recoverable diagnostics.
