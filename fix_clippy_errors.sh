#!/bin/bash
# Script to fix common clippy errors automatically

echo "🔧 Fixing common clippy errors..."

# Fix assert!(true) issues
find crates/ -name "*.rs" -exec sed -i '' 's/assert!(true);.*//g' {} \;

# Fix boolean comparisons
find crates/ -name "*.rs" -exec sed -i '' 's/== true//g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/== false/! /g' {} \;

# Fix useless vec! usage
find crates/ -name "*.rs" -exec sed -i '' 's/vec!\[/[/g' {} \;

# Fix manual range contains
find crates/ -name "*.rs" -exec sed -i '' 's/hour < 1 || hour > 12/!(1..=12).contains(\&hour)/g' {} \;

# Fix overly complex boolean expressions
find crates/ -name "*.rs" -exec sed -i '' 's/validation\.is_valid || !validation\.is_valid/true/g' {} \;

# Fix module inception by adding allow attribute
find crates/ -name "*.rs" -exec sed -i '' 's/mod integration_tests {/#[allow(clippy::module_inception)]\nmod integration_tests {/g' {} \;
find crates/ -name "*.rs" -exec sed -i '' 's/mod simple_tests {/#[allow(clippy::module_inception)]\nmod simple_tests {/g' {} \;

echo "✅ Common clippy errors fixed!"
