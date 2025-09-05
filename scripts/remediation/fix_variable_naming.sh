#!/bin/bash

# Fix variable naming inconsistencies
# This script addresses E0425 cannot find value errors

echo "🔧 Fixing variable naming inconsistencies..."

# Fix common variable naming issues
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_disabled/disabled/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_checked/checked/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_visible/visible/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_open/open/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_dark/dark/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_loading/loading/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_indeterminate/indeterminate/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_hour/hour/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_multi_select/multi_select/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_show_prev_next/show_prev_next/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_show_page_numbers/show_page_numbers/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_show_first_last/show_first_last/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_required/required/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_interactive/interactive/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_dismissible/dismissible/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_selected/selected/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_pulsing/pulsing/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_focused/focused/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_current/current/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_allow_multiple/allow_multiple/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/_show_zero/show_zero/g' {} \;

echo "✅ Variable naming inconsistencies fixed!"
