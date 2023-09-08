import argparse
import os
import yaml

def is_special_directory(directory):
    return directory == ".cargo" or directory == ".git" or directory == "target"

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Check if all detectors were tested by the CI")
    parser.add_argument("gh_workflow_path", type=str, help="Path to the github workflow.")
    parser.add_argument("detectors_path", type=str, help="Path to detectors folder")
    args = parser.parse_args()

    with open(args.gh_workflow_path, "r") as f:
        workflow = yaml.safe_load(f)
        detectors_to_test = workflow["jobs"]["test"]["strategy"]["matrix"]["test"]

        detectors = [ f.name for f in os.scandir(args.detectors_path) if f.is_dir() and not is_special_directory(f.name) ]
        detectors.sort()

        if (detectors != detectors_to_test):
            print("Detectors to test in the workflow are not the same as the detectors in the detectors folder.")
            print("Detectors to test: ", detectors_to_test)
            print("Detectors in the folder: ", detectors)
            exit(1)
