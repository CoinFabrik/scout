ci: validate fmt lint test
ci-no-test: validate fmt lint

validate:
	@echo "\033[0;32m\n==> Validating the project structure and test cases... \033[0m"
	@python3 scripts/validate-detectors.py

fmt:
	@echo "\033[0;32m\n---> Formatting test cases and detectors... \033[0m"
	@python3 scripts/run-fmt.py --dir test-cases detectors

lint: lint-detectors lint-test-cases

lint-detectors:
	@echo "\033[0;32m\n--> Linting detectors... \033[0m"
	@python3 scripts/run-clippy.py --dir detectors

lint-test-cases:
	@echo "\033[0;32m\n--> Linting test cases... \033[0m"
	@python3 scripts/run-clippy.py --dir test-cases


test:
	@echo "\033[0;32m\n--> Running tests for test cases... \033[0m"
	@for dir in test-cases/*; do \
		if [ -d "$$dir" ]; then \
			detector_name=$$(basename "$$dir"); \
			python3 scripts/run-tests.py --detector=$$detector_name; \
		fi; \
	done
