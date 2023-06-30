ci: fmt lint test
ci-check: fmt-check lint test

fmt: fmt-rust
fmt-check: fmt-rust-check
lint: lint-rust

fmt-rust:
	@echo "Formatting Rust code..."
	@./scripts/list-cargo-directories.sh | ./scripts/run-cargo-fmt.sh

fmt-rust-check:
	@echo "Checking Rust code formatting..."
	@./scripts/list-cargo-directories.sh | ./scripts/run-cargo-fmt.sh --check

lint-rust:
	@echo "Linting Rust code..."
	@./scripts/list-cargo-directories.sh | ./scripts/run-cargo-clippy.sh

test:
	@echo "Running tests..."
	@cd apps/cargo-scout-audit && cargo test --all --all-features -- --nocapture
	@cd test-cases && ../scripts/list-cargo-directories.sh | ../scripts/run-cargo-test.sh
