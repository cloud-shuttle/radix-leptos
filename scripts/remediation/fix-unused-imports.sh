#!/bin/bash

# Fix Unused Imports Script
# This script removes unused imports from example files

set -e

echo "🔧 Fixing unused imports..."

# Function to fix unused imports in a file
fix_unused_imports() {
    local file="$1"
    echo "  Processing: $file"
    
    # Create backup
    cp "$file" "$file.backup"
    
    # Remove unused imports
    sed -i.tmp '/^use leptos::\*;$/d' "$file"
    sed -i.tmp '/^use leptos::logging::log;$/d' "$file"
    
    # Clean up temporary files
    rm -f "$file.tmp"
    
    echo "    ✅ Fixed unused imports"
}

# Files with unused imports
FILES=(
    "examples/src/alert_examples.rs"
    "examples/src/badge_examples.rs"
    "examples/src/timeline_examples.rs"
    "examples/src/toast_examples.rs"
)

for file in "${FILES[@]}"; do
    if [[ -f "$file" ]]; then
        fix_unused_imports "$file"
    else
        echo "  ⚠️  File not found: $file"
    fi
done

echo "✅ Unused imports fixed!"
echo "🔄 Run 'cargo clippy' to verify fixes"
