#!/bin/bash

# Fix Boolean Logic Errors Script
# This script fixes the 18 critical clippy errors related to tautological boolean expressions

set -e

echo "🔧 Fixing boolean logic errors..."

# List of files with boolean logic errors
FILES=(
    "crates/radix-leptos-primitives/src/components/accordion.rs"
    "crates/radix-leptos-primitives/src/components/alert.rs"
    "crates/radix-leptos-primitives/src/components/checkbox.rs"
    "crates/radix-leptos-primitives/src/components/dialog.rs"
    "crates/radix-leptos-primitives/src/components/progress.rs"
    "crates/radix-leptos-primitives/src/components/radio_group.rs"
    "crates/radix-leptos-primitives/src/components/select.rs"
    "crates/radix-leptos-primitives/src/components/slider.rs"
    "crates/radix-leptos-primitives/src/components/switch.rs"
    "crates/radix-leptos-primitives/src/components/tooltip.rs"
    "crates/radix-leptos-primitives/src/components/tabs.rs"
)

# Function to fix boolean logic errors in a file
fix_boolean_logic() {
    local file="$1"
    echo "  Processing: $file"
    
    # Create backup
    cp "$file" "$file.backup"
    
    # Fix tautological assertions - remove them as they're always true
    # Pattern: assert!(variable || !variable) -> // Removed tautological assertion
    
    sed -i.tmp 's/assert!(allow_multiple || !allow_multiple);/\/\/ Removed tautological assertion: allow_multiple || !allow_multiple/' "$file"
    sed -i.tmp 's/assert!(disabled || !disabled);/\/\/ Removed tautological assertion: disabled || !disabled/' "$file"
    sed -i.tmp 's/assert!(dismissible || !dismissible);/\/\/ Removed tautological assertion: dismissible || !dismissible/' "$file"
    sed -i.tmp 's/assert!(visible || !visible);/\/\/ Removed tautological assertion: visible || !visible/' "$file"
    sed -i.tmp 's/assert!(checked || !checked);/\/\/ Removed tautological assertion: checked || !checked/' "$file"
    sed -i.tmp 's/assert!(indeterminate || !indeterminate);/\/\/ Removed tautological assertion: indeterminate || !indeterminate/' "$file"
    sed -i.tmp 's/assert!(open || !open);/\/\/ Removed tautological assertion: open || !open/' "$file"
    
    # Clean up temporary files
    rm -f "$file.tmp"
    
    echo "    ✅ Fixed boolean logic errors"
}

# Process each file
for file in "${FILES[@]}"; do
    if [[ -f "$file" ]]; then
        fix_boolean_logic "$file"
    else
        echo "  ⚠️  File not found: $file"
    fi
done

echo "✅ Boolean logic errors fixed!"
echo "📝 Note: Review the commented assertions and implement proper test logic"
echo "🔄 Run 'cargo clippy' to verify fixes"
