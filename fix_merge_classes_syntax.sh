#!/bin/bash

# Fix merge_classes syntax errors
echo "🔧 Fixing merge_classes syntax errors..."

# Fix the pattern: merge_classes([ ... } ]);
find crates/ -name "*.rs" -type f -exec sed -i '' 's/merge_classes(\[\s*$//g' {} \;
find crates/ -name "*.rs" -type f -exec sed -i '' 's/^\s*}\s*\]);/]);/g' {} \;

# Fix the pattern: merge_classes([ ... } };
find crates/ -name "*.rs" -type f -exec sed -i '' 's/^\s*}\s*};/]);/g' {} \;

# Fix incomplete merge_classes calls
find crates/ -name "*.rs" -type f -exec sed -i '' 's/merge_classes(\[\s*$/merge_classes([/g' {} \;

echo "✅ merge_classes syntax errors fixed!"
