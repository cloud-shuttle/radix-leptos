# 🧪 Test-Driven Development Guide for Radix-Leptos

This guide provides a comprehensive approach to implementing Test-Driven Development (TDD) practices in the Radix-Leptos component library.

## 🎯 What is TDD?

Test-Driven Development is a software development approach where you:

1. **RED**: Write a failing test first
2. **GREEN**: Write minimal code to make the test pass
3. **REFACTOR**: Improve the code while keeping tests green
4. **REPEAT**: Continue the cycle for each feature

## 🚀 Quick Start

### 1. Set Up TDD Environment

```bash
# Install TDD tools
cargo install cargo-tarpaulin cargo-mutants

# Make TDD workflow script executable
chmod +x scripts/tdd-workflow.sh
```

### 2. Start TDD for New Component

```bash
# Create new component structure
./scripts/tdd-workflow.sh new-component checkbox

# Follow TDD cycle
./scripts/tdd-workflow.sh red test_checkbox_renders
./scripts/tdd-workflow.sh green
./scripts/tdd-workflow.sh refactor
```

### 3. Use TDD Commands

```bash
# Run tests in watch mode
make test-watch

# Check TDD compliance
make test-tdd-check

# Generate coverage report
make test-coverage

# Run mutation testing
make test-mutants
```

## 📋 TDD Process

### Phase 1: RED (Write Failing Test)

Before writing any implementation code:

1. **Write a failing test** that describes the desired behavior
2. **Run the test** to confirm it fails
3. **Commit the failing test** (optional but recommended)

```rust
#[wasm_bindgen_test]
fn test_checkbox_renders() {
    run_test(|cx| {
        let view = view! { cx,
            <Checkbox>
                "Test Checkbox"
            </Checkbox>
        };
        
        // This test should FAIL initially
        // We haven't implemented the Checkbox component yet
    });
}
```

### Phase 2: GREEN (Make Test Pass)

Write the minimal code to make the test pass:

1. **Implement the simplest solution** that makes the test pass
2. **Don't worry about code quality** yet - just make it work
3. **Run the test** to confirm it passes

```rust
#[component]
pub fn Checkbox(children: Children) -> impl IntoView {
    view! {
        <input type="checkbox" />
        {children()}
    }
}
```

### Phase 3: REFACTOR (Improve Code)

Now improve the code while keeping tests green:

1. **Clean up the implementation**
2. **Add proper error handling**
3. **Improve performance**
4. **Add documentation**
5. **Run tests** to ensure they still pass

```rust
#[component]
pub fn Checkbox(
    #[prop(optional, default = false)] checked: bool,
    #[prop(optional, default = false)] disabled: bool,
    #[prop(optional)] on_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let checkbox_id = generate_id("checkbox");
    
    let handle_change = move |e: web_sys::Event| {
        if let Some(target) = e.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                if let Some(on_change) = on_change {
                    on_change.run(input.checked());
                }
            }
        }
    };
    
    view! {
        <div class="checkbox-container">
            <input
                id=checkbox_id
                type="checkbox"
                checked=checked
                disabled=disabled
                on:change=handle_change
            />
            <label for=checkbox_id>
                {children()}
            </label>
        </div>
    }
}
```

## 🧪 Test Categories

### 1. Basic Rendering Tests

Test that components render without errors:

```rust
#[wasm_bindgen_test]
fn test_component_renders() {
    run_test(|cx| {
        let view = view! { cx,
            <ComponentName>
                "Test Content"
            </ComponentName>
        };
        
        // Component should render without panicking
    });
}
```

### 2. Props Validation Tests

Test that props are handled correctly:

```rust
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
        
        // Props should be applied correctly
    });
}
```

### 3. State Management Tests

Test component state changes:

```rust
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
```

### 4. Event Handling Tests

Test user interactions:

```rust
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
```

### 5. Accessibility Tests

Test accessibility features:

```rust
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
```

### 6. Edge Case Tests

Test boundary conditions:

```rust
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
```

### 7. Property-Based Tests

Test component invariants with random inputs:

```rust
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
```

## 🛠️ TDD Tools

### 1. Test Runner

```bash
# Run all tests
cargo test --workspace

# Run specific test
cargo test test_component_renders

# Run tests in watch mode
cargo watch -x "test --workspace"
```

### 2. Coverage Analysis

```bash
# Generate coverage report
cargo tarpaulin --all-features --workspace --out Html

# View coverage report
open tarpaulin-report.html
```

### 3. Mutation Testing

```bash
# Run mutation tests
cargo mutants

# This helps identify weak tests that don't catch bugs
```

### 4. TDD Workflow Script

```bash
# Start new component
./scripts/tdd-workflow.sh new-component checkbox

# Follow TDD cycle
./scripts/tdd-workflow.sh red test_checkbox_renders
./scripts/tdd-workflow.sh green
./scripts/tdd-workflow.sh refactor

# Check compliance
./scripts/tdd-workflow.sh check
```

## 📊 Success Metrics

### Immediate Goals (1 month)
- [ ] 100% of new components follow TDD workflow
- [ ] 90%+ test coverage for all components
- [ ] Zero placeholder assertions in test code
- [ ] All TODOs in test code resolved

### Long-term Goals (3 months)
- [ ] Property-based tests for all core components
- [ ] Mutation testing score >80%
- [ ] Automated TDD workflow enforcement
- [ ] Comprehensive test documentation

## 🎯 Best Practices

### 1. Test Naming

Use descriptive test names that explain what is being tested:

```rust
// Good
fn test_button_renders_with_destructive_variant() { }

// Bad
fn test_button() { }
```

### 2. Test Organization

Organize tests by category and use clear comments:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // 1. Basic Rendering Tests
    #[test]
    fn test_component_renders() { }
    
    // 2. Props Validation Tests
    #[test]
    fn test_component_props() { }
    
    // 3. State Management Tests
    #[test]
    fn test_component_state() { }
}
```

### 3. Test Data

Use realistic test data and edge cases:

```rust
// Good - realistic data
let user_email = "user@example.com";
let long_content = "x".repeat(1000);

// Bad - unrealistic data
let test_data = "test";
```

### 4. Assertions

Make assertions specific and meaningful:

```rust
// Good
assert_eq!(button.get_attribute("disabled"), Some("true".to_string()));

// Bad
assert!(button.has_attribute("disabled"));
```

### 5. Test Isolation

Each test should be independent and not rely on other tests:

```rust
// Good - each test is independent
#[test]
fn test_button_initial_state() {
    let button = create_button();
    assert!(!button.is_disabled());
}

#[test]
fn test_button_disabled_state() {
    let button = create_button();
    button.set_disabled(true);
    assert!(button.is_disabled());
}
```

## 🚨 Common Pitfalls

### 1. Writing Tests After Implementation

**Problem**: Writing tests after the code is already implemented
**Solution**: Always write tests first (RED phase)

### 2. Over-Engineering in GREEN Phase

**Problem**: Writing complex code in the GREEN phase
**Solution**: Write minimal code to make tests pass, refactor later

### 3. Skipping REFACTOR Phase

**Problem**: Not improving code after tests pass
**Solution**: Always refactor to improve code quality

### 4. Weak Assertions

**Problem**: Tests that don't actually verify behavior
**Solution**: Write specific, meaningful assertions

### 5. Testing Implementation Details

**Problem**: Testing internal implementation rather than behavior
**Solution**: Test public interface and observable behavior

## 🔄 TDD Cycle Example

Let's walk through a complete TDD cycle for a simple component:

### Step 1: RED - Write Failing Test

```rust
#[wasm_bindgen_test]
fn test_alert_renders_with_message() {
    run_test(|cx| {
        let view = view! { cx,
            <Alert message="Test alert message" />
        };
        
        // This test will FAIL - Alert component doesn't exist yet
    });
}
```

### Step 2: GREEN - Make Test Pass

```rust
#[component]
pub fn Alert(message: String) -> impl IntoView {
    view! {
        <div class="alert">
            {message}
        </div>
    }
}
```

### Step 3: REFACTOR - Improve Code

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum AlertVariant {
    Default,
    Success,
    Warning,
    Error,
}

#[component]
pub fn Alert(
    message: String,
    #[prop(optional, default = AlertVariant::Default)] variant: AlertVariant,
    #[prop(optional)] on_close: Option<Callback<()>>,
) -> impl IntoView {
    let alert_id = generate_id("alert");
    
    let handle_close = move |_| {
        if let Some(on_close) = on_close {
            on_close.run(());
        }
    };
    
    view! {
        <div 
            id=alert_id
            class=format!("alert alert-{}", variant.as_str())
            role="alert"
            aria-live="polite"
        >
            <span class="alert-message">{message}</span>
            <Show when=move || on_close.is_some()>
                <button 
                    class="alert-close"
                    aria-label="Close alert"
                    on:click=handle_close
                >
                    "×"
                </button>
            </Show>
        </div>
    }
}
```

### Step 4: Add More Tests

```rust
#[wasm_bindgen_test]
fn test_alert_with_different_variants() {
    run_test(|cx| {
        let variants = vec![
            AlertVariant::Default,
            AlertVariant::Success,
            AlertVariant::Warning,
            AlertVariant::Error,
        ];
        
        for variant in variants {
            let view = view! { cx,
                <Alert 
                    message="Test message" 
                    variant=variant 
                />
            };
            // Each variant should render without errors
        }
    });
}

#[wasm_bindgen_test]
fn test_alert_close_functionality() {
    run_test(|cx| {
        let (is_visible, set_is_visible) = create_signal(cx, true);
        
        let view = view! { cx,
            <Show when=is_visible>
                <Alert 
                    message="Test message"
                    on_close=move |_| set_is_visible.set(false)
                />
            </Show>
        };
        
        // Initially visible
        assert!(is_visible.get());
        
        // Simulate close
        set_is_visible.set(false);
        assert!(!is_visible.get());
    });
}
```

## 🎉 Conclusion

TDD is a powerful approach that leads to:

- **Higher code quality** - Tests catch bugs early
- **Better design** - Writing tests first leads to better APIs
- **Confidence** - Comprehensive tests provide safety for refactoring
- **Documentation** - Tests serve as living documentation
- **Faster development** - Fewer bugs mean less debugging time

Start with simple components and gradually apply TDD to more complex features. Remember: the goal is not just to have tests, but to have a robust, maintainable codebase that you can confidently modify and extend.

## 📚 Additional Resources

- [TDD Template](./TDD_TEMPLATE.md) - Standardized test structure
- [Test Strategy](./test-strategy.md) - Comprehensive testing approach
- [Component Examples](../examples/) - Working examples of TDD components
- [Makefile Commands](../Makefile) - TDD-related make commands

Happy testing! 🧪✨
