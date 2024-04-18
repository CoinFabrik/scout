import unittest
from contextlib import contextmanager
import argparse
import os
import subprocess
import time

J = os.path.join


RED = "\033[91m"
GREEN = "\033[92m"
BLUE = "\033[94m"
ENDC = "\033[0m"


def Color(color, endcolor=ENDC):
    def f(msg):
        return '\n'.join( [color+line+endcolor if line != '' else '' for line in msg.split('\n')] )
    return f


Red = Color(RED)
Green = Color(GREEN)
Blue = Color(BLUE)


class ColorTest(unittest.TestCase):
    def test_1(self):
        self.assertEqual(f"\n{RED}Clippy issues found in: root{ENDC}\n",
                         Red(f"\nClippy issues found in: root\n"))



@contextmanager
def timeit(msg=None):
    class Elapsed:
        def __init__(self):
            self.start = time.time()
            self.end = None
        def done(self):
            if self.end is None:
                self.end = time.time()
        @property
        def value(self):
            self.done()
            return self.end-self.start
        def show(self, msg):
            self.done()
            print(Blue(f"{BLUE}[> {self.value:.2f} sec]") + f" - {msg}.")

    elapsed = Elapsed()
    try:
        yield elapsed
    finally:
        elapsed.done()
        if msg: elapsed.show(msg)


class FakeRet:
    returncode = 0



def show_error(result, root):
    if result.returncode != 0:
        print(Red(f"\nClippy issues found in: {root}\n"))
        error_message = result.stderr.strip()
        for line in error_message.split("\n"):
            print(f"| {line}")
        print("\n")
        return [root]
    return []


def run_clippy_in_tests():
    base = 'test-cases'
    errors = []
    for _dir in [entry for entry in os.listdir('test-cases') if os.path.isdir(J(base, entry))]:
        _dir = J(base, _dir)
        print("dir:", _dir)
        with timeit(f"Completed clippy check in: {_dir}."):
            result = get_command('test-cases', _dir)
        clean_up(_dir)
        errors += show_error(result, _dir)
    return errors


SKIP_DIRS = set(
    ["test-cases/ink-version/ink-version-1/vulnerable-example",
    "test-cases/ink-version/ink-version-1/remediated-example"])


def run_clippy(directories):
    errors = []
    for directory in directories:
        if directory=='test-cases':
            errors += run_clippy_in_tests()
            continue
        if not os.path.isdir(directory):
            errors.append(f"Error: The specified path '{directory}' is not a directory or does not exist.")
            continue

        print(Green(f"\nRunning clippy in {directory}:"))
        for root, _, files in os.walk(directory):
            if root in SKIP_DIRS:
                print(f"Skipping {root} due to known issues.")
                continue

            if "Cargo.toml" in files:
                with timeit(f"Completed clippy check in: {root}."):
                    result = get_command(directory, root)
                clean_up(root)
                errors += show_error(result, root)
    return errors


def clean_up(root):
    ret = subprocess.run(['du', '-hs', 'target'],
            cwd=root,
            capture_output=True,
            text=True)
    if ret.returncode==0:
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
        cmd = ("cargo +nightly-2023-12-16 clippy --target=wasm32-unknown-unknown -Zbuild-std=std,core,alloc "
               "--no-default-features -- -D warnings -A clippy::new_without_default")
    else:
        cmd = ("cargo clippy -- -D warnings -A clippy::new_without_default")

    return subprocess.run(
        cmd.split(' '),
        cwd=root,
        capture_output=True,
        text=True)


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
