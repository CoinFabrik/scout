use std::collections::HashMap;
use std::io::{self, Write};
use std::process::{Command, Output};

use colored::Colorize;
use config::Config;
use serde::{Deserialize, Serialize};

const CONFIG_FILENAME: &str = "integration_test_configuration.yaml";

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Detectors {
    detectors: HashMap<String, DetectorConfig>,
}

#[test]
fn test() {
    assert!(
        Command::new("cargo")
            .arg("scout-audit")
            .arg("--help")
            .output()
            .expect("Failed to execute 'cargo scout-audit --help'")
            .status
            .success(),
        "{:?}",
        print_cargo_scout_not_found()
    );

    // Get environment variable to determine integration tests to run
    let integration_tests_to_run = std::env::var("INTEGRATION_TESTS_TO_RUN")
        .ok()
        .map(|e| e.split(',').map(|s| s.to_string()).collect::<Vec<String>>());
    let mut ran_integration_tests =
        vec![false; integration_tests_to_run.as_ref().map_or(0, |v| v.len())];

    // Get the configuration
    let configuration = get_configuration()
        .unwrap_or_else(|_| panic!("{}", "Failed to get the configuration".red().to_string()));

    for (detector_name, detector_config) in configuration.detectors.iter() {
        if let Some(integration_tests_to_run) = &integration_tests_to_run {
            let integration_tests_to_run_i = integration_tests_to_run
                .iter()
                .position(|t| t == detector_name);
            match integration_tests_to_run_i {
                Some(i) => ran_integration_tests[i] = true,
                None => continue,
            };
        }

        println!("\n{} {}", "Testing detector:".bright_cyan(), detector_name);
        for example in detector_config.testcases.iter() {
            if let Some(vulnerable_path) = &example.vulnerable_path {
                execute_and_validate_testcase(
                    &detector_config.warning_message,
                    &vulnerable_path,
                    true,
                );
            }
            if let Some(remediated_path) = &example.remediated_path {
                execute_and_validate_testcase(
                    &detector_config.warning_message,
                    &remediated_path,
                    false,
                );
            }
        }
    }

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

pub fn get_configuration() -> Result<Detectors, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory.");
    let configuration_directory = base_path.join("tests");
    Config::builder()
        .add_source(config::File::from(
            configuration_directory.join(CONFIG_FILENAME),
        ))
        .build()?
        .try_deserialize()
}

fn execute_and_validate_testcase(warning_message: &str, path: &str, is_vulnerable: bool) {
    let start_time = std::time::Instant::now();
    print!("{} {}", "Running testcase:".green(), path);
    io::stdout().flush().unwrap();
    let output = execute_command(path);
    let end_time = std::time::Instant::now();

    if !output.status.success() {
        println!(
            "\n\n{}\n\n",
            format!(
                "Error: failed to execute the command, probably due to an invalid {} path. Elapsed time: {} ms",
                if is_vulnerable {
                    "vulnerable"
                } else {
                    "remediated"
                },
                end_time.duration_since(start_time).as_millis() as f64 / 1000.0
            )
            .red()
        );
        println!("stdout:\n{}", String::from_utf8(output.stdout).unwrap());
        println!("stderr:\n{}", String::from_utf8(output.stderr).unwrap());
        panic!();
    }

    let output = String::from_utf8(output.stderr).expect("Failed to parse output");
    assert!(
        output.contains(warning_message) == is_vulnerable,
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

fn execute_command(path: &str) -> Output {
    let mut command = command_builder(path);
    command.output().expect("Failed to execute command")
}

fn command_builder(path: &str) -> Command {
    let mut command = Command::new("cargo");
    command.arg("scout-audit");
    command.arg("-m");
    command.arg(path);
    command
}

fn print_cargo_scout_not_found() {
    let separator = "─".repeat(65);
    let upper_border = format!("┌{}┐", separator).bright_yellow();
    let lower_border = format!("└{}┘", separator).bright_yellow();
    let empty_line = format!("│{:65}│", "").bright_yellow();

    println!("{}", upper_border);
    println!("{}", empty_line);
    println!(
        "{}{: ^66}{}",
        "│".bright_yellow(),
        "⚠️  Cargo Scout-Audit is not installed, please install it with:".bright_yellow(),
        "│".bright_yellow()
    );
    println!("{}", empty_line);
    println!(
        "{}{: ^65}{}",
        "│".bright_yellow(),
        "cargo install --path <PATH-TO-CARGO-SCOUT-AUDIT>".bright_yellow(),
        "│".bright_yellow()
    );
    println!("{}", empty_line);
    println!("{}", lower_border);
}
