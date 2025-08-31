# Radix-Leptos

A comprehensive collection of accessible UI components built with [Leptos](https://leptos.dev/) and Rust, inspired by [Radix UI](https://www.radix-ui.com/).

## 🚀 Features

- **15+ Accessible Components** - Built with WCAG 2.1 AA compliance
- **Modern Rust Stack** - Built with Leptos 0.8+ and WebAssembly
- **Type-Safe** - Full Rust type safety and compile-time guarantees
- **Customizable** - Flexible styling and theming options
- **Production Ready** - Comprehensive testing and documentation

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
radix-leptos-primitives = "0.1.0"
leptos = { version = "0.8", features = ["csr", "ssr"] }
```

## 🎯 Quick Start

```rust
use leptos::*;
use radix_leptos_primitives::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <Button on_click=Callback::new(|_| println!("Clicked!"))>
                "Click me!"
            </Button>
            
            <Checkbox 
                checked=true 
                on_change=Callback::new(|state| println!("Checkbox: {:?}", state))
            />
            
            <Dialog>
                <DialogTrigger>
                    <Button>"Open Dialog"</Button>
                </DialogTrigger>
                <DialogContent>
                    <DialogTitle>"Hello World"</DialogTitle>
                    <DialogDescription>
                        "This is a dialog built with Radix-Leptos!"
                    </DialogDescription>
                </DialogContent>
            </Dialog>
        </div>
    }
}
```

## 🧩 Available Components

### Phase 1: Core Components
- **Button** - Accessible button with multiple variants
- **Label** - Form labels with proper accessibility
- **Separator** - Visual dividers with orientation options
- **Dialog** - Modal dialogs with focus management

### Phase 2: Form Components
- **Checkbox** - Accessible checkbox with indeterminate state
- **Switch** - Toggle switches with proper ARIA attributes
- **RadioGroup** - Radio button groups with keyboard navigation
- **TextInput** - Text input fields with validation support

### Phase 3: Advanced Components
- **Accordion** - Collapsible content sections
- **Tabs** - Tabbed interfaces with keyboard navigation
- **Popover** - Floating content overlays
- **Tooltip** - Contextual help tooltips
- **Select** - Dropdown selection components

## 🎨 Styling

Components come with sensible defaults but are fully customizable. You can:

1. **Use CSS classes** - All components accept a `class` prop
2. **Apply custom styles** - Use the `style` attribute
3. **Theme with CSS variables** - Override default design tokens

```rust
<Button 
    class="my-custom-button"
    style="background: linear-gradient(45deg, #ff6b6b, #4ecdc4);"
>
    "Custom Styled Button"
</Button>
```

## ♿ Accessibility

All components follow WCAG 2.1 AA guidelines and include:

- **Proper ARIA attributes** - Screen reader support
- **Keyboard navigation** - Full keyboard accessibility
- **Focus management** - Logical tab order and focus indicators
- **Semantic HTML** - Proper HTML structure and semantics

## 🧪 Testing

Run the interactive test suite:

```bash
# Build the examples
cd examples
./build.sh

# Serve the test suite
python3 -m http.server 8000
# Visit http://localhost:8000
```

## 📚 Documentation

- **[Component API Reference](COMPONENTS.md)** - Detailed component documentation
- **[Progress Report](PROGRESS_REPORT.md)** - Development status and roadmap
- **[Validation Report](VALIDATION_REPORT.md)** - Testing and validation results

## 🔧 Development

### Prerequisites

- Rust 1.70+
- wasm-pack
- Node.js (for examples)

### Building

```bash
# Build all crates
cargo build

# Build examples
cd examples
./build.sh
```

### Testing

```bash
# Run unit tests
cargo test

# Run examples in browser
cd examples
./build.sh
python3 -m http.server 8000
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Radix UI](https://www.radix-ui.com/) - Design inspiration and accessibility patterns
- [Leptos](https://leptos.dev/) - The amazing Rust web framework
- [Rust Community](https://www.rust-lang.org/community) - For the incredible ecosystem

## 🗺️ Roadmap

### Phase 4: Complex Components (Coming Soon)
- **Combobox** - Searchable dropdown with keyboard navigation
- **DatePicker** - Calendar-based date selection
- **Slider** - Range input with drag support
- **Progress** - Progress indicators and loading states

### Future Enhancements
- **Animation Support** - Smooth transitions and micro-interactions
- **Theme System** - Comprehensive theming solution
- **Performance Optimizations** - Bundle size and runtime optimizations
- **Server-Side Rendering** - Enhanced SSR support

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/cloud-shuttle/radix-leptos/issues)
- **Discussions**: [GitHub Discussions](https://github.com/cloud-shuttle/radix-leptos/discussions)
- **Documentation**: [Component API Reference](COMPONENTS.md)

---

**Built with ❤️ by the Cloud Shuttle team**