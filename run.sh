#!/bin/bash

# Check if a Rust file was provided as an argument
if [ -z "$1" ]; then
  echo "Usage: $0 <rust_file>"
  exit 1
fi

# Get the Rust file path from the first argument
RUST_FILE="$1"

# Compile the Rust file
rustc "$RUST_FILE"

# Check if the compilation was successful
if [ $? -eq 0 ]; then
  # Get the binary name from the Rust file name
  BINARY_NAME="${RUST_FILE%.*}"
  
  # Run the compiled binary
  "./$BINARY_NAME"
else
  echo "Compilation failed"
fi


# Wait for user input before exiting
read -n1 -r -p "Press any key to exit..." key