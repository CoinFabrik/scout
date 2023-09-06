extern crate rustc_errors;
extern crate rustc_lint;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_and_help as span_lint_and_help_clippy;
use rustc_lint::{Lint, LintContext};
use rustc_span::Span;
use serde_json::json;

pub fn span_lint_and_help<T: LintContext>(
    cx: &T,
    lint: &'static Lint,
    span: Span,
    msg: &str,
    help_span: Option<Span>,
    help: &str,
) {
    let span_debug_string: Vec<String> = format!("{:?}", span)
        .split(':')
        .map(|s| s.to_string())
        .collect();

    let val = json!({
        "file": span_debug_string[0],
        "startLine": span_debug_string[1].trim().parse::<i32>().unwrap(),
        "startColumn": span_debug_string[2].trim().parse::<i32>().unwrap(),
        "endLine": span_debug_string[3].trim().parse::<i32>().unwrap(),
        "endColumn": span_debug_string[4].split(' ').collect::<Vec<&str>>()[0].trim().parse::<i32>().unwrap(),
    });

    println!(
        "scout-internal:{}@{}",
        lint.name_lower().replace('_', "-"),
        val
    );

    span_lint_and_help_clippy(cx, lint, span, msg, help_span, help);
}
