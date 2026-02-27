// File: main.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   dustfmt – official formatter for DPL (Dust Programming Language).
//   This Rust implementation:
//     - Scans input text into tokens, rebuilds in canonical form
//     - Preserves comments, 4-space indentation, brace placement
//     - Each syntactic item on its own line
//     - Removes trailing whitespace, ensures final newline
//   NOTE: Deliberately avoids external dependencies for bootstrapping.

use std::env;
use std::fs;
use std::io::{self, Read};

mod format;
mod frontend;
mod lexer;

use format::format_tokens;
use frontend::lexer as comp_lexer;
use frontend::parser as comp_parser;
use lexer::{Lexer, Token};

pub fn format_source(src: &str) -> Result<String, String> {
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
    let mut fmt_lexer = Lexer::new(src);
    let tokens = fmt_lexer.lex()?;
    Ok(format_tokens(&tokens))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut check_only = false;
    let mut files: Vec<String> = Vec::new();
    for arg in &args {
        if arg == "--check" || arg == "-c" {
            check_only = true;
        } else {
            files.push(arg.clone());
        }
    }
    if files.is_empty() {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .expect("Failed to read from stdin");
        match format_source(&buf) {
            Ok(formatted) => {
                if check_only {
                    if formatted != buf {
                        eprintln!("check failed: input differs from formatted output");
                        std::process::exit(1);
                    }
                } else {
                    print!("{}", formatted);
                }
            }
            Err(e) => {
                eprintln!("format error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        for file in &files {
            let src = fs::read_to_string(file).expect("Failed to read file");
            match format_source(&src) {
                Ok(formatted) => {
                    if check_only {
                        if formatted != src {
                            eprintln!("check failed: {} differs from formatted output", file);
                            std::process::exit(1);
                        }
                    } else {
                        fs::write(file, &formatted).expect("Failed to write file");
                    }
                }
                Err(e) => {
                    eprintln!("{}: format error: {}", file, e);
                    std::process::exit(1);
                }
            }
        }
    }
}
