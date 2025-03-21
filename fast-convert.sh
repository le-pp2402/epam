#!/bin/bash

INPUT_DIR="."
OUTPUT_DIR="./output"

mkdir -p "$OUTPUT_DIR"

for file in "$INPUT_DIR"/*.xopp; do
    if [[ -f "$file" ]]; then
        filename=$(basename -- "$file")
        output_file="$OUTPUT_DIR/${filename%.xopp}.pdf"
        
        echo "Đang chuyển đổi: $file -> $output_file"
        xournalpp -p "$output_file" "$file"
    fi
done

echo "Hoàn tất chuyển đổi!"
