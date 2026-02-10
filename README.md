# dustfmt

`dustfmt` is the official formatter for programs written in the **Dust
Programming Language** (DPL).  It is responsible for turning a valid Dust
source file into a canonical textual representation.  The formatter is
deterministic and idempotent: formatting the same program repeatedly yields
the same result.  It never changes the semantics of the program – comments
are preserved and only whitespace is adjusted.

This repository provides a **Rust** implementation of `dustfmt`.  The
formatter is written in Rust to align with the Dust compiler and uses
the compiler's own lexer and parser to validate input before applying
formatting.  Previous experimental versions included a Python script,
but this version (v0.1) is entirely Rust‑based and should be used for
production and tooling integration.

The formatter adheres to the **v0** formatting rules outlined in the
original `dustfmt` specification【737876738800205†L0-L55】:

* Indentation uses **4 spaces** per block level; tabs are not emitted and
  any existing tab characters are treated as four spaces.
* Opening braces `{` remain on the **same line** as the construct they
  introduce, with a space separating the name and the brace【403632972283735†L285-L293】.
* Each syntactic item (statement, field, parameter, etc.) appears on its
  own line【737876738800205†L46-L54】.
* There is **no trailing whitespace** on any line.
* A final newline is always present at the end of the file【737876738800205†L46-L54】.

These rules apply to the entire Dust language, whose syntax and lexical
structure are defined in the DPL specification.  The specification notes
that source files are UTF‑8 encoded text files and that comments (line
`//` and block `/* … */`) are treated as whitespace【213087216729029†L0-L20】.  Our
formatter recognises these constructs and preserves them while reflowing
code.

## Usage

You can run the formatter via Rust's `cargo` or using the Python
interpreter.  Both support formatting files in place and checking
formatting without making changes.

### Rust

Format files in place:

```
cargo run -- file1.ds file2.ds
```

Check formatting (exit code 1 if formatting differs):

```
cargo run -- --check file.ds
```

If no file name is supplied or `-` is passed, the formatter reads from
standard input and writes the result to standard output.

<!--
The Python implementation from earlier prototypes has been removed in
v0.1.  All formatting should now be done via the Rust binary.  The
section below is intentionally left blank to preserve anchor links in
external documentation.
-->

## Implementation notes

The formatter uses two lexers under the hood.  First, it runs the
official Dust compiler lexer and parser (embedded in the `frontend`
module of this crate) to parse the input into an Abstract Syntax Tree.
If lexing or parsing fails, `dustfmt` reports a diagnostic and aborts
without modifying the file.  This validation step ensures that
formatting does not alter the meaning of a program and that only valid
Dust code is processed.

Once validated, `dustfmt` tokenises the source again with a second
lexer that preserves comments.  It then emits a canonical form by
walking the token stream, applying the indentation and spacing rules
above.  Comments are preserved verbatim and appear at the same block
indentation as the code that follows.  Invalid or unrecognised input
produces a diagnostic on stderr and leaves the original source
untouched.

Formatting in Dust is **structural rather than cosmetic** and is not
configurable【403632972283735†L248-L263】.  The Rust version aligns with
the rest of the Dust compiler, and this v0.1 release is intended to be
a production‑ready formatter for the DPL v0.1 language subset.

## License

This project is licensed under the **Dust Open Source License (DOSL)**.  See
the `LICENSE` file for details.