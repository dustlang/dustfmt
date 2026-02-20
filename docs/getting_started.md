# Getting Started

## Prerequisites

- Rust toolchain with `cargo`.
- Dust source files (`.ds`) to format.

## Build

From `dustfmt/`:

```bash
cargo build --release
```

Binary path (Windows):

```text
target\release\dustfmt.exe
```

## Format Files In Place

```bash
cargo run -p dustfmt -- file1.ds file2.ds
```

Behavior:

- each listed file is read
- formatted output overwrites the same file
- stops on first formatting/parse/lex error

## Check Mode

```bash
cargo run -p dustfmt -- --check file.ds
```

or:

```bash
cargo run -p dustfmt -- -c file.ds
```

Behavior:

- no writes
- exits `1` if any file differs from canonical formatted output

## STDIN Mode

If no files are supplied, input is read from stdin.

```bash
cat file.ds | cargo run -p dustfmt --
```

- normal mode: prints formatted source to stdout
- check mode: compares stdin text to formatted result and exits `1` on mismatch
