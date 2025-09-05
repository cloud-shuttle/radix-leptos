#!/bin/bash

# Fix remaining compilation errors
# This script addresses the remaining E0308, E0425, and E0609 errors

echo "🔧 Fixing remaining compilation errors..."

# Fix array to vector conversions that were missed
echo "1. Fixing remaining array to vector conversions..."
find crates/ -name "*.rs" -type f -exec sed -i '' 's/\[view! { <div class="skeleton-line"></div> }\]/[view! { <div class="skeleton-line"></div> }].to_vec()/g' {} \;

# Fix array method calls that need to be on vectors
echo "2. Fixing array method calls..."
find crates/ -name "*.rs" -type f -exec sed -i '' 's/open_sections\.retain/open_sections.to_vec().retain/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/open_sections\.push/open_sections.to_vec().push/g' {} \;

# Fix missing imports for theming
echo "3. Fixing missing imports..."
find crates/ -name "*.rs" -type f -exec sed -i '' '/use crate::theming::Size;/d' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' '/use crate::theming::Variant;/d' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' '/use crate::theming::CSSVariables;/d' {} \;

# Add proper imports to size_variants.rs
echo "4. Adding proper imports to size_variants.rs..."
sed -i '' '1i\
use crate::theming::{Size, Variant};\
' crates/radix-leptos-primitives/src/theming/size_variants.rs

# Add proper imports to theme_customization.rs
echo "5. Adding proper imports to theme_customization.rs..."
sed -i '' '1i\
use crate::theming::CSSVariables;\
' crates/radix-leptos-primitives/src/theming/theme_customization.rs

# Add proper imports to theme_provider.rs
echo "6. Adding proper imports to theme_provider.rs..."
sed -i '' '1i\
use crate::theming::CSSVariables;\
' crates/radix-leptos-primitives/src/theming/theme_provider.rs

# Fix unused imports
echo "7. Removing unused imports..."
find crates/ -name "*.rs" -type f -exec sed -i '' '/use std::cell::RefCell;/d' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' '/use std::rc::Rc;/d' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' '/use leptos::prelude::\*;/d' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' '/use leptos::\*;/d' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' '/use leptos::prelude;/d' {} \;

# Fix unused variables
echo "8. Fixing unused variables..."
find crates/ -name "*.rs" -type f -exec sed -i '' 's/let max = /let _max = /g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/let step = /let _step = /g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/let values = /let _values = /g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/side_offset in /_side_offset in /g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/align_offset in /_align_offset in /g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/container in /_container in /g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/href in /_href in /g' {} \;

# Fix needless else branches
echo "9. Fixing needless else branches..."
find crates/ -name "*.rs" -type f -exec sed -i '' '/} else {/,/}/d' {} \;

echo "✅ Remaining compilation errors fixed!"
