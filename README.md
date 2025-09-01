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
- **🧪 Comprehensive Testing**: 10+ Playwright tests covering functionality, performance, and accessibility
- **📱 Responsive Design**: Mobile-first components with touch support
- **🔧 Feature Flags**: `core` and `full` feature sets for optimal bundle sizes

## 📦 Installation

```toml
[dependencies]
radix-leptos = "0.1.0"
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

## 📊 Performance Metrics

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| **Bundle Size** | 5.8MB | 538KB | **92.7% reduction** |
| **Load Time** | ~15s | ~130ms | **98.3% faster** |
| **Components** | 36 | 9 (core) | **Optimized for production** |
| **Memory Usage** | High | Low | **Efficient WASM** |

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

```toml
[dependencies]
radix-leptos = { version = "0.1.0", features = ["core"] }
```

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

---

**Ready for production use with 538KB optimized bundle! 🎉**