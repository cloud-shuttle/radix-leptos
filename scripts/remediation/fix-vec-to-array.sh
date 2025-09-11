#!/bin/bash

# Fix Vec! to Array Script
# This script replaces vec![] with arrays for static data

set -e

echo "🔧 Fixing vec! to array conversions..."

# Function to fix vec! to array in a file
fix_vec_to_array() {
    local file="$1"
    echo "  Processing: $file"
    
    # Create backup
    cp "$file" "$file.backup"
    
    # Replace vec! with arrays for static data
    sed -i.tmp 's/let _button_variants = vec!\[/let _button_variants = [/g' "$file"
    sed -i.tmp 's/let _button_sizes = vec!\[/let _button_sizes = [/g' "$file"
    sed -i.tmp 's/let _directions = vec!\[/let _directions = [/g' "$file"
    
    # Clean up temporary files
    rm -f "$file.tmp"
    
    echo "    ✅ Fixed vec! to array conversions"
}

# File with vec! warnings
FILE="crates/radix-leptos-primitives/tests/compilation_tests.rs"

if [[ -f "$FILE" ]]; then
    fix_vec_to_array "$FILE"
else
    echo "  ⚠️  File not found: $FILE"
fi

echo "✅ Vec! to array conversions fixed!"
echo "🔄 Run 'cargo clippy' to verify fixes"
