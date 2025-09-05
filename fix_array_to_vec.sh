#!/bin/bash

# Fix array to vector conversions
# This script addresses E0308 mismatched types errors

echo "🔧 Fixing array to vector conversions..."

# Fix empty arrays to Vec::new()
find crates/ -name "*.rs" -type f -exec sed -i '' 's/\[\]/Vec::new()/g' {} \;

# Fix Some([]) to Some(Vec::new())
find crates/ -name "*.rs" -type f -exec sed -i '' 's/Some(\[\])/Some(Vec::new())/g' {} \;

# Fix specific array patterns that need .to_vec()
find crates/ -name "*.rs" -type f -exec sed -i '' 's/errors: \[\],/errors: Vec::new(),/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/warnings: \[\],/warnings: Vec::new(),/g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/children: Some(\[\])/children: Some(Vec::new())/g' {} \;

echo "✅ Array to vector conversions fixed!"
