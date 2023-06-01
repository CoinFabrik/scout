---
sidebar_position: 1
---

# Getting Started

Let's discover **Scout in less than 5 minutes!**.

## About Scout

Scout is an extensible open-source tool intended to assist ink! smart contract developers and auditors detect common security issues and deviations from best practices. This tool helps developers write secure and more robust smart contracts.

## Features

* A list of vulnerabilities, best practices and enhancements, together with associated detectors to identify these issues in your code
* Command Line Interface (CLI)
* VSCode Extension

### What you'll need

Make sure that [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) is installed on your computer. For using the VSCode Extension you must be using [VSCode](https://code.visualstudio.com/).

You should be able to install and run Scout without issues on Mac, Linux or Windows.

## Command Line Interface (CLI)

The command line interface is designed to allow you to run Scout on an entire project. It is especially useful for auditing or performing a final review of your code.

### Installation

In order to install the Command Line Interface, install Scout by running the following command:

```bash
cargo install cargo-scout cargo-dylint dylint-link
```

### Usage

To run Scout on your project, navigate to its root directory and execute the following command:

```bash
cargo scout
```

In the table below, we specify all the option available for the CLI.

| Command/Option | Explanation |
| --- | --- |
| `cargo scout` | The command to run the static analyzer on the current directory |
| `cargo scout --manifest-path <PATH_TO_CARGO_TOML>` | This option is used to specify the path to the Cargo.toml file that you want to analyze. |
| `cargo scout --filter-by-categories <CATEGORY_LIST_SEPARATED_BY_COMAS>` | Use this option to analyze only specific categories of code. You can provide a comma-separated list of categories for analysis. |
| `cargo scout --filter-by-detectors <DETECTOR_LIST_SEPARATED_BY_COMAS>` | This option allows you to analyze code using specific detectors. Provide a comma-separated list of detectors for this purpose. |
| `cargo scout --exclude-categories <CATEGORY_LIST_SEPARATED_BY_COMAS>` | Use this command to exclude certain categories from the analysis. Provide a comma-separated list of the categories to be excluded. |
| `cargo scout --exclude-detectors <DETECTOR_LIST_SEPARATED_BY_COMAS>` | With this command, you can exclude specific detectors from the analysis. You need to give a comma-separated list of the detectors to be excluded. |
| `cargo scout --list-detectors` | Use this option to display a list of all available detectors. |
| `cargo scout --list-categories` | This option is used to display a list of all available categories. |
| `cargo scout --output-file <OUTPUT_FILE_NAME>` | Use this option to specify the name of the file where the analysis results will be stored. |
| `cargo scout --help` | This command provides a brief explanation of all the available commands and their usage. |
| `cargo scout --version` | This command displays the current version of the static analyzer. |


## VSCode Extension

We built the Scout VSCode Extension to help developers write secure and more robust smart contracts. Listing security issues, and highlighting issues with squiggles and hover-over descriptions, we hope our extension will help you catch vulnerabilities during development.


### Installation

Install Scout from the Visual Studio Marketplace within the Extensions tab of Visual Studio Code.


### Usage

After you've installed the extension, simply open a project workspace that contains any ink! (.rs) files. This will trigger the activation of the extension, symbolized by the Scout logo that will appear on the action bar. If you click on this logo, you will see a new panel that has a results explorer and a detector filter tree.

When you move your cursor over the explorer tree, buttons will appear on the upper title bar. These buttons allow you to execute Scout, refresh the panel, change the viewing mode, and remove results.

When you click on a detector filter, you can control its visibility. Moving your cursor over the detector filter tree will cause a flag icon button to appear. This button serves as a toggle all switch.

If you left-click on a result within the result explorer, it will direct you to the associated piece of code. On the other hand, right-clicking on a result will display additional options like showing more information.

In case the source has been modified since the last run of Scout, and there is a discrepancy in the source mapping, the related issue and its annotations will be removed. An "out-of-sync" icon will then appear next to the result in the explorer tree, indicating that the Scout analysis should be initiated again.

