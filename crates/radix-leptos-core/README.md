# Radix-Leptos Core

Core utilities and hooks for Radix-Leptos components.

## Overview

This crate provides the foundational utilities, hooks, and shared functionality that power the Radix-Leptos component library. It includes:

- **Utility Functions**: Common helper functions for component development
- **Hooks**: Reusable state management and side effect hooks
- **Types**: Shared type definitions and enums
- **Constants**: Common constants and configuration values

## Features

- 🎯 **Focused**: Core utilities without component dependencies
- 🔧 **Utilities**: Helper functions for common operations
- 🪝 **Hooks**: Reusable Leptos hooks for state management
- 📦 **Lightweight**: Minimal dependencies for maximum compatibility

## Usage

```rust
use radix_leptos_core::*;

// Use utility functions
let merged_classes = merge_classes(vec!["btn", "btn-primary"]);

// Use hooks
let (count, set_count) = create_signal(cx, 0);
```

## Documentation

For complete documentation, visit [docs.rs/radix-leptos-core](https://docs.rs/radix-leptos-core).

## License

MIT License - see [LICENSE](../../LICENSE) for details.
