# 🐚 Nix Development Environment Setup

This project uses **Nix** to provide a reproducible development environment with Rust, Node.js, and pnpm. This ensures everyone working on the project has the exact same toolchain and dependencies.

## 🚀 Quick Start

### Prerequisites
- **Nix** installed on your system
- **Nix Flakes** enabled

### Installation

1. **Install Nix** (if not already installed):
   ```bash
   # macOS/Linux
   curl --proto '=https' --tlsv1.2 -sSf https://get.determinate.systems/nix | sh -s -- install
   
   # Or using the official installer
   sh <(curl -L https://nixos.org/nix/install) --daemon
   ```

2. **Enable Nix Flakes**:
   ```bash
   # Add to ~/.config/nix/nix.conf
   echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
   ```

3. **Enter the development environment**:
   ```bash
   nix develop
   ```

## 🛠️ Available Commands

### Nix Commands
```bash
# Enter development shell
nix develop

# Build the project
nix build

# Run the project
nix run

# Show available packages
nix flake show
```

### Make Commands
```bash
# Show all available commands
make help

# Start development environment
make dev

# Build WASM examples
make build

# Run tests
make test

# Start development server
make serve

# Check project status
make status
```

## 🔧 Development Workflow

### 1. **Start Development Environment**
```bash
# Option 1: Using Nix directly
nix develop

# Option 2: Using Make
make dev

# Option 3: Using direnv (if installed)
direnv allow
```

### 2. **Build and Test**
```bash
# Build WASM examples
make build

# Run all tests
make test

# Run specific component tests
make test-carousel
make test-tabs

# Check code quality
make lint
make check-format
```

### 3. **Development Server**
```bash
# Start server
make serve

# Or use the full dev environment
make dev
```

## 📦 What's Included

### Rust Toolchain
- **Rust** (latest stable)
- **Cargo** package manager
- **wasm-pack** for WebAssembly builds
- **rust-analyzer** for IDE support
- **cargo-watch** for development

### Node.js Environment
- **Node.js 20** (LTS)
- **pnpm** package manager
- **Playwright** for testing

### Development Tools
- **Python 3** for HTTP server
- **Git** for version control
- **Build tools** and utilities

## 🌟 Benefits of Nix

### ✅ **Reproducible Builds**
- Same environment on every machine
- No more "works on my machine" issues
- Exact dependency versions

### ✅ **Isolated Environment**
- Clean development environment
- No conflicts with system packages
- Easy to switch between projects

### ✅ **Team Consistency**
- Everyone uses the same toolchain
- Easy onboarding for new developers
- CI/CD parity with local development

### ✅ **Version Management**
- Automatic toolchain updates
- Easy rollbacks if needed
- No manual version management

## 🔍 Troubleshooting

### Common Issues

#### 1. **Nix not found**
```bash
# Add Nix to your PATH
source ~/.nix-profile/etc/profile.d/nix.sh
```

#### 2. **Flakes not enabled**
```bash
# Enable flakes in nix.conf
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
```

#### 3. **Permission issues**
```bash
# Restart your shell after Nix installation
exec $SHELL
```

#### 4. **Build failures**
```bash
# Clean and rebuild
make clean
make build

# Or use Nix
nix build --rebuild
```

### Getting Help

```bash
# Check Nix status
nix doctor

# Check flake status
nix flake check

# Show available commands
make help

# Check project status
make status
```

## 🚀 Alternative: Docker

If you prefer Docker over Nix:

```bash
# Start development environment with Docker
make docker-dev

# Or manually
docker run -it --rm \
  -v $(PWD):/workspace \
  -w /workspace \
  -p 8080:8080 \
  rust:latest \
  bash -c "cd examples && wasm-pack build --target web && python3 -m http.server 8080"
```

## 📚 Additional Resources

- [Nix Documentation](https://nixos.org/learn.html)
- [Nix Flakes Guide](https://nixos.wiki/wiki/Flakes)
- [Rust + Nix](https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html#using-nix)
- [wasm-pack Documentation](https://rustwasm.github.io/docs/wasm-pack/)

## 🎯 Next Steps

1. **Enter the development environment**: `nix develop`
2. **Build the project**: `make build`
3. **Start development**: `make dev`
4. **Run tests**: `make test`
5. **Explore components**: Visit `http://localhost:8080`

Happy coding! 🚀✨
