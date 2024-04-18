import argparse

from common import J, RED, GREEN, ENDC, Runner, Green


class FmtRunner(Runner):
    def runCmd(self, root, dirtype=None):
        # first fix ..
        super().run_cmd("cargo +nightly fmt", root)
        # .. then check!
        return super().run_cmd("cargo +nightly fmt -- --check -v", root)


def print_fmt_errors(errors):
    if errors:
        print(Green(f"\nFormatting errors detected in the following directories:"))
        for error_dir in errors:
            print(f"• {error_dir}")
    else:
        print(Green(f"\nNo formatting issues found across all directories."))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Run cargo-fmt for specified directories")

    parser.add_argument(
        "--dir",
        nargs="+",
        required=True,
        help="Specify the directories to run cargo-fmt on. Multiple directories can be specified.")

    args = parser.parse_args()
    errors = FmtRunner().run(args.dir)  # errors = run_fmt(args.dir)
    print_fmt_errors(errors)
    if errors:
        exit(1)
