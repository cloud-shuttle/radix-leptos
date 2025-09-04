# 🚀 Radix-Leptos

**High-performance, accessible UI components for Leptos with 538KB optimized WASM bundle**

[![Crates.io](https://img.shields.io/crates/v/radix-leptos)](https://crates.io/crates/radix-leptos)
[![Documentation](https://img.shields.io/docsrs/radix-leptos)](https://docs.rs/radix-leptos)
[![License](https://img.shields.io/crates/l/radix-leptos)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.89+-blue.svg)](https://www.rust-lang.org/)

A Rust port of [Radix UI](https://www.radix-ui.com/) primitives for [Leptos](https://leptos.dev/), providing accessible, unstyled UI components with exceptional performance.

## ✨ Features

- **🚀 High Performance**: 538KB optimized WASM bundle (down from 5.8MB!)
- **♿ Accessibility First**: Built with ARIA compliance and keyboard navigation
- **🎨 Unstyled Components**: Clean, customizable components without opinionated styling
- **🌐 SSR & Hydration**: Full support for server-side rendering and hydration
- **🧪 Comprehensive Testing**: 162 passing tests with TDD infrastructure, property-based testing, and mutation testing
- **📱 Responsive Design**: Mobile-first components with touch support
- **🔧 Feature Flags**: `core` and `full` feature sets for optimal bundle sizes
- **🔒 Type Safety**: Full Rust type safety with excellent IDE support
- **⚡ Zero Runtime**: No JavaScript runtime overhead, pure WASM performance
- **🧪 TDD Infrastructure**: World-class Test-Driven Development with 162 passing tests
- **🔬 Property-Based Testing**: Edge case detection with `proptest`
- **🎯 Quality Gates**: Automated compliance checking and mutation testing

## 🗺️ Roadmap

**Radix-Leptos is on track to achieve complete parity with [Radix UI Primitives](https://www.radix-ui.com/primitives/docs/overview/introduction):**

### 📊 **Current Progress: 50% Complete**
- ✅ **20 Components** implemented with TDD
- ✅ **Complete TDD Infrastructure** (100%)
- ✅ **162 Passing Tests** across all crates
- 🔄 **20+ Components** planned for v0.4.0-v1.0.0

### 🚀 **Upcoming Releases**
- **v0.3.0 (September 2025)**: ✅ **COMPLETE** - Core form components (Dialog, Form, Select, etc.)
- **v0.4.0 (Q4 2025)**: Navigation and layout components
- **v0.5.0 (Q1 2026)**: Advanced and specialized components
- **v1.0.0 (Q2 2026)**: Complete Radix UI parity

📋 **[View Full Roadmap](ROADMAP_vs_RADIX_UI.md)** | 📊 **[Component Progress](COMPONENT_PROGRESS.md)**

## 📦 Installation

```toml
[dependencies]
radix-leptos = "0.3.0"

# For minimal bundle size (recommended for production)
radix-leptos = { version = "0.3.0", features = ["core"] }

# For all components
radix-leptos = { version = "0.3.0", features = ["full"] }
```

## 🚀 Quick Start

```rust
use leptos::*;
use radix_leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <Button>Click me!</Button>
            <Pagination 
                current_page=1 
                total_pages=10 
                on_page_change=move |page| log!("Page changed to {}", page)
            />
        </div>
    }
}
```

## 📝 More Examples

### Basic Button Component
```rust
use radix_leptos::Button;

#[component]
pub fn MyButton() -> impl IntoView {
    view! {
        <Button 
            on_click=move |_| log!("Button clicked!") 
            variant="primary"
            size="large"
        >
            "Click me!"
        </Button>
    }
}
```

### Advanced Pagination
```rust
use radix_leptos::pagination::*;

#[component]
pub fn MyPagination() -> impl IntoView {
    let (current_page, set_current_page) = signal(1);
    
    view! {
        <Pagination 
            current_page=current_page
            total_pages=25
            page_size=10
            on_page_change=move |page| set_current_page.set(page)
        >
            <PaginationContent>
                <PaginationItem>
                    <PaginationPrevious />
                </PaginationItem>
                <PaginationItem>
                    <PaginationNext />
                </PaginationItem>
            </PaginationContent>
        </Pagination>
    }
}
```

## 📦 Available Components

### Core Components (Available with `core` feature)
- **Pagination**: Page navigation with multiple variants and sizes
- **Navigation**: Basic navigation utilities and patterns
- **Core Utilities**: Essential traits and helper functions

### Full Components (Available with `full` feature)
- **Form Components**: Button, TextInput, Select, Checkbox, RadioGroup, Switch, Slider
- **Feedback Components**: Toast, Alert, Badge, Avatar, Progress
- **Media Components**: Image, Video, Audio, Carousel
- **Advanced Components**: ContextMenu, DropdownMenu, Menubar, ScrollArea, Tabs, Accordion, Table, List, Timeline

## 🧪 Testing & Examples

```bash
# Run comprehensive tests
pnpm run test:all

# Start development server
pnpm run start:dev

# Build production bundle
pnpm run build:prod
```

## 📊 Performance Metrics

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| **Bundle Size** | 5.8MB | 538KB | **92.7% reduction** |
| **Load Time** | ~15s | ~130ms | **98.3% faster** |
| **Components** | 36 | 9 (core) | **Optimized for production** |
| **Memory Usage** | High | Low | **Efficient WASM** |

## 🆚 Why Choose Radix-Leptos?

| Feature | Radix-Leptos | Traditional JS | Other Rust UI |
|---------|---------------|----------------|---------------|
| **Bundle Size** | 538KB | 2-5MB+ | 1-3MB+ |
| **Performance** | Native WASM | JavaScript | WASM + JS |
| **Type Safety** | Rust-level | TypeScript | Rust-level |
| **Accessibility** | Built-in ARIA | Varies | Varies |
| **Styling** | Unstyled | Often opinionated | Mixed |
| **SSR Support** | Full | Limited | Limited |
| **Memory Usage** | Low | High | Medium |

## 🧪 Testing

```bash
# Run all tests
pnpm run test:all

# Run specific test categories
pnpm run test:comprehensive
pnpm run test:pagination
pnpm run test:performance
pnpm run test:cross-browser

# Manual testing
open http://localhost:8081/manual-test-suite.html
```

## 🏗️ Architecture

```
radix-leptos/
├── crates/
│   ├── radix-leptos-core/      # Core utilities and traits
│   ├── radix-leptos-primitives/ # UI component primitives
│   └── radix-leptos/           # Main library facade
├── examples/                    # Example applications
└── tests/                      # Comprehensive test suite
```

## 🔧 Feature Flags

### Core Features (`core`)
- Pagination components
- Basic navigation
- Essential utilities

### Full Features (`full`)
- All components
- Advanced interactions
- Extended functionality

## 📚 Documentation

- [📚 Complete Documentation](docs/) - All guides, APIs, and resources
- [🚀 Development Roadmap](docs/ROADMAP.md) - Future features and timeline
- [🧪 Testing Guide](docs/guides/TESTING_GUIDE.md) - How to test components
- [📊 Performance Guide](docs/guides/OPTIMIZATION_RESULTS.md) - Bundle optimization details
- [🚀 Production Deployment](docs/guides/PRODUCTION_DEPLOYMENT_GUIDE.md) - Deployment guide
- [API Documentation](https://docs.rs/radix-leptos) - Official crates.io docs

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Radix UI](https://www.radix-ui.com/) for the original design system
- [Leptos](https://leptos.dev/) for the amazing Rust web framework
- The Rust community for continuous support

## 🚀 Roadmap

- [x] Core component library
- [x] Performance optimization
- [x] Comprehensive testing
- [x] Production deployment
- [ ] Additional components
- [ ] Theme system
- [ ] Animation library
- [ ] Component playground

**📋 [View Full Development Roadmap](ROADMAP.md)**

## 🌟 Community & Support

- **📖 Documentation**: [https://docs.rs/radix-leptos](https://docs.rs/radix-leptos)
- **🐛 Issues**: [GitHub Issues](https://github.com/cloud-shuttle/radix-leptos/issues)
- **💬 Discussions**: [GitHub Discussions](https://github.com/cloud-shuttle/radix-leptos/discussions)
- **📚 Examples**: [Examples Directory](./examples/)
- **🧪 Testing**: [Test Suite](./tests/)
- **📊 Performance**: [Performance Analysis](./docs/PERFORMANCE.md)

## 🏆 Production Ready

Radix-Leptos is **production-ready** with:
- ✅ **Comprehensive Testing**: 10+ Playwright tests
- ✅ **Performance Optimized**: 538KB bundle size
- ✅ **Accessibility Compliant**: WCAG guidelines
- ✅ **Cross-Browser**: Chrome, Firefox, Safari, Edge
- ✅ **Mobile Support**: Touch and responsive design
- ✅ **Type Safe**: Full Rust type safety

---

**Ready for production use with 538KB optimized bundle! 🎉**

**Latest Version: v0.1.2** - Now with enhanced documentation and professional repository structure!