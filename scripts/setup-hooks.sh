#!/bin/bash
# Setup script for development hooks and tools

set -e

echo "🚀 Setting up development environment for Radix-Leptos..."

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Not in a Rust project root. Please run from project root."
    exit 1
fi

# 1. Install pre-commit (if not already installed)
print_info "Installing pre-commit..."
if ! command -v pre-commit &> /dev/null; then
    if command -v pip3 &> /dev/null; then
        pip3 install pre-commit
    elif command -v pip &> /dev/null; then
        pip install pre-commit
    else
        print_error "pip not found. Please install Python and pip first."
        exit 1
    fi
    print_status "pre-commit installed"
else
    print_status "pre-commit already installed"
fi

# 2. Install pre-commit hooks
print_info "Installing pre-commit hooks..."
if [ -f ".pre-commit-config.yaml" ]; then
    pre-commit install
    pre-commit install --hook-type commit-msg
    print_status "Pre-commit hooks installed"
else
    print_warning ".pre-commit-config.yaml not found, skipping pre-commit setup"
fi

# 3. Install Rust tools
print_info "Installing Rust development tools..."

# Install rustfmt if not present
if ! cargo fmt --version &> /dev/null; then
    rustup component add rustfmt
    print_status "rustfmt installed"
else
    print_status "rustfmt already installed"
fi

# Install clippy if not present
if ! cargo clippy --version &> /dev/null; then
    rustup component add clippy
    print_status "clippy installed"
else
    print_status "clippy already installed"
fi

# Install cargo-audit for security checks
if ! cargo audit --version &> /dev/null; then
    cargo install cargo-audit
    print_status "cargo-audit installed"
else
    print_status "cargo-audit already installed"
fi

# Install cargo-outdated for dependency checks
if ! cargo outdated --version &> /dev/null; then
    cargo install cargo-outdated
    print_status "cargo-outdated installed"
else
    print_status "cargo-outdated already installed"
fi

# 4. Install Node.js tools (if package.json exists)
if [ -f "package.json" ]; then
    print_info "Installing Node.js development tools..."
    
    if command -v pnpm &> /dev/null; then
        pnpm install
        print_status "Node.js dependencies installed with pnpm"
    elif command -v npm &> /dev/null; then
        npm install
        print_status "Node.js dependencies installed with npm"
    else
        print_warning "Neither pnpm nor npm found. Please install Node.js tools."
    fi
fi

# 5. Create secrets baseline (if detect-secrets is available)
if command -v detect-secrets &> /dev/null; then
    print_info "Creating secrets baseline..."
    if [ ! -f ".secrets.baseline" ]; then
        detect-secrets scan --baseline .secrets.baseline
        print_status "Secrets baseline created"
    else
        print_status "Secrets baseline already exists"
    fi
fi

# 6. Run initial checks
print_info "Running initial checks..."

# Format code
print_info "Formatting code..."
cargo fmt --all
print_status "Code formatted"

# Run clippy
print_info "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings
print_status "Clippy checks passed"

# Run tests
print_info "Running tests..."
cargo test --workspace --lib --quiet
print_status "Tests passed"

print_status "Development environment setup complete! 🎉"
echo ""
echo "📋 Available commands:"
echo "  • pre-commit run --all-files    # Run all pre-commit hooks"
echo "  • cargo fmt                     # Format code"
echo "  • cargo clippy                  # Run linter"
echo "  • cargo test                    # Run tests"
echo "  • cargo audit                   # Security audit"
echo "  • cargo outdated                # Check for outdated dependencies"
echo ""
echo "🔧 Pre-commit hooks will now run automatically on every commit."
echo "   To skip hooks temporarily, use: git commit --no-verify"
