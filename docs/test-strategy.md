# Radix-Leptos Comprehensive Test Strategy

## Executive Summary

This test strategy ensures radix-leptos delivers production-ready, accessible, and performant components through comprehensive testing at multiple levels. The strategy emphasizes automated testing, accessibility compliance, and real-world usage scenarios.

## 1. Testing Philosophy

### Core Principles
- **Test Early, Test Often**: Write tests before or alongside implementation
- **Accessibility First**: Every component must pass WCAG 2.1 AA standards
- **Real User Scenarios**: Tests should reflect actual usage patterns
- **Fast Feedback**: Unit tests run in milliseconds, integration tests in seconds
- **Comprehensive Coverage**: Minimum 90% code coverage, 100% for critical paths
- **Cross-Platform**: Test across browsers, devices, and assistive technologies

## 2. Test Architecture

### 2.1 Test Pyramid

```
         ╱╲
        ╱E2E╲        <- 10% (Critical user journeys)
       ╱──────╲
      ╱ Visual ╲     <- 15% (UI regression)
     ╱──────────╲
    ╱Integration ╲   <- 25% (Component interactions)
   ╱──────────────╲
  ╱  Unit Tests    ╲ <- 50% (Component logic)
 ╱──────────────────╲
```

### 2.2 Test Categories

| Category | Purpose | Tools | Execution Time |
|----------|---------|-------|----------------|
| Unit | Component logic, hooks | cargo test, wasm-bindgen-test | <100ms |
| Integration | Component interactions | wasm-bindgen-test, leptos-test | <1s |
| E2E | User workflows | Playwright, Cypress | <10s |
| Accessibility | WCAG compliance | axe-core, pa11y | <5s |
| Visual | UI regression | Percy, Chromatic | <30s |
| Performance | Runtime metrics | Lighthouse, custom | <30s |
| Property | Invariant testing | proptest, quickcheck | <5s |

## 3. Testing Tools & Infrastructure

### 3.1 Core Testing Stack

```toml
# Cargo.toml dev-dependencies
[dev-dependencies]
# Unit testing
leptos-test = "0.1"
wasm-bindgen-test = "0.3"
proptest = "1.0"
quickcheck = "1.0"

# DOM testing
web-sys = { version = "0.3", features = ["HtmlElement", "Document"] }
wasm-bindgen = "0.2"

# Async testing
tokio-test = "0.4"
async-std = { version = "1.12", features = ["attributes"] }

# Mocking
mockall = "0.11"
fake = "2.5"

# Coverage
cargo-tarpaulin = "0.27"
```

### 3.2 JavaScript Testing Tools

```json
{
  "devDependencies": {
    "@playwright/test": "^1.40",
    "@axe-core/playwright": "^4.8",
    "@percy/playwright": "^1.0",
    "lighthouse": "^11.0",
    "pa11y": "^6.2"
  }
}
```

### 3.3 CI/CD Integration

```yaml
# .github/workflows/test.yml
name: Test Suite

on: [push, pull_request]

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
      - run: cargo tarpaulin --out Xml
      - uses: codecov/codecov-action@v3

  wasm-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: wasm-pack test --headless --firefox
      - run: wasm-pack test --headless --chrome

  e2e-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - run: npx playwright install
      - run: npm run test:e2e

  accessibility-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: npm run test:a11y

  visual-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: npm run test:visual
      - uses: percy/snapshot-action@v1
```

## 4. Test Organization

### 4.1 Directory Structure

```
radix-leptos/
├── tests/
│   ├── unit/
│   │   ├── hooks/
│   │   │   ├── use_controllable_state.rs
│   │   │   └── use_focus_trap.rs
│   │   └── components/
│   │       ├── dialog/
│   │       │   ├── dialog_state.rs
│   │       │   └── dialog_focus.rs
│   │       └── select/
│   ├── integration/
│   │   ├── dialog_select_interaction.rs
│   │   └── nested_overlays.rs
│   ├── e2e/
│   │   ├── playwright/
│   │   │   ├── dialog.spec.ts
│   │   │   └── form_flow.spec.ts
│   │   └── fixtures/
│   ├── accessibility/
│   │   ├── aria_compliance.rs
│   │   └── screen_reader/
│   ├── visual/
│   │   └── snapshots/
│   └── performance/
│       ├── benchmarks.rs
│       └── metrics.json
```

### 4.2 Test File Naming Convention

- Unit tests: `{module}_test.rs` or `{module}.test.rs`
- Integration tests: `{feature}_integration.rs`
- E2E tests: `{user_flow}.spec.ts`
- Accessibility tests: `{component}_a11y.rs`
- Performance tests: `{component}_bench.rs`

## 5. Component Testing Strategy

### 5.1 Unit Testing

Each component requires unit tests for:

```rust
// Example: Dialog component unit tests
#[cfg(test)]
mod dialog_tests {
    use super::*;
    use leptos_test::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn dialog_initial_state() {
        create_test_scope(|cx| {
            let (open, set_open) = create_signal(cx, false);
            
            let node = render_component(cx, || {
                view! { cx,
                    <DialogRoot open=open>
                        <DialogContent>"Test"</DialogContent>
                    </DialogRoot>
                }
            });

            assert!(!node.query_selector("[role='dialog']").is_some());
        });
    }

    #[wasm_bindgen_test]
    fn dialog_opens_on_trigger_click() {
        create_test_scope(|cx| {
            let node = render_component(cx, || {
                view! { cx,
                    <DialogRoot>
                        <DialogTrigger>"Open"</DialogTrigger>
                        <DialogContent>"Content"</DialogContent>
                    </DialogRoot>
                }
            });

            let trigger = node.query_selector("button").unwrap();
            trigger.click();
            
            assert!(node.query_selector("[role='dialog']").is_some());
        });
    }

    #[wasm_bindgen_test]
    fn dialog_controlled_mode() {
        create_test_scope(|cx| {
            let (open, set_open) = create_signal(cx, false);
            let on_open_change = move |value: bool| set_open.set(value);

            let node = render_component(cx, || {
                view! { cx,
                    <DialogRoot 
                        open=open 
                        on_open_change=on_open_change
                    >
                        <DialogContent>"Test"</DialogContent>
                    </DialogRoot>
                }
            });

            set_open.set(true);
            assert!(node.query_selector("[role='dialog']").is_some());
            
            set_open.set(false);
            assert!(!node.query_selector("[role='dialog']").is_some());
        });
    }
}
```

### 5.2 Integration Testing

Test component interactions:

```rust
#[wasm_bindgen_test]
fn dialog_with_select_integration() {
    create_test_scope(|cx| {
        let node = render_component(cx, || {
            view! { cx,
                <DialogRoot>
                    <DialogTrigger>"Open"</DialogTrigger>
                    <DialogContent>
                        <SelectRoot>
                            <SelectTrigger>"Choose"</SelectTrigger>
                            <SelectContent>
                                <SelectItem value="1">"Option 1"</SelectItem>
                            </SelectContent>
                        </SelectRoot>
                    </DialogContent>
                </DialogRoot>
            }
        });

        // Open dialog
        node.query_selector("[data-dialog-trigger]").unwrap().click();
        assert!(node.query_selector("[role='dialog']").is_some());

        // Open select within dialog
        node.query_selector("[data-select-trigger]").unwrap().click();
        assert!(node.query_selector("[role='listbox']").is_some());

        // Both overlays should be present
        assert_eq!(node.query_selector_all("[data-radix-portal]").length(), 2);
    });
}
```

### 5.3 Accessibility Testing

#### 5.3.1 ARIA Compliance Tests

```rust
#[wasm_bindgen_test]
fn dialog_aria_attributes() {
    create_test_scope(|cx| {
        let node = render_component(cx, || {
            view! { cx,
                <DialogRoot default_open=true>
                    <DialogTrigger>"Open"</DialogTrigger>
                    <DialogContent>
                        <DialogTitle>"Title"</DialogTitle>
                        <DialogDescription>"Description"</DialogDescription>
                    </DialogContent>
                </DialogRoot>
            }
        });

        let dialog = node.query_selector("[role='dialog']").unwrap();
        let title = node.query_selector("[data-dialog-title]").unwrap();
        let desc = node.query_selector("[data-dialog-description]").unwrap();

        // Verify ARIA attributes
        assert_eq!(dialog.get_attribute("role"), Some("dialog".into()));
        assert_eq!(dialog.get_attribute("aria-modal"), Some("true".into()));
        assert_eq!(
            dialog.get_attribute("aria-labelledby"), 
            Some(title.id())
        );
        assert_eq!(
            dialog.get_attribute("aria-describedby"), 
            Some(desc.id())
        );
    });
}
```

#### 5.3.2 Keyboard Navigation Tests

```rust
#[wasm_bindgen_test]
fn select_keyboard_navigation() {
    create_test_scope(|cx| {
        let node = render_component(cx, || {
            view! { cx,
                <SelectRoot default_open=true>
                    <SelectContent>
                        <SelectItem value="1">"Apple"</SelectItem>
                        <SelectItem value="2">"Banana"</SelectItem>
                        <SelectItem value="3">"Cherry"</SelectItem>
                    </SelectContent>
                </SelectRoot>
            }
        });

        let items = node.query_selector_all("[role='option']");
        
        // Test arrow down navigation
        dispatch_keyboard_event(&items[0], "ArrowDown");
        assert!(items[1].has_focus());

        // Test arrow up navigation
        dispatch_keyboard_event(&items[1], "ArrowUp");
        assert!(items[0].has_focus());

        // Test home key
        dispatch_keyboard_event(&items[1], "Home");
        assert!(items[0].has_focus());

        // Test end key
        dispatch_keyboard_event(&items[0], "End");
        assert!(items[2].has_focus());

        // Test typeahead
        dispatch_keyboard_event(&items[0], "b");
        assert!(items[1].has_focus()); // "Banana"
    });
}
```

#### 5.3.3 Screen Reader Tests

```rust
#[wasm_bindgen_test]
async fn screen_reader_announcements() {
    create_test_scope(|cx| {
        let node = render_component(cx, || {
            view! { cx,
                <form>
                    <Label for="email">"Email Address"</Label>
                    <input id="email" type="email" required=true />
                </form>
            }
        });

        // Verify screen reader associations
        let input = node.query_selector("#email").unwrap();
        let label = node.query_selector("label").unwrap();
        
        assert_eq!(label.get_attribute("for"), Some("email".into()));
        assert!(input.has_attribute("required"));
        
        // Test live region updates
        let live_region = get_or_create_live_region();
        announce(&live_region, "Form submitted successfully", "polite");
        
        assert_eq!(
            live_region.text_content(),
            Some("Form submitted successfully".into())
        );
    });
}
```

### 5.4 E2E Testing with Playwright

```typescript
// tests/e2e/dialog.spec.ts
import { test, expect } from '@playwright/test';
import { injectAxe, checkA11y } from '@axe-core/playwright';

test.describe('Dialog Component', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/components/dialog');
    await injectAxe(page);
  });

  test('should open and close dialog', async ({ page }) => {
    // Open dialog
    await page.click('[data-testid="dialog-trigger"]');
    await expect(page.locator('[role="dialog"]')).toBeVisible();

    // Close with escape key
    await page.keyboard.press('Escape');
    await expect(page.locator('[role="dialog"]')).not.toBeVisible();
  });

  test('should trap focus within dialog', async ({ page }) => {
    await page.click('[data-testid="dialog-trigger"]');
    
    // First focusable element should have focus
    await expect(page.locator('[data-testid="dialog-close"]')).toBeFocused();
    
    // Tab through elements
    await page.keyboard.press('Tab');
    await expect(page.locator('input[type="text"]')).toBeFocused();
    
    await page.keyboard.press('Tab');
    await expect(page.locator('button[type="submit"]')).toBeFocused();
    
    // Should cycle back to first element
    await page.keyboard.press('Tab');
    await expect(page.locator('[data-testid="dialog-close"]')).toBeFocused();
  });

  test('should pass accessibility checks', async ({ page }) => {
    await page.click('[data-testid="dialog-trigger"]');
    await checkA11y(page);
  });

  test('should handle nested dialogs', async ({ page }) => {
    // Open first dialog
    await page.click('[data-testid="dialog-1-trigger"]');
    await expect(page.locator('[data-testid="dialog-1"]')).toBeVisible();
    
    // Open nested dialog
    await page.click('[data-testid="dialog-2-trigger"]');
    await expect(page.locator('[data-testid="dialog-2"]')).toBeVisible();
    
    // Close nested dialog
    await page.keyboard.press('Escape');
    await expect(page.locator('[data-testid="dialog-2"]')).not.toBeVisible();
    await expect(page.locator('[data-testid="dialog-1"]')).toBeVisible();
  });
});
```

### 5.5 Visual Regression Testing

```typescript
// tests/visual/dialog.visual.ts
import { test, expect } from '@playwright/test';

test.describe('Dialog Visual Tests', () => {
  test('dialog default state', async ({ page }) => {
    await page.goto('/components/dialog');
    await page.click('[data-testid="dialog-trigger"]');
    await expect(page).toHaveScreenshot('dialog-open.png');
  });

  test('dialog with form', async ({ page }) => {
    await page.goto('/examples/dialog-form');
    await page.click('[data-testid="open-form"]');
    await expect(page).toHaveScreenshot('dialog-form.png');
  });

  test('dialog mobile view', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/components/dialog');
    await page.click('[data-testid="dialog-trigger"]');
    await expect(page).toHaveScreenshot('dialog-mobile.png');
  });
});
```

### 5.6 Performance Testing

```rust
// tests/performance/benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn dialog_render_benchmark(c: &mut Criterion) {
    c.bench_function("dialog initial render", |b| {
        b.iter(|| {
            create_test_scope(|cx| {
                render_component(cx, || {
                    view! { cx,
                        <DialogRoot>
                            <DialogTrigger>"Open"</DialogTrigger>
                            <DialogContent>"Content"</DialogContent>
                        </DialogRoot>
                    }
                })
            })
        });
    });

    c.bench_function("dialog with 100 items", |b| {
        b.iter(|| {
            create_test_scope(|cx| {
                let items = (0..100).map(|i| {
                    view! { cx, <div>{i}</div> }
                }).collect_view(cx);

                render_component(cx, || {
                    view! { cx,
                        <DialogContent>{items}</DialogContent>
                    }
                })
            })
        });
    });
}

criterion_group!(benches, dialog_render_benchmark);
criterion_main!(benches);
```

## 6. Test Data Management

### 6.1 Test Fixtures

```rust
// tests/fixtures/mod.rs
pub mod test_data {
    use fake::{Fake, Faker};
    
    pub struct TestUser {
        pub name: String,
        pub email: String,
    }
    
    impl TestUser {
        pub fn fake() -> Self {
            Self {
                name: Faker.fake(),
                email: Faker.fake(),
            }
        }
    }
    
    pub fn select_options(count: usize) -> Vec<(String, String)> {
        (0..count)
            .map(|i| (format!("value-{}", i), format!("Option {}", i)))
            .collect()
    }
}
```

### 6.2 Component Test Utilities

```rust
// tests/utils/mod.rs
use leptos::*;
use web_sys::{Element, Event, KeyboardEvent, MouseEvent};

pub struct TestHarness {
    root: Element,
    scope: Scope,
}

impl TestHarness {
    pub fn new<F, V>(f: F) -> Self 
    where
        F: FnOnce(Scope) -> V,
        V: IntoView,
    {
        create_test_scope(|cx| {
            let root = render_component(cx, || f(cx));
            Self { root, scope: cx }
        })
    }

    pub fn click(&self, selector: &str) {
        let element = self.root.query_selector(selector)
            .expect(&format!("Element not found: {}", selector));
        element.dispatch_event(&create_mouse_event("click"));
    }

    pub fn type_text(&self, selector: &str, text: &str) {
        let input = self.root.query_selector(selector)
            .expect(&format!("Input not found: {}", selector));
        
        for char in text.chars() {
            input.dispatch_event(&create_keyboard_event("keydown", &char.to_string()));
            input.set_value(&format!("{}{}", input.value(), char));
            input.dispatch_event(&create_keyboard_event("keyup", &char.to_string()));
        }
    }

    pub fn press_key(&self, selector: &str, key: &str) {
        let element = self.root.query_selector(selector)
            .expect(&format!("Element not found: {}", selector));
        element.dispatch_event(&create_keyboard_event("keydown", key));
    }

    pub fn assert_visible(&self, selector: &str) {
        assert!(
            self.root.query_selector(selector).is_some(),
            "Element should be visible: {}",
            selector
        );
    }

    pub fn assert_not_visible(&self, selector: &str) {
        assert!(
            self.root.query_selector(selector).is_none(),
            "Element should not be visible: {}",
            selector
        );
    }

    pub fn assert_text(&self, selector: &str, expected: &str) {
        let element = self.root.query_selector(selector)
            .expect(&format!("Element not found: {}", selector));
        assert_eq!(element.text_content(), Some(expected.into()));
    }

    pub fn assert_attribute(&self, selector: &str, attr: &str, expected: &str) {
        let element = self.root.query_selector(selector)
            .expect(&format!("Element not found: {}", selector));
        assert_eq!(
            element.get_attribute(attr),
            Some(expected.into()),
            "Attribute {} should be {}",
            attr,
            expected
        );
    }
}
```

## 7. Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn select_handles_any_string_value(value in any::<String>()) {
        create_test_scope(|cx| {
            let (selected, set_selected) = create_signal(cx, value.clone());
            
            let node = render_component(cx, || {
                view! { cx,
                    <SelectRoot value=selected>
                        <SelectItem value=value.clone()>{value.clone()}</SelectItem>
                    </SelectRoot>
                }
            });

            assert_eq!(selected.get(), value);
        });
    }

    #[test]
    fn dialog_focus_trap_with_n_elements(count in 1..100usize) {
        create_test_scope(|cx| {
            let buttons = (0..count).map(|i| {
                view! { cx, <button>{format!("Button {}", i)}</button> }
            }).collect_view(cx);

            let node = render_component(cx, || {
                view! { cx,
                    <DialogContent>
                        {buttons}
                    </DialogContent>
                }
            });

            let focusable = get_focusable_elements(&node);
            assert_eq!(focusable.len(), count);
        });
    }
}
```

## 8. Cross-Browser Testing Matrix

### 8.1 Browser Coverage

| Browser | Versions | Test Level |
|---------|----------|------------|
| Chrome | Latest, Latest-1 | Full |
| Firefox | Latest, Latest-1 | Full |
| Safari | Latest, Latest-1 | Full |
| Edge | Latest | Full |
| Mobile Chrome | Latest | E2E + Visual |
| Mobile Safari | Latest | E2E + Visual |

### 8.2 Assistive Technology Testing

| Technology | Platform | Components |
|------------|----------|------------|
| NVDA | Windows | All interactive |
| JAWS | Windows | Critical paths |
| VoiceOver | macOS/iOS | All interactive |
| TalkBack | Android | Critical paths |
| Dragon | Windows | Form components |

### 8.3 Platform-Specific Tests

```rust
#[cfg(target_os = "windows")]
#[wasm_bindgen_test]
fn windows_high_contrast_mode() {
    // Test high contrast mode compatibility
}

#[cfg(target_os = "macos")]
#[wasm_bindgen_test]
fn macos_voiceover_navigation() {
    // Test VoiceOver specific behavior
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn wasm_specific_behavior() {
    // Test WASM-specific functionality
}
```

## 9. Test Coverage Requirements

### 9.1 Coverage Targets

| Component Type | Line Coverage | Branch Coverage | Function Coverage |
|---------------|---------------|-----------------|-------------------|
| Core Utilities | 100% | 100% | 100% |
| Hooks | 95% | 90% | 100% |
| Simple Components | 90% | 85% | 95% |
| Complex Components | 85% | 80% | 90% |
| Examples | 70% | 60% | 75% |

### 9.2 Coverage Reporting

```bash
# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# Coverage with specific features
cargo tarpaulin --features "ssr hydrate" --out Xml

# Exclude examples from coverage
cargo tarpaulin --exclude-files "examples/*" --out Lcov
```

## 10. Continuous Testing

### 10.1 Pre-commit Hooks

```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: rust-test
        name: Run unit tests
        entry: cargo test --lib
        language: system
        pass_filenames: false
      
      - id: format-check
        name: Check formatting
        entry: cargo fmt -- --check
        language: system
        pass_filenames: false
      
      - id: clippy
        name: Clippy lints
        entry: cargo clippy -- -D warnings
        language: system
        pass_filenames: false
```

### 10.2 PR Testing Requirements

All PRs must pass:
- [ ] Unit tests (100% pass rate)
- [ ] Integration tests (100% pass rate)
- [ ] Coverage requirements met
- [ ] No accessibility violations
- [ ] Visual regression approved
- [ ] Performance benchmarks within threshold
- [ ] Documentation tests pass

### 10.3 Nightly Testing

```yaml
# .github/workflows/nightly.yml
name: Nightly Tests

on:
  schedule:
    - cron: '0 2 * * *'  # 2 AM UTC daily

jobs:
  full-test-suite:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta, nightly]
    
    steps:
      - uses: actions/checkout@v3
      - run: ./scripts/run-full-test-suite.sh
```

## 11. Test Debugging & Troubleshooting

### 11.1 Debug Utilities

```rust
// tests/debug/mod.rs
pub fn debug_component_tree(root: &Element) {
    console::log_1(&format!("Component tree:").into());
    print_tree(root, 0);
}

fn print_tree(element: &Element, depth: usize) {
    let indent = "  ".repeat(depth);
    let tag = element.tag_name().to_lowercase();
    let id = element.id();
    let classes = element.class_name();
    
    console::log_1(&format!(
        "{}{} id='{}' class='{}'",
        indent, tag, id, classes
    ).into());
    
    // Recursively print children
    let children = element.children();
    for i in 0..children.length() {
        if let Some(child) = children.item(i) {
            print_tree(&child, depth + 1);
        }
    }
}

pub fn pause_test(duration_ms: i32) {
    // Useful for debugging visual states
    let start = js_sys::Date::now();
    while js_sys::Date::now() - start < duration_ms as f64 {
        // Busy wait
    }
}
```

### 11.2 Test Failure Analysis

```rust
#[macro_export]
macro_rules! assert_eventually {
    ($cond:expr, $timeout_ms:expr) => {
        let start = instant::Instant::now();
        let timeout = Duration::from_millis($timeout_ms);
        
        while !$cond {
            if start.elapsed() > timeout {
                panic!(
                    "Condition not met within {} ms: {}",
                    $timeout_ms,
                    stringify!($cond)
                );
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    };
}
```

## 12. Test Documentation

### 12.1 Test Case Template

```markdown
## Test Case: [Component] - [Scenario]

**Test ID**: TC-[Component]-[Number]
**Priority**: High/Medium/Low
**Type**: Unit/Integration/E2E/Accessibility

### Description
Brief description of what is being tested

### Prerequisites
- Component state
- Required props
- Environment setup

### Test Steps
1. Step one
2. Step two
3. Step three

### Expected Results
- Result one
- Result two

### Actual Results
[Filled during test execution]

### Pass/Fail
[Pass/Fail status]

### Notes
Additional observations or issues
```

### 12.2 Test Reports

Generate comprehensive test reports:

```bash
#!/bin/bash
# scripts/generate-test-report.sh

echo "# Test Report - $(date)" > test-report.md
echo "" >> test-report.md

echo "## Coverage Summary" >> test-report.md
cargo tarpaulin --print-summary >> test-report.md

echo "" >> test-report.md
echo "## Test Results" >> test-report.md
cargo test --no-fail-fast 2>&1 | tee -a test-report.md

echo "" >> test-report.md
echo "## Accessibility Results" >> test-report.md
npm run test:a11y -- --reporter json > a11y-results.json
cat a11y-results.json | jq '.violations | length' >> test-report.md

echo "" >> test-report.md
echo "## Performance Metrics" >> test-report.md
cargo bench --bench benchmarks -- --save-baseline current
```

## 13. Testing Best Practices

### 13.1 Component Testing Checklist

For each component, ensure:

- [ ] **State Management**
  - [ ] Default state
  - [ ] Controlled mode
  - [ ] Uncontrolled mode
  - [ ] State transitions

- [ ] **User Interactions**
  - [ ] Mouse events
  - [ ] Keyboard events
  - [ ] Touch events
  - [ ] Focus management

- [ ] **Accessibility**
  - [ ] ARIA attributes
  - [ ] Keyboard navigation
  - [ ] Screen reader announcements
  - [ ] Color contrast

- [ ] **Edge Cases**
  - [ ] Empty states
  - [ ] Loading states
  - [ ] Error states
  - [ ] Boundary conditions

- [ ] **Performance**
  - [ ] Initial render time
  - [ ] Re-render efficiency
  - [ ] Memory usage
  - [ ] Bundle size impact

### 13.2 Test Writing Guidelines

1. **Descriptive Names**: Test names should clearly describe what is being tested
2. **Single Responsibility**: Each test should verify one behavior
3. **Arrange-Act-Assert**: Follow AAA pattern for test structure
4. **Independent Tests**: Tests should not depend on execution order
5. **Fast Execution**: Keep tests fast, mock expensive operations
6. **Deterministic**: Tests should produce same results every run
7. **Maintainable**: Use test utilities and helpers to reduce duplication

### 13.3 Common Testing Patterns

```rust
// Pattern: Testing async behavior
#[wasm_bindgen_test]
async fn async_data_loading() {
    create_test_scope(|cx| {
        let (data, set_data) = create_signal(cx, None);
        
        spawn_local(async move {
            let result = fetch_data().await;
            set_data.set(Some(result));
        });

        // Wait for data to load
        assert_eventually!(data.get().is_some(), 1000);
        
        assert_eq!(data.get().unwrap(), expected_data);
    });
}

// Pattern: Testing error boundaries
#[wasm_bindgen_test]
fn error_boundary_catches_panics() {
    create_test_scope(|cx| {
        let result = std::panic::catch_unwind(|| {
            render_component(cx, || {
                view! { cx,
                    <ErrorBoundary fallback=|_| view! { cx, "Error" }>
                        <PanickingComponent />
                    </ErrorBoundary>
                }
            })
        });

        assert!(result.is_ok());
        assert_text(".error-message", "Error");
    });
}
```

## 14. Performance Testing Thresholds

### 14.1 Performance Budgets

| Metric | Target | Warning | Error |
|--------|--------|---------|-------|
| First Render | <16ms | <33ms | >50ms |
| Re-render | <8ms | <16ms | >33ms |
| Memory (per component) | <1KB | <2KB | >5KB |
| Bundle size (per component) | <5KB | <10KB | >15KB |
| Time to Interactive | <100ms | <200ms | >500ms |

### 14.2 Performance Monitoring

```rust
#[wasm_bindgen_test]
fn measure_render_performance() {
    let mut render_times = Vec::new();
    
    for _ in 0..100 {
        let start = performance::now();
        
        create_test_scope(|cx| {
            render_component(cx, || {
                view! { cx, <ComplexComponent /> }
            });
        });
        
        let duration = performance::now() - start;
        render_times.push(duration);
    }

    let avg_time = render_times.iter().sum::<f64>() / render_times.len() as f64;
    let p95_time = percentile(&render_times, 95.0);
    
    assert!(avg_time < 16.0, "Average render time {} exceeds 16ms", avg_time);
    assert!(p95_time < 33.0, "P95 render time {} exceeds 33ms", p95_time);
}
```

## 15. Test Maintenance

### 15.1 Test Review Process

1. **Regular Audits**: Monthly review of test effectiveness
2. **Flaky Test Detection**: Automated detection and fixing
3. **Coverage Analysis**: Quarterly coverage review
4. **Performance Baseline Updates**: Update benchmarks with optimizations
5. **Dependency Updates**: Keep testing tools current

### 15.2 Test Refactoring Guidelines

When to refactor tests:
- Duplication exceeds 3 instances
- Test execution time exceeds targets
- False positives exceed 1% of runs
- New patterns emerge from implementation

## Conclusion

This comprehensive test strategy ensures radix-leptos delivers high-quality, accessible, and performant components. By implementing testing at multiple levels and automating as much as possible, we can maintain confidence in the codebase while enabling rapid development.

The strategy emphasizes:
- **Early detection** of issues through comprehensive unit testing
- **Real-world validation** through integration and E2E testing
- **Accessibility compliance** through automated and manual testing
- **Performance assurance** through continuous benchmarking
- **Quality maintenance** through automated CI/CD pipelines

Success metrics:
- 90%+ code coverage
- Zero accessibility violations
- <1% test flakiness
- <5 second average PR validation time
- 100% component test coverage
