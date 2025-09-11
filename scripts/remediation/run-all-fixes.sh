#!/bin/bash

# Master Clippy Remediation Script
# This script runs all remediation scripts in the correct order

set -e

echo "🚀 Starting comprehensive clippy remediation process..."
echo "=================================================="

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]]; then
    echo "❌ Error: Please run this script from the project root directory"
    exit 1
fi

# Create backup branch
echo "📦 Creating backup branch..."
git checkout -b clippy-remediation-$(date +%Y%m%d-%H%M%S) || echo "Branch may already exist"

# Phase 1: Critical errors
echo ""
echo "🔥 Phase 1: Fixing critical errors..."
echo "====================================="
./scripts/remediation/fix-boolean-logic-errors.sh

# Phase 2: High-impact warnings
echo ""
echo "⚡ Phase 2: Fixing high-impact warnings..."
echo "=========================================="
./scripts/remediation/fix-unused-variables.sh
./scripts/remediation/fix-unused-imports.sh
./scripts/remediation/fix-assertion-patterns.sh

# Phase 3: Performance optimizations
echo ""
echo "🚀 Phase 3: Performance optimizations..."
echo "========================================"
./scripts/remediation/fix-unnecessary-clones.sh
./scripts/remediation/fix-vec-to-array.sh

# Verify fixes
echo ""
echo "🔍 Verifying fixes..."
echo "===================="
echo "Running clippy to check remaining issues..."

if cargo clippy --all-targets --all-features; then
    echo ""
    echo "✅ SUCCESS: All clippy errors and warnings resolved!"
    echo ""
    echo "📊 Summary:"
    echo "  - Critical errors: Fixed"
    echo "  - High-impact warnings: Fixed"
    echo "  - Performance optimizations: Applied"
    echo ""
    echo "🎉 Clippy remediation complete!"
    echo ""
    echo "📝 Next steps:"
    echo "  1. Review the changes: git diff"
    echo "  2. Run tests: cargo test"
    echo "  3. Commit changes: git add . && git commit -m 'Fix clippy errors and warnings'"
    echo "  4. Merge to main branch when ready"
else
    echo ""
    echo "⚠️  WARNING: Some clippy issues remain"
    echo "Please review the output above and fix remaining issues manually"
    echo ""
    echo "📝 You can:"
    echo "  1. Review remaining issues: cargo clippy --all-targets --all-features"
    echo "  2. Use cargo clippy --fix for auto-fixable issues"
    echo "  3. Fix remaining issues manually"
fi

echo ""
echo "🏁 Remediation process complete!"
