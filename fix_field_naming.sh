#!/bin/bash

# Fix field naming inconsistencies
# This script addresses E0560 and E0609 struct field errors

echo "🔧 Fixing field naming inconsistencies..."

# Fix double underscore field names to single underscore
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__disabled/disabled/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__checked/checked/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__expanded/expanded/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__selected/selected/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__is_valid/is_valid/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__is_complete/is_complete/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__require_uppercase/require_uppercase/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__require_lowercase/require_lowercase/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__require_numbers/require_numbers/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__require_symbols/require_symbols/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__separator/separator/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__multi_select/multi_select/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__focused/focused/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__current/current/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__show_prev_next/show_prev_next/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__show_page_numbers/show_page_numbers/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/__show_first_last/show_first_last/g' {} \;

echo "✅ Field naming inconsistencies fixed!"
