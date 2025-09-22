# Agent Development Guide for Radix-Leptos

## Commands
- **Build**: `make build` (builds WASM examples), `cargo build` (builds workspace)
- **Test all**: `make test` (Playwright), `cargo test --workspace` (unit/integration)
- **Test single**: `pnpm exec playwright test -g "ComponentName"` or `cargo test test_name`
- **Lint**: `make lint` (`cargo clippy -- -D warnings`), `make format` (`cargo fmt`)
- **Dev server**: `make dev` (builds WASM + serves on :8080), `make serve` (server only)
- **TDD workflow**: `make test-watch` (cargo watch), `make tdd-complete` (full TDD suite)

## Architecture
- **Workspace structure**: `crates/{radix-leptos-core, radix-leptos-primitives, radix-leptos}`
- **Core**: Low-level hooks, primitives (Portal, Slot, VisuallyHidden), utilities
- **Primitives**: 57+ UI components with theming system (button, dialog, tabs, etc.)
- **Main crate**: Re-exports all functionality with feature flags (`core`, `full`)
- **Examples**: WASM demos in `/examples`, tested with Playwright
- **Testing**: Unit tests (cargo), E2E tests (Playwright), property-based (proptest), mutation testing

## Code Style
- **Imports**: Use `leptos::prelude::*`, `leptos::callback::Callback`, `leptos::children::Children`
- **Components**: Props structs with `#[component]` macro, `IntoView` return type
- **Formatting**: 100 char line width, 4 spaces, Unix newlines (see rustfmt.toml)
- **Clippy**: Max 30 cognitive complexity, 8 args, 200 lines per function
- **Error handling**: Use `thiserror` for error types, avoid unwrap() in components
- **Naming**: PascalCase components, snake_case functions/props, kebab-case CSS classes
- **Features**: Use feature flags for optional functionality, maintain backward compatibility
