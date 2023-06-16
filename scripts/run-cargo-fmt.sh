#!/bin/bash

# Parse command line arguments
check=0
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --check) check=1 ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

# Read the list of directories from stdin
directories=$(cat)

# Run cargo fmt on each directory
for directory in $directories; do
    echo "Running cargo fmt on $directory"
    if [[ $check -eq 1 ]]; then
        (cd "$directory" && cargo +nightly fmt --check)
    else
        (cd "$directory" && cargo +nightly fmt)
    fi
done
