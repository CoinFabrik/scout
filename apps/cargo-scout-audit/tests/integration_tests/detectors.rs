use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;

use cargo_scout_audit::startup::{run_scout, OutputFormat, Scout};
use colored::Colorize;
use configuration::Configuration;
use serde::{Deserialize, Serialize};

mod configuration;
mod utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct Detectors {
    detectors: HashMap<String, DetectorConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DetectorConfig {
    warning_message: String,
    testcases: Vec<Testcase>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Testcase {
    vulnerable_path: Option<String>,
    remediated_path: Option<String>,
}

/// Test that all detectors run successfully on testcases and
/// that lint messages match the expected ones.
///
/// The following environment variable can be used:
///
/// - `INTEGRATION_TESTS_TO_RUN`: comma separated list of integration tests to run.
///  If not set, all integration tests are run.
#[test]
fn test() {
    // Get environment variable to determine integration tests to run
    let integration_tests_to_run = std::env::var("INTEGRATION_TESTS_TO_RUN")
        .ok()
        .map(|e| e.split(',').map(|s| s.to_string()).collect::<Vec<String>>());
    let mut ran_integration_tests =
        vec![false; integration_tests_to_run.as_ref().map_or(0, |v| v.len())];

    // Get the configuration
    let detectors_config = Configuration::build().expect(&"Failed to get the configuration".red());

    // Run all integration tests
    for detector_config in detectors_config.detectors.iter() {
        let detector_name = detector_config.detector.to_string();
        let lint_message = detector_config.detector.get_lint_message();

        if let Some(integration_tests_to_run) = &integration_tests_to_run {
            let integration_tests_to_run_i = integration_tests_to_run
                .iter()
                .position(|t| t == &detector_name);
            match integration_tests_to_run_i {
                Some(i) => ran_integration_tests[i] = true,
                None => continue,
            };
        }

        println!("\n{} {}", "Testing detector:".bright_cyan(), detector_name);
        for example in detector_config.testcases.iter() {
            if let Some(vulnerable_path) = &example.vulnerable_path {
                execute_and_validate_testcase(&detector_name, lint_message, &vulnerable_path, true);
            }
            if let Some(remediated_path) = &example.remediated_path {
                execute_and_validate_testcase(
                    &detector_name,
                    lint_message,
                    &remediated_path,
                    false,
                );
            }
        }
    }

    // If integration tests to run were specified, check that all of them were run
    if let Some(integration_tests_to_run) = &integration_tests_to_run {
        let panic_exit = ran_integration_tests.iter().any(|t| !t);
        for (i, ran_integration_test) in ran_integration_tests.iter().enumerate() {
            if !ran_integration_test {
                println!(
                    "{} {}",
                    "Error: integration test not found:".bright_red(),
                    integration_tests_to_run[i]
                );
            }
        }
        if panic_exit {
            panic!();
        }
    }
}

fn execute_and_validate_testcase(
    detector_name: &str,
    lint_message: &str,
    path: &str,
    is_vulnerable: bool,
) {
    print!("{} {}", "Running testcase:".green(), path);
    let start_time = std::time::Instant::now();

    // Create tempfile for storing the output
    let mut tempfile = tempfile::NamedTempFile::new().expect("Failed to create tempfile");

    // Run scout
    let scout_config = Scout {
        output_format: OutputFormat::Text,
        output_path: Some(PathBuf::from(tempfile.path())),
        local_detectors: Some(get_detectors_path()),
        manifest_path: Some(PathBuf::from(path.to_string())),
        filter: Some(detector_name.to_string()),
        verbose: true,
        ..Default::default()
    };
    run_scout(scout_config).unwrap();

    // Read output
    let mut output = String::new();
    tempfile
        .read_to_string(&mut output)
        .expect("Failed to read tempfile");

    let end_time = std::time::Instant::now();

    assert!(
        output.contains(lint_message) == is_vulnerable,
        "\n\n{}\n\n{}\n\n",
        if is_vulnerable {
            "Error: vulnerability not found on a vulnerable path".red()
        } else {
            "Error: vulnerability found on a non vulnerable path".red()
        },
        output
    );

    println!(
        " - {} {} secs.",
        "Elapsed time:".bright_purple(),
        end_time.duration_since(start_time).as_millis() as f64 / 1000.0
    );
}

fn get_detectors_path() -> PathBuf {
    utils::get_repository_root_path()
        .expect("Failed to get detectors path")
        .join("detectors")
}
