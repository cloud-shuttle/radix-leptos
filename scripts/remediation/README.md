# Clippy Remediation Scripts

This directory contains automated scripts to fix clippy errors and warnings in the radix-leptos codebase.

## Quick Start

To fix all clippy issues automatically:

```bash
./scripts/remediation/run-all-fixes.sh
```

## Individual Scripts

### Critical Error Fixes

#### `fix-boolean-logic-errors.sh`
- **Purpose:** Fixes 18 critical boolean logic errors
- **Pattern:** Removes tautological assertions like `assert!(variable || !variable)`
- **Files:** All component test files
- **Impact:** Enables successful compilation

### High-Impact Warning Fixes

#### `fix-unused-variables.sh`
- **Purpose:** Fixes unused variable warnings
- **Pattern:** Prefixes unused variables with underscore
- **Files:** Theming system files
- **Impact:** Improves code cleanliness

#### `fix-unused-imports.sh`
- **Purpose:** Removes unused imports
- **Pattern:** Removes `use leptos::*;` and other unused imports
- **Files:** Example files
- **Impact:** Reduces compilation time

#### `fix-assertion-patterns.sh`
- **Purpose:** Fixes assertion patterns
- **Pattern:** Replaces `assert!(false)` with `panic!()` and removes `assert!(true)`
- **Files:** Component files
- **Impact:** Improves test reliability

### Performance Optimizations

#### `fix-unnecessary-clones.sh`
- **Purpose:** Removes unnecessary `.clone()` calls on Copy types
- **Pattern:** Removes `.clone()` from Callback types
- **Files:** Example files
- **Impact:** Improves performance

#### `fix-vec-to-array.sh`
- **Purpose:** Converts `vec![]` to arrays for static data
- **Pattern:** Replaces `vec![...]` with `[...]`
- **Files:** Test files
- **Impact:** Reduces memory allocation

## Usage

### Run All Fixes
```bash
./scripts/remediation/run-all-fixes.sh
```

### Run Individual Scripts
```bash
# Fix critical errors
./scripts/remediation/fix-boolean-logic-errors.sh

# Fix unused variables
./scripts/remediation/fix-unused-variables.sh

# Fix unused imports
./scripts/remediation/fix-unused-imports.sh

# Fix assertion patterns
./scripts/remediation/fix-assertion-patterns.sh

# Fix unnecessary clones
./scripts/remediation/fix-unnecessary-clones.sh

# Fix vec! to array
./scripts/remediation/fix-vec-to-array.sh
```

## Safety Features

### Backup Creation
- Each script creates a `.backup` file before making changes
- Original files can be restored if needed

### Git Integration
- Master script creates a backup branch
- Changes are isolated in feature branch
- Easy rollback if issues occur

### Verification
- Scripts include verification steps
- Master script runs clippy to verify fixes
- Clear success/failure reporting

## Manual Review Required

### Boolean Logic Errors
After running `fix-boolean-logic-errors.sh`, you need to:
1. Review commented assertions
2. Implement proper test logic
3. Replace placeholder assertions with meaningful tests

### Test Assertions
Some assertions may need manual review to ensure they test the correct behavior.

## Troubleshooting

### Script Fails
1. Check file permissions: `chmod +x scripts/remediation/*.sh`
2. Verify you're in project root directory
3. Check if files exist before running scripts

### Clippy Still Shows Errors
1. Run `cargo clippy --all-targets --all-features` to see remaining issues
2. Use `cargo clippy --fix` for auto-fixable issues
3. Fix remaining issues manually

### Restore Original Files
```bash
# Restore from backup
cp file.rs.backup file.rs

# Or restore from git
git checkout -- file.rs
```

## Best Practices

1. **Run from project root:** Always run scripts from the project root directory
2. **Review changes:** Always review changes before committing
3. **Test after fixes:** Run tests to ensure functionality is preserved
4. **Incremental approach:** Fix one phase at a time if needed
5. **Backup first:** Scripts create backups, but consider git backup too

## Integration with CI/CD

Add to your CI/CD pipeline:

```yaml
- name: Run Clippy
  run: cargo clippy --all-targets --all-features -- -D warnings

- name: Fix Clippy Issues (if needed)
  run: ./scripts/remediation/run-all-fixes.sh
```

## Contributing

When adding new scripts:
1. Follow the naming convention: `fix-<issue-type>.sh`
2. Include backup creation
3. Add verification steps
4. Update this README
5. Test thoroughly before committing