#!/bin/bash
# Script to fix unused variables in proptest macros

echo "🔧 Fixing unused variables in proptest macros..."

# Fix unused variables in proptest macros by prefixing with underscore
find crates/ -name "*.rs" -exec sed -i '' 's/proptest!(|(\([^)]*\))| {/proptest!(|(_\1)| {/g' {} \;

# Fix specific patterns that are common
find crates/ -name "*.rs" -exec sed -i '' 's/class in "\.\*"/_class in ".*"/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/style in "\.\*"/_style in ".*"/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/value in "\.\*"/_value in ".*"/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/format in "\.\*"/_format in ".*"/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/locale in "\.\*"/_locale in ".*"/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/key in "\.\*"/_key in ".*"/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/variant in "\.\*"/_variant in ".*"/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/orientation in "\.\*"/_orientation in ".*"/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/position in "\.\*"/_position in ".*"/g' {} \;

# Fix numeric ranges
find crates/ -name "*.rs" -exec sed -i '' 's/\([a-zA-Z_][a-zA-Z0-9_]*\) in \([0-9.]*\.\.[0-9.]*\)/_&/g' {} \;

# Fix boolean variables
find crates/ -name "*.rs" -exec sed -i '' 's/\([a-zA-Z_][a-zA-Z0-9_]*\): bool/_&/g' {} \;

# Fix count variables
find crates/ -name "*.rs" -exec sed -i '' 's/\([a-zA-Z_]*count\) in \([0-9.]*\.\.[0-9.]*\)/_&/g' {} \;

echo "✅ Proptest variables fixed!"
