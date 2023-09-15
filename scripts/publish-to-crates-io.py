from pathlib import Path
import shutil
import subprocess
import tempfile
import time

def get_package_name(file_path):
    with open(file_path, "r") as f:
        for line in f:
            if line.startswith("name"):
                return line.split("=")[1].strip().strip('"')
    raise Exception("Could not find package name in Cargo.toml")

def get_package_version(file_path):
    with open(file_path, "r") as f:
        for line in f:
            if line.startswith("version"):
                return line.split("=")[1].strip().strip('"')
    raise Exception("Could not find package version in Cargo.toml")

def is_package_published(package_name, package_version, package_path):
    with tempfile.TemporaryDirectory() as tmpdirname:
        subprocess.call(["cargo", "init"], cwd=tmpdirname)
        if (package_path / "rust-toolchain").exists():
            shutil.copy(package_path / "rust-toolchain", Path(tmpdirname) / "rust-toolchain")
        with open(Path(tmpdirname) / "Cargo.toml", "a") as f:
            f.write(f'{package_name} = "={package_version}"')
        print(f"Checking if {package_name} {package_version} is published...")
        try:
            subprocess.check_output(["cargo", "check"], cwd=tmpdirname)
            return True
        except subprocess.CalledProcessError:
            return False
        except:
            raise

def publish_package(package_path):
    subprocess.check_output(["cargo", "publish"], cwd=package_path)

if __name__ == "__main__":
    ROOT_PATH = Path(__file__).parent.parent
    CARGO_SCOUT_AUDIT_PATH = ROOT_PATH / "apps" / "cargo-scout-audit"
    SCOUT_AUDIT_CLIPPY_UTILS = ROOT_PATH / "scout-audit-clippy-utils"
    SCOUT_AUDIT_INTERNAL_PATH = ROOT_PATH / "scout-audit-internal"

    packages_paths = [SCOUT_AUDIT_CLIPPY_UTILS, SCOUT_AUDIT_INTERNAL_PATH, CARGO_SCOUT_AUDIT_PATH]

    for path in packages_paths:
        package_name = get_package_name(path / "Cargo.toml")
        package_version = get_package_version(path / "Cargo.toml")
        print(f"Publishing {package_name} {package_version}...")

        if not is_package_published(package_name, package_version, path):
            publish_package(path)
            while not is_package_published(package_name, package_version, path):
                print(f"{package_name} {package_version} is not published yet, waiting 10 seconds...")
                time.sleep(10)
            print(f"{package_name} {package_version} is published")
        else:
            print(f"{package_name} {package_version} is already published")
