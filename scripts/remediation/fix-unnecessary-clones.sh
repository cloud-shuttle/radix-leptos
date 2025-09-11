#!/bin/bash

# Fix Unnecessary Clones Script
# This script removes unnecessary .clone() calls on Copy types

set -e

echo "🔧 Fixing unnecessary clones..."

# Function to fix unnecessary clones in a file
fix_unnecessary_clones() {
    local file="$1"
    echo "  Processing: $file"
    
    # Create backup
    cp "$file" "$file.backup"
    
    # Remove .clone() calls on Callback types (which implement Copy)
    sed -i.tmp 's/handle_basic_selection\.clone()/handle_basic_selection/g' "$file"
    sed -i.tmp 's/handle_item_click\.clone()/handle_item_click/g' "$file"
    sed -i.tmp 's/handle_item_focus\.clone()/handle_item_focus/g' "$file"
    sed -i.tmp 's/handle_multi_selection\.clone()/handle_multi_selection/g' "$file"
    sed -i.tmp 's/handle_large_dataset_selection\.clone()/handle_large_dataset_selection/g' "$file"
    sed -i.tmp 's/handle_virtualized_selection\.clone()/handle_virtualized_selection/g' "$file"
    sed -i.tmp 's/handle_basic_page_change\.clone()/handle_basic_page_change/g' "$file"
    sed -i.tmp 's/handle_compact_page_change\.clone()/handle_compact_page_change/g' "$file"
    sed -i.tmp 's/handle_detailed_page_change\.clone()/handle_detailed_page_change/g' "$file"
    sed -i.tmp 's/handle_custom_page_change\.clone()/handle_custom_page_change/g' "$file"
    sed -i.tmp 's/handle_large_dataset_page_change\.clone()/handle_large_dataset_page_change/g' "$file"
    
    # Clean up temporary files
    rm -f "$file.tmp"
    
    echo "    ✅ Fixed unnecessary clones"
}

# Files with unnecessary clone warnings
FILES=(
    "examples/src/list_examples.rs"
    "examples/src/pagination_examples.rs"
)

for file in "${FILES[@]}"; do
    if [[ -f "$file" ]]; then
        fix_unnecessary_clones "$file"
    else
        echo "  ⚠️  File not found: $file"
    fi
done

echo "✅ Unnecessary clones fixed!"
echo "🔄 Run 'cargo clippy' to verify fixes"
