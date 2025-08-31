# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Radix-Leptos is a Rust library providing accessible, unstyled UI primitives for Leptos applications. It's a port of Radix UI patterns to the Leptos ecosystem, focusing on accessibility-first component design with complete styling flexibility.

## Architecture

### Workspace Structure
- **`crates/radix-leptos`** - Main library crate that re-exports core and primitives
- **`crates/radix-leptos-core`** - Foundational utilities, hooks, and primitives  
- **`crates/radix-leptos-primitives`** - High-level UI components built on core
- **`examples/docs-site`** - Documentation website built with Trunk

### Core Architecture Layers

**Layer 1: Core (`radix-leptos-core`)**
- `hooks/` - Reusable behavioral hooks (use_controllable_state, use_focus_trap, etc.)
- `utils/` - Utility functions and shared logic
- `context/` - React-style context providers for state management
- `primitives/` - Low-level primitive components

**Layer 2: Primitives (`radix-leptos-primitives`)**
- `components/` - High-level components (Button, Label, Separator, etc.)
- Each component follows enum-based variant patterns (ButtonVariant, ButtonSize)

**Layer 3: Main Library (`radix-leptos`)**
- Re-exports both core and primitives for convenient single import

## Development Commands

### Building and Testing
```bash
# Build entire workspace
cargo build --all-features --workspace

# Run all tests
cargo test --all-features --workspace

# WASM-specific testing (from crate directory)
cd crates/radix-leptos-core
wasm-pack test --headless --firefox
wasm-pack test --headless --chrome

# Run all WASM tests across workspace
find crates -name "Cargo.toml" -execdir wasm-pack test --headless --firefox \;
```

### Code Quality
```bash
# Format code
cargo fmt --all

# Lint with Clippy
cargo clippy --all-features --workspace -- -D warnings

# Check without building
cargo check --all-features --workspace

# Generate documentation
cargo doc --all-features --workspace --no-deps
```

### Development Site
```bash
# Start documentation site with live reload
cd examples/docs-site
trunk serve
# Site runs at http://127.0.0.1:3000
```

### Security and Coverage
```bash
# Security audit
cargo deny check advisories

# Code coverage
cargo tarpaulin --all-features --workspace --timeout 120 --out Xml
```

## Component Development Patterns

### Component Structure
Components follow a consistent pattern:
1. **Enum Variants** - Define styling variants (e.g., `ButtonVariant::Default`, `ButtonSize::Large`)
2. **Props Struct** - Component properties with builder pattern
3. **Documentation** - Comprehensive rustdoc with examples
4. **Accessibility** - WAI-ARIA compliance built-in

### Hook Usage
Core hooks provide common accessibility behaviors:
- `use_controllable_state` - Controlled/uncontrolled state management
- `use_focus_trap` - Focus containment for modals/overlays  
- `use_escape_keydown` - Escape key handling
- `use_outside_click` - Click-outside detection
- `use_compose_refs` - Reference composition for flexibility

### Feature Flags
The workspace uses consistent feature flags:
- `ssr` - Server-side rendering support
- `hydrate` - Client-side hydration support

## Testing Strategy

Multi-layer testing approach:
- **Unit Tests** - Component logic and state management
- **WASM Tests** - Browser-specific functionality with wasm-pack
- **Integration Tests** - Component interactions and APIs  
- **Accessibility Tests** - WCAG 2.1 AA compliance
- **CI Pipeline** - Automated testing across Firefox and Chrome

## Current Development Status

The project is in **active development** (Foundation Phase):
- ✅ Core utilities and hooks implemented
- ✅ Basic primitives (Portal, Slot, VisuallyHidden)
- 🔄 First component implementations (Button, Label, Separator)
- ⏳ Advanced components and documentation site

Components marked as `[ ]` in README.md are not yet implemented.

## Key Dependencies

- **Leptos 0.6** - Core reactive framework with SSR support
- **leptos-use** - Additional utility hooks
- **web-sys/wasm-bindgen** - DOM interaction and WASM bindings
- **uuid** - Component ID generation
- **thiserror** - Structured error handling

## Important Notes

- All components must maintain accessibility-first design
- Styling is deliberately omitted - components provide structure and behavior only
- Components use data attributes for styling hooks rather than CSS classes
- Server-side rendering compatibility is required for all components
- Follow existing enum-based variant patterns for consistency