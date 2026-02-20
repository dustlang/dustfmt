# Formatting Rules (Current Implementation)

Source: `crates/dustfmt/src/format.rs`

## Indentation

- indentation level is tracked by `{` and `}` tokens
- each level uses 4 spaces

## Braces

- `{` is emitted on current line (with a preceding space if needed)
- `}` is emitted on its own line at decremented indentation

## Statement Breaks

- `;` forces line flush
- comments force current line flush before comment emission

## Commas and Colons

- `,` and `:` are followed by one space

## Line Normalization

- trailing whitespace is trimmed per emitted line
- final newline is always ensured (`result` ends with an empty final line)

## Newline Tokens

- lexer newline tokens are ignored by formatter logic; output newlines are structural

## Unknown Tokens

- unknown characters are emitted as standalone indented lines

## Spacing Heuristic

Spacing between tokens uses a local previous-token heuristic (for example around operators, delimiters, and scope operators). This is rule-based and not AST-aware.
