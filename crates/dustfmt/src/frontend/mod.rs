// File: mod.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   Dust Frontend module - lexing, parsing and AST for DPL v0.1.
//   Contains adapted copy of Dust compiler's frontend crates for dustfmt.
//   Exposes ast, lexer, and parser modules for input validation.
//   Code derived from dustlang/dust under DOSL, reproduced here for
//   bootstrapping without full compiler dependency.

pub mod ast;
pub mod lexer;
pub mod parser;
