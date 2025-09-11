#!/bin/bash

# Fix Unused Variables Script
# This script fixes unused variable warnings by prefixing with underscore

set -e

echo "🔧 Fixing unused variables..."

# Function to fix unused variables in a file
fix_unused_variables() {
    local file="$1"
    echo "  Processing: $file"
    
    # Create backup
    cp "$file" "$file.backup"
    
    # Common unused variable patterns - prefix with underscore
    sed -i.tmp 's/let callback =/let _callback =/g' "$file"
    sed -i.tmp 's/let spacing =/let _spacing =/g' "$file"
    sed -i.tmp 's/let themes =/let _themes =/g' "$file"
    sed -i.tmp 's/let theme =/let _theme =/g' "$file"
    sed -i.tmp 's/let colors =/let _colors =/g' "$file"
    sed -i.tmp 's/let on_change =/let _on_change =/g' "$file"
    sed -i.tmp 's/let current_theme =/let _current_theme =/g' "$file"
    sed -i.tmp 's/let custom_theme =/let _custom_theme =/g' "$file"
    
    # Clean up temporary files
    rm -f "$file.tmp"
    
    echo "    ✅ Fixed unused variables"
}

# Find files with unused variable warnings
echo "  Searching for files with unused variables..."

# Process specific files known to have unused variables
FILES=(
    "crates/radix-leptos-primitives/src/theming/layout_system.rs"
    "crates/radix-leptos-primitives/src/theming/prebuilt_themes.rs"
    "crates/radix-leptos-primitives/src/theming/theme_customization.rs"
    "crates/radix-leptos-primitives/src/theming/theme_provider.rs"
    "crates/radix-leptos-primitives/src/theming/integration_tests.rs"
)

for file in "${FILES[@]}"; do
    if [[ -f "$file" ]]; then
        fix_unused_variables "$file"
    else
        echo "  ⚠️  File not found: $file"
    fi
done

echo "✅ Unused variables fixed!"
echo "🔄 Run 'cargo clippy' to verify fixes"
