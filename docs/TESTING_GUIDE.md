# Testing Guide for Radix-Leptos

This comprehensive guide covers all testing strategies and tools available for Radix-Leptos components.

## Table of Contents

- [Getting Started](#getting-started)
- [Unit Testing](#unit-testing)
- [Integration Testing](#integration-testing)
- [Property-Based Testing](#property-based-testing)
- [Mutation Testing](#mutation-testing)
- [Performance Benchmarking](#performance-benchmarking)
- [Accessibility Testing](#accessibility-testing)
- [Test Automation](#test-automation)
- [Best Practices](#best-practices)

## Getting Started

### Installation

Add testing dependencies to your `Cargo.toml`:

```toml
[dev-dependencies]
proptest = "1.0"
wasm-bindgen-test = "0.3"
leptos = "0.6"
radix-leptos-primitives = "0.9.0"
```

### Basic Test Setup

```rust
use leptos::prelude::*;
use radix_leptos_primitives::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_basic_component() {
    let runtime = create_runtime();
    let view = view! {
        <Button>"Test Button"</Button>
    };
    
    assert!(view.into_any().is_some());
    runtime.dispose();
}
```

## Unit Testing

### Component Testing

```rust
#[wasm_bindgen_test]
fn test_button_component() {
    let runtime = create_runtime();
    
    // Test basic rendering
    let view = view! {
        <Button>"Click me"</Button>
    };
    assert!(view.into_any().is_some());
    
    // Test with props
    let view = view! {
        <Button variant=ButtonVariant::Destructive size=ButtonSize::Large>
            "Delete"
        </Button>
    };
    assert!(view.into_any().is_some());
    
    runtime.dispose();
}

#[wasm_bindgen_test]
fn test_input_component() {
    let runtime = create_runtime();
    let (value, set_value) = create_signal("".to_string());
    
    let view = view! {
        <Input
            value=value
            on_change=set_value
            placeholder="Enter text"
        />
    };
    
    assert!(view.into_any().is_some());
    assert_eq!(value(), "");
    
    set_value("test".to_string());
    assert_eq!(value(), "test");
    
    runtime.dispose();
}
```

### State Testing

```rust
#[wasm_bindgen_test]
fn test_signal_behavior() {
    let runtime = create_runtime();
    let (value, set_value) = create_signal(0);
    
    // Test initial value
    assert_eq!(value(), 0);
    
    // Test direct update
    set_value(42);
    assert_eq!(value(), 42);
    
    // Test update closure
    set_value.update(|v| *v += 1);
    assert_eq!(value(), 43);
    
    runtime.dispose();
}

#[wasm_bindgen_test]
fn test_derived_signals() {
    let runtime = create_runtime();
    let (count, set_count) = create_signal(0);
    let doubled = move || count() * 2;
    let quadrupled = move || doubled() * 2;
    
    assert_eq!(doubled(), 0);
    assert_eq!(quadrupled(), 0);
    
    set_count(5);
    assert_eq!(doubled(), 10);
    assert_eq!(quadrupled(), 20);
    
    runtime.dispose();
}
```

### Form Validation Testing

```rust
#[wasm_bindgen_test]
fn test_email_validation() {
    // Valid emails
    assert!(crate::components::form_validation::validation::is_valid_email("test@example.com"));
    assert!(crate::components::form_validation::validation::is_valid_email("user.name@domain.co.uk"));
    
    // Invalid emails
    assert!(!crate::components::form_validation::validation::is_valid_email("invalid-email"));
    assert!(!crate::components::form_validation::validation::is_valid_email("test@"));
    assert!(!crate::components::form_validation::validation::is_valid_email("@example.com"));
}

#[wasm_bindgen_test]
fn test_password_validation() {
    // Valid passwords
    assert!(crate::components::form_validation::validation::is_valid_password("password123"));
    assert!(crate::components::form_validation::validation::is_valid_password("SecurePass123!"));
    
    // Invalid passwords
    assert!(!crate::components::form_validation::validation::is_valid_password("short"));
    assert!(!crate::components::form_validation::validation::is_valid_password(""));
}
```

## Integration Testing

### Component Integration

```rust
#[wasm_bindgen_test]
fn test_form_integration() {
    let runtime = create_runtime();
    let (name, set_name) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (errors, set_errors) = create_signal(HashMap::new());
    
    let handle_submit = move |_| {
        let mut new_errors = HashMap::new();
        
        if name().is_empty() {
            new_errors.insert("name".to_string(), "Name is required".to_string());
        }
        
        if email().is_empty() {
            new_errors.insert("email".to_string(), "Email is required".to_string());
        }
        
        set_errors(new_errors);
    };
    
    let view = view! {
        <form on:submit=handle_submit>
            <FormField>
                <FormLabel for="name">"Name"</FormLabel>
                <Input
                    id="name"
                    value=name
                    on_change=set_name
                />
            </FormField>
            <FormField>
                <FormLabel for="email">"Email"</FormLabel>
                <Input
                    id="email"
                    type="email"
                    value=email
                    on_change=set_email
                />
            </FormField>
            <Button type="submit">"Submit"</Button>
        </form>
    };
    
    assert!(view.into_any().is_some());
    runtime.dispose();
}
```

### State Management Integration

```rust
#[wasm_bindgen_test]
fn test_state_management_integration() {
    let runtime = create_runtime();
    let (todos, set_todos) = create_signal(vec![
        "Learn Radix-Leptos".to_string(),
        "Build amazing apps".to_string(),
    ]);
    
    // Test initial state
    assert_eq!(todos().len(), 2);
    
    // Test adding item
    set_todos.update(|todos| todos.push("Test components".to_string()));
    assert_eq!(todos().len(), 3);
    assert_eq!(todos().last().unwrap(), "Test components");
    
    // Test removing item
    set_todos.update(|todos| todos.retain(|todo| todo != "Learn Radix-Leptos"));
    assert_eq!(todos().len(), 2);
    assert!(!todos().contains(&"Learn Radix-Leptos".to_string()));
    
    runtime.dispose();
}
```

## Property-Based Testing

### Component Property Testing

```rust
use proptest::prelude::*;
use radix_leptos_primitives::testing::*;

#[test]
fn test_button_properties() {
    let result = ComponentPropertyTester::test_button_properties();
    assert!(result.is_ok());
}

#[test]
fn test_input_properties() {
    let result = ComponentPropertyTester::test_input_properties();
    assert!(result.is_ok());
}

#[test]
fn test_form_validation_properties() {
    let result = ComponentPropertyTester::test_form_validation_properties();
    assert!(result.is_ok());
}
```

### State Property Testing

```rust
#[test]
fn test_state_invariants() {
    let result = StateInvariantTester::test_component_state_invariants();
    assert!(result.is_ok());
}

#[test]
fn test_signal_consistency() {
    let result = StateInvariantTester::test_signal_consistency();
    assert!(result.is_ok());
}
```

### Performance Property Testing

```rust
#[test]
fn test_render_performance_properties() {
    let result = PerformancePropertyTester::test_render_performance_properties();
    assert!(result.is_ok());
}

#[test]
fn test_virtual_scrolling_performance_properties() {
    let result = PerformancePropertyTester::test_virtual_scrolling_performance_properties();
    assert!(result.is_ok());
}
```

### Accessibility Property Testing

```rust
#[test]
fn test_accessibility_properties() {
    let result = AccessibilityPropertyTester::test_accessibility_properties();
    assert!(result.is_ok());
}

#[test]
fn test_keyboard_navigation_properties() {
    let result = AccessibilityPropertyTester::test_keyboard_navigation_properties();
    assert!(result.is_ok());
}
```

## Mutation Testing

### Component Mutation Testing

```rust
use radix_leptos_primitives::testing::*;

#[test]
fn test_button_mutations() {
    let result = ComponentMutationTester::test_button_mutations();
    assert!(result.total_mutations > 0);
    assert!(result.mutation_score > 0.0);
}

#[test]
fn test_input_mutations() {
    let result = ComponentMutationTester::test_input_mutations();
    assert!(result.total_mutations > 0);
    assert!(result.mutation_score > 0.0);
}

#[test]
fn test_form_validation_mutations() {
    let result = ComponentMutationTester::test_form_validation_mutations();
    assert!(result.total_mutations > 0);
    assert!(result.mutation_score > 0.0);
}
```

### State Mutation Testing

```rust
#[test]
fn test_signal_mutations() {
    let result = StateMutationTester::test_signal_mutations();
    assert!(result.total_mutations > 0);
    assert!(result.mutation_score > 0.0);
}

#[test]
fn test_state_management_mutations() {
    let result = StateMutationTester::test_state_management_mutations();
    assert!(result.total_mutations > 0);
    assert!(result.mutation_score > 0.0);
}
```

### Performance Mutation Testing

```rust
#[test]
fn test_render_performance_mutations() {
    let result = PerformanceMutationTester::test_render_performance_mutations();
    assert!(result.total_mutations > 0);
    assert!(result.mutation_score > 0.0);
}

#[test]
fn test_virtual_scrolling_performance_mutations() {
    let result = PerformanceMutationTester::test_virtual_scrolling_performance_mutations();
    assert!(result.total_mutations > 0);
    assert!(result.mutation_score > 0.0);
}
```

### Accessibility Mutation Testing

```rust
#[test]
fn test_accessibility_mutations() {
    let result = AccessibilityMutationTester::test_accessibility_mutations();
    assert!(result.total_mutations > 0);
    assert!(result.mutation_score > 0.0);
}

#[test]
fn test_keyboard_navigation_mutations() {
    let result = AccessibilityMutationTester::test_keyboard_navigation_mutations();
    assert!(result.total_mutations > 0);
    assert!(result.mutation_score > 0.0);
}
```

## Performance Benchmarking

### Component Performance Benchmarking

```rust
use radix_leptos_primitives::testing::*;

#[test]
fn benchmark_button_performance() {
    let result = ComponentPerformanceBenchmarks::benchmark_button();
    assert!(result.average_time.as_millis() < 10);
    assert!(result.iterations > 0);
}

#[test]
fn benchmark_input_performance() {
    let result = ComponentPerformanceBenchmarks::benchmark_input();
    assert!(result.average_time.as_millis() < 10);
    assert!(result.iterations > 0);
}

#[test]
fn benchmark_form_performance() {
    let result = ComponentPerformanceBenchmarks::benchmark_form();
    assert!(result.average_time.as_millis() < 50);
    assert!(result.iterations > 0);
}

#[test]
fn benchmark_table_performance() {
    let result = ComponentPerformanceBenchmarks::benchmark_table();
    assert!(result.average_time.as_millis() < 100);
    assert!(result.iterations > 0);
}
```

### State Performance Benchmarking

```rust
#[test]
fn benchmark_signal_updates() {
    let result = StatePerformanceBenchmarks::benchmark_signal_updates();
    assert!(result.average_time.as_millis() < 1);
    assert!(result.iterations > 0);
}

#[test]
fn benchmark_signal_derivations() {
    let result = StatePerformanceBenchmarks::benchmark_signal_derivations();
    assert!(result.average_time.as_millis() < 1);
    assert!(result.iterations > 0);
}
```

### Virtual Scrolling Performance Benchmarking

```rust
#[test]
fn benchmark_virtual_scrolling_scalability() {
    let results = VirtualScrollingPerformanceBenchmarks::benchmark_virtual_scrolling_scalability();
    assert!(!results.is_empty());
    
    for result in results {
        assert!(result.average_time.as_millis() < 100);
        assert!(result.iterations > 0);
    }
}
```

### Memory Performance Benchmarking

```rust
#[test]
fn benchmark_component_memory_usage() {
    let result = MemoryPerformanceBenchmarks::benchmark_component_memory_usage();
    assert!(result.iterations > 0);
    
    if let Some(memory) = &result.memory_usage {
        assert!(memory.peak_memory > 0);
        assert!(memory.average_memory > 0);
    }
}

#[test]
fn benchmark_state_memory_usage() {
    let result = MemoryPerformanceBenchmarks::benchmark_state_memory_usage();
    assert!(result.iterations > 0);
    
    if let Some(memory) = &result.memory_usage {
        assert!(memory.peak_memory > 0);
        assert!(memory.average_memory > 0);
    }
}
```

## Accessibility Testing

### ARIA Testing

```rust
#[wasm_bindgen_test]
fn test_aria_attributes() {
    let runtime = create_runtime();
    
    let view = view! {
        <FormField>
            <FormLabel for="test-input">"Test Label"</FormLabel>
            <Input
                id="test-input"
                aria_required=true
                aria_label="Test input"
            />
        </FormField>
    };
    
    assert!(view.into_any().is_some());
    runtime.dispose();
}

#[wasm_bindgen_test]
fn test_keyboard_navigation() {
    let runtime = create_runtime();
    let (focused_index, set_focused_index) = create_signal(0);
    let items = vec!["Item 1", "Item 2", "Item 3"];
    
    // Test focus management
    assert!(focused_index() >= 0);
    assert!(focused_index() < items.len());
    
    set_focused_index(1);
    assert_eq!(focused_index(), 1);
    
    runtime.dispose();
}
```

### Screen Reader Testing

```rust
#[wasm_bindgen_test]
fn test_screen_reader_support() {
    let runtime = create_runtime();
    
    let view = view! {
        <div role="main" aria_labelledby="main-title">
            <h1 id="main-title">"Main Content"</h1>
            <div role="alert" aria_live="polite">
                "Important message"
            </div>
        </div>
    };
    
    assert!(view.into_any().is_some());
    runtime.dispose();
}
```

## Test Automation

### CI/CD Integration

```yaml
# .github/workflows/test.yml
name: Test Suite

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Install wasm-pack
      run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
      
    - name: Run tests
      run: |
        cargo test --workspace
        wasm-pack test --headless --firefox
        wasm-pack test --headless --chrome
        
    - name: Run property-based tests
      run: cargo test --features proptest
      
    - name: Run mutation tests
      run: cargo test --features mutation-testing
      
    - name: Run performance benchmarks
      run: cargo test --features performance-benchmarking
```

### Test Configuration

```toml
# Cargo.toml
[features]
default = []
proptest = ["proptest"]
mutation-testing = ["mutation-testing"]
performance-benchmarking = ["performance-benchmarking"]

[dev-dependencies]
proptest = { version = "1.0", optional = true }
mutation-testing = { version = "0.1", optional = true }
performance-benchmarking = { version = "0.1", optional = true }
```

## Best Practices

### Test Organization

```rust
// tests/component_tests.rs
mod button_tests;
mod input_tests;
mod form_tests;

// tests/integration_tests.rs
mod form_integration;
mod state_integration;
mod performance_integration;

// tests/property_tests.rs
mod component_properties;
mod state_properties;
mod performance_properties;
```

### Test Naming

```rust
// Good test names
#[test]
fn test_button_renders_with_correct_variant() { }

#[test]
fn test_input_validates_email_format() { }

#[test]
fn test_form_submission_clears_errors() { }

// Bad test names
#[test]
fn test1() { }

#[test]
fn test_button() { }

#[test]
fn test_stuff() { }
```

### Test Data Management

```rust
// test_utils.rs
pub struct TestData {
    pub users: Vec<User>,
    pub forms: Vec<FormData>,
    pub settings: Settings,
}

impl TestData {
    pub fn new() -> Self {
        Self {
            users: vec![
                User { id: "1".to_string(), name: "John Doe".to_string(), email: "john@example.com".to_string() },
                User { id: "2".to_string(), name: "Jane Smith".to_string(), email: "jane@example.com".to_string() },
            ],
            forms: vec![
                FormData { name: "Test Form".to_string(), fields: vec![] },
            ],
            settings: Settings::default(),
        }
    }
}
```

### Test Coverage

```rust
// coverage_tests.rs
#[test]
fn test_component_coverage() {
    // Test all component variants
    for variant in ButtonVariant::all() {
        let view = view! { <Button variant=variant>"Test"</Button> };
        assert!(view.into_any().is_some());
    }
    
    // Test all component sizes
    for size in ButtonSize::all() {
        let view = view! { <Button size=size>"Test"</Button> };
        assert!(view.into_any().is_some());
    }
}
```

### Performance Testing

```rust
#[test]
fn test_render_performance() {
    let start = std::time::Instant::now();
    
    for _ in 0..1000 {
        let runtime = create_runtime();
        let _ = view! { <Button>"Test"</Button> };
        runtime.dispose();
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 1000); // Should complete in under 1 second
}
```

### Memory Testing

```rust
#[test]
fn test_memory_usage() {
    let initial_memory = get_memory_usage();
    
    let runtime = create_runtime();
    let (state, _) = create_signal(vec![0; 10000]);
    runtime.dispose();
    
    let final_memory = get_memory_usage();
    let memory_growth = final_memory - initial_memory;
    
    assert!(memory_growth < 1024 * 1024); // Should use less than 1MB
}
```

## Conclusion

This comprehensive testing guide provides all the tools and strategies needed to ensure the quality and reliability of Radix-Leptos components. By following these practices, you can:

- **Ensure Quality**: Comprehensive test coverage
- **Prevent Regressions**: Automated testing
- **Optimize Performance**: Performance benchmarking
- **Ensure Accessibility**: Accessibility testing
- **Maintain Reliability**: Mutation testing

For more information, see the [API Reference](api-reference/testing-api.md) and [Examples](examples/testing-examples.md).
