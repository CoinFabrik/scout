#!/bin/bash

# Read the list of directories from stdin
directories=$(cat)

# Run cargo fmt on each directory
for directory in $directories; do
    echo "Running cargo test on $directory"
    (cd "$directory" && cargo test --all --all-features)
done
