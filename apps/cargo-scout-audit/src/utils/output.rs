use std::collections::HashMap;
use std::fs::File;

use anyhow::Context;
use regex::RegexBuilder;
use serde_json::json;

pub fn format_into_json(mut stderr: File) -> anyhow::Result<String> {
    let regex = RegexBuilder::new(r"^warning:.*\n  --> .*$")
        .multi_line(true)
        .case_insensitive(true)
        .build()
        .unwrap();

    let mut stderr_string = String::new();
    std::io::Read::read_to_string(&mut stderr, &mut stderr_string)?;

    let msg_to_name = error_map();

    let mut errors: HashMap<String, (Vec<String>, String)> = ERROR_NAMES
        .into_iter()
        .map(|e| (e.to_string(), (vec![], "".to_string())))
        .collect();

    for elem in regex.find_iter(&stderr_string) {
        let parts = elem.as_str().split('\n').collect::<Vec<&str>>();

        for err in SCOUT_ERRORS.iter() {
            if parts[0].contains(err) && parts[1].starts_with("  --> ") {
                let name = msg_to_name.get(err).with_context(|| {
                    format!("Error making json: {} not found in the error map", err)
                })?;

                let span = parts[1].replace("  --> ", "");

                if let Some((spans, error)) = errors.get_mut(*name) {
                    spans.push(span);
                    *error = err.to_string();
                }
            }
        }
    }
    let mut json_errors = json!({});
    for (name, (spans, error)) in errors {
        if spans.is_empty() {
            continue;
        }
        let mut json_error = json!({});

        json_error["error_msg"] = json!(error);
        json_error["spans"] = json!(spans);
        json_errors[name] = json_error;
    }

    Ok(serde_json::to_string_pretty(&json_errors)?)
}

pub fn format_into_html(stderr: File) -> anyhow::Result<String> {
    let json = format_into_json(stderr)?;
    let mut html = String::new();
    html.push_str(
        r#"
    <!DOCTYPE html>
    <html>
    <head>
    <style>
    body {
      font-family: monospace;
      font-size: 1rem;
    }
    table {
      border-collapse: collapse;
    }
    td, th {
      border: 1px solid #999;
      padding: 0.5rem;
    }
    th {
      background-color: #eee;
    }
    </style>
    </head>
    <body>
    <table>
    <tr>
    <th>Error</th>
    <th>Spans</th>
    <th>Message</th>
    </tr>
    "#,
    );

    let json: serde_json::Value = serde_json::from_str(&json)?;

    for (key, value) in json.as_object().unwrap() {
        let error_msg = value["error_msg"].as_str().unwrap();
        let spans = value["spans"].as_array().unwrap();
        let mut spans_html = String::new();
        for span in spans {
            spans_html.push_str(&format!("<li>{}</li>", span.as_str().unwrap()));
        }
        html.push_str(&format!(
            r#"
        <tr>
        <td>{}</td>
        <td><ul>{}</ul></td>
        <td>{}</td>
        </tr>
        "#,
            key, spans_html, error_msg
        ));
    }

    Ok(html)
}

const SCOUT_ERRORS: [&str; 23] =  [
        "Assert causes panic. Instead, return a proper error.",
        "Using `core::mem::forget` is not recommended.",
        "The format! macro should not be used.",
        "Passing arguments to the target of a delegate call is not safe, as it allows the caller to set a malicious hash as the target.",
        "Division before multiplication might result in a loss of precision",
        "In order to prevent a single transaction from consuming all the gas in a block, unbounded operations must be avoided",
        "This vector operation is called without access control",
        "The latest ink! version",
        "In order to prevent randomness manipulations by validators block_timestamp should not be used as random number source",
        "Potential for integer arithmetic overflow/underflow",
        "Hardcoding an index could lead to panic if the top bound is out of bounds.",
        "Non-lazy non-mapping storage",
        "The panic! macro is used to stop execution when a condition is not met. This is useful for testing and prototyping, but should be avoided in production code",
        "External calls could open the opportunity for a malicious contract to execute any arbitrary code",
        "This set_code_hash is called without access control",
        "This terminate_contract is called without access control",
        "This mapping operation is called without access control on a different key than the caller's address",
        "Not checking for a zero-address could lead to a locked contract",
        "unused return enum",
        "Unsafe usage of `expect`",
        "Unsafe usage of `unwrap`",
        "Abitrary users should not have control over keys because it implies writing any value of left mapping, lazy variable, or the main struct of the contract located in position 0 of the storage",
        "This argument comes from a user-supplied argument",

];

const ERROR_NAMES: [&str; 23] = [
    "assert-violation",
    "avoid-core-mem-forget",
    "avoid-format!-string",
    "delegate-call",
    "divide-before-multiply",
    "dos-unbounded-operation",
    "dos-unexpected-revert-with-vector",
    "ink-version",
    "insufficiently-random-values",
    "integer-overflow-or-underflow",
    "iterators-over-indexing",
    "lazy-delegate",
    "panic-error",
    "reentrancy",
    "set-code-hash",
    "unprotected-self-destruct",
    "unprotected-mapping-operation",
    "zero-or-test-address",
    "unuse-return-enum",
    "unsafe-expect",
    "unsafe-unwrap",
    "set-contract-storage",
    "unrestricted-transfer-from",
];

fn error_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    SCOUT_ERRORS
        .into_iter()
        .zip(ERROR_NAMES)
        .for_each(|(k, v)| {
            map.insert(k, v);
        });
    map
}
