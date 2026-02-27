// File: format.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   Test suite for dustfmt formatter.
//   Tests include:
//     - simple_forge: Basic forge formatting
//     - Various formatting scenarios
//   Uses dustfmt::format_source for testing.

use std::fs;

// Import the public formatter function from the crate root.
use dustfmt::format_source;

/// Helper to normalise newlines for comparison.
fn normalise(s: &str) -> String {
    s.replace("\r\n", "\n")
}

#[test]
fn simple_forge() {
    let input = "forge F { }\n";
    let expected = ["forge F {", "}", ""].join("\n");
    let out = format_source(input).expect("format success");
    assert_eq!(normalise(&out), normalise(&expected));
}

#[test]
fn preserves_line_comments() {
    let input = "forge F {\n// comment\n}\n";
    let expected = ["forge F {", "    / comment", "}", ""].join("\n");
    let out = format_source(input).expect("format success");
    assert_eq!(normalise(&out), normalise(&expected));
}

#[test]
fn invalid_syntax_reports_error() {
    // Missing semicolon should cause parse error
    let input = "forge F { X }";
    let err = format_source(input).expect_err("should be parse error");
    assert!(err.contains("parse error"));
}
