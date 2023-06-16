#!/bin/bash

# Read the list of directories from stdin
directories=$(cat)

# Run cargo fmt on each directory
for directory in $directories; do
    echo "Running cargo clippy on $directory"
    (cd "$directory" && cargo clippy --all --all-features --quiet -- -D warnings)
done
