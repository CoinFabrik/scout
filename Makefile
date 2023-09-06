ci: fmt lint test
ci-check: fmt-check lint test

fmt: fmt-rust
fmt-check: fmt-rust-check
lint: lint-cargo-scout-audit lint-detectors lint-scout-audit-internal

fmt-rust:
	@echo "Formatting Rust code..."
	@./scripts/list-cargo-directories.sh | ./scripts/run-cargo-fmt.sh

fmt-rust-check:
	@echo "Checking Rust code formatting..."
	@./scripts/list-cargo-directories.sh | ./scripts/run-cargo-fmt.sh --check

lint-cargo-scout-audit:
	@echo "Linting cargo-scout-audit..."
	@cd apps/cargo-scout-audit && cargo clippy --all --all-features --quiet -- -D warnings

lint-detectors:
	@echo "Linting detectors..."
	@cd detectors && ../scripts/list-cargo-directories.sh | ../scripts/run-cargo-clippy.sh

lint-scout-audit-internal:
	@echo "Linting scout-audit-internal..."
	@cd scout-audit-internal && cargo clippy --all --all-features --quiet -- -D warnings

test:
	@echo "Running tests..."
	@cd apps/cargo-scout-audit && cargo test --all --all-features -- --nocapture
	@cd test-cases && ../scripts/list-cargo-directories.sh | ../scripts/run-cargo-test.sh
