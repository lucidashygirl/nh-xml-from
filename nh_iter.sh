#!/bin/bash

# Check if a directory is provided as an argument
if [ -z "$1" ]; then
    echo "Usage: $0 <directory>"
    exit 1
fi

# Check if the provided argument is a directory
if [ ! -d "$1" ]; then
    echo "Error: $1 is not a directory"
    exit 1
fi

# Iterate over every file in the directory
for file in "$1"/*; do
    if [ -f "$file" ]; then
        echo "Processing file: $file"
        nh-xml-from "$file"
    fi
done
