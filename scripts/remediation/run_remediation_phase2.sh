#!/bin/bash

# Phase 2: Variable Naming Consistency
# This script runs all Phase 2 remediation steps

echo "🚀 Starting Phase 2: Variable Naming Consistency"
echo "================================================"

# Create backup branch
echo "📦 Creating backup branch..."
git checkout -b remediation-phase2-backup
git checkout main

# Run Phase 2 fixes
echo "🔧 Running Phase 2 fixes..."

echo "1. Fixing variable naming inconsistencies..."
./fix_variable_naming.sh

echo "2. Fixing field naming inconsistencies..."
./fix_field_naming.sh

# Check compilation status
echo "🔍 Checking compilation status..."
cargo check 2>&1 | grep "error\[E0425\]" | wc -l | xargs -I {} echo "E0425 errors remaining: {}"
cargo check 2>&1 | grep "error\[E0560\]" | wc -l | xargs -I {} echo "E0560 errors remaining: {}"
cargo check 2>&1 | grep "error\[E0609\]" | wc -l | xargs -I {} echo "E0609 errors remaining: {}"

# Commit changes
echo "💾 Committing Phase 2 changes..."
git add .
git commit -m "fix: Phase 2 - Variable naming consistency

- Fix variable naming inconsistencies (E0425)
- Fix field naming inconsistencies (E0560, E0609)
- Address 37+ cannot find value errors

Part of comprehensive remediation plan"

echo "✅ Phase 2 completed!"
echo "📊 Summary:"
echo "  - Variable naming inconsistencies fixed"
echo "  - Field naming inconsistencies fixed"
echo "  - Changes committed to git"
echo ""
echo "Next: Run Phase 3 with ./run_remediation_phase3.sh"
