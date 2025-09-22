# 🏗️ Radix-Leptos Module Architecture

**Comprehensive guide to the modular architecture and feature flag system**

## 📋 Table of Contents

- [Overview](#overview)
- [Module Structure](#module-structure)
- [Feature Flags](#feature-flags)
- [Component Organization](#component-organization)
- [Theming System](#theming-system)
- [Utilities](#utilities)
- [Usage Examples](#usage-examples)
- [Migration Guide](#migration-guide)

## 🎯 Overview

Radix-Leptos v0.8.5+ features a completely refactored modular architecture designed for:

- **🔧 Maintainability**: Clean separation of concerns
- **📦 Bundle Optimization**: Feature flags for optimal bundle sizes
- **🚀 Performance**: Reduced compilation time and binary size
- **🧪 Testing**: Isolated modules for better test coverage
- **📚 Documentation**: Clear component organization

## 📦 Module Structure

### **High-Level Architecture**

```
radix-leptos-primitives/
├── src/
│   ├── components/           # All UI components
│   ├── theming/             # Theme system
│   ├── utils.rs             # Shared utilities
│   └── lib.rs               # Library entry point
└── Cargo.toml               # Feature flags configuration
```

### **Component Module Structure**

```
components/
├── Core Components (always available)
│   ├── button.rs
│   ├── checkbox.rs
│   ├── dialog.rs
│   ├── form.rs
│   ├── input.rs
│   ├── select.rs
│   ├── pagination/          # Modular pagination system
│   │   ├── context.rs
│   │   ├── items.rs
│   │   ├── helpers.rs
│   │   └── mod.rs
│   └── form_validation/     # Modular form validation
│       ├── validation.rs
│       ├── fields.rs
│       ├── controls.rs
│       └── mod.rs
├── Experimental Components (feature-gated)
│   ├── chart.rs
│   ├── data_table.rs
│   ├── virtual_list.rs
│   ├── rich_text_editor.rs
│   └── [Many more...]
└── mod.rs                   # Feature-gated exports
```

### **Theming Module Structure**

```
theming/
├── prebuilt_themes/         # Light/dark themes
│   ├── light_themes.rs
│   ├── dark_themes.rs
│   ├── color_schemes.rs
│   ├── theme_builder.rs
│   └── mod.rs
├── component_variants/      # Component styling variants
│   ├── button_variants.rs
│   ├── input_variants.rs
│   ├── layout_variants.rs
│   ├── feedback_variants.rs
│   ├── data_variants.rs
│   └── mod.rs
├── layout_system/           # Layout utilities
│   ├── container.rs
│   ├── spacing.rs
│   ├── responsive.rs
│   └── mod.rs
├── theme_customization/     # Theme customization tools
│   ├── css_editor.rs
│   ├── theme_export.rs
│   └── mod.rs
└── mod.rs                   # Theme system exports
```

## 🔧 Feature Flags

### **Available Features**

| Feature | Description | Components Included |
|---------|-------------|-------------------|
| `core` | **Production-ready components** | Button, Checkbox, Dialog, Form, Input, Select, Pagination, FormValidation, etc. |
| `experimental` | **Incomplete/experimental components** | Chart, DataTable, VirtualList, RichTextEditor, ColorPicker, etc. |
| `full` | **All components** | `core` + `experimental` |

### **Feature Flag Configuration**

```toml
# Cargo.toml
[features]
default = []
core = []
experimental = []
full = ["core", "experimental"]
```

### **Component Gating**

Components are gated using Rust's `#[cfg(feature = "...")]` attribute:

```rust
// mod.rs
// Core components (always available)
pub mod button;
pub mod checkbox;
pub mod dialog;

// Experimental components (feature-gated)
#[cfg(feature = "experimental")]
pub mod chart;
#[cfg(feature = "experimental")]
pub mod data_table;

// Re-exports with feature gating
pub use button::*;
pub use checkbox::*;

#[cfg(feature = "experimental")]
pub use chart::*;
#[cfg(feature = "experimental")]
pub use data_table::*;
```

## 🎨 Component Organization

### **Core Components (Production-Ready)**

These components are always available and have been thoroughly tested:

- **Form Components**: Button, Checkbox, Input, Select, Form, FormValidation
- **Layout Components**: Dialog, Sheet, Accordion, Tabs, Pagination
- **Feedback Components**: Alert, Toast, Progress, Skeleton
- **Navigation Components**: DropdownMenu, Popover, Tooltip, HoverCard
- **Data Components**: List, Table, Calendar, DatePicker

### **Experimental Components (Feature-Gated)**

These components are incomplete or experimental:

- **Data Visualization**: Chart, LineChart, BarChart, PieChart, ScatterPlot
- **Advanced UI**: DataTable, VirtualList, RichTextEditor, CodeEditor
- **Specialized**: ColorPicker, ImageViewer, Gauge, CommandPalette
- **Mobile**: TouchButton, SwipeGestures, PullToRefresh
- **Performance**: InfiniteScroll, LazyLoading, LazyLoadingOptimized

## 🎨 Theming System

### **Prebuilt Themes**

- **Light Themes**: Default, Minimal, Modern, Corporate
- **Dark Themes**: Dark, Midnight, Ocean, Forest
- **Color Schemes**: Monochrome, Vibrant, Pastel, High-Contrast

### **Component Variants**

- **Button Variants**: Primary, Secondary, Ghost, Link, Destructive
- **Input Variants**: Default, Filled, Outlined, Underlined
- **Layout Variants**: Card, Panel, Modal, Drawer
- **Feedback Variants**: Success, Warning, Error, Info

### **Layout System**

- **Container**: Responsive containers with max-width constraints
- **Spacing**: Consistent spacing utilities
- **Responsive**: Breakpoint-based responsive design

## 🛠️ Utilities

### **Shared Utilities**

```rust
// utils.rs
pub fn merge_classes(classes: Vec<&str>) -> String
pub fn merge_optional_classes(existing: Option<&str>, additional: Option<&str>) -> Option<String>
pub fn generate_id(prefix: &str) -> String
```

### **Usage Examples**

```rust
use radix_leptos::utils::{merge_classes, generate_id};

// Merge multiple classes
let classes = merge_classes(vec!["button", "primary", "large"]);

// Generate unique IDs
let id = generate_id("button");
```

## 📝 Usage Examples

### **Basic Usage (Core Features)**

```toml
[dependencies]
radix-leptos = { version = "0.8.5", features = ["core"] }
```

```rust
use radix_leptos::{Button, Checkbox, Dialog, Form};

#[component]
pub fn MyApp() -> impl IntoView {
    view! {
        <Dialog>
            <Form>
                <Button>"Submit"</Button>
                <Checkbox />
            </Form>
        </Dialog>
    }
}
```

### **Full Features (All Components)**

```toml
[dependencies]
radix-leptos = { version = "0.8.5", features = ["full"] }
```

```rust
use radix_leptos::{Button, Chart, DataTable, VirtualList};

#[component]
pub fn MyApp() -> impl IntoView {
    view! {
        <div>
            <Button>"Click me"</Button>
            <Chart data=chart_data />
            <DataTable rows=table_data />
            <VirtualList items=long_list />
        </div>
    }
}
```

### **Experimental Features Only**

```toml
[dependencies]
radix-leptos = { version = "0.8.5", features = ["experimental"] }
```

```rust
use radix_leptos::{Chart, DataTable, RichTextEditor};

#[component]
pub fn MyApp() -> impl IntoView {
    view! {
        <div>
            <Chart data=chart_data />
            <DataTable rows=table_data />
            <RichTextEditor content=editor_content />
        </div>
    }
}
```

## 🔄 Migration Guide

### **From v0.8.4 to v0.8.5**

#### **1. Update Dependencies**

```toml
# Before
radix-leptos = "0.8.4"

# After
radix-leptos = { version = "0.8.5", features = ["core"] }
```

#### **2. Update Imports**

```rust
// Before
use radix_leptos::Button;

// After (no change needed)
use radix_leptos::Button;
```

#### **3. Feature Flag Usage**

```rust
// Before (all components available)
use radix_leptos::{Button, Chart, DataTable};

// After (with feature flags)
use radix_leptos::Button; // Always available

#[cfg(feature = "experimental")]
use radix_leptos::{Chart, DataTable}; // Feature-gated
```

### **Bundle Size Optimization**

```toml
# Minimal bundle (production)
radix-leptos = { version = "0.8.5", features = ["core"] }

# Full bundle (development)
radix-leptos = { version = "0.8.5", features = ["full"] }
```

## 🎯 Best Practices

### **Production Applications**

1. **Use `core` features** for production applications
2. **Avoid `experimental` features** in production
3. **Test thoroughly** before upgrading

### **Development**

1. **Use `full` features** for development
2. **Test experimental components** for future use
3. **Report issues** with experimental components

### **Library Development**

1. **Depend on `core` features** for stability
2. **Document feature requirements** clearly
3. **Provide fallbacks** for experimental features

## 🔍 Troubleshooting

### **Common Issues**

#### **Component Not Found**

```rust
// Error: cannot find `Chart` in scope
use radix_leptos::Chart;

// Solution: Enable experimental features
// Cargo.toml
radix-leptos = { version = "0.8.5", features = ["experimental"] }
```

#### **Feature Flag Conflicts**

```toml
# Error: conflicting feature flags
radix-leptos = { version = "0.8.5", features = ["core", "experimental"] }

# Solution: Use 'full' instead
radix-leptos = { version = "0.8.5", features = ["full"] }
```

#### **Build Errors**

```bash
# Error: experimental components have syntax errors
cargo build --features experimental

# Solution: Use core features only
cargo build --features core
```

## 📚 Additional Resources

- [API Reference](api-reference/COMPONENT_API_DOCS.md)
- [Repository Structure](REPOSITORY_STRUCTURE.md)
- [Development Guide](developer-guide/README.md)
- [Testing Guide](guides/TESTING_GUIDE.md)

---

**Last Updated**: January 2025  
**Version**: 0.8.5  
**Status**: ✅ Production Ready
