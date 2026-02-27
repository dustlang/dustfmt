// File: lexer.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   Dustfmt lexer - tokenizes Dust source for formatting.
//   Provides Token enum with:
//     - Newline, Comment, BlockComment
//     - Ident, Keyword, Regime
//     - Number, BoolLit, StringLit
//     - Op, Delim
//   Lexical rules derived from DPL specification.

/// Token kinds recognised by the formatter's lexer.
#[derive(Debug, Clone)]
pub enum Token {
    Newline,
    Comment(String),
    BlockComment(String),
    Ident(String),
    Keyword(String),
    Regime(String),
    Number(String),
    BoolLit(String),
    StringLit(String),
    Op(String),
    Delim(char),
    Unknown(char),
    Skip,
}

/// Simple lexer for DPL source.
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn bump(&mut self) {
        if self.pos < self.input.len() {
            self.pos += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' {
                self.bump();
            } else {
                break;
            }
        }
    }

    fn read_ident(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                s.push(c);
                self.bump();
            } else {
                break;
            }
        }
        s
    }

    fn read_number(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_numeric() || c == '.' {
                s.push(c);
                self.bump();
            } else {
                break;
            }
        }
        s
    }

    fn read_string(&mut self) -> String {
        let mut s = String::new();
        self.bump(); // skip opening "
        while let Some(c) = self.peek() {
            if c == '"' {
                self.bump();
                break;
            }
            s.push(c);
            self.bump();
        }
        s
    }

    fn read_line_comment(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            s.push(c);
            self.bump();
        }
        s
    }

    fn read_block_comment(&mut self) -> String {
        let mut s = String::new();
        self.bump(); // skip /*
        self.bump(); // skip first *
        while let Some(c) = self.peek() {
            if c == '*' {
                self.bump();
                if let Some(next) = self.peek() {
                    if next == '/' {
                        self.bump();
                        break;
                    }
                }
            }
            s.push(c);
            self.bump();
        }
        s
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(c) = self.peek() {
            match c {
                '\n' => {
                    tokens.push(Token::Newline);
                    self.bump();
                }
                ' ' | '\t' => {
                    self.skip_whitespace();
                }
                '/' => {
                    self.bump();
                    if let Some(next) = self.peek() {
                        if next == '/' {
                            let comment = self.read_line_comment();
                            tokens.push(Token::Comment(comment));
                        } else if next == '*' {
                            let comment = self.read_block_comment();
                            tokens.push(Token::BlockComment(comment));
                        } else {
                            tokens.push(Token::Op("/".to_string()));
                        }
                    } else {
                        tokens.push(Token::Op("/".to_string()));
                    }
                }
                '"' => {
                    let s = self.read_string();
                    tokens.push(Token::StringLit(s));
                }
                '{' | '}' | '(' | ')' | '[' | ']' | ';' | ',' => {
                    tokens.push(Token::Delim(c));
                    self.bump();
                }
                c if c.is_alphabetic() || c == '_' => {
                    let s = self.read_ident();
                    match s.as_str() {
                        "K" | "Q" | "Φ" => tokens.push(Token::Regime(s)),
                        "true" | "false" => tokens.push(Token::BoolLit(s)),
                        "forge" | "shape" | "proc" | "process" | "bind" | "contract" | "uses"
                        | "let" | "mut" | "constrain" | "prove" | "from" | "observe" | "emit"
                        | "seal" | "return" | "linear" | "if" | "else" | "for" | "while"
                        | "break" | "continue" | "in" | "match" | "loop" | "type" | "trait"
                        | "enum" | "struct" | "impl" | "alloc" | "free" | "spawn" | "join"
                        | "mutex_new" | "mutex_lock" | "mutex_unlock" | "open" | "read"
                        | "write" | "close" | "unsafe" => tokens.push(Token::Keyword(s)),
                        _ => tokens.push(Token::Ident(s)),
                    }
                }
                c if c.is_numeric() => {
                    let s = self.read_number();
                    tokens.push(Token::Number(s));
                }
                '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' | '&' | '|' => {
                    let mut op = String::new();
                    op.push(c);
                    self.bump();
                    if let Some(next) = self.peek() {
                        if (c == '=' && next == '=')
                            || (c == '<' && next == '=')
                            || (c == '>' && next == '=')
                            || (c == '<' && next == '<')
                            || (c == '>' && next == '>')
                            || (c == '&' && next == '&')
                            || (c == '|' && next == '|')
                        {
                            op.push(next);
                            self.bump();
                        }
                    }
                    tokens.push(Token::Op(op));
                }
                _ => {
                    tokens.push(Token::Unknown(c));
                    self.bump();
                }
            }
        }

        Ok(tokens)
    }
}
