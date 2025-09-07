# Radix-Leptos Primitives

Primitive UI components for Leptos, built with accessibility and performance in mind.

## Overview

This crate provides a comprehensive set of accessible, unstyled UI primitives that serve as the foundation for building modern web applications with Leptos. Each component is designed to be:

- **Accessible**: WCAG 2.1 AA compliant with proper ARIA attributes
- **Unstyled**: Clean, semantic HTML without opinionated styling
- **Performant**: Optimized for minimal bundle size and fast rendering
- **Composable**: Designed to work together seamlessly

## Components

### Form Controls
- **Button**: Interactive button with multiple variants
- **Checkbox**: Accessible checkbox with indeterminate state
- **Switch**: Toggle switch component
- **Slider**: Range input with keyboard support
- **Select**: Dropdown selection with search
- **RadioGroup**: Radio button groups
- **Input**: Text input with validation
- **Textarea**: Multi-line text input

### Layout & Navigation
- **Dialog**: Modal dialog with focus management
- **Sheet**: Slide-out panel component
- **AlertDialog**: Confirmation and alert dialogs
- **Tabs**: Tabbed interface with keyboard navigation
- **Accordion**: Collapsible content sections
- **DropdownMenu**: Contextual menu system
- **Tooltip**: Hover and focus tooltips
- **Popover**: Floating content containers

### Data Display
- **Card**: Content container with header, body, footer
- **Badge**: Status and notification badges
- **Avatar**: User profile images with fallbacks
- **Progress**: Progress indicators
- **Skeleton**: Loading placeholders
- **Separator**: Visual content dividers

### Advanced Components
- **Calendar**: Date picker with keyboard navigation
- **DatePicker**: Date selection component
- **TimePicker**: Time selection component
- **TreeView**: Hierarchical data display
- **Carousel**: Image and content carousel
- **Toast**: Notification system
- **Form**: Form validation and submission

## Usage

```rust
use radix_leptos_primitives::*;

// Basic button
let view = view! { cx,
    <Button variant=ButtonVariant::Primary>
        "Click me"
    </Button>
};

// Dialog with state management
let (is_open, set_is_open) = create_signal(cx, false);
let view = view! { cx,
    <Dialog _open=is_open onopen_change=Callback::new(move |open| set_is_open.set(open))>
        <DialogContent>
            <DialogHeader>
                <DialogTitle>"Confirm Action"</DialogTitle>
                <DialogDescription>"Are you sure you want to proceed?"</DialogDescription>
            </DialogHeader>
        </DialogContent>
    </Dialog>
};
```

## Theming

All components support theming through CSS custom properties and can be styled to match your design system:

```rust
use radix_leptos_primitives::theming::*;

// Apply theme
let view = view! { cx,
    <ThemeProvider theme=light_theme>
        <Button>"Themed Button"</Button>
    </ThemeProvider>
};
```

## Accessibility

Every component is built with accessibility in mind:

- **Keyboard Navigation**: Full keyboard support for all interactive elements
- **Screen Reader Support**: Proper ARIA attributes and semantic HTML
- **Focus Management**: Logical focus flow and focus trapping
- **Color Contrast**: WCAG 2.1 AA compliant color schemes
- **Motion Preferences**: Respects user's motion preferences

## Testing

The crate includes comprehensive tests:

- **Unit Tests**: Component behavior and state management
- **Integration Tests**: Component interaction workflows
- **Accessibility Tests**: WCAG compliance verification
- **Property-Based Tests**: Edge case validation

Run tests with:
```bash
cargo test
```

## Documentation

For complete API documentation, visit [docs.rs/radix-leptos-primitives](https://docs.rs/radix-leptos-primitives).

## License

MIT License - see [LICENSE](../../LICENSE) for details.
