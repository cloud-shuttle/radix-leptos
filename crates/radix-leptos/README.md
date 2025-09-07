# Radix-Leptos

Accessible, unstyled UI primitives for Leptos - High-performance, accessible UI components with 538KB optimized WASM bundle.

## Overview

Radix-Leptos is a comprehensive UI component library for Leptos that provides accessible, unstyled primitives for building modern web applications. It's a Rust port of the popular Radix UI library, designed specifically for the Leptos framework.

## Features

- 🎯 **40+ Components**: Comprehensive set of UI primitives
- ♿ **Accessible**: WCAG 2.1 AA compliant with proper ARIA attributes
- 🎨 **Unstyled**: Clean, semantic HTML without opinionated styling
- ⚡ **Performant**: 538KB optimized WASM bundle, <400KB target
- 🧪 **Well Tested**: 1,768 tests covering unit, integration, and accessibility
- 🔧 **Composable**: Components designed to work together seamlessly
- 🎨 **Themable**: CSS custom properties and theme system
- 📱 **Responsive**: Mobile-first design with responsive utilities

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
radix-leptos = "0.8.3"
leptos = "0.8.8"
```

Basic usage:

```rust
use leptos::*;
use radix_leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    
    view! { cx,
        <div>
            <Button 
                variant=ButtonVariant::Primary
                on_click=Callback::new(move |_| set_count.update(|c| *c += 1))
            >
                "Count: " {count}
            </Button>
        </div>
    }
}
```

## Component Categories

### Form Controls
- **Button**: Interactive buttons with multiple variants
- **Checkbox**: Accessible checkboxes with indeterminate state
- **Switch**: Toggle switches
- **Slider**: Range inputs with keyboard support
- **Select**: Dropdown selections with search
- **RadioGroup**: Radio button groups
- **Input**: Text inputs with validation
- **Textarea**: Multi-line text inputs

### Layout & Navigation
- **Dialog**: Modal dialogs with focus management
- **Sheet**: Slide-out panels
- **AlertDialog**: Confirmation and alert dialogs
- **Tabs**: Tabbed interfaces with keyboard navigation
- **Accordion**: Collapsible content sections
- **DropdownMenu**: Contextual menu systems
- **Tooltip**: Hover and focus tooltips
- **Popover**: Floating content containers

### Data Display
- **Card**: Content containers with header, body, footer
- **Badge**: Status and notification badges
- **Avatar**: User profile images with fallbacks
- **Progress**: Progress indicators
- **Skeleton**: Loading placeholders
- **Separator**: Visual content dividers

### Advanced Components
- **Calendar**: Date pickers with keyboard navigation
- **DatePicker**: Date selection components
- **TimePicker**: Time selection components
- **TreeView**: Hierarchical data display
- **Carousel**: Image and content carousels
- **Toast**: Notification system
- **Form**: Form validation and submission

## Theming

Radix-Leptos includes a powerful theming system:

```rust
use radix_leptos::theming::*;

#[component]
fn ThemedApp(cx: Scope) -> impl IntoView {
    view! { cx,
        <ThemeProvider theme=light_theme>
            <Button variant=ButtonVariant::Primary>
                "Themed Button"
            </Button>
        </ThemeProvider>
    }
}
```

## Accessibility

Every component is built with accessibility as a first-class citizen:

- **Keyboard Navigation**: Full keyboard support for all interactive elements
- **Screen Reader Support**: Proper ARIA attributes and semantic HTML
- **Focus Management**: Logical focus flow and focus trapping
- **Color Contrast**: WCAG 2.1 AA compliant color schemes
- **Motion Preferences**: Respects user's motion preferences

## Testing

The library includes comprehensive test coverage:

- **1,768 Tests**: Unit, integration, and accessibility tests
- **WCAG Compliance**: Automated accessibility testing
- **Property-Based Testing**: Edge case validation
- **Performance Testing**: Bundle size and build time monitoring

Run tests:
```bash
cargo test
```

## Performance

- **Bundle Size**: 538KB optimized WASM bundle (target: <400KB)
- **Build Time**: Optimized compilation profiles
- **Runtime Performance**: Efficient rendering and state management
- **Tree Shaking**: Only include components you use

## Examples

Check out the [examples directory](../../examples/) for comprehensive usage examples:

- Component showcases
- Interactive demos
- Theming examples
- Accessibility demonstrations

## Documentation

- **API Reference**: [docs.rs/radix-leptos](https://docs.rs/radix-leptos)
- **Component Guide**: [docs/guides/](docs/guides/)
- **Accessibility Guide**: [docs/accessibility/](docs/accessibility/)
- **Theming Guide**: [docs/theming/](docs/theming/)

## Contributing

We welcome contributions! Please see our [Contributing Guide](docs/developer-guide/CONTRIBUTING.md) for details.

## License

MIT License - see [LICENSE](../../LICENSE) for details.

## Acknowledgments

- Inspired by [Radix UI](https://www.radix-ui.com/)
- Built for [Leptos](https://leptos.dev/)
- Powered by [Rust](https://www.rust-lang.org/)
