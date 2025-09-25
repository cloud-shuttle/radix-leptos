#!/bin/bash
# Fix proptest array references
# This script addresses E0308 errors in proptest by adding proper references to arrays

echo "Fixing proptest array references..."

# Find and replace prop::sample::select([ with prop::sample::select(&[
find crates/ -name "*.rs" -exec sed -i '' 's/prop::sample::select(\[/prop::sample::select(\&[/g' {} \;

# Find and replace prop::sample::select(vec![ with prop::sample::select(&vec![
find crates/ -name "*.rs" -exec sed -i '' 's/prop::sample::select(vec!/prop::sample::select(\&vec!/g' {} \;

# Find and replace prop::sample::select(Vec::new() with prop::sample::select(&Vec::new()
find crates/ -name "*.rs" -exec sed -i '' 's/prop::sample::select(Vec::new()/prop::sample::select(\&Vec::new()/g' {} \;

echo "Proptest array references completed."
echo "Please review changes and test compilation with: cargo check"
