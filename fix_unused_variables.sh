#!/bin/bash

echo "Fixing unused variables..."

# Find files with unused 'class' variables and fix them
echo "Fixing unused 'class' variables..."
for file in $(find crates/ -name "*.rs" -exec grep -l "unused variable.*class" {} \; 2>/dev/null); do
    if [ -f "$file" ]; then
        echo "Processing $file for unused 'class' variables..."
        # Fix function parameters
        sed -i '' 's/class: Option<String>,/_class: Option<String>,/g' "$file"
        sed -i '' 's/class: Option<&str>,/_class: Option<&str>,/g' "$file"
        sed -i '' 's/class: Option<String>/_class: Option<String>/g' "$file"
        # Fix local variables
        sed -i '' 's/let class =/let _class =/g' "$file"
        sed -i '' 's/let mut class =/let mut _class =/g' "$file"
    fi
done

# Find files with unused 'children' variables and fix them
echo "Fixing unused 'children' variables..."
for file in $(find crates/ -name "*.rs" -exec grep -l "unused variable.*children" {} \; 2>/dev/null); do
    if [ -f "$file" ]; then
        echo "Processing $file for unused 'children' variables..."
        # Fix function parameters
        sed -i '' 's/children: Children,/_children: Children,/g' "$file"
        sed -i '' 's/children: Children)/_children: Children)/g' "$file"
    fi
done

# Find files with unused 'value' variables and fix them
echo "Fixing unused 'value' variables..."
for file in $(find crates/ -name "*.rs" -exec grep -l "unused variable.*value" {} \; 2>/dev/null); do
    if [ -f "$file" ]; then
        echo "Processing $file for unused 'value' variables..."
        # Fix function parameters
        sed -i '' 's/value: Option<String>,/_value: Option<String>,/g' "$file"
        sed -i '' 's/value: Option<Vec<String>>,/_value: Option<Vec<String>>,/g' "$file"
        sed -i '' 's/value: Option<bool>,/_value: Option<bool>,/g' "$file"
        sed -i '' 's/value: Option<f64>,/_value: Option<f64>,/g' "$file"
        sed -i '' 's/value: Option<i32>,/_value: Option<i32>,/g' "$file"
    fi
done

# Find files with unused 'on_change' variables and fix them
echo "Fixing unused 'on_change' variables..."
for file in $(find crates/ -name "*.rs" -exec grep -l "unused variable.*on_change" {} \; 2>/dev/null); do
    if [ -f "$file" ]; then
        echo "Processing $file for unused 'on_change' variables..."
        # Fix function parameters
        sed -i '' 's/on_change: Option<Callback/_on_change: Option<Callback/g' "$file"
    fi
done

# Find files with unused 'disabled' variables and fix them
echo "Fixing unused 'disabled' variables..."
for file in $(find crates/ -name "*.rs" -exec grep -l "unused variable.*disabled" {} \; 2>/dev/null); do
    if [ -f "$file" ]; then
        echo "Processing $file for unused 'disabled' variables..."
        # Fix function parameters
        sed -i '' 's/disabled: bool,/_disabled: bool,/g' "$file"
        sed -i '' 's/disabled: bool)/_disabled: bool)/g' "$file"
    fi
done

# Find files with unused 'style' variables and fix them
echo "Fixing unused 'style' variables..."
for file in $(find crates/ -name "*.rs" -exec grep -l "unused variable.*style" {} \; 2>/dev/null); do
    if [ -f "$file" ]; then
        echo "Processing $file for unused 'style' variables..."
        # Fix function parameters
        sed -i '' 's/style: Option<String>,/_style: Option<String>,/g' "$file"
        sed -i '' 's/style: Option<String>/_style: Option<String>/g' "$file"
    fi
done

echo "Done fixing unused variables!"
