use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{html, markdown};

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    pub name: String,
    pub description: String,
    pub date: NaiveDate,
    pub source_url: String,
    pub summary: Summary,
    pub categories: Vec<Category>,
    pub findings: Vec<Finding>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Summary {
    pub total_vulnerabilities: u32,
    pub by_severity: HashMap<String, u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub vulnerabilities: Vec<Vulnerability>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vulnerability {
    pub id: String,
    pub name: String,
    pub short_message: String,
    pub long_message: String,
    pub severity: String,
    pub help: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Finding {
    pub id: u32,
    pub occurrence_index: u32,
    pub category_id: String,
    pub vulnerability_id: String,
    pub error_message: String,
    pub span: String,
    pub code_snippet: String,
    pub file: String,
}

impl Report {
    pub fn new(
        name: String,
        description: String,
        date: NaiveDate,
        source_url: String,
        summary: Summary,
        categories: Vec<Category>,
        findings: Vec<Finding>,
    ) -> Self {
        Report {
            name,
            description,
            date,
            source_url,
            summary,
            categories,
            findings,
        }
    }

    pub fn generate_html(&self) -> Result<&'static str> {
        html::generate_html(self)
    }

    pub fn generate_markdown(&self) -> Result<&'static str> {
        markdown::generate_markdown(self)
    }
}
