import os
import sys
import re
from fuzzywuzzy import process


def is_rust_project(dir_path):
    """Check if a directory contains a Rust project with a Cargo.toml and src/lib.rs."""
    errors = []
    has_cargo_toml = os.path.isfile(os.path.join(dir_path, "Cargo.toml"))
    has_lib_rs = os.path.isfile(os.path.join(dir_path, "src", "lib.rs"))

    if not has_cargo_toml:
        errors.append(f"Missing Cargo.toml in {dir_path}.")
    if not has_lib_rs:
        errors.append(f"Missing src/lib.rs in {dir_path}.")

    return errors


def check_for_extra_files(directory):
    """Ensure there are no unexpected files in a given directory."""
    errors = []
    for item in os.listdir(directory):
        item_path = os.path.join(directory, item)
        if os.path.isfile(item_path):
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
    detector_name = os.path.basename(detector_path)
    example_suffixes = set()

    for example in examples:
        example_path = os.path.join(detector_path, example)
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


def validate_detectors(base_path):
    """Validate the structure of the test-cases directory."""
    all_errors = []

    for detector in os.listdir(base_path):
        detector_path = os.path.join(base_path, detector)
        if detector == "README.md" or not os.path.isdir(detector_path):
            continue

        all_errors.extend(check_for_extra_files(detector_path))
        examples = [
            e
            for e in os.listdir(detector_path)
            if os.path.isdir(os.path.join(detector_path, e))
        ]
        if not examples:
            all_errors.append(f"No examples found in {detector}.")
        else:
            all_errors.extend(validate_examples(detector_path, examples))

    if all_errors:
        print("Validation errors found:")
        for error in all_errors:
            print(f"* {error}")
        sys.exit(1)
    else:
        print("No validation errors found.")


if __name__ == "__main__":
    BASE_PATH = "test-cases"
    validate_detectors(BASE_PATH)
