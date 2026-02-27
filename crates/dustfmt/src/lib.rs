// File: lib.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   Dustfmt library - public API for the Dust formatter.
//   This module:
//     - Re-exports formatter functions for integration tests
//     - Provides format_source() entry point
//     - Uses compiler frontend for validation
//     - Runs formatter on separate lexer for idempotence

mod frontend;
use frontend::lexer as comp_lexer;
use frontend::parser as comp_parser;

mod lexer;
use lexer::Lexer;
use lexer::Token;

mod format;
use format::format_tokens;

pub fn format_source(src: &str) -> Result<String, String> {
    // First, use the compiler frontend lexer/parser to validate the
    // program.  If the source cannot be lexed or parsed, return an
    // error.  The formatter itself runs on a separate lexer that
    // preserves comments.
    match comp_lexer::Lexer::new(src).lex_all() {
        Ok(toks) => {
            let mut parser = comp_parser::Parser::new(toks);
            if let Err(e) = parser.parse_file() {
                return Err(format!("parse error: {}", e));
            }
        }
        Err(e) => {
            return Err(format!(
                "lex error: {:?} at {}..{}",
                e.kind, e.span.start, e.span.end
            ));
        }
    }
    // Use the formatter's lexer to produce a token stream with comments.
    let mut fmt_lexer = Lexer::new(src);
    let tokens = fmt_lexer.lex()?;
    Ok(format_tokens(&tokens))
}
