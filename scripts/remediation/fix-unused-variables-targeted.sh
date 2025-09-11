#!/bin/bash

# Fix Unused Variables Script (Targeted)
# This script fixes only the specific unused variables identified by clippy

set -e

echo "🔧 Fixing unused variables (targeted approach)..."

# Function to fix specific unused variables
fix_specific_unused_variables() {
    local file="$1"
    echo "  Processing: $file"
    
    # Create backup
    cp "$file" "$file.backup"
    
    # Only fix variables that are clearly unused in test functions
    # These are the ones clippy specifically identified as unused
    
    # In test functions, prefix variables that are only declared but never used
    sed -i.tmp 's/let callback = Callback::new(|_layout: LayoutSystem| {});/let _callback = Callback::new(|_layout: LayoutSystem| {});/g' "$file"
    sed -i.tmp 's/let spacing = SpacingSystem::default();/let _spacing = SpacingSystem::default();/g' "$file"
    sed -i.tmp 's/let callback = Callback::new(|_theme: String| {});/let _callback = Callback::new(|_theme: String| {});/g' "$file"
    sed -i.tmp 's/let themes = \[ThemeInfo::default()\];/let _themes = [ThemeInfo::default()];/g' "$file"
    sed -i.tmp 's/let theme = ThemeInfo::default();/let _theme = ThemeInfo::default();/g' "$file"
    sed -i.tmp 's/let colors = ThemeColors::default();/let _colors = ThemeColors::default();/g' "$file"
    sed -i.tmp 's/let on_change = Callback::new(|_: String| {});/let _on_change = Callback::new(|_: String| {});/g' "$file"
    sed -i.tmp 's/let custom_theme = CSSVariables::default();/let _custom_theme = CSSVariables::default();/g' "$file"
    sed -i.tmp 's/let current_theme = light_theme\.clone();/let _current_theme = light_theme.clone();/g' "$file"
    
    # Clean up temporary files
    rm -f "$file.tmp"
    
    echo "    ✅ Fixed specific unused variables"
}

# Process only the files that have unused variables in test functions
FILES=(
    "crates/radix-leptos-primitives/src/theming/layout_system.rs"
    "crates/radix-leptos-primitives/src/theming/prebuilt_themes.rs"
    "crates/radix-leptos-primitives/src/theming/theme_customization.rs"
    "crates/radix-leptos-primitives/src/theming/theme_provider.rs"
    "crates/radix-leptos-primitives/src/theming/integration_tests.rs"
)

for file in "${FILES[@]}"; do
    if [[ -f "$file" ]]; then
        fix_specific_unused_variables "$file"
    else
        echo "  ⚠️  File not found: $file"
    fi
done

echo "✅ Targeted unused variables fixed!"
echo "🔄 Run 'cargo clippy' to verify fixes"
