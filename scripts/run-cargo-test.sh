#!/bin/bash

# Read the list of directories from stdin
directories=$(cat)

# Create a list of directories to push into if any fail
failed_directories=""

# Run cargo fmt on each directory
for directory in $directories; do
    echo "Running cargo test on $directory"
    (cd "$directory" && cargo test --all --all-features --quiet)
    if [ $? -ne 0 ]; then
        failed_directories="$failed_directories\n$directory"
    fi
done

# If any directories failed, print them out and exit with an error
if [ -n "$failed_directories" ]; then
    printf "\nThe following directories failed cargo test:$failed_directories\n"
    exit 1
fi
