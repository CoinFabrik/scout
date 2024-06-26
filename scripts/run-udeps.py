import argparse
import os
import subprocess
import time

RED = "\033[91m"
GREEN = "\033[92m"
BLUE = "\033[94m"
ENDC = "\033[0m"


def run_udeps(directories):
    errors = []
    for directory in directories:
        if not os.path.isdir(directory):
            errors.append(
                f"Error: The specified path '{directory}' is not a directory or does not exist."
            )
            continue

        print(f"\n{GREEN}Checking unused dependencies in {directory}:{ENDC}")
        for root, _, files in os.walk(directory):
            if "Cargo.toml" in files:
                start_time = time.time()
                result = subprocess.run(
                    [
                        "cargo",
                        "udeps",
                        "--all-targets",
                    ],
                    cwd=root,
                    capture_output=True,
                    text=True,
                )
                end_time = time.time()
                elapsed_time = end_time - start_time
                print(
                    f"{BLUE}[> {elapsed_time:.2f} sec]{ENDC} - Completed udeps check in: {root}."
                )
                if result.returncode != 0:
                    print(f"\n{RED}Unused dependencies found in: {root}{ENDC}\n")
                    error_message = result.stderr.strip()
                    for line in error_message.split("\n"):
                        print(f"| {line}")
                    print("\n")
                    errors.append(root)
    return errors


def print_udeps_errors(errors):
    if errors:
        print(
            f"{RED}\nUnused dependencies detected in the following directories:{ENDC}"
        )
        for error_dir in errors:
            print(f"• {error_dir}")
    else:
        print(f"{GREEN}\nNo unused dependencies found across all directories.{ENDC}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Run cargo-udeps for specified directories"
    )
    parser.add_argument(
        "--dir",
        nargs="+",
        required=True,
        help="Specify the directories to run cargo-udeps on. Multiple directories can be specified.",
    )

    args = parser.parse_args()

    errors = run_udeps(args.dir)
    print_udeps_errors(errors)
    if errors:
        exit(1)
