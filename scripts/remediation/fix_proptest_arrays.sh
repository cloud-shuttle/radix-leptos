#!/bin/bash

# Fix proptest array references
# This script addresses E0277 trait bound errors in proptest

echo "🔧 Fixing proptest array references..."

# Fix prop::sample::select([ to prop::sample::select(&[
find crates/ -name "*.rs" -type f -exec sed -i '' 's/prop::sample::select(\[/prop::sample::select(\&[/g' {} \;

echo "✅ Proptest array references fixed!"
