# Integration Tests

This directory contains integration tests that test component interactions and workflows.

## Structure

```
integration/
├── component-interactions/  # Tests for component combinations
├── user-workflows/         # End-to-end user scenarios
├── form-integration/       # Form component integration
├── navigation/             # Navigation component tests
└── data-flow/              # Data binding and state tests
```

## Running Integration Tests

```bash
# Run all integration tests
cargo test --workspace --test integration

# Run specific integration test
cargo test --package radix-leptos-primitives form_integration

# Run with verbose output
cargo test --workspace --test integration -- --nocapture
```

## Test Categories

### Component Interactions
- **Form Components**: Input, Select, Checkbox interactions
- **Navigation**: Menu, Tabs, Pagination workflows
- **Data Display**: Table, List, Card combinations
- **Layout**: Grid, Flex, Container interactions

### User Workflows
- **Registration Flow**: Complete user registration
- **Data Entry**: Multi-step form completion
- **Search & Filter**: Search with filtering
- **CRUD Operations**: Create, Read, Update, Delete

### Performance Integration
- **Large Datasets**: Performance with many items
- **Real-time Updates**: Live data updates
- **Memory Management**: Memory usage patterns
- **Rendering Performance**: Component render times

## Test Guidelines

- **Realistic Scenarios**: Test real-world usage patterns
- **Error Handling**: Test error states and recovery
- **Accessibility**: Ensure keyboard navigation works
- **Performance**: Monitor performance metrics
- **Cross-browser**: Test in multiple browsers
