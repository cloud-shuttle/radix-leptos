# 🛠️ Radix-Leptos Scripts

This directory contains all the automation scripts for the Radix-Leptos project, organized by purpose and functionality.

## 📁 Directory Structure

```
scripts/
├── README.md                    # This file
├── remediation/                 # Remediation and error fixing scripts
├── maintenance/                 # Build, deployment, and maintenance scripts
├── testing/                     # Test automation and validation scripts
├── bundle-size-monitor.js       # Bundle size monitoring
├── performance-benchmark.js     # Performance benchmarking
├── prepare-release.sh          # Release preparation
├── release.sh                  # Automated release process
├── setup-hooks.sh              # Git hooks setup
└── tdd-workflow.sh             # Test-driven development workflow
```

## 🔧 Script Categories

### 🚨 Remediation Scripts (`remediation/`)

These scripts were created to systematically fix the 400+ compilation errors in the codebase:

#### Core Fix Scripts
- `fix_array_to_vec.sh` - Converts array literals to vectors
- `fix_proptest_arrays.sh` - Fixes proptest array references
- `fix_variable_naming.sh` - Standardizes variable naming
- `fix_field_naming.sh` - Standardizes struct field naming
- `fix_remaining_errors.sh` - Addresses remaining compilation issues
- `fix_merge_classes_syntax.sh` - Fixes merge_classes syntax errors

#### Phase Execution Scripts
- `run_remediation_phase1.sh` - Phase 1: Critical type fixes
- `run_remediation_phase2.sh` - Phase 2: Variable naming consistency
- `run_remediation_phase3.sh` - Phase 3: Struct definition alignment
- `run_remediation_phase4.sh` - Phase 4: Final cleanup and validation
- `run_full_remediation.sh` - Master script for complete remediation

#### Usage
```bash
# Run individual fixes
./scripts/remediation/fix_array_to_vec.sh

# Run specific phase
./scripts/remediation/run_remediation_phase1.sh

# Run complete remediation
./scripts/remediation/run_full_remediation.sh
```

### 🔨 Maintenance Scripts (`maintenance/`)

Scripts for building, deploying, and maintaining the project:

- `build.sh` - Standard build process
- `build-production.sh` - Production build with optimizations
- `deploy-production.sh` - Production deployment
- `run-tests.sh` - Test execution

#### Usage
```bash
# Build the project
./scripts/maintenance/build.sh

# Build for production
./scripts/maintenance/build-production.sh

# Deploy to production
./scripts/maintenance/deploy-production.sh
```

### 🧪 Testing Scripts (`testing/`)

Scripts for test automation and validation:

- Test execution scripts
- Test result analysis
- Test coverage reporting

### 📊 Monitoring Scripts

- `bundle-size-monitor.js` - Monitors bundle size changes
- `performance-benchmark.js` - Performance benchmarking

### 🚀 Release Scripts

- `prepare-release.sh` - Prepares release artifacts
- `release.sh` - Automated release process

### ⚙️ Setup Scripts

- `setup-hooks.sh` - Sets up Git hooks for development
- `tdd-workflow.sh` - Test-driven development workflow

## 🎯 Usage Guidelines

### Running Scripts

1. **Make scripts executable** (if not already):
   ```bash
   chmod +x scripts/**/*.sh
   ```

2. **Run from project root**:
   ```bash
   ./scripts/category/script-name.sh
   ```

3. **Check script help** (if available):
   ```bash
   ./scripts/category/script-name.sh --help
   ```

### Script Safety

- All scripts include error handling and safety checks
- Scripts create backup branches before making changes
- Scripts provide detailed output and progress information
- Scripts can be run multiple times safely (idempotent)

### Development Workflow

1. **Before making changes**: Run relevant maintenance scripts
2. **During development**: Use testing scripts for validation
3. **After development**: Use remediation scripts if issues arise
4. **For releases**: Use release scripts for automation

## 📚 Documentation

- **Remediation**: See `docs/remediation/` for detailed remediation documentation
- **Releases**: See `docs/releases/` for release notes and documentation
- **Development**: See `docs/developer-guide/` for development guidelines

## 🤝 Contributing

When adding new scripts:

1. **Place in appropriate directory** based on purpose
2. **Add to this README** with description and usage
3. **Include error handling** and safety checks
4. **Make executable** with `chmod +x`
5. **Test thoroughly** before committing

## 🎉 Success Story

These scripts were instrumental in transforming the Radix-Leptos codebase from 400+ compilation errors to a fully functional, production-ready component library. They represent a systematic approach to code quality and maintenance that can be applied to other projects.

---

*For more information, see the main project documentation in the `docs/` directory.*