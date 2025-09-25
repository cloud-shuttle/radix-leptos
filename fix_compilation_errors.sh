#!/bin/bash

# Script to fix compilation errors from warning cleanup
# This fixes references to variables that were prefixed with underscores

echo "Fixing compilation errors from warning cleanup..."

# List of files that need fixing
FILES=(
    "crates/radix-leptos-primitives/src/components/collapsible.rs"
    "crates/radix-leptos-primitives/src/components/combobox.rs"
    "crates/radix-leptos-primitives/src/components/avatar.rs"
    "crates/radix-leptos-primitives/src/components/timeline.rs"
)

for file in "${FILES[@]}"; do
    echo "Fixing $file..."
    
    # Fix class references
    sed -i '' 's/class\.as_deref()\.unwrap_or("")/_class.as_deref().unwrap_or("")/g' "$file"
    sed -i '' 's/class\.as_deref()/_class.as_deref()/g' "$file"
    sed -i '' 's/class\.unwrap_or_default()/_class.unwrap_or_default()/g' "$file"
    
    # Fix style references
    sed -i '' 's/style=style/style=_style/g' "$file"
    sed -i '' 's/style\.unwrap_or_default()/_style.unwrap_or_default()/g' "$file"
    
    # Fix children references
    sed -i '' 's/children\.map(|c| c())/_children.map(|c| c())/g' "$file"
    sed -i '' 's/children\.unwrap_or_default()/_children.unwrap_or_default()/g' "$file"
    
    # Fix callback references
    sed -i '' 's/if let Some(callback) = on_load/if let Some(callback) = _on_load/g' "$file"
    sed -i '' 's/if let Some(callback) = on_error/if let Some(callback) = _on_error/g' "$file"
    sed -i '' 's/if let Some(callback) = on_click/if let Some(callback) = _on_click/g' "$file"
    sed -i '' 's/if let Some(callback) = on_change/if let Some(callback) = _on_change/g' "$file"
    sed -i '' 's/if let Some(callback) = on_select/if let Some(callback) = _on_select/g' "$file"
    
    # Fix other common patterns
    sed -i '' 's/value\.unwrap_or_default()/_value.unwrap_or_default()/g' "$file"
    sed -i '' 's/placeholder\.unwrap_or_default()/_placeholder.unwrap_or_default()/g' "$file"
    sed -i '' 's/disabled\.unwrap_or_default()/_disabled.unwrap_or_default()/g' "$file"
    sed -i '' 's/required\.unwrap_or_default()/_required.unwrap_or_default()/g' "$file"
    sed -i '' 's/options\.unwrap_or_default()/_options.unwrap_or_default()/g' "$file"
    sed -i '' 's/multiple\.unwrap_or_default()/_multiple.unwrap_or_default()/g' "$file"
    sed -i '' 's/searchable\.unwrap_or_default()/_searchable.unwrap_or_default()/g' "$file"
    sed -i '' 's/clearable\.unwrap_or_default()/_clearable.unwrap_or_default()/g' "$file"
    sed -i '' 's/visible\.unwrap_or_default()/_visible.unwrap_or_default()/g' "$file"
    sed -i '' 's/selected\.unwrap_or_default()/_selected.unwrap_or_default()/g' "$file"
    sed -i '' 's/active\.unwrap_or_default()/_active.unwrap_or_default()/g' "$file"
    sed -i '' 's/animated\.unwrap_or_default()/_animated.unwrap_or_default()/g' "$file"
    sed -i '' 's/enabled\.unwrap_or_default()/_enabled.unwrap_or_default()/g' "$file"
    sed -i '' 's/loading\.unwrap_or_default()/_loading.unwrap_or_default()/g' "$file"
    sed -i '' 's/error\.unwrap_or_default()/_error.unwrap_or_default()/g' "$file"
    sed -i '' 's/success\.unwrap_or_default()/_success.unwrap_or_default()/g' "$file"
    sed -i '' 's/warning\.unwrap_or_default()/_warning.unwrap_or_default()/g' "$file"
    sed -i '' 's/info\.unwrap_or_default()/_info.unwrap_or_default()/g' "$file"
    sed -i '' 's/primary\.unwrap_or_default()/_primary.unwrap_or_default()/g' "$file"
    sed -i '' 's/secondary\.unwrap_or_default()/_secondary.unwrap_or_default()/g' "$file"
    sed -i '' 's/tertiary\.unwrap_or_default()/_tertiary.unwrap_or_default()/g' "$file"
    sed -i '' 's/quaternary\.unwrap_or_default()/_quaternary.unwrap_or_default()/g' "$file"
    sed -i '' 's/quinary\.unwrap_or_default()/_quinary.unwrap_or_default()/g' "$file"
    sed -i '' 's/senary\.unwrap_or_default()/_senary.unwrap_or_default()/g' "$file"
    sed -i '' 's/septenary\.unwrap_or_default()/_septenary.unwrap_or_default()/g' "$file"
    sed -i '' 's/octonary\.unwrap_or_default()/_octonary.unwrap_or_default()/g' "$file"
    sed -i '' 's/nonary\.unwrap_or_default()/_nonary.unwrap_or_default()/g' "$file"
    sed -i '' 's/denary\.unwrap_or_default()/_denary.unwrap_or_default()/g' "$file"
done

echo "Done fixing compilation errors!"
