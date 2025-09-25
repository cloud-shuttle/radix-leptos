#!/bin/bash
# Fix array to vector conversions
# This script addresses E0308 mismatched types errors by converting arrays to vectors

echo "Fixing array to vector conversions..."

# Find and replace empty arrays with Vec::new()
find crates/ -name "*.rs" -exec sed -i '' 's/\[\]/Vec::new()/g' {} \;

# Find and replace Some([]) with Some(Vec::new())
find crates/ -name "*.rs" -exec sed -i '' 's/Some(\[\])/Some(Vec::new())/g' {} \;

# Find and replace None with Some(Vec::new()) where appropriate for Vec<T> fields
# This is more complex and may need manual review
find crates/ -name "*.rs" -exec sed -i '' 's/None::<Vec<[^>]*>>/Some(Vec::new())/g' {} \;

echo "Array to vector conversions completed."
echo "Please review changes and test compilation with: cargo check"
