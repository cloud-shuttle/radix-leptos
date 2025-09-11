#!/bin/bash

# Fix Assertion Patterns Script
# This script fixes assertion patterns by replacing assert!(false) with panic!() and removing assert!(true)

set -e

echo "🔧 Fixing assertion patterns..."

# Function to fix assertion patterns in a file
fix_assertion_patterns() {
    local file="$1"
    echo "  Processing: $file"
    
    # Create backup
    cp "$file" "$file.backup"
    
    # Replace assert!(false) with panic!() or unreachable!()
    sed -i.tmp 's/assert!(false);/panic!("Unexpected condition reached");/g' "$file"
    
    # Remove assert!(true) as it's optimized out by compiler
    sed -i.tmp 's/assert!(true);/\/\/ Removed assert!(true) - always true/g' "$file"
    
    # Clean up temporary files
    rm -f "$file.tmp"
    
    echo "    ✅ Fixed assertion patterns"
}

# Find files with assertion pattern warnings
echo "  Searching for files with assertion patterns..."

# Process specific files known to have assertion issues
FILES=(
    "crates/radix-leptos-primitives/src/components/checkbox.rs"
    "crates/radix-leptos-primitives/src/components/radio_group.rs"
    "crates/radix-leptos-primitives/src/components/switch.rs"
    "crates/radix-leptos-primitives/src/components/tooltip.rs"
    "crates/radix-leptos-primitives/src/components/tabs.rs"
    "crates/radix-leptos-primitives/src/components/select.rs"
)

for file in "${FILES[@]}"; do
    if [[ -f "$file" ]]; then
        fix_assertion_patterns "$file"
    else
        echo "  ⚠️  File not found: $file"
    fi
done

echo "✅ Assertion patterns fixed!"
echo "🔄 Run 'cargo clippy' to verify fixes"
