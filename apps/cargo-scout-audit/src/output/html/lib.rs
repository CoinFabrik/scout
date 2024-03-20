use crate::output::{report::Report, utils::write_to_file};

use super::tera::{create_context, render_template};
use anyhow::{Context, Result};
use std::path::PathBuf;

const BASE_TEMPLATE: &str = "base.html";
const REPORT_HTML_PATH: &str = "build/report.html";
const OUTPUT_CSS_PATH: &str = "build/output.css";
const STYLES_CSS: &[u8] = include_bytes!("templates/styles.css");

// Generates an HTML report from a given `Report` object.
pub fn generate_html(report: &Report) -> Result<&'static str> {
    let context = create_context(report);
    let html = render_template(BASE_TEMPLATE, &context)
        .with_context(|| format!("Failed to render template '{}'", BASE_TEMPLATE))?;

    write_to_file(&PathBuf::from(REPORT_HTML_PATH), html.as_bytes())
        .with_context(|| format!("Failed to write HTML to '{}'", REPORT_HTML_PATH))?;

    write_to_file(&PathBuf::from(OUTPUT_CSS_PATH), STYLES_CSS)
        .with_context(|| format!("Failed to write CSS to '{}'", OUTPUT_CSS_PATH))?;

    Ok(REPORT_HTML_PATH)
}
