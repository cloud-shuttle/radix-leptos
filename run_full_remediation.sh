#!/bin/bash

# Full Remediation Plan Execution
# This script runs all phases of the remediation plan

echo "🚀 Starting Full Remediation Plan"
echo "================================="
echo "This will fix all compilation errors in the Radix-Leptos project"
echo ""

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "❌ Error: Not in a git repository"
    exit 1
fi

# Check if there are uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo "⚠️  Warning: You have uncommitted changes"
    echo "   Please commit or stash them before running remediation"
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Create main remediation branch
echo "📦 Creating remediation branch..."
git checkout -b remediation-$(date +%Y%m%d-%H%M%S)

# Run all phases
echo "🔧 Running all remediation phases..."
echo ""

echo "Phase 1: Critical Type System Fixes"
echo "-----------------------------------"
./run_remediation_phase1.sh

echo ""
echo "Phase 2: Variable Naming Consistency"
echo "------------------------------------"
./run_remediation_phase2.sh

echo ""
echo "Phase 3: Struct Definition Alignment"
echo "------------------------------------"
./run_remediation_phase3.sh

echo ""
echo "Phase 4: Final Cleanup"
echo "----------------------"
./run_remediation_phase4.sh

# Final status check
echo ""
echo "🔍 Final Status Check"
echo "===================="
echo "Checking for remaining compilation errors..."

ERROR_COUNT=$(cargo check 2>&1 | grep "error\[E" | wc -l)
echo "Remaining errors: $ERROR_COUNT"

if [ $ERROR_COUNT -eq 0 ]; then
    echo "🎉 SUCCESS: All compilation errors have been resolved!"
    echo ""
    echo "📊 Final Summary:"
    echo "  ✅ Phase 1: Critical type fixes completed"
    echo "  ✅ Phase 2: Variable naming consistency completed"
    echo "  ✅ Phase 3: Struct definition alignment completed"
    echo "  ✅ Phase 4: Final cleanup completed"
    echo ""
    echo "🚀 Ready to merge back to main branch!"
else
    echo "⚠️  WARNING: $ERROR_COUNT errors still remain"
    echo "   Please review the output above and address remaining issues"
fi

echo ""
echo "📝 Next Steps:"
echo "  1. Review the changes: git log --oneline -10"
echo "  2. Test the build: cargo build"
echo "  3. Run tests: cargo test"
echo "  4. Merge to main: git checkout main && git merge remediation-*"
echo ""
echo "📚 Documentation:"
echo "  - See REMEDIATION_PLAN.md for detailed information"
echo "  - All changes are committed with descriptive messages"
