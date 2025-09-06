#!/bin/bash

# Bundle Size Monitoring Script
echo "📦 Monitoring bundle size..."

# Build with optimizations
cd examples
wasm-pack build --target web --release -- --features wee_alloc

# Check bundle size
node ../scripts/bundle-size-monitor.js

# Alert if size exceeds target
BUNDLE_SIZE=$(cat ../bundle-size-report.json | jq '.totalSize')
TARGET_SIZE=409600  # 400KB in bytes

if [ $BUNDLE_SIZE -gt $TARGET_SIZE ]; then
    echo "⚠️  Bundle size exceeds target!"
    echo "Current: $((BUNDLE_SIZE / 1024))KB"
    echo "Target: $((TARGET_SIZE / 1024))KB"
    echo "Excess: $(((BUNDLE_SIZE - TARGET_SIZE) / 1024))KB"
    exit 1
else
    echo "✅ Bundle size within target!"
    echo "Current: $((BUNDLE_SIZE / 1024))KB"
    echo "Target: $((TARGET_SIZE / 1024))KB"
    echo "Under by: $(((TARGET_SIZE - BUNDLE_SIZE) / 1024))KB"
fi