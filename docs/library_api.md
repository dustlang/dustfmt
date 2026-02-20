# Library API

Source: `crates/dustfmt/src/lib.rs`

## Public Function

```rust
pub fn format_source(src: &str) -> Result<String, String>
```

## Behavior

1. run embedded frontend lex+parse validation
2. run formatter lexer to preserve comments
3. return canonical formatted text

## Error Contract

Returns `Err(String)` for:

- lexing errors from frontend validation
- parse errors from frontend validation
- formatter lexer failures

## Usage Example

```rust
use dustfmt::format_source;

let formatted = format_source("forge F { }\n")?;
println!("{}", formatted);
```

## Stability Note

This crate exposes a minimal public API by design. Most behavior is driven through the CLI binary.
