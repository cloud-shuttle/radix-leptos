# Contributing to Radix-Leptos

Thank you for your interest in contributing to Radix-Leptos! This document provides guidelines and information for contributors.

## 🤝 How to Contribute

### Reporting Issues

Before creating an issue, please:

1. **Search existing issues** - Your issue might already be reported
2. **Check the documentation** - The answer might be in our docs
3. **Provide a minimal reproduction** - Include code that demonstrates the issue
4. **Include environment details** - OS, Rust version, Leptos version, etc.

### Feature Requests

We welcome feature requests! Please:

1. **Describe the use case** - What problem does this solve?
2. **Provide examples** - How would you use this feature?
3. **Consider alternatives** - Are there existing solutions?

### Pull Requests

We love pull requests! Here's how to contribute:

## 🛠️ Development Setup

### Prerequisites

- Rust 1.70+
- wasm-pack
- Node.js (for examples)

### Getting Started

1. **Fork the repository**
   ```bash
   git clone https://github.com/YOUR_USERNAME/radix-leptos.git
   cd radix-leptos
   ```

2. **Build the project**
   ```bash
   cargo build
   ```

3. **Run tests**
   ```bash
   cargo test
   ```

4. **Build examples**
   ```bash
   cd examples
   ./build.sh
   python3 -m http.server 8000
   # Visit http://localhost:8000
   ```

## 📝 Code Style

### Rust Code

- Follow [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for common issues
- Add `#[allow(clippy::...)]` only when necessary

### Component Guidelines

- **Accessibility first** - All components must be accessible
- **Type safety** - Use strong types, avoid `String` when possible
- **Composability** - Design components to work together
- **Documentation** - Document all public APIs

### Example Component Structure

```rust
use leptos::*;
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ComponentState {
    // State fields
}

#[component]
pub fn Component(
    #[prop(optional)] prop1: Option<String>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] children: Children,
) -> impl IntoView {
    // Component implementation
    view! {
        <div class={format!("component {}", class.unwrap_or_default())}>
            {children()}
        </div>
    }
}
```

## 🧪 Testing

### Running Tests

```bash
# Unit tests
cargo test

# WASM tests
wasm-pack test --headless --firefox

# Examples (manual testing)
cd examples
./build.sh
python3 -m http.server 8000
```

### Test Guidelines

- **Unit tests** for component logic
- **Integration tests** for component interactions
- **Accessibility tests** for ARIA compliance
- **Manual testing** in browser for complex interactions

### Example Test

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_component_renders() {
        let component = Component {
            prop1: Some("test".to_string()),
            on_change: None,
            class: None,
            children: Children::new(|_| view! { <span>"Test"</span> }),
        };
        
        // Test implementation
    }
}
```

## 📚 Documentation

### Code Documentation

- Document all public APIs with `///` comments
- Include usage examples in documentation
- Follow [Rust Documentation Style](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments)

### Example Documentation

```rust
/// A component that does something amazing.
///
/// # Examples
///
/// ```rust
/// use radix_leptos_primitives::*;
///
/// view! {
///     <Component prop1="value" />
/// }
/// ```
#[component]
pub fn Component(/* props */) -> impl IntoView {
    // Implementation
}
```

## 🚀 Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR** - Breaking changes
- **MINOR** - New features (backward compatible)
- **PATCH** - Bug fixes (backward compatible)

### Before Release

1. **Update version numbers** in `Cargo.toml` files
2. **Update CHANGELOG.md** with new features/fixes
3. **Run full test suite** - All tests must pass
4. **Build examples** - Ensure everything works
5. **Update documentation** - Keep docs current

## 🐛 Bug Reports

When reporting bugs, please include:

1. **Description** - What happened vs. what you expected
2. **Reproduction steps** - Step-by-step instructions
3. **Code example** - Minimal code that reproduces the issue
4. **Environment** - OS, Rust version, Leptos version
5. **Error messages** - Full error output

## 💡 Feature Requests

When requesting features, please include:

1. **Use case** - What problem does this solve?
2. **Proposed API** - How would you use this feature?
3. **Alternatives** - What have you tried?
4. **Priority** - How important is this to you?

## 📞 Getting Help

- **Issues**: [GitHub Issues](https://github.com/cloud-shuttle/radix-leptos/issues)
- **Discussions**: [GitHub Discussions](https://github.com/cloud-shuttle/radix-leptos/discussions)
- **Documentation**: [Component API Reference](COMPONENTS.md)

## 🙏 Recognition

Contributors will be recognized in:

- **README.md** - For significant contributions
- **CHANGELOG.md** - For all contributions
- **GitHub Contributors** - Automatic recognition

Thank you for contributing to Radix-Leptos! 🎉
