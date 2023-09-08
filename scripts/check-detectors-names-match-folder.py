import argparse
import os
from pathlib import Path

def get_recursive_package_names(root_path):
    results = []

    # Walk through the directory structure
    for dirpath, dirnames, filenames in os.walk(root_path):
        if "Cargo.toml" in filenames:
            file_path = os.path.join(dirpath, "Cargo.toml")
            with open(file_path, "r") as f:
                lines = f.readlines()
                found_package_name = False
                for line in lines:
                    if line.strip() == "[workspace]":
                        found_package_name = True
                        break
                    if "name =" in line:
                        package_name = line.split("=")[1].strip().replace('"', "")
                        results.append((file_path, package_name))
                        found_package_name = True
                        break
                if not found_package_name:
                    raise Exception(f"Failed to parse package name from {file_path}")
    return results

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Check if any detectors names don't match their folder"
    )
    parser.add_argument("detectors_path", type=str, help="Path to detectors folder")
    args = parser.parse_args()

    results = get_recursive_package_names(args.detectors_path)
    sorted_results = sorted(results, key=lambda x: x[1])

    # Check if they match the name of the folder
    failed_results = []
    for r in sorted_results:
        folder_name = Path(r[0]).parent.name
        if folder_name != r[1]:
            failed_results.append(r)

    if len(failed_results) > 0:
        print("Found detectors with different package names than their folder:")
        for fr in failed_results:
            print(f"\t{fr[0]} - {fr[1]}")
        exit(1)
