# Formatter Lexer

Source: `crates/dustfmt/src/lexer.rs`

## Token Model

Formatter lexer tokens include:

- structural tokens: delimiters/operators/newlines
- lexical atoms: identifiers, keywords, regimes, numbers, bools, strings
- comments: line and block comments preserved as token payload
- fallback tokens: unknown characters

## Comment Handling

- line comments and block comments are tokenized for preservation
- unlike the validation lexer, comments are not discarded

## Keyword Coverage

Keyword matching includes a broad set (core DPL plus several v0.2-oriented terms), independent of whether parser/formatter fully supports all constructs semantically.

## String and Number Reading

- strings are read until closing `"` without escape processing parity with compiler lexer
- number scanning allows digits and `.`

## Operational Role

The formatter lexer is a layout-oriented tokenizer. It is not authoritative for syntax validity; syntax validity is delegated to the embedded frontend validation pass.
