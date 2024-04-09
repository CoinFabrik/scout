import json
import os
import re
import subprocess
import argparse
import time

RED = "\033[91m"
GREEN = "\033[92m"
BLUE = "\033[94m"
ENDC = "\033[0m"


def parse_json_from_string(console_output):
    brace_count = 0
    json_start = None
    json_end = None

    for i, char in enumerate(console_output):
        if char == "{":
            brace_count += 1
            if brace_count == 1:
                json_start = i
        elif char == "}":
            brace_count -= 1
            if brace_count == 0 and json_start is not None:
                json_end = i + 1
                break

    if json_start is not None and json_end is not None:
        json_str = console_output[json_start:json_end]
        try:
            return json.loads(json_str)
        except json.JSONDecodeError:
            return "Extracted string is not valid JSON"
    else:
        return "No JSON found in the console output"


def run_unit_tests(root):
    start_time = time.time()
    result = subprocess.run(
        ["cargo", "test", "--all-features", "--all"],
        cwd=root,
        capture_output=True,
        text=True,
    )
    end_time = time.time()
    elapsed_time = end_time - start_time
    print(f"{BLUE}[> {elapsed_time:.2f} sec]{ENDC} - Completed unit test in: {root}.")
    if result.returncode != 0:
        print(f"\n{RED}Test error found in: {root}{ENDC}\n")
        error_message = result.stdout.strip()
        for line in error_message.split("\n"):
            print(f"| {line}")
        print("\n")
        return True
    return False


def run_integration_tests(detector, root):
    start_time = time.time()
    short_message = ""
    detector_metadata_result = subprocess.run(
        ["cargo", "scout-audit", "--filter", detector, "--metadata"],
        cwd=root,
        capture_output=True,
        text=True,
    )

    detector_metadata = parse_json_from_string(detector_metadata_result.stdout)
    if not isinstance(detector_metadata, dict):
        print("Failed to extract JSON:", detector_metadata)
        return True

    detector_key = detector.replace("-", "_")
    short_message = detector_metadata.get(detector_key, {}).get("short_message")

    result = subprocess.run(
        ["cargo", "scout-audit", "--filter", detector],
        cwd=root,
        capture_output=True,
        text=True,
    )

    elapsed_time = time.time() - start_time
    print(
        f"{BLUE}[> {elapsed_time:.2f} sec]{ENDC} - Completed integration test in: {root}."
    )

    should_lint = root.endswith("vulnerable-example")
    if result.returncode != 0 or (
        should_lint and short_message and short_message not in result.stderr
    ):
        print(f"\n{RED}Test error found in: {root}{ENDC}\n")
        error_message = result.stderr.strip()
        for line in error_message.split("\n"):
            print(f"| {line}")
        print("\n")
        return True

    return False


def run_tests(detector):
    errors = []
    directory = os.path.join("test-cases", detector)
    print(f"\n{GREEN}Performing tests in {directory}:{ENDC}")
    if os.path.exists(directory):
        for root, _, files in os.walk(directory):
            if "Cargo.toml" in files:
                if run_unit_tests(root):
                    errors.append(root)
                if run_integration_tests(detector, root):
                    errors.append(root)
    else:
        print(f"{RED}The specified directory does not exist.{ENDC}")
    return errors


def print_tests_errors(errors):
    if errors:
        print(f"{RED}\nErrors detected in the following directories:{ENDC}")
        for error_dir in errors:
            print(f"• {error_dir}")
    else:
        print(f"{GREEN}\nNo errors found in the specified directory.{ENDC}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run tests for a specific detector.")
    parser.add_argument(
        "--detector",
        type=str,
        help='The detector to run tests for, e.g., "unsafe-unwrap"',
    )

    args = parser.parse_args()

    if args.detector:
        errors = run_tests(args.detector)
        print_tests_errors(errors)
        if errors:
            exit(1)
    else:
        print(f"{RED}No detector specified. Please provide a detector argument.{ENDC}")
