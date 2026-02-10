//! Dust Frontend - lexing, parsing and AST for DPL v0.1
//!
//! This module contains a lightly adapted copy of the Dust compiler's
//! frontend crates.  It exposes the `ast`, `lexer` and `parser` modules
//! used by `dustfmt` to validate input source files before formatting.
//!
//! The code in this module is derived from the `dustlang/dust` project
//! under the Dust Open Source License.  It is reproduced here so that
//! `dustfmt` can depend on a stable implementation of the DPL grammar
//! without pulling in the full compiler as a crate dependency.  Only
//! minimal changes have been made to make the modules compile within
//! this crate (e.g. removing `pub` re‑exports and adjusting paths).

pub mod ast;
pub mod lexer;
pub mod parser;