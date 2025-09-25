#!/bin/bash
# Fix syntax errors introduced by automated scripts

echo "Fixing syntax errors..."

# Fix vec!Vec::new() -> Vec::new()
find crates/ -name "*.rs" -exec sed -i '' 's/vec!Vec::new()/Vec::new()/g' {} \;

# Fix on:drop=move -> on:drop
find crates/ -name "*.rs" -exec sed -i '' 's/on:drop=move/on:drop/g' {} \;

# Fix callback.run(vec!Vec::new()) -> callback.run(Vec::new())
find crates/ -name "*.rs" -exec sed -i '' 's/callback\.run(vec!Vec::new())/callback.run(Vec::new())/g' {} \;

echo "Syntax errors fixed."
echo "Please review changes and test compilation with: cargo check"
