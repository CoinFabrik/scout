#!/bin/bash
set -eo pipefail

# Set the command output (redirecting stderr to stdout)
output=$(cargo run scout -m "/Users/josegarcia/Desktop/coinfabrik/web3grant/web3-grant/vulnerabilities/examples/panic-error/vulnerable-example/Cargo.toml" 2>&1)

echo "$output" | while IFS= read -r line
do
  # Check if the line contains the warning message
  if [[ $line == *"warning: The panic! macro is used to stop execution when a condition is not met. This is useful for testing and prototyping, but should be avoided in production code"* ]]; then
    echo "Found the warning message in the output"
  fi
done


- Adds a vulnerabilities and/or tests folder to the repo with the [vulnerability examples (before and after remediation)](https://www.youtube.com/watch?v=6p8zAbFKpW0) from the PoC milestone.
