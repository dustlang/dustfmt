# dustfmt Documentation

This directory contains complete Markdown documentation for `dustfmt`.

## Documentation Index

- `getting_started.md`: build, install, and first formatting runs.
- `cli_reference.md`: command-line contract and exit behavior.
- `architecture.md`: formatter pipeline and module boundaries.
- `formatting_rules.md`: canonical formatting behavior implemented today.
- `frontend_validation.md`: parser/lexer validation path used before formatting.
- `formatter_lexer.md`: comment-preserving formatter lexer behavior.
- `library_api.md`: public Rust API (`format_source`).
- `testing.md`: current test coverage and test caveats.
- `known_limitations.md`: implementation limits and behavior mismatches to track.
- `developer_guide.md`: local workflow for contributors.

## Scope

`dustfmt` is the Rust formatter for Dust source files. It validates input through an embedded frontend parser, then re-lexes source with a formatter-specific lexer to preserve comments while normalizing layout.

This documentation describes current behavior from source in `crates/dustfmt/src`.
