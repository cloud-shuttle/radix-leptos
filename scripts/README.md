# Scripts Directory

This directory contains various utility scripts for the radix-leptos project.

## Directory Structure

### `test-fixes/`
Contains scripts used to fix test compilation issues during development:

- `fix_component_variants_tests.sh` - Fixes embedded tests in component_variants.rs
- `fix_dark_mode_tests.sh` - Fixes embedded tests in dark_mode.rs  
- `fix_layout_system_tests.sh` - Fixes embedded tests in layout_system.rs
- `fix_prebuilt_themes_tests.sh` - Fixes embedded tests in prebuilt_themes.rs
- `fix_proptest.sh` - Fixes proptest macro issues throughout codebase
- `fix_size_variants_tests.sh` - Fixes embedded tests in size_variants.rs
- `fix_theme_customization_tests.sh` - Fixes embedded tests in theme_customization.rs
- `fix_theme_provider_tests.sh` - Fixes embedded tests in theme_provider.rs

### `maintenance/`
Contains scripts for ongoing project maintenance:

- `run-tests.sh` - Main test runner script for the project

### `bundle-size-monitor.js`
Monitors bundle size and provides alerts when size increases.

### `performance-benchmark.js`
Runs performance benchmarks on components.

### `prepare-release.sh`
Prepares the project for release by running checks and building.

### `tdd-workflow.sh`
Implements TDD workflow automation.

## Usage

Most scripts are designed to be run from the project root directory. Check individual script files for specific usage instructions.

## Contributing

When adding new scripts:
1. Place them in the appropriate subdirectory
2. Make them executable (`chmod +x script_name.sh`)
3. Update this README with a description
4. Add usage instructions in the script header
