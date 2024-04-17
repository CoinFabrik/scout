import argparse
import os
import subprocess
import time

RED = "\033[91m"
GREEN = "\033[92m"
BLUE = "\033[94m"
ENDC = "\033[0m"


def run_clippy(directories):
    errors = []
    for directory in directories:
        if not os.path.isdir(directory):
            errors.append(
                f"Error: The specified path '{directory}' is not a directory or does not exist."
            )
            continue

        print(f"\n{GREEN}Running clippy in {directory}:{ENDC}")
        for root, _, files in os.walk(directory):
            if root == "test-cases/ink-version/ink-version-1/vulnerable-example" or root == "test-cases/ink-version/ink-version-1/remediated-example":
                print(f"Skipping {root} due to known issues.")
                continue
            if "Cargo.toml" in files:
                start_time = time.time()
                result = get_command(directory, root)
                end_time = time.time()

                clean_up(root)

                elapsed_time = end_time - start_time
                print(
                    f"{BLUE}[> {elapsed_time:.2f} sec]{ENDC} - Completed clippy check in: {root}."
                )
                if result.returncode != 0:
                    print(f"\n{RED}Clippy issues found in: {root}{ENDC}\n")
                    error_message = result.stderr.strip()
                    for line in error_message.split("\n"):
                        print(f"| {line}")
                    print("\n")
                    errors.append(root)
    return errors


def clean_up(root):
    ret = subprocess.run(['du', '-hs', 'target'],
            cwd=root,
            capture_output=True,
            text=True)
    if ret.returncode!=0:
        print(" ?> ", ret.returncode)
        print(" -> ", ret.stdout)
        print(" e> ", ret.stderr)
        print(" -------------- ")
    ret = subprocess.run(['rm', '-rf', 'target'],
            cwd=root,
            capture_output=True,
            text=True)
    if ret.returncode!=0:
        print(" ?> ", ret.returncode)
        print(" -> ", ret.stdout)
        print(" e> ", ret.stderr)
        print(" -------------- ")


def get_command(directory, root):
    if directory == "test-cases":
        return subprocess.run(
            [
            "cargo",
            "+nightly-2023-12-16",
            "clippy",
            "--target=wasm32-unknown-unknown",
            "-Zbuild-std=std,core,alloc",
            "--no-default-features",
            "--",
            "-D",
            "warnings",
            "-A",
            "clippy::new_without_default", # this is not needed for ink!
            ],
            cwd=root,
            capture_output=True,
            text=True,
        )

    else:
        return subprocess.run(
            [
            "cargo",
            "clippy",
            "--",
            "-D",
            "warnings",
            "-A",
            "clippy::new_without_default", # this is not needed for ink!
            ],
            cwd=root,
            capture_output=True,
            text=True,
        )

def print_clippy_errors(errors):
    if errors:
        print(f"{RED}\nClippy errors detected in the following directories:{ENDC}")
        for error_dir in errors:
            print(f"• {error_dir}")
    else:
        print(f"{GREEN}\nNo clippy issues found across all directories.{ENDC}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Run cargo-clippy for specified directories"
    )
    parser.add_argument(
        "--dir",
        nargs="+",
        required=True,
        help="Specify the directories to run cargo-clippy on. Multiple directories can be specified.",
    )

    args = parser.parse_args()

    errors = run_clippy(args.dir)
    print_clippy_errors(errors)
    if errors:
        exit(1)
