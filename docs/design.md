# Radix UI to Leptos Port - Design Document

## Executive Summary

This document outlines the design and architecture for porting Radix UI primitives to Rust using the Leptos web framework. The goal is to create `radix-leptos`, a collection of low-level, unstyled, accessible UI components that maintain the philosophy and quality of Radix UI while leveraging Rust's type safety and Leptos's fine-grained reactivity.

## 1. Project Overview

### Goals
- **Accessibility First**: Maintain WAI-ARIA compliance for all components
- **Unstyled by Design**: Provide behavioral primitives without imposing visual styles
- **Type Safety**: Leverage Rust's type system for compile-time guarantees
- **Performance**: Utilize Leptos's fine-grained reactivity for optimal updates
- **Developer Experience**: Provide intuitive APIs with comprehensive documentation
- **Composability**: Enable flexible component composition patterns

### Non-Goals
- Styled components (leave styling to consumers)
- Animation libraries (beyond basic transition support)
- Form validation logic (focus on primitive behaviors)

## 2. Architecture

### 2.1 Layer Architecture

```
┌─────────────────────────────────────┐
│         Application Layer           │
│    (Consumer applications)          │
└─────────────────────────────────────┘
                  ↑
┌─────────────────────────────────────┐
│      Component Primitives           │
│  (Dialog, Select, Accordion, etc.)  │
└─────────────────────────────────────┘
                  ↑
┌─────────────────────────────────────┐
│        Core Primitives              │
│  (Portal, Slot, VisuallyHidden)     │
└─────────────────────────────────────┘
                  ↑
┌─────────────────────────────────────┐
│         Utilities Layer             │
│  (Hooks, State Management, A11y)    │
└─────────────────────────────────────┘
                  ↑
┌─────────────────────────────────────┐
│         Leptos Runtime              │
│    (Signals, Effects, Components)   │
└─────────────────────────────────────┘
```

### 2.2 Module Structure

```
radix-leptos/
├── src/
│   ├── lib.rs                 # Main library entry
│   ├── primitives/
│   │   ├── mod.rs
│   │   ├── core/
│   │   │   ├── portal.rs      # Portal primitive
│   │   │   ├── slot.rs        # Slot composition
│   │   │   ├── visually_hidden.rs
│   │   │   └── presence.rs    # Animation presence
│   │   └── components/
│   │       ├── accordion/
│   │       │   ├── mod.rs
│   │       │   ├── root.rs
│   │       │   ├── item.rs
│   │       │   ├── trigger.rs
│   │       │   └── content.rs
│   │       ├── dialog/
│   │       ├── select/
│   │       ├── popover/
│   │       └── ...
│   ├── hooks/
│   │   ├── mod.rs
│   │   ├── use_controllable_state.rs
│   │   ├── use_focus_trap.rs
│   │   ├── use_escape_key.rs
│   │   └── use_outside_click.rs
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── accessibility.rs
│   │   ├── dom.rs
│   │   └── events.rs
│   └── context/
│       ├── mod.rs
│       └── collection.rs       # Collection context for lists
├── examples/
├── tests/
└── Cargo.toml
```

## 3. Core Design Patterns

### 3.1 Component Composition Pattern

Following Radix's compound component pattern adapted for Rust:

```rust
use leptos::*;
use radix_leptos::accordion::*;

#[component]
pub fn MyAccordion() -> impl IntoView {
    view! {
        <AccordionRoot type_="single" collapsible=true>
            <AccordionItem value="item-1">
                <AccordionTrigger>"Is it accessible?"</AccordionTrigger>
                <AccordionContent>
                    "Yes. It adheres to WAI-ARIA patterns."
                </AccordionContent>
            </AccordionItem>
        </AccordionRoot>
    }
}
```

### 3.2 Slot Pattern (as_child equivalent)

Implementing Radix's `asChild` pattern using Rust traits:

```rust
pub trait Slottable: IntoView {
    fn slot(self, props: SlotProps) -> impl IntoView;
}

#[component]
pub fn DialogTrigger(
    #[prop(optional)] as_child: bool,
    children: Children,
) -> impl IntoView {
    if as_child {
        // Merge props with child element
        children().slot(trigger_props)
    } else {
        view! {
            <button {...trigger_props}>
                {children()}
            </button>
        }
    }
}
```

### 3.3 Controllable State Pattern

Supporting both controlled and uncontrolled modes:

```rust
#[derive(Clone)]
pub struct UseControllableStateReturn<T> {
    pub value: Signal<T>,
    pub set_value: WriteSignal<T>,
}

pub fn use_controllable_state<T>(
    prop: Option<Signal<T>>,
    default_prop: Option<T>,
    on_change: Option<Box<dyn Fn(T)>>,
) -> UseControllableStateReturn<T> 
where 
    T: Clone + 'static
{
    // Implementation handling both controlled and uncontrolled cases
}
```

### 3.4 Context Pattern

Using Leptos's context API for component communication:

```rust
#[derive(Clone)]
struct AccordionContext {
    type_: AccordionType,
    value: RwSignal<AccordionValue>,
    collapsible: bool,
    disabled: bool,
}

#[component]
pub fn AccordionRoot(
    #[prop(default = AccordionType::Single)]
    type_: AccordionType,
    children: Children,
) -> impl IntoView {
    let ctx = AccordionContext { /* ... */ };
    
    provide_context(ctx);
    
    view! {
        <div data-radix-accordion="" role="region">
            {children()}
        </div>
    }
}
```

## 4. Component Implementation Strategy

### 4.1 Priority Components (Phase 1)

1. **Core Primitives**
   - Portal
   - Slot
   - VisuallyHidden
   - Presence (for animations)

2. **Form Components**
   - Label
   - Button
   - Checkbox
   - RadioGroup
   - Switch

3. **Overlay Components**
   - Dialog
   - Popover
   - Tooltip
   - DropdownMenu

### 4.2 Advanced Components (Phase 2)

- Accordion
- Select
- Tabs
- ScrollArea
- Slider
- Progress
- AlertDialog
- ContextMenu
- NavigationMenu

### 4.3 Complex Components (Phase 3)

- Combobox
- DatePicker
- Toast
- HoverCard
- Menubar
- Toolbar

## 5. State Management

### 5.1 Signal-Based State

Leveraging Leptos's signals for reactive state:

```rust
#[component]
pub fn Select(
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] default_value: Option<String>,
    #[prop(optional)] on_value_change: Option<Box<dyn Fn(String)>>,
) -> impl IntoView {
    let state = use_controllable_state(value, default_value, on_value_change);
    // Component implementation
}
```

### 5.2 Collection Management

For components with dynamic lists:

```rust
pub struct CollectionItem<T> {
    pub value: T,
    pub node_ref: NodeRef<web_sys::Element>,
    pub disabled: bool,
}

pub struct Collection<T> {
    items: RwSignal<Vec<CollectionItem<T>>>,
}
```

## 6. Accessibility Implementation

### 6.1 ARIA Attributes

Automatic ARIA attribute management:

```rust
pub fn apply_aria_attributes(
    element: &web_sys::Element,
    role: &str,
    state: &ComponentState,
) {
    element.set_attribute("role", role).unwrap();
    element.set_attribute("aria-expanded", 
        &state.expanded.get().to_string()).unwrap();
    // Additional attributes...
}
```

### 6.2 Keyboard Navigation

Implementing keyboard handlers:

```rust
pub fn use_roving_focus(items: Signal<Vec<NodeRef>>) -> impl Fn(KeyboardEvent) {
    move |event: KeyboardEvent| {
        match event.key().as_str() {
            "ArrowDown" => focus_next_item(),
            "ArrowUp" => focus_previous_item(),
            "Home" => focus_first_item(),
            "End" => focus_last_item(),
            _ => {}
        }
    }
}
```

### 6.3 Focus Management

```rust
pub fn use_focus_trap(
    container_ref: NodeRef<web_sys::Element>,
    active: Signal<bool>,
) {
    create_effect(move |_| {
        if active.get() {
            trap_focus(container_ref.get());
        }
    });
}
```

## 7. Styling Strategy

### 7.1 Data Attributes

Using data attributes for styling hooks:

```rust
#[component]
pub fn DialogContent(
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-radix-dialog-content=""
            data-state=move || if open.get() { "open" } else { "closed" }
            class=class
        >
            {children()}
        </div>
    }
}
```

### 7.2 CSS Custom Properties

Exposing positioning values:

```rust
pub fn set_positioning_styles(element: &web_sys::Element, position: Position) {
    element.style().set_property(
        "--radix-popper-transform-origin",
        &position.transform_origin,
    ).unwrap();
}
```

## 8. Testing Strategy

### 8.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos_test::*;

    #[test]
    fn accordion_single_mode() {
        // Test single accordion behavior
    }

    #[test]
    fn accordion_keyboard_navigation() {
        // Test keyboard interactions
    }
}
```

### 8.2 Accessibility Tests

Using automated accessibility testing:

```rust
#[wasm_bindgen_test]
fn dialog_aria_compliance() {
    // Test ARIA attributes and roles
}
```

## 9. Documentation Strategy

### 9.1 Component Documentation

Each component will include:
- Usage examples
- API reference
- Accessibility features
- Keyboard interactions
- Data attributes for styling

### 9.2 Migration Guide

Providing guides for React developers:

```markdown
## Migrating from Radix UI (React)

### React
```jsx
<Dialog.Root>
  <Dialog.Trigger>Open</Dialog.Trigger>
  <Dialog.Content>Content</Dialog.Content>
</Dialog.Root>
```

### Leptos
```rust
<DialogRoot>
    <DialogTrigger>"Open"</DialogTrigger>
    <DialogContent>"Content"</DialogContent>
</DialogRoot>
```
```

## 10. Performance Considerations

### 10.1 Bundle Size Optimization

- Tree-shaking friendly exports
- Modular component structure
- Minimal dependencies

### 10.2 Runtime Performance

- Leverage Leptos's fine-grained reactivity
- Minimize DOM operations
- Efficient event delegation

## 11. Implementation Roadmap

### Phase 1: Foundation (Months 1-2)
- [ ] Project setup and CI/CD
- [ ] Core primitives (Portal, Slot, VisuallyHidden)
- [ ] Basic hooks and utilities
- [ ] First 3 simple components (Label, Button, Switch)
- [ ] Documentation site setup

### Phase 2: Essential Components (Months 3-4)
- [ ] Dialog and Popover
- [ ] Select and Dropdown
- [ ] Accordion and Tabs
- [ ] Form components (Checkbox, Radio, etc.)
- [ ] Initial release (0.1.0)

### Phase 3: Advanced Features (Months 5-6)
- [ ] Complex components (NavigationMenu, Combobox)
- [ ] Animation support with Presence
- [ ] Advanced positioning with floating-ui
- [ ] Comprehensive test coverage
- [ ] Beta release (0.5.0)

### Phase 4: Production Ready (Months 7-8)
- [ ] Performance optimization
- [ ] Complete documentation
- [ ] Migration guides
- [ ] Community feedback incorporation
- [ ] 1.0.0 release

## 12. Example Implementation: Dialog Component

```rust
// dialog/mod.rs
use leptos::*;
use crate::hooks::*;
use crate::primitives::portal::Portal;

#[derive(Clone)]
struct DialogContext {
    open: RwSignal<bool>,
    trigger_ref: NodeRef<web_sys::Element>,
}

#[component]
pub fn DialogRoot(
    #[prop(optional)] open: Option<RwSignal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Box<dyn Fn(bool)>>,
    children: Children,
) -> impl IntoView {
    let state = use_controllable_state(
        open,
        default_open.unwrap_or(false),
        on_open_change,
    );
    
    let ctx = DialogContext {
        open: state.value,
        trigger_ref: create_node_ref(),
    };
    
    provide_context(ctx);
    
    children()
}

#[component]
pub fn DialogTrigger(
    #[prop(optional)] as_child: bool,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<DialogContext>()
        .expect("DialogTrigger must be used within DialogRoot");
    
    let on_click = move |_| {
        ctx.open.update(|v| *v = !*v);
    };
    
    view! {
        <button
            ref=ctx.trigger_ref
            on:click=on_click
            aria-haspopup="dialog"
            aria-expanded=move || ctx.open.get()
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DialogContent(
    #[prop(optional)] force_mount: bool,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<DialogContext>()
        .expect("DialogContent must be used within DialogRoot");
    
    let content_ref = create_node_ref::<web_sys::Element>();
    
    // Focus trap
    use_focus_trap(content_ref, ctx.open.into());
    
    // Escape key handler
    use_escape_key(move || {
        ctx.open.set(false);
    });
    
    // Outside click handler
    use_outside_click(content_ref, move || {
        ctx.open.set(false);
    });
    
    view! {
        <Show when=move || ctx.open.get() || force_mount>
            <Portal>
                <div
                    data-radix-dialog-overlay=""
                    data-state=move || if ctx.open.get() { "open" } else { "closed" }
                />
                <div
                    ref=content_ref
                    role="dialog"
                    aria-modal="true"
                    data-radix-dialog-content=""
                    data-state=move || if ctx.open.get() { "open" } else { "closed" }
                >
                    {children()}
                </div>
            </Portal>
        </Show>
    }
}
```

## 13. Community and Governance

### 13.1 Open Source Strategy
- MIT License (matching Radix UI)
- Public GitHub repository
- Open RFC process for major changes
- Regular community meetings

### 13.2 Contribution Guidelines
- Code style guide following Rust conventions
- PR review process
- Component proposal template
- Accessibility checklist for new components

## 14. Success Metrics

- **Adoption**: Number of downloads and GitHub stars
- **Quality**: Test coverage > 90%, zero accessibility violations
- **Performance**: Bundle size < 50KB for core components
- **Developer Experience**: Time to implement common patterns
- **Community**: Active contributors and issue resolution time

## 15. Conclusion

This design document provides a comprehensive blueprint for porting Radix UI to Leptos. By leveraging Rust's type safety and Leptos's fine-grained reactivity, we can create a component library that maintains Radix UI's commitment to accessibility and developer experience while bringing the benefits of Rust to frontend development.

The modular architecture ensures that developers can adopt components incrementally, while the focus on unstyled primitives maintains maximum flexibility for design systems. With careful attention to API design, accessibility, and performance, radix-leptos can become a foundational library for building accessible web applications in Rust.
