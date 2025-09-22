# Test Implementation Plan

**Priority**: HIGH  
**Timeline**: Weeks 2-6  
**Goal**: Replace placeholder tests with real behavioral tests

## Current Test Quality Assessment

### Issues Identified
- **1,792+ claimed tests** are mostly empty placeholders
- **Compilation tests** use mock types instead of real components  
- **Property tests** have no assertions
- **WASM tests** won't run in standard `cargo test`
- **No DOM behavior testing** for interactive components
- **No accessibility compliance testing**

## Test Strategy Overhaul

### Test Categories (Priority Order)

1. **Unit Tests** - Pure function testing (Week 2)
2. **Component Tests** - Basic rendering and props (Week 3)  
3. **Integration Tests** - Component interactions (Week 4)
4. **Contract Tests** - API and accessibility contracts (Week 5)
5. **E2E Tests** - Full user workflows (Week 6)

## Week 2: Unit Test Foundation

### Target: Pure Functions and Utilities

#### Pagination Helpers
**File**: `crates/radix-leptos-primitives/src/components/pagination/helpers.rs`
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_page_range() {
        // Test boundary conditions
        assert_eq!(calculate_page_range(1, 10, 5), (1, 5));
        assert_eq!(calculate_page_range(5, 10, 5), (3, 7));
        assert_eq!(calculate_page_range(10, 10, 5), (6, 10));
    }

    #[test] 
    fn test_get_visible_page_numbers() {
        let result = get_visible_page_numbers(5, 100, 7);
        assert_eq!(result.len(), 7);
        assert!(result.contains(&5)); // Current page always visible
    }
}
```

#### CSS Variables Serialization  
**File**: `crates/radix-leptos-primitives/src/theming/css_variables.rs`
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_css_string() {
        let mut vars = CSSVariables::default();
        vars.set_color("primary", "#007bff");
        
        let css = vars.to_css_string();
        assert!(css.contains("--primary: #007bff"));
    }

    #[test]
    fn test_merge_variables() {
        let base = CSSVariables::default();
        let override_vars = CSSVariables::new_with_color("accent", "#28a745");
        
        let merged = base.merge(override_vars);
        assert_eq!(merged.get_color("accent"), Some("#28a745"));
    }
}
```

#### Utility Functions
**File**: `crates/radix-leptos-primitives/src/utils/mod.rs`  
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_classes() {
        assert_eq!(merge_classes(vec!["btn", "btn-primary"]), "btn btn-primary");
        assert_eq!(merge_classes(vec!["btn", "", "active"]), "btn active");
    }

    #[test]
    fn test_generate_id() {
        let id1 = generate_id("button");
        let id2 = generate_id("button");
        
        assert!(id1.starts_with("button-"));
        assert_ne!(id1, id2); // Should be unique
    }
}
```

## Week 3: Component Rendering Tests

### Use leptos-testing for DOM verification

#### Button Component Tests
**File**: `crates/radix-leptos-primitives/src/components/button/tests.rs`
```rust
use leptos::*;
use leptos_testing::*;
use super::*;

#[test]
fn test_button_renders_children() {
    let button = view! {
        <Button>"Click me"</Button>
    };
    
    let html = render_to_string(button);
    assert!(html.contains("Click me"));
    assert!(html.contains("<button"));
}

#[test] 
fn test_button_disabled_state() {
    let button = view! {
        <Button disabled=true>"Disabled"</Button>
    };
    
    let html = render_to_string(button);
    assert!(html.contains("disabled"));
    assert!(html.contains("aria-disabled=\"true\""));
}

#[test]
fn test_button_variants() {
    let primary_button = view! {
        <Button variant=ButtonVariant::Primary>"Primary"</Button>
    };
    
    let html = render_to_string(primary_button);
    assert!(html.contains("btn-primary"));
}
```

#### Checkbox Component Tests
**File**: `crates/radix-leptos-primitives/src/components/checkbox/tests.rs`
```rust
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn test_checkbox_click_toggles_state() {
    use web_sys::*;
    
    let (checked, set_checked) = signal(false);
    
    let checkbox = view! {
        <Checkbox 
            checked=checked
            on_checked_change=move |new_checked| set_checked(new_checked)
        />
    };
    
    mount_to_body(checkbox);
    
    let checkbox_element = document()
        .query_selector("input[type='checkbox']")
        .unwrap()
        .unwrap();
        
    checkbox_element.click();
    
    assert_eq!(checked.get(), true);
}
```

## Week 4: Integration Tests

### Component Interaction Testing

#### Tabs Integration
**File**: `crates/radix-leptos-primitives/src/components/tabs/integration_tests.rs`
```rust
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg(target_arch = "wasm32")]  
#[wasm_bindgen_test]
fn test_tabs_keyboard_navigation() {
    let tabs = view! {
        <Tabs>
            <TabsList>
                <TabsTrigger value="tab1">"Tab 1"</TabsTrigger>
                <TabsTrigger value="tab2">"Tab 2"</TabsTrigger>
            </TabsList>
            <TabsContent value="tab1">"Content 1"</TabsContent>
            <TabsContent value="tab2">"Content 2"</TabsContent>
        </Tabs>
    };
    
    mount_to_body(tabs);
    
    // Test arrow key navigation
    let first_tab = document()
        .query_selector("[role='tab']")
        .unwrap()
        .unwrap();
        
    first_tab.focus().unwrap();
    
    // Simulate arrow right key
    let event = KeyboardEvent::new_with_keyboard_event_init_dict(
        "keydown", 
        KeyboardEventInitDict::new().key("ArrowRight")
    ).unwrap();
    
    first_tab.dispatch_event(&event).unwrap();
    
    // Verify focus moved to second tab
    let active_element = document().active_element().unwrap();
    assert!(active_element.text_content().unwrap().contains("Tab 2"));
}
```

## Week 5: Contract Tests

### Accessibility Contract Enforcement

#### ARIA Compliance Tests
**File**: `crates/radix-leptos-primitives/src/accessibility/contract_tests.rs`
```rust
use super::*;

pub trait AccessibilityContract {
    fn required_roles() -> Vec<&'static str>;
    fn required_aria_attributes() -> Vec<&'static str>; 
    fn keyboard_interactions() -> Vec<KeyboardInteraction>;
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn test_button_accessibility_contract() {
    let button = view! {
        <Button disabled=true>"Test Button"</Button>
    };
    
    mount_to_body(button);
    
    let button_element = document()
        .query_selector("button")
        .unwrap()
        .unwrap();
    
    // Contract: Button must have role="button" (implicit for <button>)
    assert_eq!(button_element.tag_name(), "BUTTON");
    
    // Contract: Disabled buttons must have aria-disabled
    assert_eq!(
        button_element.get_attribute("aria-disabled").unwrap(),
        "true"
    );
    
    // Contract: Must be focusable when enabled, not when disabled
    assert_eq!(button_element.get_attribute("tabindex"), Some("-1".to_string()));
}

#[cfg(target_arch = "wasm32")]  
#[wasm_bindgen_test]
fn test_accordion_accessibility_contract() {
    let accordion = view! {
        <Accordion>
            <AccordionItem value="item1">
                <AccordionTrigger>"Section 1"</AccordionTrigger>
                <AccordionContent>"Content 1"</AccordionContent>
            </AccordionItem>
        </Accordion>
    };
    
    mount_to_body(accordion);
    
    let trigger = document()
        .query_selector("[role='button']")
        .unwrap()
        .unwrap();
        
    let content = document()
        .query_selector("[role='region']")
        .unwrap()
        .unwrap();
    
    // Contract: aria-controls must link trigger to content
    let controls_id = trigger.get_attribute("aria-controls").unwrap();
    let content_id = content.get_attribute("id").unwrap();
    assert_eq!(controls_id, content_id);
    
    // Contract: aria-expanded must reflect state
    let expanded = trigger.get_attribute("aria-expanded").unwrap();
    let content_hidden = content.get_attribute("aria-hidden").unwrap();
    
    if expanded == "true" {
        assert_eq!(content_hidden, "false");
    } else {
        assert_eq!(content_hidden, "true");
    }
}
```

## Week 6: E2E Tests with Playwright

### User Workflow Tests
**File**: `tests/e2e/user_workflows.spec.ts`
```typescript
import { test, expect } from '@playwright/test';

test('complete form workflow', async ({ page }) => {
  await page.goto('/examples/form');
  
  // Fill out form
  await page.fill('[data-testid="name-input"]', 'John Doe');
  await page.fill('[data-testid="email-input"]', 'john@example.com');
  await page.check('[data-testid="terms-checkbox"]');
  
  // Submit form
  await page.click('[data-testid="submit-button"]');
  
  // Verify success message
  await expect(page.locator('[data-testid="success-message"]')).toBeVisible();
});

test('pagination navigation', async ({ page }) => {
  await page.goto('/examples/pagination');
  
  // Test page navigation
  await page.click('[aria-label="Go to page 2"]');
  await expect(page.locator('[aria-current="page"]')).toHaveText('2');
  
  // Test previous/next buttons
  await page.click('[aria-label="Go to next page"]');
  await expect(page.locator('[aria-current="page"]')).toHaveText('3');
  
  await page.click('[aria-label="Go to previous page"]');
  await expect(page.locator('[aria-current="page"]')).toHaveText('2');
});
```

## Test Infrastructure Setup

### Testing Dependencies
```toml
[dev-dependencies]
leptos-testing = "0.1"
wasm-bindgen-test = "0.3"
web-sys = "0.3"
proptest = "1.0" 
criterion = "0.5" # For performance tests
```

### Test Configuration
**File**: `.cargo/config.toml`
```toml
[target.wasm32-unknown-unknown]
runner = 'wasm-bindgen-test-runner'

[target.'cfg(target_arch = "wasm32")']
rustflags = ['--cfg', 'web_sys_unstable_apis']
```

### CI Integration
**File**: `.github/workflows/tests.yml`
```yaml
- name: Run Unit Tests
  run: cargo test --workspace --lib

- name: Run WASM Tests  
  run: |
    wasm-pack test --node --all-features
    
- name: Run E2E Tests
  run: |
    npm run build
    npx playwright test
```

## Success Metrics

- [ ] >90% line coverage on core components
- [ ] All accessibility contracts enforced with tests
- [ ] Zero placeholder or empty tests
- [ ] All DOM behavior verified with actual interaction tests  
- [ ] E2E tests cover main user workflows
- [ ] Performance regression tests in place
- [ ] Test suite runs in <2 minutes locally
- [ ] All tests pass in CI consistently

## Migration Strategy

1. **Keep existing tests running** during migration
2. **Add new tests alongside** old ones initially
3. **Remove placeholder tests** only after real tests added
4. **Validate test quality** with mutation testing
5. **Document test patterns** for consistency across components
