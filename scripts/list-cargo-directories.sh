#!/bin/bash

# Find all directories with a Cargo.toml file
directories=$(find . -name Cargo.toml -exec dirname {} \; | sort -u)

# Print out the list of directories
for directory in $directories; do
  echo "$directory"
done
