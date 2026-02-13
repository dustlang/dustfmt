// dustfmt – official formatter for DPL (Dust Programming Language).
//
// This Rust implementation follows the same formatting rules outlined in the
// project README.  It scans the input text into a sequence of tokens, then
// rebuilds the source code in a canonical form.  Comments are preserved,
// indentation uses four spaces, braces remain on the same line, each
// syntactic item resides on its own line, trailing whitespace is removed,
// and a final newline is ensured.
//
// Because this tool is intended to be bootstrapped alongside the Dust
// compiler, it deliberately avoids external dependencies beyond the Rust
// standard library.

use std::env;
use std::fs;
use std::io::{self, Read};

// Pull in the compiler frontend for AST validation.  These modules are
// generated from the Dust compiler sources and live under the
// `frontend` module in this crate.
mod frontend;
use frontend::lexer as comp_lexer;
use frontend::parser as comp_parser;

/// Token kinds recognised by the formatter's lexer.
#[derive(Debug, Clone)]
enum Token {
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

/// Simple lexer for DPL source.  It yields a vector of tokens for a given
/// input string.  Lexical rules are derived from the DPL specification.
struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(s: &str) -> Self {
        Self {
            input: s.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self, n: usize) -> Option<&[char]> {
        if self.pos + n <= self.input.len() {
            Some(&self.input[self.pos..self.pos + n])
        } else {
            None
        }
    }

    fn advance(&mut self, n: usize) {
        self.pos += n;
    }

    fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while let Some(&ch) = self.input.get(self.pos) {
            // Newlines
            if ch == '\n' {
                self.advance(1);
                tokens.push(Token::Newline);
                continue;
            }
            // Whitespace other than newline
            if ch == ' ' || ch == '\r' || ch == '\u{000C}' {
                self.advance(1);
                continue;
            }
            // Tab – treat as skip and warn implicitly
            if ch == '\t' {
                self.advance(1);
                tokens.push(Token::Skip);
                continue;
            }
            // Line comment
            if ch == '/' {
                if let Some(peek2) = self.peek(2) {
                    if peek2 == ['/', '/'] {
                        // consume until newline or end
                        let start = self.pos;
                        self.advance(2);
                        while let Some(&c) = self.input.get(self.pos) {
                            if c == '\n' {
                                break;
                            }
                            self.advance(1);
                        }
                        let comment: String = self.input[start..self.pos].iter().collect();
                        tokens.push(Token::Comment(comment));
                        continue;
                    } else if peek2 == ['/', '*'] {
                        // block comment
                        let start = self.pos;
                        self.advance(2);
                        while let Some(peek2) = self.peek(2) {
                            if peek2 == ['*', '/'] {
                                self.advance(2);
                                let comment: String = self.input[start..self.pos].iter().collect();
                                tokens.push(Token::BlockComment(comment));
                                break;
                            }
                            // advance through block
                            self.advance(1);
                            if self.pos >= self.input.len() {
                                return Err("Unterminated block comment".to_string());
                            }
                        }
                        continue;
                    }
                }
            }
            // String literal
            if ch == '"' {
                let start = self.pos;
                self.advance(1);
                let mut escaped = false;
                while let Some(&c) = self.input.get(self.pos) {
                    self.advance(1);
                    if escaped {
                        escaped = false;
                        continue;
                    }
                    if c == '\\' {
                        escaped = true;
                    } else if c == '"' {
                        break;
                    }
                    if self.pos >= self.input.len() {
                        return Err("Unterminated string literal".to_string());
                    }
                }
                let literal: String = self.input[start..self.pos].iter().collect();
                tokens.push(Token::StringLit(literal));
                continue;
            }
            // Multi‑char operators
            if let Some(peek2) = self.peek(2) {
                let two: String = peek2.iter().collect();
                if ["::", "->", "==", "<=", ">=", "&&", "||", "..", "=>"].contains(&two.as_str()) {
                    self.advance(2);
                    tokens.push(Token::Op(two));
                    continue;
                }
            }
            // Single char operators (v0.2 addition: !)
            if ['+', '-', '*', '/', '<', '>', '=', '!'].contains(&ch) {
                self.advance(1);
                tokens.push(Token::Op(ch.to_string()));
                continue;
            }
            // Delimiters
            if ['{', '}', '(', ')', '[', ']', ',', ';', ':'].contains(&ch) {
                self.advance(1);
                tokens.push(Token::Delim(ch));
                continue;
            }
            // Number (integer or float)
            if ch.is_ascii_digit() {
                let start = self.pos;
                let mut has_dot = false;
                // Read integer part
                while let Some(&c) = self.input.get(self.pos) {
                    if c.is_ascii_digit() {
                        self.advance(1);
                    } else if c == '.' && !has_dot {
                        // Check if it's a float
                        if let Some(&next) = self.input.get(self.pos + 1) {
                            if next.is_ascii_digit() {
                                has_dot = true;
                                self.advance(1); // consume '.'
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                let num: String = self.input[start..self.pos].iter().collect();
                tokens.push(Token::Number(num));
                continue;
            }
            // Char literal ('a')
            if ch == '\'' {
                let start = self.pos;
                self.advance(1); // consume opening '
                let mut ch_val = '\0';
                if let Some(&c) = self.input.get(self.pos) {
                    if c != '\'' {
                        ch_val = c;
                        self.advance(1);
                    }
                }
                // consume closing '
                if let Some(&c) = self.input.get(self.pos) {
                    if c == '\'' {
                        self.advance(1);
                    }
                }
                tokens.push(Token::Number(format!("'{}'", ch_val)));
                continue;
            }
            // Identifier / keyword / regime / bool
            if ch.is_ascii_alphabetic() || ch == '_' {
                let start = self.pos;
                self.advance(1);
                while let Some(&c) = self.input.get(self.pos) {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        self.advance(1);
                    } else {
                        break;
                    }
                }
                let ident: String = self.input[start..self.pos].iter().collect();
                match ident.as_str() {
                    // v0.1 keywords
                    "forge" | "shape" | "proc" | "process" | "bind" | "contract" | "uses"
                    | "constrain" | "prove" | "from" | "observe" | "emit" | "seal"
                    | "return" | "linear" => tokens.push(Token::Keyword(ident)),
                    // v0.2 keywords - K-regime memory operations
                    "alloc" | "free" | "spawn" | "join" | "mutex_new" | "mutex_lock" 
                    | "mutex_unlock" | "open" | "read" | "write" | "close"
                    | "io_read" | "io_write" | "mmio_read" | "mmio_write" | "unsafe"
                    // v0.2 control flow
                    | "if" | "else" | "for" | "while" | "break" | "continue" | "in" | "match"
                    // v0.2 variable bindings
                    | "let" | "mut" | "where" | "loop" | "pub" | "mod" | "use" | "as"
                    // v0.2 type system
                    | "type" | "trait" | "enum" | "struct" | "impl" | "self" | "Self"
                    // v0.2 constraints
                    | "requires" | "ensures" | "invariant" | "assert" | "assume" | "ghost"
                    // v0.2 effects
                    | "effect" | "handler" | "do" | "try" | "catch" | "throw" => tokens.push(Token::Keyword(ident)),
                    "K" | "Q" | "Φ" => tokens.push(Token::Regime(ident)),
                    "true" | "false" => tokens.push(Token::BoolLit(ident)),
                    _ => tokens.push(Token::Ident(ident)),
                }
                continue;
            }
            // Unknown character
            self.advance(1);
            tokens.push(Token::Unknown(ch));
        }
        Ok(tokens)
    }
}

/// Format a sequence of tokens into canonical Dust source.
fn format_tokens(tokens: &[Token]) -> String {
    let mut result: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut indent_level: usize = 0;
    let mut newline_pending = false;
    let mut prev: Option<&Token> = None;
    let mut flush_line =
        |current: &mut String, result: &mut Vec<String>, newline_pending: &mut bool| {
            if !current.is_empty() {
                // remove trailing whitespace
                let trimmed = current.trim_end().to_string();
                result.push(trimmed);
                current.clear();
            }
            *newline_pending = false;
        };

    for tok in tokens {
        match tok {
            Token::Newline => {
                // ignore original newlines – we generate our own
                continue;
            }
            Token::Skip => {
                // ignore tabs/spaces that were consumed as skip
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
        // structural tokens
        match tok {
            Token::Delim('{') => {
                // if we have content on the current line, ensure space before '{'
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
        // begin new line if pending
        if newline_pending {
            flush_line(&mut current, &mut result, &mut newline_pending);
        }
        // ensure indentation at start of new line
        if current.is_empty() {
            current.push_str(&" ".repeat(indent_level * 4));
        }
        // spacing logic
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
    // ensure final newline
    if result.is_empty() || !result.last().unwrap().is_empty() {
        result.push(String::new());
    }
    result.join("\n")
}

/// Format a single source string.  Returns formatted string or error message.
///
/// This function is marked `pub` so that integration tests can invoke it
/// directly without going through the command‑line interface.  It first
/// validates the input via the compiler frontend and then applies the
/// formatter's lexing and printing logic.
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

fn main() {
    // Simple argument parsing: support --check or -c for check mode
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
        // read from stdin
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .expect("Failed to read from stdin");
        match format_source(&buf) {
            Ok(formatted) => {
                if check_only {
                    if formatted != buf {
                        std::process::exit(1);
                    }
                } else {
                    print!("{}", formatted);
                }
            }
            Err(e) => {
                eprintln!("dustfmt: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        let mut exit_code = 0;
        for path in files {
            let content = match fs::read_to_string(&path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("dustfmt: cannot read {}: {}", path, e);
                    exit_code = 1;
                    continue;
                }
            };
            match format_source(&content) {
                Ok(formatted) => {
                    if check_only {
                        if formatted != content {
                            println!("{}: not formatted", path);
                            exit_code = 1;
                        }
                    } else {
                        if formatted != content {
                            if let Err(e) = fs::write(&path, formatted) {
                                eprintln!("dustfmt: cannot write {}: {}", path, e);
                                exit_code = 1;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", path, e);
                    exit_code = 1;
                }
            }
        }
        if exit_code != 0 {
            std::process::exit(exit_code);
        }
    }
}
