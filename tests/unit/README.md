# Unit Tests

This directory contains unit tests for individual components and functions.

## Structure

```
unit/
├── components/          # Component-specific unit tests
│   ├── button/         # Button component tests
│   ├── input/          # Input component tests
│   └── ...
├── core/               # Core functionality tests
├── utils/              # Utility function tests
└── integration/        # Component integration tests
```

## Running Unit Tests

```bash
# Run all unit tests
cargo test --workspace --lib

# Run specific component tests
cargo test --package radix-leptos-primitives button

# Run with coverage
cargo tarpaulin --workspace --lib
```

## Test Guidelines

- **TDD Approach**: Write tests first, then implementation
- **Coverage**: Aim for 90%+ code coverage
- **Isolation**: Each test should be independent
- **Naming**: Use descriptive test names
- **Documentation**: Include test documentation

## Test Types

- **Component Creation**: Test component instantiation
- **Props Validation**: Test prop types and defaults
- **Event Handling**: Test event callbacks
- **State Management**: Test component state
- **Accessibility**: Test ARIA attributes and keyboard navigation
