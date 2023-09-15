import argparse
import os


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


def find_repeated(item_list):
    seen = set()
    dupes = [x for x in item_list if x in seen or seen.add(x)]
    return list(set(dupes))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Check if any detectors names are the same"
    )
    parser.add_argument("detectors_path", type=str, help="Path to detectors folder")
    args = parser.parse_args()

    results = get_recursive_package_names(args.detectors_path)

    # Find repeated and print results
    sorted_results = sorted(results, key=lambda x: x[1])
    sorted_packages = [r[1] for r in sorted_results]
    repeated_packages = find_repeated(sorted_packages)

    repeated_results = []
    for rp in repeated_packages:
        matches = [x for x in sorted_results if x[1] == rp]
        repeated_results.append(matches)

    if len(repeated_results) > 0:
        for rr in repeated_results:
            print(f"Found detectors with the same name `{rr[0][1]}`:")
            for r in rr:
                print(f"\t{r[0]}")
        exit(1)
