#!/bin/bash

# Helper script to compile and run Rust files in the basic/ directory
# Usage: ./run_basic.sh filename.rs
# Example: ./run_basic.sh hello.rs

if [ $# -eq 0 ]; then
    echo "Usage: $0 <filename.rs>"
    echo "Example: $0 hello.rs"
    echo ""
    echo "Available files in basic/:"
    ls basic/*.rs | sed 's/basic\//  /'
    exit 1
fi

FILENAME="$1"
BASENAME="${FILENAME%.rs}"

# Check if file exists
if [ ! -f "basic/$FILENAME" ]; then
    echo "Error: File 'basic/$FILENAME' not found!"
    echo ""
    echo "Available files in basic/:"
    ls basic/*.rs | sed 's/basic\//  /'
    exit 1
fi

echo "🦀 Compiling and running: $FILENAME"
echo "=================================="

# Compile and run
cd basic
if rustc "$FILENAME"; then
    echo ""
    echo "🚀 Output:"
    echo "----------"
    ./"$BASENAME"
    echo ""
    echo "✅ Execution completed!"

    # Clean up the executable
    rm -f "$BASENAME"
else
    echo "❌ Compilation failed!"
    exit 1
fi
