# dustfmt

dustfmt is the official formatter for DPL (Dust Programming Language).

It provides a canonical, deterministic formatting pass for Dust source code,
ensuring that any valid DPL program has exactly one formatted representation.

dustfmt is designed to be:
- AST-driven (never text-heuristic based)
- Opinionated and stable (no style flags in v1)
- Bootstrap-safe (usable by Dust tooling itself)
- Lossless (comments preserved, semantics unchanged)

---

## Philosophy

dustfmt follows the same design principles as the Dust toolchain:

- Formatting is not configurable (at least initially)
- Formatting is structural, not cosmetic
- Formatting is idempotent
- Formatting must never alter program meaning

If two Dust programs format to the same output, they are structurally equivalent.

---

## Usage

Format a file in place:

dustfmt file.dust

Check formatting without modifying files:

dustfmt --check file.dust

Read from stdin and write to stdout:

dustfmt -

---

## Formatting Rules (v0)

- 4-space indentation
- Tabs are forbidden
- Braces on the same line
- One item per line
- No trailing whitespace
- Final newline always present

Example:

fn main() {
    let x = 1
}

---

## Relationship to the DPL Specification

The Dust Programming Language specification is the source of truth for syntax
and semantics.

dustfmt:
- relies on the Dust parser to build an AST
- formats only valid DPL programs
- does not attempt error recovery or partial formatting

---

## Status

dustfmt is under active development.

Initial versions focus on:
- modules
- functions
- basic expressions
- block structure
- comment preservation

---

## License

This project is licensed under the Dust Open Source License (DOSL).

See the LICENSE file for the full license text.