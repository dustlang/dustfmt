# dustfmt Documentation

This directory contains Markdown documentation for `dustfmt`.

## Documentation Index

- `getting_started.md`: build, install, and first formatting runs.
- `cli_reference.md`: command-line contract and exit behavior.
- `architecture.md`: formatter pipeline and module boundaries.
- `formatting_rules.md`: canonical formatting behavior and future profile intent.
- `frontend_validation.md`: parser/lexer validation path notes.
- `formatter_lexer.md`: comment-preserving lexer behavior notes.
- `library_api.md`: legacy API notes from pre-migration Rust implementation.
- `testing.md`: test strategy and migration caveats.
- `known_limitations.md`: implementation limits and behavior gaps to track.
- `developer_guide.md`: local workflow for contributors.

## Scope

`dustfmt` now ships as a Dust-native top-level grammar profile (`src/main.ds`).
Historical docs that reference the retired Rust crate layout are retained for roadmap context.
