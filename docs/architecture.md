# Architecture

## Crate Layout

- `src/main.rs`: CLI and file/stdin orchestration.
- `src/lib.rs`: public formatting API (`format_source`).
- `src/format.rs`: token-to-text canonical formatter.
- `src/lexer.rs`: formatter lexer that preserves comments.
- `src/frontend/*`: embedded frontend lexer/parser/AST used for validation.

## Pipeline

`format_source(src)` executes two phases:

1. **Validation phase**
   - run `frontend::lexer::Lexer::lex_all`
   - run `frontend::parser::Parser::parse_file`
   - fail early on lex/parse errors

2. **Formatting phase**
   - run formatter lexer (`src/lexer.rs`) to retain comments
   - run `format::format_tokens`
   - emit canonical formatted text with trailing newline

## Design Intent

- avoid semantic changes by requiring syntactically valid input
- preserve comments while normalizing whitespace/layout
- deterministic, idempotent formatting output

## Implementation Note

The embedded frontend module is a copied/adjusted parser stack inside this crate rather than an external dependency on the full `dust` compiler crate.
