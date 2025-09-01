# Radix-Leptos Primitives

A comprehensive collection of 20+ production-ready UI components built with Rust, Leptos, and WebAssembly. This library provides accessible, customizable, and performant components that follow modern design patterns.

## 🚀 Features

- **20+ Components**: Complete set of form, feedback, media, and advanced UI components
- **Rust + WebAssembly**: Built with Rust for performance and type safety
- **Leptos Framework**: Modern reactive UI framework for Rust
- **Accessibility First**: All components follow ARIA guidelines
- **Customizable**: Extensive theming and variant support
- **Type Safe**: Full Rust type safety with compile-time guarantees
- **Production Ready**: Comprehensive examples and documentation

## 📦 Components

### Form Components
- **Button** - Multiple variants (Default, Destructive, Outline, Ghost, Link)
- **TextInput** - Text input with validation and accessibility
- **Select** - Dropdown selection with search and keyboard navigation
- **Checkbox** - Accessible checkbox with custom styling
- **RadioGroup** - Radio button groups with keyboard navigation
- **Switch** - Toggle switch component
- **Slider** - Range slider with customizable steps
- **DatePicker** - Date selection with calendar interface
- **Combobox** - Searchable dropdown with autocomplete
- **Form** - Form container with validation

### Feedback Components
- **Toast** - Notification system with positioning
- **Alert** - Status alerts with multiple variants
- **Badge** - Status indicators and labels
- **Avatar** - User profile images with fallbacks
- **Progress** - Progress indicators and loading states

### Media Components
- **Image** - Optimized image display with lazy loading
- **Video** - Video player with controls
- **Audio** - Audio player with playlist support
- **Carousel** - Image carousel with navigation

### Advanced Components
- **ContextMenu** - Right-click context menus
- **DropdownMenu** - Dropdown navigation menus
- **Menubar** - Application menu bars
- **ScrollArea** - Custom scrollable areas
- **Tabs** - Tabbed interface with multiple variants
- **Accordion** - Collapsible content sections
- **Navigation** - Navigation menus and breadcrumbs
- **Table** - Data tables with sorting and pagination
- **Pagination** - Page navigation controls
- **List** - Virtualized lists for large datasets
- **Timeline** - Chronological event displays

## 🛠️ Installation

### Prerequisites

- Rust (latest stable)
- wasm-pack
- Node.js (for development)

### Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/cloud-shuttle/radix-leptos.git
   cd radix-leptos
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Build examples**
   ```bash
   cd examples
   wasm-pack build --target web
   ```

4. **Run examples**
   ```bash
   python3 -m http.server 8080
   # Open http://localhost:8080/component_showcase.html
   ```

## 📚 Usage

### Basic Component Usage

```rust
use leptos::*;
use radix_leptos_primitives::*;

#[component]
pub fn MyApp() -> impl IntoView {
    view! {
        <div>
            <Button variant=ButtonVariant::Default>
                "Click me"
            </Button>
            
            <TextInput placeholder="Enter text...".to_string() />
            
            <Alert variant=AlertVariant::Info>
                <AlertTitle>"Information"</AlertTitle>
                <AlertDescription>"This is an alert message."</AlertDescription>
            </Alert>
        </div>
    }
}
```

### Component Variants

Most components support multiple variants for different use cases:

```rust
// Button variants
<Button variant=ButtonVariant::Default>"Default"</Button>
<Button variant=ButtonVariant::Destructive>"Delete"</Button>
<Button variant=ButtonVariant::Outline>"Outline"</Button>
<Button variant=ButtonVariant::Ghost>"Ghost"</Button>
<Button variant=ButtonVariant::Link>"Link"</Button>

// Alert variants
<Alert variant=AlertVariant::Info>"Info"</Alert>
<Alert variant=AlertVariant::Success>"Success"</Alert>
<Alert variant=AlertVariant::Warning>"Warning"</Alert>
<Alert variant=AlertVariant::Error>"Error"</Alert>
```

### Form Components

```rust
// Select component
<Select>
    <SelectTrigger>
        <SelectValue placeholder="Choose an option".to_string()>
            "Choose an option"
        </SelectValue>
    </SelectTrigger>
    <SelectContent>
        <SelectItem value="option1".to_string()>"Option 1"</SelectItem>
        <SelectItem value="option2".to_string()>"Option 2"</SelectItem>
    </SelectContent>
</Select>

// Checkbox with label
<Checkbox id="terms".to_string() />
<label for="terms">"I agree to the terms"</label>
```

### Advanced Components

```rust
// Tabs component
<Tabs>
    <TabsList>
        <TabsTrigger value="tab1".to_string()>"Tab 1"</TabsTrigger>
        <TabsTrigger value="tab2".to_string()>"Tab 2"</TabsTrigger>
    </TabsList>
    <TabsContent value="tab1".to_string()>"Content for tab 1"</TabsContent>
    <TabsContent value="tab2".to_string()>"Content for tab 2"</TabsContent>
</Tabs>

// Context menu
<ContextMenu>
    <ContextMenuTrigger>
        <div>"Right-click me"</div>
    </ContextMenuTrigger>
    <ContextMenuContent>
        <ContextMenuItem>"Copy"</ContextMenuItem>
        <ContextMenuItem>"Paste"</ContextMenuItem>
    </ContextMenuContent>
</ContextMenu>
```

## 🎨 Theming

Components support extensive theming through CSS classes and variants. All components use Tailwind CSS classes by default, but can be customized with your own CSS.

### Custom Styling

```rust
// Custom button styling
<Button 
    class="bg-purple-600 hover:bg-purple-700 text-white px-6 py-3 rounded-lg"
>
    "Custom Button"
</Button>
```

## 🔧 Development

### Project Structure

```
radix-leptos/
├── crates/
│   └── radix-leptos-primitives/    # Main component library
│       ├── src/
│       │   ├── components/         # Individual component modules
│       │   └── lib.rs             # Library entry point
├── examples/                       # Example applications
│   ├── src/                       # Example source code
│   ├── *.html                     # HTML entry points
│   └── pkg/                       # Built WASM files
└── README.md
```

### Adding New Components

1. Create a new component file in `crates/radix-leptos-primitives/src/components/`
2. Add the component to the module exports in `mod.rs`
3. Create examples in `examples/src/`
4. Add an HTML entry point in `examples/`
5. Update the component showcase

### Building

```bash
# Build the library
cargo build

# Build examples for web
cd examples
wasm-pack build --target web

# Run development server
python3 -m http.server 8080
```

## 📖 Examples

Visit the component showcase to see all components in action:

- **Component Showcase**: `http://localhost:8080/component_showcase.html`
- **Individual Examples**: Each component has its own example page

### Available Examples

- Button Examples
- TextInput Examples  
- Select Examples
- Checkbox Examples
- RadioGroup Examples
- Switch Examples
- Slider Examples
- DatePicker Examples
- Combobox Examples
- Form Examples
- Toast Examples
- Alert Examples
- Badge Examples
- Avatar Examples
- Progress Examples
- Image Examples
- Video Examples
- Audio Examples
- Carousel Examples
- ContextMenu Examples
- DropdownMenu Examples
- Menubar Examples
- ScrollArea Examples
- Tabs Examples
- Accordion Examples
- Navigation Examples
- Table Examples
- Pagination Examples
- List Examples
- Timeline Examples

## 🤝 Contributing

We welcome contributions! Please see our contributing guidelines for details.

### Development Setup

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests and examples
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Inspired by [Radix UI](https://www.radix-ui.com/)
- Built with [Leptos](https://leptos.dev/)
- Styled with [Tailwind CSS](https://tailwindcss.com/)

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/cloud-shuttle/radix-leptos/issues)
- **Discussions**: [GitHub Discussions](https://github.com/cloud-shuttle/radix-leptos/discussions)
- **Documentation**: [Component Showcase](http://localhost:8080/component_showcase.html)

---

**Built with ❤️ using Rust and WebAssembly**