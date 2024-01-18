# Changelog

## v0.2.2 (2024-01-18)

- Fixed `--message-format=json` argument for VSCode extension.

## v0.2.1 (2023-09-18)

- Fixed HTML and JSON output panic.

## v0.2.0 (2023-09-15)

- Added flags:
	- `--verbose` 
	- `--local-detectors` to use a local folder as detector, instad of the github repository
	- `--output-format` to format the output
	- `--output-path` to pipe the output
- New output formats
	Using `--output-format` now you can output Scout results in
	- HTML table
	- JSON
	- SARIF
- Wrapped dylint's `span_lint` and `span_lint_and_help` and centralized output messages in `scout-audit-internal`
- Updated various detectors 
- Now all detectors compile with the same toolchain, having the same Clippy version
- CI Upgrades
	- Now we test all detectors against their respective testcases:
		- Build time was reduced from initial 2 hours to 30 minutes
		- Tests are now ran in parallel to improve performance
		- Tests are ran in multiple platforms (Ubuntu and MacOS)
		- Detectors are locally sourced to test against most recent changes
		- There’s a new Release CI for automatic crates.io publishing, release creation and binary compilation
		- Binaries are now published into the release, allowing the usage of tools such as cargo-binstall
		- Binaries are published for Ubuntu, MacOS and Windows, for arm64 and x86_64

- We are working on a fix for windows tests, which are currently failing.
- And many more small changes


## v0.1.2 (2023-09-14)

- Test CI

## v0.1.1 (2023-06-30)

- Update detector status

## v0.1.0 (2023-06-30)

We're excited to announce the initial release of Scout! This release lays the groundwork for smart contract developers and auditors, to efficiently identify common security issues and deviations from best practices within their ink! smart contracts.
