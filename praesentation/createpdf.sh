#!/bin/bash

# Function to enable PDF output in book.toml
enable_pdf_output() {
    sed -i 's/# \(\[output.html\]\)/\1/' book.toml
    sed -i 's/# \(\[output.pdf\]\)/\1/' book.toml
    sed -i 's/# \(\[output.pdf-outline\]\)/\1/' book.toml
}

# Function to disable PDF output in book.toml
disable_pdf_output() {
    sed -i 's/\(\[output.html\]\)/# \1/' book.toml
    sed -i 's/\(\[output.pdf\]\)/# \1/' book.toml
    sed -i 's/\(\[output.pdf-outline\]\)/# \1/' book.toml
}

# Enable PDF output, build the book, and then disable PDF output
enable_pdf_output
mdbook build
disable_pdf_output
echo "xdg-open ./book/pdf-outline/output.pdf"
echo "mv ./book/pdf-outline/output.pdf ../2024-04-20-linux-tag-rust-picow-workshop.pdf"
