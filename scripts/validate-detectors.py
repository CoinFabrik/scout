import os
import re
from fuzzywuzzy import process

RED = "\033[91m"
GREEN = "\033[92m"
ENDC = "\033[0m"


def is_rust_project(dir_path):
    """Check if a directory contains a Rust project with a Cargo.toml and src/lib.rs."""
    errors = []
    has_cargo_toml = os.path.isfile(os.path.join(dir_path, "Cargo.toml"))
    has_cargo_toml_skip = os.path.isfile(os.path.join(dir_path, "Cargo.toml.skip"))
    has_lib_rs = os.path.isfile(os.path.join(dir_path, "src", "lib.rs"))

    if not (has_cargo_toml or has_cargo_toml_skip):
        errors.append(f"Missing Cargo.toml in {dir_path}.")
    if not has_lib_rs:
        errors.append(f"Missing src/lib.rs in {dir_path}.")

    return errors


def check_for_extra_files(directory):
    """Ensure there are no unexpected files in a given directory."""
    errors = []
    ignore_files = {"Cargo.lock", "Cargo.toml", "Cargo.toml.skip"}
    for item in os.listdir(directory):
        item_path = os.path.join(directory, item)
        if os.path.isfile(item_path) and item not in ignore_files:
            errors.append(f"Unexpected file found: {item_path}")
    return errors


def validate_naming_convention(example, detector_name):
    """Validate the naming convention of the example."""
    if not re.match(f"{re.escape(detector_name)}-\\d+$", example):
        return [
            f"Naming issue for '{example}' in {detector_name}: Expected format is {detector_name}-[number]."
        ]
    return []


def validate_example_structure(example_path, example_name):
    """Ensure each example has the required subdirectories with detailed errors."""
    errors = []
    expected_subs = ["vulnerable-example", "remediated-example"]
    actual_subs = [
        d
        for d in os.listdir(example_path)
        if os.path.isdir(os.path.join(example_path, d))
    ]

    for expected_sub in expected_subs:
        if expected_sub not in actual_subs:
            error_msg = f"Directory '{expected_sub}' not found in {example_path}."
            closest_match = process.extractOne(
                expected_sub, actual_subs, score_cutoff=80
            )
            if closest_match:
                error_msg += f" A similar directory exists: '{closest_match[0]}', please rename it to '{expected_sub}'."
            errors.append(error_msg)
        else:
            sub_errors = is_rust_project(os.path.join(example_path, expected_sub))
            for error in sub_errors:
                errors.append(error)

    return errors


def validate_examples(detector_path, examples):
    """Validate the structure and naming convention of examples."""
    errors = []
    ignore_dirs = {"target", ".cargo"}
    detector_name = os.path.basename(detector_path)
    example_suffixes = set()

    for example in examples:
        example_path = os.path.join(detector_path, example)
        if os.path.basename(example_path) not in ignore_dirs:
            errors.extend(check_for_extra_files(example_path))
            errors.extend(validate_naming_convention(example, detector_name))
            suffix = example.split("-")[-1]
            if suffix in example_suffixes:
                errors.append(
                    f"Duplicate example number found in {detector_name}: {example}"
                )
            else:
                example_suffixes.add(suffix)
            errors.extend(validate_example_structure(example_path, example))
    return errors


def validate_detectors(test_cases_path, detectors_path):
    """Validate the structure of the test-cases directory."""
    errors = []

    # Directories to ignore while validating
    ignore_dirs = {"target", ".cargo"}

    test_cases = [
        tc
        for tc in os.listdir(test_cases_path)
        if os.path.isdir(os.path.join(test_cases_path, tc))
    ]

    for test_case in test_cases:
        test_case_path = os.path.join(test_cases_path, test_case)

        print(f"Validating {test_case}...")

        # Validate that the detector exists
        if not os.path.exists(os.path.join(detectors_path, test_case)):
            errors.append(
                f"Detector folder missing for {test_case} in {detectors_path}"
            )
        else:
            errors.extend(is_rust_project(os.path.join(detectors_path, test_case)))

        # Check for unwanted files in the test case directory
        errors.extend(check_for_extra_files(test_case_path))
        examples = [
            e
            for e in os.listdir(test_case_path)
            if os.path.isdir(os.path.join(test_case_path, e))
        ]
        if not examples:
            errors.append(f"No examples found in {test_case}.")
        else:
            # Validate each vulnerable and remediated example
            errors.extend(validate_examples(test_case_path, examples))

    # Validate that each detector has a test case
    for detector in os.listdir(detectors_path):
        if detector in ignore_dirs or not os.path.isdir(
                os.path.join(detectors_path, detector)
        ):
            continue

        if detector not in test_cases:
            errors.append(
                f"Test case missing for detector {detector} in {test_cases_path}"
            )

    return errors


if __name__ == "__main__":
    TEST_CASES_PATH = "test-cases"
    DETECTORS_PATH = "detectors"
    errors = validate_detectors(TEST_CASES_PATH, DETECTORS_PATH)
    if errors:
        print(f"{RED}\nValidation errors found:{ENDC}")
        for error in errors:
            print(f"* {error}")
        exit(1)
    else:
        print(f"{GREEN}\nAll detectors and test cases are valid.{ENDC}")
