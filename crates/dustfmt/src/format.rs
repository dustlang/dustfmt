// File: format.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   Dustfmt formatter module - formats tokens into canonical Dust source.
//   Provides:
//     - format_tokens(): Convert token sequence to formatted source
//     - Indentation management
//     - Newline handling
//     - Whitespace trimming

use crate::lexer::Token;

/// Format a sequence of tokens into canonical Dust source.
pub fn format_tokens(tokens: &[Token]) -> String {
    let mut result: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut indent_level: usize = 0;
    let mut newline_pending = false;
    let mut prev: Option<&Token> = None;

    let mut flush_line =
        |current: &mut String, result: &mut Vec<String>, newline_pending: &mut bool| {
            if !current.is_empty() {
                let trimmed = current.trim_end().to_string();
                result.push(trimmed);
                current.clear();
            }
            *newline_pending = false;
        };

    for tok in tokens {
        match tok {
            Token::Newline => {
                continue;
            }
            Token::Skip => {
                continue;
            }
            Token::Comment(text) => {
                flush_line(&mut current, &mut result, &mut newline_pending);
                let indent = " ".repeat(indent_level * 4);
                result.push(format!("{}{}", indent, text.trim_end()));
                prev = Some(tok);
                continue;
            }
            Token::BlockComment(text) => {
                flush_line(&mut current, &mut result, &mut newline_pending);
                let lines: Vec<&str> = text.split('\n').collect();
                for line in lines {
                    let indent = " ".repeat(indent_level * 4);
                    result.push(format!("{}{}", indent, line.trim_end()));
                }
                prev = Some(tok);
                continue;
            }
            Token::Unknown(ch) => {
                flush_line(&mut current, &mut result, &mut newline_pending);
                let indent = " ".repeat(indent_level * 4);
                result.push(format!("{}{}", indent, ch));
                prev = Some(tok);
                continue;
            }
            _ => {}
        }
        match tok {
            Token::Delim('{') => {
                if !current.is_empty() {
                    current.push(' ');
                } else {
                    current.push_str(&" ".repeat(indent_level * 4));
                }
                current.push('{');
                flush_line(&mut current, &mut result, &mut newline_pending);
                indent_level += 1;
                prev = Some(tok);
                continue;
            }
            Token::Delim('}') => {
                flush_line(&mut current, &mut result, &mut newline_pending);
                if indent_level > 0 {
                    indent_level -= 1;
                }
                let indent = " ".repeat(indent_level * 4);
                result.push(format!("{}{}", indent, '}'));
                newline_pending = true;
                prev = Some(tok);
                continue;
            }
            Token::Delim(';') => {
                current.push(';');
                flush_line(&mut current, &mut result, &mut newline_pending);
                newline_pending = true;
                prev = Some(tok);
                continue;
            }
            _ => {}
        }
        if newline_pending {
            flush_line(&mut current, &mut result, &mut newline_pending);
        }
        if current.is_empty() {
            current.push_str(&" ".repeat(indent_level * 4));
        }
        let needs_space = {
            let mut ns = false;
            if let Some(prev_tok) = prev {
                match tok {
                    Token::Delim(',')
                    | Token::Delim(':')
                    | Token::Delim(')')
                    | Token::Delim(']')
                    | Token::Delim('.') => {
                        ns = false;
                    }
                    Token::Op(op) => {
                        if op == "::" {
                            ns = false;
                        } else {
                            ns = true;
                        }
                    }
                    Token::Delim('(') => {
                        ns = false;
                    }
                    _ => match prev_tok {
                        Token::Delim('(') | Token::Delim('[') => {
                            ns = false;
                        }
                        Token::Op(prev_op) => {
                            if prev_op == "::" {
                                ns = false;
                            } else {
                                ns = true;
                            }
                        }
                        _ => ns = true,
                    },
                }
            }
            ns
        };
        if needs_space && !current.ends_with(' ') {
            current.push(' ');
        }
        match tok {
            Token::Delim(',') | Token::Delim(':') => {
                current.push(match tok {
                    Token::Delim(c) => *c,
                    _ => unreachable!(),
                });
                current.push(' ');
            }
            Token::Delim(c) => {
                current.push(*c);
            }
            Token::Op(op) => {
                current.push_str(op);
            }
            Token::Keyword(s)
            | Token::Ident(s)
            | Token::Regime(s)
            | Token::Number(s)
            | Token::BoolLit(s)
            | Token::StringLit(s)
            | Token::Op(s) => {
                current.push_str(match tok {
                    Token::Keyword(s)
                    | Token::Ident(s)
                    | Token::Regime(s)
                    | Token::Number(s)
                    | Token::BoolLit(s)
                    | Token::StringLit(s)
                    | Token::Op(s) => s,
                    _ => unreachable!(),
                });
            }
            _ => {}
        }
        prev = Some(tok);
    }
    flush_line(&mut current, &mut result, &mut newline_pending);
    if result.is_empty() || !result.last().unwrap().is_empty() {
        result.push(String::new());
    }
    result.join("\n")
}
