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

# Create a list of directories to push into if any fail
failed_directories=""

# Run cargo fmt on each directory
for directory in $directories; do
    echo "Running cargo fmt on $directory"
    if [[ $check -eq 1 ]]; then
        (cd "$directory" && cargo +nightly fmt --check)
    else
        (cd "$directory" && cargo +nightly fmt)
    fi

    if [ $? -ne 0 ]; then
        failed_directories="$failed_directories\n$directory"
    fi
done

# If any directories failed, print them out and exit with an error
if [ -n "$failed_directories" ]; then
    printf "\nThe following directories failed cargo fmt:$failed_directories\n"
    exit 1
fi
