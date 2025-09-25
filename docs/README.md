# Radix-Leptos Documentation

Welcome to the comprehensive documentation for Radix-Leptos, a powerful UI component library built on top of Leptos.

## Table of Contents

- [Getting Started](#getting-started)
- [Component Gallery](#component-gallery)
- [Interactive Examples](#interactive-examples)
- [Advanced Patterns](#advanced-patterns)
- [Performance Optimization](#performance-optimization)
- [Video Tutorials](#video-tutorials)

## Getting Started

### Quick Start

```rust
use leptos::prelude::*;
use radix_leptos_primitives::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app">
            <h1>"Welcome to Radix-Leptos"</h1>
            <Button>"Click me!"</Button>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
```

### Installation

Add Radix-Leptos to your `Cargo.toml`:

```toml
[dependencies]
leptos = "0.6"
radix-leptos-primitives = "0.9.0"
radix-leptos-core = "0.9.0"
```

## Component Gallery

Explore all available components in our [Component Gallery](examples/component-gallery.md):

- **Layout Components**: Container, Grid, Flex
- **Form Components**: Input, Textarea, Select, Checkbox, Radio, Switch
- **Navigation Components**: Tabs, Navigation Menu, Breadcrumb, Pagination
- **Data Display Components**: Table, Card, Badge, Avatar, Progress, Skeleton
- **Feedback Components**: Alert, Toast, Dialog, Alert Dialog, Sheet
- **Overlay Components**: Popover, Tooltip, Hover Card, Context Menu

## Interactive Examples

Learn through hands-on examples in our [Interactive Examples](examples/interactive-examples.md):

- **Getting Started**: Basic setup and configuration
- **Form Components**: Complete form examples with validation
- **Navigation Components**: Tab and navigation examples
- **Data Display**: Table and card examples
- **Advanced Patterns**: State management and performance optimization

## Advanced Patterns

Master advanced techniques in our [Advanced Patterns Guide](examples/advanced-patterns.md):

- **State Management**: Global state, local storage, custom hooks
- **Performance Optimization**: Memoization, virtual scrolling, monitoring
- **Accessibility Patterns**: ARIA labels, keyboard navigation, focus management
- **Testing Patterns**: Unit tests, integration tests, performance tests
- **Architecture Patterns**: Feature-based architecture, plugins

## Performance Optimization

Optimize your applications with our [Performance Optimization Guide](performance-optimization-guide.md):

- **String Caching**: Reduce memory allocations
- **Component Memoization**: Prevent unnecessary re-renders
- **Virtual Scrolling**: Efficient rendering of large datasets
- **Performance Monitoring**: Real-time performance tracking
- **Memory Pooling**: Efficient memory management
- **Benchmarking**: Performance testing and regression detection

## Video Tutorials

Learn through video tutorials:

- [Getting Started Tutorial](tutorials/video-tutorial-scripts.md#getting-started-tutorial)
- [Component Deep Dive](tutorials/video-tutorial-scripts.md#component-deep-dive)
- [Advanced Patterns Tutorial](tutorials/video-tutorial-scripts.md#advanced-patterns-tutorial)
- [Performance Optimization Tutorial](tutorials/video-tutorial-scripts.md#performance-optimization-tutorial)
- [Accessibility Tutorial](tutorials/video-tutorial-scripts.md#accessibility-tutorial)
- [Testing Tutorial](tutorials/video-tutorial-scripts.md#testing-tutorial)

## Support

- **Documentation**: [docs.radix-leptos.dev](https://docs.radix-leptos.dev)
- **Examples**: [examples.radix-leptos.dev](https://examples.radix-leptos.dev)
- **GitHub**: [github.com/radix-leptos/radix-leptos](https://github.com/radix-leptos/radix-leptos)
- **Discord**: [discord.gg/radix-leptos](https://discord.gg/radix-leptos)

## License

Radix-Leptos is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

**Built with ❤️ by the Radix-Leptos team**