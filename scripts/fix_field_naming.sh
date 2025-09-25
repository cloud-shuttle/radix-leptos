#!/bin/bash
# Fix field naming inconsistencies
# This script addresses E0609 field access errors by standardizing field naming

echo "Fixing field naming inconsistencies..."

# Fix double underscore prefix issues
find crates/ -name "*.rs" -exec sed -i '' 's/__disabled/disabled/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/__checked/checked/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/__expanded/expanded/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/__selected/selected/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/__open/open/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/__visible/visible/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/__dark/dark/g' {} \;

echo "Field naming inconsistencies fixed."
echo "Please review changes and test compilation with: cargo check"
