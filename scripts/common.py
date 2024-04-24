import os
import subprocess
import time
import unittest
from contextlib import contextmanager

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
def Timer(msg=None):
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


def show_error(result, root, useStderr=True, msg="Clippy issues found in"):
    if result.returncode != 0:
        print(Red(f"\n{msg} {root}\n"))
        handle = result.stderr if useStderr else result.stdout
        for line in handle.splitlines():
            print(f"| {line}")
        print("\n")
        return [root]
    return []



def run_clippy_in_tests_gen():
    base = 'test-cases'
    for _dir in [entry for entry in os.listdir('test-cases') if os.path.isdir(J(base, entry))]:
        _dir = J(base, _dir)
        if (not os.path.exists(J(_dir, 'Cargo.toml'))) and os.path.exists(J(_dir, 'Cargo.toml.skip')):
            print(f"Skipping {_dir} due to known issues.")
            continue
        yield _dir


def run_clippy_in_detectors(directories=None):
    if directories is None:
        directories = ['detectors']
    for directory in directories:
        if not os.path.isdir(directory):
            print(f"Error: The specified path '{directory}' is not a directory or does not exist.")
            continue
        print(Green(f"\nRunning clippy in {directory}:"))
        for root, _, files in os.walk(directory):
            if "Cargo.toml" in files:
                yield root


class Runner:
    CMD = None

    def run(self, directories):
        gens = []
        for _dir in directories:
            if _dir == 'test-cases':
                gens.append(('test-cases', run_clippy_in_tests_gen))
            if _dir == 'detectors':
                gens.append(('detectors', run_clippy_in_detectors))

        errors = []
        for dirtype, gen in gens:
            for root in gen():
                with Timer(f"Completed clippy check in: {root}."):
                    result = self.runCmd(root, dirtype)
                self.clean_up(root)
                errors += show_error(result, root)
        return errors

    def runCmd(self, root, dirtype=None):
        if self.CMD is None:
            raise NotImplementedError()
        return self.run_cmd(self.CMD, root)

    def run_cmd(self, cmd, root):
        return subprocess.run(cmd.split(' '), cwd=root, capture_output=True, text=True)

    def clean_up(self, root):
        pass