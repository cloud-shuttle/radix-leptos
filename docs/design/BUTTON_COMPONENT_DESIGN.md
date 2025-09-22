# Button Component Design

**Status**: Tier 1 - Minimally Viable (needs fixes)  
**Priority**: HIGH - Core component  
**File**: `crates/radix-leptos-primitives/src/components/button.rs`  
**Current Lines**: ~280 lines (under limit ✅)

## Current State Analysis

### ✅ What Works
- Basic component structure with proper props
- Variant system (Primary, Secondary, Outline, Ghost, Link)
- Size variants (Small, Medium, Large)
- Disabled and loading states
- ARIA attributes properly set
- Click handler support

### ❌ Critical Issues  
1. **Missing Children Rendering**: `{children()}` not included in view
2. **Unused Loading State**: `loading` prop defined but spinner not shown
3. **Incomplete ARIA**: Missing `aria-label` when no text content

## API Design

### Props Interface
```rust
#[component]
pub fn Button(
    // Content
    children: Children,
    #[prop(optional)] aria_label: Option<String>,
    
    // Behavior  
    #[prop(optional)] on_click: Option<Callback<web_sys::MouseEvent>>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] loading: Option<bool>,
    #[prop(optional)] button_type: Option<ButtonType>, // "button" | "submit" | "reset"
    
    // Styling
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView
```

### Type Definitions
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum ButtonVariant {
    Primary,    // Solid primary color
    Secondary,  // Solid secondary color  
    Outline,    // Bordered, transparent background
    Ghost,      // No background, minimal styling
    Link,       // Text styling, no borders/background
}

#[derive(Clone, Debug, PartialEq)]
pub enum ButtonSize {
    Small,      // Compact padding, smaller text
    Medium,     // Default size
    Large,      // Increased padding, larger text
}

#[derive(Clone, Debug, PartialEq)]
pub enum ButtonType {
    Button,     // Default behavior
    Submit,     // Form submission
    Reset,      // Form reset
}
```

## Component Implementation

### Fixed Component Structure
```rust
#[component]
pub fn Button(
    children: Children,
    #[prop(optional)] on_click: Option<Callback<web_sys::MouseEvent>>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] loading: Option<bool>,
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    #[prop(optional)] button_type: Option<ButtonType>,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    // Derive computed values
    let disabled = disabled.unwrap_or(false);
    let loading = loading.unwrap_or(false);
    let variant = variant.unwrap_or(ButtonVariant::Primary);
    let size = size.unwrap_or(ButtonSize::Medium);
    let button_type = button_type.unwrap_or(ButtonType::Button);
    
    // Combine disabled and loading for interaction prevention
    let interactive_disabled = disabled || loading;
    
    // Build CSS classes
    let class_list = vec![
        "btn".to_string(),
        format!("btn-{}", variant.as_str()),
        format!("btn-{}", size.as_str()),
        if loading { "btn-loading".to_string() } else { String::new() },
        class.unwrap_or_default(),
    ];
    let class = merge_classes(class_list);
    
    // Event handler that respects disabled/loading state
    let handle_click = move |event: web_sys::MouseEvent| {
        if !interactive_disabled {
            if let Some(on_click) = on_click.as_ref() {
                on_click.call(event);
            }
        } else {
            event.prevent_default();
            event.stop_propagation();
        }
    };
    
    view! {
        <button
            type={button_type.as_str()}
            disabled={interactive_disabled}
            aria-disabled={interactive_disabled.to_string()}
            aria-label={aria_label}
            class={class}
            style={style.unwrap_or_default()}
            on:click=handle_click
        >
            // Loading spinner (if loading)
            {if loading {
                view! { 
                    <span class="btn-spinner" aria-hidden="true"></span>
                }.into()
            } else {
                ().into()
            }}
            
            // CRITICAL FIX: Render children
            {children()}
        </button>
    }
}
```

### Variant Implementation  
```rust
impl ButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "primary",
            ButtonVariant::Secondary => "secondary", 
            ButtonVariant::Outline => "outline",
            ButtonVariant::Ghost => "ghost",
            ButtonVariant::Link => "link",
        }
    }
}

impl ButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonSize::Small => "sm",
            ButtonSize::Medium => "md", 
            ButtonSize::Large => "lg",
        }
    }
}

impl ButtonType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonType::Button => "button",
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset", 
        }
    }
}
```

## CSS Requirements

### Base Button Styles
```css
.btn {
  /* Reset */
  border: none;
  background: none;
  font: inherit;
  cursor: pointer;
  
  /* Base styling */  
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  font-weight: 500;
  text-decoration: none;
  border-radius: var(--radius);
  transition: all 0.2s ease;
  
  /* Focus styling */
  &:focus-visible {
    outline: 2px solid var(--focus-color);
    outline-offset: 2px;
  }
  
  /* Disabled styling */
  &:disabled,
  &[aria-disabled="true"] {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
  }
}

/* Size variants */
.btn-sm {
  height: 2rem;
  padding: 0 0.75rem;
  font-size: 0.875rem;
}

.btn-md {
  height: 2.5rem; 
  padding: 0 1rem;
  font-size: 1rem;
}

.btn-lg {
  height: 3rem;
  padding: 0 1.5rem;
  font-size: 1.125rem;
}

/* Style variants */
.btn-primary {
  background-color: var(--primary);
  color: var(--primary-foreground);
  
  &:hover:not(:disabled) {
    background-color: var(--primary-hover);
  }
  
  &:active:not(:disabled) {
    background-color: var(--primary-active);
  }
}

.btn-outline {
  border: 1px solid var(--border);
  color: var(--foreground);
  
  &:hover:not(:disabled) {
    background-color: var(--accent);
    color: var(--accent-foreground);
  }
}

/* Loading state */
.btn-loading {
  position: relative;
  color: transparent;
}

.btn-spinner {
  position: absolute;
  width: 1rem;
  height: 1rem;
  border: 2px solid currentColor;
  border-right-color: transparent;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
```

## Usage Examples

### Basic Usage
```rust
view! {
    <Button on_click=move |_| console::log("Clicked!")>
        "Click me"
    </Button>
}
```

### With Variants
```rust
view! {
    <Button variant=ButtonVariant::Outline size=ButtonSize::Large>
        "Large Outline Button"  
    </Button>
}
```

### Loading State
```rust
let (loading, set_loading) = signal(false);

let handle_click = move |_| {
    set_loading(true);
    // Async operation...
    spawn_local(async move {
        TimeoutFuture::new(2000).await;
        set_loading(false);
    });
};

view! {
    <Button 
        loading=loading 
        on_click=handle_click
    >
        "Submit Form"
    </Button>
}
```

### Form Integration
```rust
view! {
    <form on:submit=handle_submit>
        <Button button_type=ButtonType::Submit>
            "Submit"
        </Button>
        <Button 
            button_type=ButtonType::Reset 
            variant=ButtonVariant::Ghost
        >
            "Reset"
        </Button>
    </form>
}
```

## Accessibility Requirements

### ARIA Attributes
- `aria-disabled="true"` when disabled or loading
- `aria-label` when button has no text content (icon-only)
- `aria-describedby` for additional context if needed

### Keyboard Support  
- **Enter/Space**: Activate button (native behavior)
- **Tab**: Move focus to/from button
- **Disabled state**: Not focusable (`tabindex="-1"`)

### Screen Reader Support
- Button text must be descriptive
- Loading state announced as "loading" 
- Disabled state announced as "disabled"

## Testing Requirements

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos_testing::*;

    #[test]
    fn renders_children() {
        let button = view! { <Button>"Test"</Button> };
        let html = render_to_string(button);
        assert!(html.contains("Test"));
    }

    #[test] 
    fn applies_variant_classes() {
        let button = view! { 
            <Button variant=ButtonVariant::Outline>"Test"</Button> 
        };
        let html = render_to_string(button);
        assert!(html.contains("btn-outline"));
    }
    
    #[test]
    fn disabled_state() {
        let button = view! { 
            <Button disabled=true>"Test"</Button> 
        };
        let html = render_to_string(button);
        assert!(html.contains("disabled"));
        assert!(html.contains("aria-disabled=\"true\""));
    }
}
```

### Integration Tests
```rust
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn click_handler_called() {
    let (clicked, set_clicked) = signal(false);
    
    let button = view! {
        <Button on_click=move |_| set_clicked(true)>
            "Click me"
        </Button>
    };
    
    mount_to_body(button);
    
    let button_el = document()
        .query_selector("button")
        .unwrap()
        .unwrap();
        
    button_el.click();
    assert!(clicked.get());
}
```

## Implementation Priority

### Phase 1: Critical Fixes (Day 1)
1. ✅ Add `{children()}` to render function
2. ✅ Fix loading spinner display
3. ✅ Test basic rendering

### Phase 2: Polish (Week 1)  
1. ✅ Implement all variant styles
2. ✅ Add comprehensive tests
3. ✅ Verify accessibility compliance

### Phase 3: Enhancement (Week 2)
1. ✅ Add icon support
2. ✅ Implement button groups
3. ✅ Add animation transitions

## Success Criteria
- [ ] Children render correctly in all cases
- [ ] All variants display proper styling
- [ ] Loading state shows spinner and prevents interaction
- [ ] Disabled state prevents clicks and shows proper styling
- [ ] ARIA attributes correctly reflect component state  
- [ ] Keyboard navigation works as expected
- [ ] All tests pass with >95% coverage
