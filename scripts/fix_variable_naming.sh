#!/bin/bash
# Fix variable naming inconsistencies
# This script addresses E0425 cannot find value errors by fixing variable naming

echo "Fixing variable naming inconsistencies..."

# Fix underscore prefix issues
find crates/ -name "*.rs" -exec sed -i '' 's/_disabled/disabled/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/_checked/checked/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/_visible/visible/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/_open/open/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/_dark/dark/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/_indeterminate/indeterminate/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/_hour/hour/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/_expanded/expanded/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/_selected/selected/g' {} \;

echo "Variable naming inconsistencies fixed."
echo "Please review changes and test compilation with: cargo check"
