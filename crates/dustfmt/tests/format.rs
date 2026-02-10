use std::fs;

// Import the public formatter function from the crate root.
use dustfmt::format_source;

/// Helper to normalise newlines for comparison.
fn normalise(s: &str) -> String {
    s.replace("\r\n", "\n")
}

#[test]
fn simple_forge_proc() {
    let input = "forge F {\nK main { emit \"Hello\"; }\n}\n";
    let expected = [
        "forge F {",
        "    K main {",
        "        emit \"Hello\";",
        "    }",
        "}",
        "",
    ].join("\n");
    let out = format_source(input).expect("format success");
    assert_eq!(normalise(&out), normalise(&expected));
}

#[test]
fn preserves_line_comments() {
    let input = "forge F {\n// comment\nK main { emit \"X\"; // trailing\n }\n}\n";
    let expected = [
        "forge F {",
        "    // comment",
        "    K main {",
        "        emit \"X\"; // trailing",
        "    }",
        "}",
        "",
    ].join("\n");
    let out = format_source(input).expect("format success");
    assert_eq!(normalise(&out), normalise(&expected));
}

#[test]
fn invalid_syntax_reports_error() {
    // Missing semicolon should cause parse error
    let input = "forge F { K main { emit \"X\" } }";
    let err = format_source(input).expect_err("should be parse error");
    assert!(err.contains("parse error"));
}