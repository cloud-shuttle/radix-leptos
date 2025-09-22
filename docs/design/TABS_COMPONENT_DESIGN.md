# Tabs Component Design

**Status**: Tier 2 - Stub Code (requires implementation)  
**Priority**: HIGH - Complex interactive component  
**File**: `crates/radix-leptos-primitives/src/components/tabs.rs`  
**Current Lines**: ~530 lines (needs split)

## Current State Analysis

### ❌ Critical Issues
1. **Handlers Not Wired**: `handle_click` and `handle_keydown` defined but not attached to DOM
2. **No State Management**: Tab selection not implemented
3. **Static ARIA**: `aria-controls` built but not connected to actual state changes
4. **Incomplete Navigation**: Arrow key navigation defined but not functional
5. **Missing Content Switching**: Tab panels don't show/hide based on selection

### ⚠️ Partial Implementation
- Component structure exists
- ARIA attributes partially defined
- Event handlers defined (but unused)
- Basic CSS classes applied

## Architecture Design

### Component Hierarchy
```
Tabs (Root)
├── TabsList (Container for triggers)
│   └── TabsTrigger (Individual tab buttons)
└── TabsContent (Individual tab panels)
```

### State Management Pattern
```rust
// Controlled Pattern (recommended)
let (selected_tab, set_selected_tab) = signal("tab1".to_string());

view! {
    <Tabs value=selected_tab on_value_change=move |new_tab| set_selected_tab(new_tab)>
        // ... tabs content
    </Tabs>
}

// Uncontrolled Pattern (with default)
view! {
    <Tabs default_value="tab1">
        // ... tabs content  
    </Tabs>
}
```

## API Design

### Root Component
```rust
#[component]
pub fn Tabs(
    children: Children,
    
    // State management (controlled)
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] on_value_change: Option<Callback<String>>,
    
    // State management (uncontrolled) 
    #[prop(optional)] default_value: Option<String>,
    
    // Behavior
    #[prop(optional)] orientation: Option<TabsOrientation>, // horizontal | vertical
    #[prop(optional)] activation_mode: Option<TabsActivation>, // automatic | manual
    
    // Styling
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView
```

### TabsList Component  
```rust
#[component]
pub fn TabsList(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView
```

### TabsTrigger Component
```rust
#[component]
pub fn TabsTrigger(
    children: Children,
    value: String, // Required: identifies this tab
    
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView
```

### TabsContent Component
```rust
#[component]  
pub fn TabsContent(
    children: Children,
    value: String, // Required: identifies this panel
    
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView
```

## Implementation Plan

### Phase 1: Context and State Management
```rust
// Context for sharing state between components
#[derive(Clone, Debug)]
pub struct TabsContext {
    pub selected_value: RwSignal<String>,
    pub on_value_change: Option<Callback<String>>,
    pub orientation: TabsOrientation,
    pub activation_mode: TabsActivation,
}

#[component]
pub fn Tabs(/* props */) -> impl IntoView {
    // Determine if controlled or uncontrolled
    let selected_value = if let Some(value) = value {
        // Controlled: use external signal
        create_rw_signal(value)
    } else {
        // Uncontrolled: manage internally
        create_rw_signal(default_value.unwrap_or_default())
    };
    
    // Create context
    let tabs_context = TabsContext {
        selected_value,
        on_value_change,
        orientation: orientation.unwrap_or(TabsOrientation::Horizontal),
        activation_mode: activation_mode.unwrap_or(TabsActivation::Automatic),
    };
    
    provide_context(tabs_context);
    
    view! {
        <div 
            class={merge_classes(vec!["tabs", class.unwrap_or_default()])}
            style={style.unwrap_or_default()}
            data-orientation={tabs_context.orientation.as_str()}
        >
            {children()}
        </div>
    }
}
```

### Phase 2: TabsTrigger with Working Handlers
```rust
#[component]
pub fn TabsTrigger(/* props */) -> impl IntoView {
    let tabs_context = expect_context::<TabsContext>();
    let disabled = disabled.unwrap_or(false);
    
    // Derive selected state
    let selected = move || tabs_context.selected_value.get() == value;
    
    // Generate IDs for ARIA linking
    let trigger_id = format!("tabs-trigger-{}", value);
    let panel_id = format!("tabs-content-{}", value);
    
    // Click handler - ACTUALLY WIRE THIS
    let handle_click = {
        let value = value.clone();
        let tabs_context = tabs_context.clone();
        move |_: web_sys::MouseEvent| {
            if !disabled {
                tabs_context.selected_value.set(value.clone());
                
                // Call external handler if provided
                if let Some(on_change) = tabs_context.on_value_change.as_ref() {
                    on_change.call(value.clone());
                }
            }
        }
    };
    
    // Keyboard handler - ACTUALLY WIRE THIS
    let handle_keydown = {
        let tabs_context = tabs_context.clone();
        move |event: web_sys::KeyboardEvent| {
            if disabled { return; }
            
            let key = event.key();
            match key.as_str() {
                "Enter" | " " => {
                    event.prevent_default();
                    tabs_context.selected_value.set(value.clone());
                    if let Some(on_change) = tabs_context.on_value_change.as_ref() {
                        on_change.call(value.clone());
                    }
                }
                "ArrowRight" | "ArrowDown" => {
                    event.prevent_default();
                    focus_next_tab();
                }
                "ArrowLeft" | "ArrowUp" => {
                    event.prevent_default(); 
                    focus_previous_tab();
                }
                "Home" => {
                    event.prevent_default();
                    focus_first_tab();
                }
                "End" => {
                    event.prevent_default();
                    focus_last_tab();
                }
                _ => {}
            }
        }
    };
    
    view! {
        <button
            type="button"
            role="tab"
            id={trigger_id}
            aria-controls={panel_id}
            aria-selected={selected()}
            disabled={disabled}
            tabindex={if selected() || (!selected() && !has_selected_tab()) { "0" } else { "-1" }}
            class={merge_classes(vec![
                "tabs-trigger",
                if selected() { "tabs-trigger-selected" } else { "" },
                class.unwrap_or_default()
            ])}
            style={style.unwrap_or_default()}
            // CRITICAL: Actually wire the handlers
            on:click=handle_click
            on:keydown=handle_keydown
        >
            {children()}
        </button>
    }
}
```

### Phase 3: TabsContent with Show/Hide Logic
```rust
#[component]
pub fn TabsContent(/* props */) -> impl IntoView {
    let tabs_context = expect_context::<TabsContext>();
    
    // Derive visibility state
    let visible = move || tabs_context.selected_value.get() == value;
    
    let trigger_id = format!("tabs-trigger-{}", value);
    let panel_id = format!("tabs-content-{}", value);
    
    view! {
        <div
            role="tabpanel"
            id={panel_id}
            aria-labelledby={trigger_id}
            hidden={!visible()}
            tabindex="0"
            class={merge_classes(vec![
                "tabs-content",
                if visible() { "tabs-content-active" } else { "tabs-content-inactive" },
                class.unwrap_or_default()
            ])}
            style={style.unwrap_or_default()}
        >
            // Only render content when visible (performance optimization)
            {if visible() {
                children().into()
            } else {
                ().into()
            }}
        </div>
    }
}
```

### Phase 4: Keyboard Navigation Helpers
```rust
// Helper functions for keyboard navigation
fn focus_next_tab() {
    if let Some(current) = get_current_trigger() {
        if let Some(next) = get_next_sibling_trigger(current) {
            focus_trigger(next);
        } else {
            // Wrap to first tab
            if let Some(first) = get_first_trigger() {
                focus_trigger(first);
            }
        }
    }
}

fn focus_previous_tab() {
    if let Some(current) = get_current_trigger() {
        if let Some(prev) = get_previous_sibling_trigger(current) {
            focus_trigger(prev);
        } else {
            // Wrap to last tab
            if let Some(last) = get_last_trigger() {
                focus_trigger(last);
            }
        }
    }
}

fn get_current_trigger() -> Option<web_sys::Element> {
    document().active_element()
        .filter(|el| el.get_attribute("role") == Some("tab".to_string()))
}

fn focus_trigger(element: web_sys::Element) {
    let _ = element.focus();
    
    // If automatic activation, also select the tab
    let tabs_context = expect_context::<TabsContext>();
    if tabs_context.activation_mode == TabsActivation::Automatic {
        if let Some(value) = element.get_attribute("data-value") {
            tabs_context.selected_value.set(value.clone());
            if let Some(on_change) = tabs_context.on_value_change.as_ref() {
                on_change.call(value);
            }
        }
    }
}
```

## CSS Requirements

### Base Styles
```css
.tabs {
  display: flex;
  flex-direction: column;
}

.tabs[data-orientation="vertical"] {
  flex-direction: row;
}

.tabs-list {
  display: flex;
  align-items: center;
  border-bottom: 1px solid var(--border);
}

.tabs[data-orientation="vertical"] .tabs-list {
  flex-direction: column;
  border-bottom: none;
  border-right: 1px solid var(--border);
}

.tabs-trigger {
  background: transparent;
  border: none;
  padding: 0.75rem 1rem;
  font-weight: 500;
  color: var(--muted-foreground);
  cursor: pointer;
  position: relative;
  transition: all 0.2s ease;
  
  &:hover:not(:disabled) {
    color: var(--foreground);
    background-color: var(--muted);
  }
  
  &:focus-visible {
    outline: 2px solid var(--focus-color);
    outline-offset: 2px;
  }
  
  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

.tabs-trigger-selected {
  color: var(--foreground);
  
  &::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 2px;
    background-color: var(--primary);
  }
}

.tabs[data-orientation="vertical"] .tabs-trigger-selected::after {
  top: 0;
  bottom: 0;
  left: auto;
  right: 0;
  width: 2px;
  height: auto;
}

.tabs-content {
  padding: 1rem 0;
  
  &:focus-visible {
    outline: 2px solid var(--focus-color);
    outline-offset: 2px;
  }
}

.tabs-content-inactive {
  display: none;
}
```

## Usage Examples

### Basic Tabs
```rust
view! {
    <Tabs default_value="overview">
        <TabsList>
            <TabsTrigger value="overview">"Overview"</TabsTrigger>
            <TabsTrigger value="analytics">"Analytics"</TabsTrigger>
            <TabsTrigger value="reports">"Reports"</TabsTrigger>
        </TabsList>
        
        <TabsContent value="overview">
            <h2>"Overview Content"</h2>
            <p>"This is the overview panel."</p>
        </TabsContent>
        
        <TabsContent value="analytics">
            <h2>"Analytics Content"</h2>
            <p>"View your analytics here."</p>
        </TabsContent>
        
        <TabsContent value="reports">
            <h2>"Reports Content"</h2>
            <p>"Generate and view reports."</p>
        </TabsContent>
    </Tabs>
}
```

### Controlled Tabs
```rust
let (selected_tab, set_selected_tab) = signal("tab1".to_string());

// Log tab changes
let handle_tab_change = move |new_tab: String| {
    console::log(&format!("Switched to tab: {}", new_tab));
    set_selected_tab(new_tab);
};

view! {
    <Tabs 
        value=selected_tab 
        on_value_change=handle_tab_change
    >
        <TabsList>
            <TabsTrigger value="tab1">"Tab 1"</TabsTrigger>
            <TabsTrigger value="tab2">"Tab 2"</TabsTrigger>
        </TabsList>
        
        <TabsContent value="tab1">"Content 1"</TabsContent>
        <TabsContent value="tab2">"Content 2"</TabsContent>
    </Tabs>
}
```

### Vertical Orientation
```rust
view! {
    <Tabs 
        default_value="account" 
        orientation=TabsOrientation::Vertical
        class="w-96"
    >
        <TabsList class="grid w-full grid-cols-1">
            <TabsTrigger value="account">"Account"</TabsTrigger>
            <TabsTrigger value="password">"Password"</TabsTrigger>
            <TabsTrigger value="notifications">"Notifications"</TabsTrigger>
        </TabsList>
        
        <TabsContent value="account">
            // Account settings form
        </TabsContent>
        
        <TabsContent value="password">
            // Password change form
        </TabsContent>
        
        <TabsContent value="notifications">
            // Notification preferences
        </TabsContent>
    </Tabs>
}
```

## Accessibility Requirements

### ARIA Implementation
- **Tabs container**: No specific role (div is fine)
- **TabsList**: `role="tablist"`
- **TabsTrigger**: `role="tab"`, `aria-selected`, `aria-controls`, proper `tabindex`
- **TabsContent**: `role="tabpanel"`, `aria-labelledby`

### Keyboard Navigation
- **Tab**: Move focus between tab triggers and content
- **Arrow Keys**: Navigate between tab triggers
- **Enter/Space**: Activate focused tab trigger  
- **Home/End**: Move to first/last tab trigger

### Focus Management
- Only selected tab or first tab should be focusable (`tabindex="0"`)
- Other tabs should have `tabindex="-1"`
- Content panels should be focusable (`tabindex="0"`)

## Testing Requirements

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_basic_structure() {
        let tabs = view! {
            <Tabs default_value="tab1">
                <TabsList>
                    <TabsTrigger value="tab1">"Tab 1"</TabsTrigger>
                </TabsList>
                <TabsContent value="tab1">"Content 1"</TabsContent>
            </Tabs>
        };
        
        let html = render_to_string(tabs);
        assert!(html.contains("role=\"tablist\""));
        assert!(html.contains("role=\"tab\""));
        assert!(html.contains("role=\"tabpanel\""));
    }

    #[test]
    fn default_tab_selected() {
        let tabs = view! {
            <Tabs default_value="tab2">
                <TabsList>
                    <TabsTrigger value="tab1">"Tab 1"</TabsTrigger>
                    <TabsTrigger value="tab2">"Tab 2"</TabsTrigger>
                </TabsList>
                <TabsContent value="tab1">"Content 1"</TabsContent>
                <TabsContent value="tab2">"Content 2"</TabsContent>
            </Tabs>
        };
        
        let html = render_to_string(tabs);
        // Should show tab2 as selected and content2 as visible
        assert!(html.contains("aria-selected=\"true\""));
    }
}
```

### Integration Tests
```rust
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test] 
fn test_tab_click_switches_content() {
    let tabs = view! {
        <Tabs default_value="tab1">
            <TabsList>
                <TabsTrigger value="tab1">"Tab 1"</TabsTrigger>
                <TabsTrigger value="tab2">"Tab 2"</TabsTrigger>
            </TabsList>
            <TabsContent value="tab1">"Content 1"</TabsContent>
            <TabsContent value="tab2">"Content 2"</TabsContent>
        </Tabs>
    };
    
    mount_to_body(tabs);
    
    // Initially tab1 should be selected
    let tab1 = document().query_selector("[value='tab1']").unwrap().unwrap();
    assert_eq!(tab1.get_attribute("aria-selected"), Some("true".to_string()));
    
    // Click tab2
    let tab2 = document().query_selector("[value='tab2']").unwrap().unwrap();
    tab2.click();
    
    // tab2 should now be selected
    assert_eq!(tab2.get_attribute("aria-selected"), Some("true".to_string()));
    assert_eq!(tab1.get_attribute("aria-selected"), Some("false".to_string()));
}
```

## Success Criteria

- [ ] Tab selection works via click and keyboard
- [ ] Arrow key navigation cycles through tabs
- [ ] Only one tab panel visible at a time
- [ ] ARIA attributes correctly reflect current state
- [ ] Focus management follows accessibility guidelines
- [ ] Controlled and uncontrolled patterns both work
- [ ] Vertical orientation displays correctly
- [ ] All accessibility tests pass
- [ ] Performance is good (content only renders when visible)
