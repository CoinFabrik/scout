use std::collections::HashMap;
use std::fs::File;

use anyhow::Context;
use regex::RegexBuilder;
use scout_audit_internal::{Detector, IntoEnumIterator};
use serde_json::json;

pub fn format_into_json(scout_output: File) -> anyhow::Result<String> {
    let json_errors = jsonify(scout_output)?;
    Ok(serde_json::to_string_pretty(&json_errors)?)
}

fn jsonify(mut scout_output: File) -> anyhow::Result<serde_json::Value> {
    let regex = RegexBuilder::new(r"warning:.*\n*.*-->.*$")
        .multi_line(true)
        .case_insensitive(true)
        .build()?;

    let mut stderr_string = String::new();
    std::io::Read::read_to_string(&mut scout_output, &mut stderr_string)?;

    let msg_to_name: HashMap<String, String> = Detector::iter()
        .map(|e| (e.get_lint_message().to_string(), e.to_string()))
        .collect();

    let mut errors: HashMap<String, (Vec<String>, String)> = Detector::iter()
        .map(|e| (e.to_string(), (vec![], "".to_string())))
        .collect();

    for elem in regex.find_iter(&stderr_string) {
        let parts = elem.as_str().split('\n').collect::<Vec<&str>>();

        for err in Detector::iter().map(|e| e.get_lint_message()) {
            if parts[0].contains(err) && parts[1].trim().starts_with("-->") {
                let name = msg_to_name.get(err).with_context(|| {
                    format!("Error making json: {} not found in the error map", err)
                })?;

                let span = parts[1].replace("--> ", "");

                if let Some((spans, error)) = errors.get_mut(name) {
                    spans.push(span.trim().to_string());
                    *error = err.to_string();
                }
            }
        }
    }
    let json_errors: serde_json::Value = errors
        .iter()
        .filter(|(_, (spans, _))| !spans.is_empty())
        .map(|(name, (spans, error))| {
            (
                name,
                json!({
                    "error_msg": error,
                    "spans": spans
                }),
            )
        })
        .collect();

    Ok(json_errors)
}

pub fn format_into_html(scout_output: File) -> anyhow::Result<String> {
    let json = jsonify(scout_output)?;
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
