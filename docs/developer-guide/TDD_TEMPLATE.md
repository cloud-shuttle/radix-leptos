# 🧪 TDD Component Testing Template

This template provides a standardized approach to Test-Driven Development for Radix-Leptos components.

## 📋 TDD Checklist

### Phase 1: RED (Write Failing Test)
- [ ] Write test for basic component rendering
- [ ] Write test for props handling
- [ ] Write test for state management
- [ ] Write test for event handling
- [ ] Write test for accessibility features
- [ ] Write test for edge cases

### Phase 2: GREEN (Make Tests Pass)
- [ ] Implement minimal component to pass rendering test
- [ ] Add props support to pass props test
- [ ] Add state management to pass state test
- [ ] Add event handling to pass event test
- [ ] Add accessibility features to pass a11y test
- [ ] Handle edge cases to pass edge case test

### Phase 3: REFACTOR (Improve Implementation)
- [ ] Clean up code while keeping tests green
- [ ] Optimize performance
- [ ] Improve readability
- [ ] Add documentation

## 🧪 Test Structure Template

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use proptest::prelude::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    // 1. Basic Rendering Tests
    #[wasm_bindgen_test]
    fn test_component_renders() {
        run_test(|cx| {
            let view = view! { cx,
                <ComponentName>
                    "Test Content"
                </ComponentName>
            };
            
            // Assert component renders without errors
            // This test should FAIL initially (RED phase)
        });
    }
    
    // 2. Props Validation Tests
    #[wasm_bindgen_test]
    fn test_component_props() {
        run_test(|cx| {
            let view = view! { cx,
                <ComponentName 
                    variant=ComponentVariant::Primary
                    size=ComponentSize::Large
                    disabled=true
                >
                    "Test Content"
                </ComponentName>
            };
            
            // Assert props are applied correctly
        });
    }
    
    // 3. State Management Tests
    #[wasm_bindgen_test]
    fn test_component_state() {
        run_test(|cx| {
            let (state, set_state) = create_signal(cx, false);
            
            let view = view! { cx,
                <ComponentName 
                    value=state
                    on_change=move |new_value| set_state.set(new_value)
                >
                    "Test Content"
                </ComponentName>
            };
            
            // Test state changes
            set_state.set(true);
            assert!(state.get());
        });
    }
    
    // 4. Event Handling Tests
    #[wasm_bindgen_test]
    fn test_component_events() {
        run_test(|cx| {
            let (click_count, set_click_count) = create_signal(cx, 0);
            
            let view = view! { cx,
                <ComponentName 
                    on_click=move |_| set_click_count.update(|c| *c += 1)
                >
                    "Test Content"
                </ComponentName>
            };
            
            // Simulate click event
            // Assert click handler is called
        });
    }
    
    // 5. Accessibility Tests
    #[wasm_bindgen_test]
    fn test_component_accessibility() {
        run_test(|cx| {
            let view = view! { cx,
                <ComponentName 
                    aria_label="Test Component"
                    role="button"
                >
                    "Test Content"
                </ComponentName>
            };
            
            // Assert ARIA attributes are present
            // Assert keyboard navigation works
            // Assert screen reader compatibility
        });
    }
    
    // 6. Edge Case Tests
    #[wasm_bindgen_test]
    fn test_component_edge_cases() {
        run_test(|cx| {
            // Test with empty content
            let view1 = view! { cx, <ComponentName></ComponentName> };
            
            // Test with very long content
            let long_content = "x".repeat(10000);
            let view2 = view! { cx, 
                <ComponentName>{long_content}</ComponentName> 
            };
            
            // Test with special characters
            let special_content = "🚀 Test with émojis & spéciál chars";
            let view3 = view! { cx, 
                <ComponentName>{special_content}</ComponentName> 
            };
            
            // Assert all cases handle gracefully
        });
    }
    
    // 7. Property-Based Tests
    proptest! {
        #[test]
        fn test_component_properties(
            variant in prop::sample::select(vec![
                ComponentVariant::Primary,
                ComponentVariant::Secondary,
                ComponentVariant::Destructive
            ]),
            size in prop::sample::select(vec![
                ComponentSize::Small,
                ComponentSize::Medium,
                ComponentSize::Large
            ]),
            disabled in prop::bool::ANY,
            content in ".*"
        ) {
            run_test(|cx| {
                let view = view! { cx,
                    <ComponentName 
                        variant=variant
                        size=size
                        disabled=disabled
                    >
                        {content}
                    </ComponentName>
                };
                
                // Property: Component should always render without panicking
                // Property: Disabled state should be respected
                // Property: Size should affect styling
            });
        }
    }
    
    // Helper function for running tests
    fn run_test<F>(f: F) where F: FnOnce(Scope) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let _ = create_runtime();
            run_scope(create_runtime(), f);
        });
    }
}
```

## 🎯 TDD Workflow Commands

```bash
# Start TDD for new component
make tdd-new-component

# Run tests in watch mode
make test-watch

# Run specific test types
make test-unit
make test-integration
make test-property
make test-mutants

# Generate coverage report
make test-coverage
```

## 📊 Success Criteria

### Component is TDD Complete When:
- [ ] All tests pass (100% green)
- [ ] Tests were written before implementation
- [ ] Property-based tests cover edge cases
- [ ] Accessibility tests pass
- [ ] Mutation testing score >80%
- [ ] Code coverage >90%
- [ ] No placeholder assertions
- [ ] Documentation is complete

## 🔄 Red-Green-Refactor Cycle

1. **RED**: Write a failing test
2. **GREEN**: Write minimal code to make test pass
3. **REFACTOR**: Improve code while keeping tests green
4. **REPEAT**: Continue cycle for each feature

## 📝 Example TDD Session

```bash
# 1. Create new component test file
touch crates/radix-leptos-primitives/src/components/new_component.rs

# 2. Write failing test (RED)
# Add test_component_renders() that fails

# 3. Run test to confirm it fails
cargo test test_component_renders

# 4. Implement minimal component (GREEN)
# Add basic component implementation

# 5. Run test to confirm it passes
cargo test test_component_renders

# 6. Refactor implementation
# Improve code while keeping tests green

# 7. Repeat for next feature
```

This template ensures consistent, high-quality TDD practices across all Radix-Leptos components.
