# dustfmt

`dustfmt` is now expressed in the currently supported top-level Dust grammar profile:

- single executable unit: `src/main.ds`
- entrypoint form: `K main { ... }`

This profile is designed to build with the current `dust` compiler parser without forge/proc namespace syntax.

## Build

From `dust/` compiler workspace:

```bash
CARGO_HOME=/tmp/cargo-home cargo run -p dust -- build ../dustfmt/src --out ../dustfmt/target/dust/dustfmt
```

## Notes

- The previous Rust crate layout (`Cargo.toml`, `Cargo.lock`, and `crates/dustfmt`) was retired for Dust-native migration.
- Existing docs are retained for formatter intent and roadmap continuity.
