ci: fmt lint test
ci-check: fmt-check lint test

fmt: fmt-rust
fmt-check: fmt-rust-check
lint: lint-rust

fmt-rust:
	@echo "Formatting Rust code..."
	@cargo +nightly fmt --all

fmt-rust-check:
	@echo "Checking Rust code formatting..."
	@cargo +nightly fmt --all -- --check

lint-rust:
	@echo "Linting Rust code..."
	@cargo clippy --all --all-features -- -D warnings

test:
	@echo "Running tests..."
	@cargo test --all --all-features
