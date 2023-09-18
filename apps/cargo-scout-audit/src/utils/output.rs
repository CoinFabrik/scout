use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::vec;

use anyhow::Context;
use regex::RegexBuilder;
use scout_audit_internal::{Detector, IntoEnumIterator};
use serde_json::{json, Value};

pub fn format_into_json(scout_output: File, internals: File) -> anyhow::Result<String> {
    let json_errors = jsonify(scout_output, internals)?;
    Ok(serde_json::to_string_pretty(&json_errors)?)
}

fn jsonify(scout_output: File, internals: File) -> anyhow::Result<serde_json::Value> {
    let json_errors: serde_json::Value = get_errors_from_output(scout_output, internals)?
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

fn get_errors_from_output(
    mut scout_output: File,
    mut scout_internals: File,
) -> anyhow::Result<HashMap<String, (Vec<String>, String)>> {
    let regex = RegexBuilder::new(r"warning:.*")
        .multi_line(true)
        .case_insensitive(true)
        .build()?;

    let mut stderr_string = String::new();
    std::io::Read::read_to_string(&mut scout_output, &mut stderr_string)?;

    let mut scout_internals_spans: Vec<String> = vec![];

    for line in std::io::BufReader::new(&mut scout_internals).lines() {
        let line = line?;
        let span = line.split('@').collect::<Vec<&str>>()[1];
        scout_internals_spans.push(span.to_string());
    }

    let msg_to_name: HashMap<String, String> = Detector::iter()
        .map(|e| (e.get_lint_message().to_string(), e.to_string()))
        .collect();

    let mut errors: HashMap<String, (Vec<String>, String)> = Detector::iter()
        .map(|e| (e.to_string(), (vec![], "".to_string())))
        .collect();

    for (i, elem) in regex.find_iter(&stderr_string).enumerate() {
        let parts = elem.as_str().split('\n').collect::<Vec<&str>>();

        for err in Detector::iter().map(|e| e.get_lint_message()) {
            if parts[0].contains(err) {
                let name = msg_to_name.get(err).with_context(|| {
                    format!("Error making json: {} not found in the error map", err)
                })?;

                if let Some((spans, error)) = errors.get_mut(name) {
                    spans.push(scout_internals_spans[i].to_string());
                    *error = err.to_string();
                }
            }
        }
    }
    Ok(errors)
}

pub fn format_into_html(scout_output: File, internals: File) -> anyhow::Result<String> {
    let json = jsonify(scout_output, internals)?;
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

fn serify(scout_output: File, scout_internals: File) -> anyhow::Result<serde_json::Value> {
    let errors: HashMap<String, (Vec<String>, String)> =
        get_errors_from_output(scout_output, scout_internals)?;

    let sarif_output = json!({
        "$schema": "https://json.schemastore.org/sarif-2.1.0",
        "version": "2.1.0",
        "runs": [
            {
                "tool": {
                    "driver": {
                        "name": env!("CARGO_PKG_NAME"),
                        "version": env!("CARGO_PKG_VERSION"),
                        "rules": Detector::iter().filter(|e| {
                            errors.contains_key(&e.to_string()) && !errors.get(&e.to_string()).unwrap().0.is_empty()
                        }).map(|e| {
                            json!({
                                "id": e.to_string(),
                                "shortDescription": {
                                    "text": e.get_lint_message()
                                }})

                        }).collect::<Vec<serde_json::Value>>(),
                        "informationUri": "https://coinfabrik.github.io/scout/",
                    }
                },
                "results": build_sarif_results(&errors)?,
            }
        ]
    });
    let json_errors = serde_json::to_value(sarif_output)?;
    Ok(json_errors)
}

pub fn format_into_sarif(scout_output: File, scout_internals: File) -> anyhow::Result<String> {
    Ok(serify(scout_output, scout_internals)?.to_string())
}

fn build_sarif_results(
    errors: &HashMap<String, (Vec<String>, String)>,
) -> anyhow::Result<Vec<serde_json::Value>> {
    let runs: Vec<Value> = errors
        .iter()
        .flat_map(|(name, (spans, msg))| {
            spans.iter().filter_map(move |span| {
                let span: Result<serde_json::Value, _> = serde_json::from_str(span);
                if let Ok(span_value) = span {
                    Some(json!({
                        "ruleId": name,
                        "level": "error",
                        "message": {
                            "text": msg
                        },
                        "locations": [span_value],
                    }))
                } else {
                    None
                }
            })
        })
        .collect();

    Ok(runs)
}
