# Frontend Validation

Sources:

- `crates/dustfmt/src/frontend/lexer.rs`
- `crates/dustfmt/src/frontend/parser.rs`
- `crates/dustfmt/src/frontend/ast.rs`

## Purpose

Before formatting, `dustfmt` validates that source can be lexed and parsed by the embedded frontend parser.

## Validation Surface

`format_source` rejects input when:

- frontend lexing fails (`LexError`)
- frontend parsing fails (`ParseError`)

Error strings are normalized as:

- `lex error: <kind> at <start>..<end>`
- `parse error: <message> at <start>..<end>`

## Parser Coverage in This Crate

The embedded parser is intentionally limited in current state:

- supports forge parsing and top-level shorthand procs
- supports `proc` declarations and shorthand `K/Q/F name { ... }`
- statement support is narrow (`emit` statement path in parser)
- many keywords/features return explicit unsupported errors

Because validation runs before formatting, unsupported-but-lexable syntax can still fail format with parser errors.
