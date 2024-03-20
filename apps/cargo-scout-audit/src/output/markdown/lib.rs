use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::output::{report::Report, utils::write_to_file};

use super::generator::{generate_body, generate_header, generate_summary};

const REPORT_MD_PATH: &str = "build/report.md";

// Generates a markdown report from a given `Report` object.
pub fn generate_markdown(report: &Report) -> Result<&'static str> {
    let mut report_markdown = String::new();

    // Header
    report_markdown.push_str(&generate_header(report.date));

    // Summary
    report_markdown.push_str(&generate_summary(&report.categories, &report.findings));

    // Body
    report_markdown.push_str(&generate_body(&report.categories, &report.findings));

    write_to_file(&PathBuf::from(REPORT_MD_PATH), report_markdown.as_bytes())
        .with_context(|| format!("Failed to write markdown to '{}'", REPORT_MD_PATH))?;

    Ok(REPORT_MD_PATH)
}
