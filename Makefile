ci: fmt lint test
ci-check: fmt-check lint test

fmt: fmt-rust
fmt-check: fmt-rust-check
lint: lint-rust

fmt-rust:
	@echo "Formatting Rust code..."
	@cd apps/cargo-scout && cargo +nightly fmt --all
	@cd detectors && cargo +nightly fmt --all

fmt-rust-check:
	@echo "Checking Rust code formatting..."
	@cd apps/cargo-scout && cargo +nightly fmt --all -- --check
	@cd detectors && cargo +nightly fmt --all -- --check

lint-rust:
	@echo "Linting Rust code..."
	@cd apps/cargo-scout && cargo clippy --all --all-features -- -D warnings
	@cd detectors && cargo clippy --all --all-features -- -D warnings

test:
	@echo "Running tests..."
	@cd apps/cargo-scout && cargo test --all --all-features
	@cd detectors && cargo test --all --all-features
