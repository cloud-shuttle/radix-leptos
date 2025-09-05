# Development Hooks & Quality Assurance

This document describes the pre-commit hooks and development tools configured for the Radix-Leptos project to ensure code quality and consistency.

## 🎯 Overview

We use a comprehensive set of pre-commit hooks to automatically check code quality, formatting, and security before commits are allowed. This ensures:

- **Code Quality**: Consistent formatting and linting
- **Security**: Detection of potential secrets and vulnerabilities  
- **Testing**: All tests must pass before commits
- **Documentation**: Proper formatting of documentation files

## 🚀 Quick Setup

### Option 1: Automated Setup (Recommended)
```bash
# Run the setup script from project root
./scripts/setup-hooks.sh
```

### Option 2: Manual Setup
```bash
# Install pre-commit
pip install pre-commit

# Install hooks
pre-commit install

# Install Rust tools
rustup component add rustfmt clippy
cargo install cargo-audit cargo-outdated
```

## 🔧 Configured Hooks

### Rust-Specific Hooks

#### 1. **rustfmt** - Code Formatting
- **Purpose**: Ensures consistent code formatting
- **Configuration**: Uses `rustfmt.toml` settings
- **Runs on**: All `.rs` files
- **Action**: Automatically formats code

#### 2. **clippy** - Linting
- **Purpose**: Catches common mistakes and enforces best practices
- **Configuration**: Uses `clippy.toml` settings
- **Runs on**: All `.rs` files
- **Action**: Treats warnings as errors (`-D warnings`)

#### 3. **cargo check** - Compilation Check
- **Purpose**: Ensures code compiles without errors
- **Runs on**: All `.rs` files
- **Action**: Fails if compilation errors exist

#### 4. **cargo test** - Test Suite
- **Purpose**: Ensures all tests pass
- **Runs on**: All `.rs` files
- **Action**: Fails if any tests fail

### General File Hooks

#### 5. **trailing-whitespace** - Whitespace Cleanup
- **Purpose**: Removes trailing whitespace
- **Excludes**: Markdown files (`.md`)
- **Action**: Automatically fixes whitespace issues

#### 6. **end-of-file-fixer** - File Endings
- **Purpose**: Ensures files end with newline
- **Excludes**: Markdown files (`.md`)
- **Action**: Adds newline if missing

#### 7. **check-yaml** - YAML Validation
- **Purpose**: Validates YAML syntax
- **Runs on**: All `.yaml` and `.yml` files
- **Action**: Fails on invalid YAML

#### 8. **check-toml** - TOML Validation
- **Purpose**: Validates TOML syntax
- **Runs on**: All `.toml` files
- **Action**: Fails on invalid TOML

#### 9. **check-json** - JSON Validation
- **Purpose**: Validates JSON syntax
- **Runs on**: All `.json` files
- **Action**: Fails on invalid JSON

#### 10. **check-merge-conflict** - Conflict Detection
- **Purpose**: Detects unresolved merge conflicts
- **Action**: Fails if conflict markers found

#### 11. **check-added-large-files** - File Size Check
- **Purpose**: Prevents accidentally committing large files
- **Limit**: 1MB per file
- **Action**: Warns about large files

#### 12. **mixed-line-ending** - Line Ending Consistency
- **Purpose**: Ensures consistent line endings (LF)
- **Action**: Converts to LF if needed

### Documentation Hooks

#### 13. **prettier** - Code Formatting
- **Purpose**: Formats JavaScript, TypeScript, JSON, Markdown, HTML, CSS
- **Excludes**: `node_modules/`, `target/`, `pkg/`, `.git/`
- **Action**: Automatically formats files

### Security Hooks

#### 14. **detect-secrets** - Secret Detection
- **Purpose**: Detects potential secrets in code
- **Configuration**: Uses `.secrets.baseline`
- **Excludes**: `package.lock.json`
- **Action**: Fails if new secrets detected

### Custom Hooks

#### 15. **cargo-doc** - Documentation Check
- **Purpose**: Ensures documentation builds without errors
- **Action**: Fails if doc generation fails

#### 16. **wasm-build-check** - WASM Build Check
- **Purpose**: Ensures WASM builds work (manual trigger only)
- **Action**: Fails if WASM build fails

## 📋 Usage

### Running Hooks Manually

```bash
# Run all hooks on all files
pre-commit run --all-files

# Run specific hook
pre-commit run rustfmt

# Run hooks on staged files only
pre-commit run

# Update hook versions
pre-commit autoupdate
```

### Bypassing Hooks (Emergency Only)

```bash
# Skip all hooks for a commit
git commit --no-verify -m "Emergency fix"

# Skip specific hook
SKIP=clippy git commit -m "Commit message"
```

### Common Commands

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test

# Security audit
cargo audit

# Check outdated dependencies
cargo outdated
```

## ⚙️ Configuration Files

### `.pre-commit-config.yaml`
Main configuration file for pre-commit hooks. Defines which hooks to run and their settings.

### `clippy.toml`
Clippy configuration with allowed/denied lints and thresholds.

### `rustfmt.toml`
Rustfmt configuration for code formatting rules.

### `.secrets.baseline`
Baseline file for detect-secrets to track known false positives.

## 🚨 Troubleshooting

### Common Issues

#### 1. **Hook Installation Fails**
```bash
# Ensure pre-commit is installed
pip install pre-commit

# Reinstall hooks
pre-commit uninstall
pre-commit install
```

#### 2. **Rust Tools Not Found**
```bash
# Install Rust components
rustup component add rustfmt clippy

# Update Rust
rustup update
```

#### 3. **Tests Fail in Hooks**
```bash
# Run tests manually to debug
cargo test --workspace --lib

# Check specific test
cargo test test_name
```

#### 4. **Large Files Detected**
```bash
# Add to .gitignore
echo "large-file.bin" >> .gitignore

# Use Git LFS for large files
git lfs track "*.bin"
```

#### 5. **Secrets Detected**
```bash
# Update baseline if false positive
detect-secrets scan --update .secrets.baseline

# Remove actual secrets from code
# Use environment variables instead
```

### Performance Optimization

#### 1. **Slow Hook Execution**
```bash
# Run hooks in parallel
pre-commit run --all-files --parallel

# Skip expensive hooks during development
SKIP=wasm-build-check git commit -m "Quick fix"
```

#### 2. **Large Repository**
```bash
# Use .pre-commit-config.yaml with file filters
# Only run hooks on changed files
pre-commit run --files $(git diff --cached --name-only)
```

## 📊 Quality Metrics

The hooks help maintain:

- **Code Coverage**: Tests must pass
- **Security**: No secrets or vulnerabilities
- **Performance**: No large files or inefficient code
- **Consistency**: Uniform formatting and style
- **Documentation**: Properly formatted docs

## 🔄 Continuous Integration

These hooks complement our CI/CD pipeline:

- **GitHub Actions**: Runs same checks in CI
- **Code Review**: Hooks catch issues before review
- **Automated Testing**: Ensures all tests pass
- **Security Scanning**: Regular vulnerability checks

## 📚 Additional Resources

- [Pre-commit Documentation](https://pre-commit.com/)
- [Rust Clippy Lints](https://doc.rust-lang.org/clippy/)
- [Rustfmt Configuration](https://rust-lang.github.io/rustfmt/)
- [Detect Secrets](https://github.com/Yelp/detect-secrets)

## 🤝 Contributing

When contributing to Radix-Leptos:

1. **Run setup script**: `./scripts/setup-hooks.sh`
2. **Make changes**: Write code following our standards
3. **Commit**: Hooks will run automatically
4. **Fix issues**: Address any hook failures
5. **Submit PR**: All checks should pass

The hooks ensure consistent quality across all contributions! 🎉
