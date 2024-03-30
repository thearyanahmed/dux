#!/bin/bash

# Define an array of file extensions
extensions=("png" "jpg" "jpeg" "mp4" "php" "py" "rs" "docs" "dox" "xl")

# Define the number of files to create
num_files=5

# Create tests directory if it doesn't exist
mkdir -p tests/fake

# Loop through each extension and create files inside tests directory
for ext in "${extensions[@]}"; do
    for (( i=1; i<=$num_files; i++ )); do
        touch "tests/fake/file$i.$ext"
    done
done

echo "files created successfully inside tests directory."
