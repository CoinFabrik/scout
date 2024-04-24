import argparse
import os
import subprocess

from common import J, RED, GREEN, ENDC, Green, Timer, show_error, Runner, Red


class ClippyRunner(Runner):
    def runCmd(self, root, dirtype=None):
        if dirtype == "test-cases":
            self.CMD = ("cargo +nightly-2023-12-16 clippy --target=wasm32-unknown-unknown -Zbuild-std=std,core,alloc "
                   "--no-default-features -- -D warnings -A clippy::new_without_default")
        else:
            self.CMD = ("cargo clippy -- -D warnings -A clippy::new_without_default")
        return super().runCmd(root, dirtype)


    def clean_up(self, root):
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


def print_clippy_errors(errors):
    if errors:
<<<<<<< HEAD
        print(f"{RED}\nClippy errors detected in the following directories:{ENDC}")
        for error_dir in errors:
            print(f"• {error_dir}")
    else:
        print(f"{GREEN}\nNo clippy issues found across all directories.{ENDC}")
=======
        print(Red(f"\nClippy errors detected in the following directories:"))
        for error_dir in errors:
            print(f"• {error_dir}")
    else:
        print(Green(f"\nNo clippy issues found across all directories."))
>>>>>>> main


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
<<<<<<< HEAD

    errors = run_clippy(args.dir)
=======
    errors = ClippyRunner().run(args.dir)   # errors = run_clippy(args.dir)
>>>>>>> main
    print_clippy_errors(errors)
    if errors:
        exit(1)
