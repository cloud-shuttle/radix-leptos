# 🚀 Radix-Leptos v0.6.0 Release Notes

**Release Date:** September 4, 2025  
**Version:** 0.6.0  
**Codename:** "Enhanced Components & Advanced UI Patterns"

## 🎯 Overview

Radix-Leptos v0.6.0 introduces 12 new and enhanced components, bringing the total component count to **57+ components** with **1200+ passing tests**. This release focuses on advanced UI patterns, form enhancements, and specialized components that complete the core component library.

## ✨ New Features

### 🆕 8 New Components

#### **Context Menu** - Right-click context menus with keyboard navigation
- Full keyboard navigation support
- Customizable menu items with icons and separators
- Accessibility-compliant ARIA attributes
- Event handling for clicks and keyboard interactions

#### **Collapsible** - Collapsible content areas with smooth animations
- Smooth expand/collapse animations
- Customizable trigger elements
- Controlled and uncontrolled modes
- Accessibility features for screen readers

#### **Aspect Ratio** - Maintain aspect ratio containers
- Flexible aspect ratio calculations
- Responsive design support
- Customizable ratios (16:9, 4:3, 1:1, etc.)
- Perfect for media containers

#### **Calendar** - Date picker and calendar component
- Full calendar view with month/year navigation
- Date selection with validation
- Customizable date ranges and disabled dates
- Internationalization support

#### **Date Picker** - Date selection with validation
- Input field with calendar popup
- Date format validation
- Min/max date constraints
- Keyboard navigation support

#### **File Upload** - File upload with drag & drop support
- Drag and drop file upload
- Multiple file selection
- File type validation
- Upload progress tracking

#### **Search** - Search input with suggestions and filtering
- Real-time search suggestions
- Customizable suggestion rendering
- Keyboard navigation
- Debounced input handling

#### **Combobox** - Searchable select component with autocomplete
- Searchable dropdown with autocomplete
- Custom option rendering
- Keyboard navigation
- Multi-select support

### 🔄 4 Enhanced Components

#### **Avatar** - User profile images with fallbacks
- Multiple size variants (sm, md, lg, xl)
- Fallback text and icons
- Customizable styling
- Accessibility features

#### **Separator** - Visual dividers with orientation support
- Horizontal and vertical orientations
- Customizable thickness
- Decorative and functional variants
- ARIA-compliant implementation

#### **Label** - Form labels with accessibility features
- Proper form association
- Required field indicators
- Customizable styling
- Screen reader support

#### **Toast** - Enhanced notification system with positioning
- Multiple positioning options
- Auto-dismiss functionality
- Customizable duration
- Stack management

## 🧪 Testing & Quality

### **Comprehensive Test Coverage**
- **1200+ passing tests** across all components
- **Property-based testing** for form components
- **Accessibility testing** for all new components
- **Integration tests** for complex interactions

### **TDD Implementation**
- All components developed using Test-Driven Development
- Red-Green-Refactor cycles followed
- Test-first approach ensures reliability
- Comprehensive test suites for each component

## 📊 Technical Improvements

### **Performance Optimizations**
- Optimized rendering for large lists
- Efficient event handling
- Reduced bundle size impact
- Memory usage optimizations

### **Accessibility Enhancements**
- WCAG 2.1 AA compliance
- Screen reader optimization
- Keyboard navigation improvements
- Focus management enhancements

### **Developer Experience**
- Improved TypeScript-like type safety
- Better error messages
- Enhanced documentation
- Consistent API patterns

## 🔧 Breaking Changes

**None** - This release maintains full backward compatibility with v0.5.0.

## 📦 Installation

```toml
[dependencies]
radix-leptos = "0.6.0"
```

## 🎨 Usage Examples

### Context Menu
```rust
use radix_leptos::*;

view! {
    <ContextMenu>
        <ContextMenuTrigger>
            "Right-click me"
        </ContextMenuTrigger>
        <ContextMenuContent>
            <ContextMenuItem on_click=move |_| { /* action */ }>
                "Copy"
            </ContextMenuItem>
            <ContextMenuItem on_click=move |_| { /* action */ }>
                "Paste"
            </ContextMenuItem>
        </ContextMenuContent>
    </ContextMenu>
}
```

### File Upload
```rust
use radix_leptos::*;

view! {
    <FileUpload
        on_change=move |files| {
            // Handle uploaded files
        }
        accept=".jpg,.png,.pdf"
        multiple=true
    />
}
```

### Search with Suggestions
```rust
use radix_leptos::*;

view! {
    <Search
        placeholder="Search..."
        suggestions=vec!["Option 1", "Option 2", "Option 3"]
        on_click=move |suggestion| {
            // Handle suggestion selection
        }
    />
}
```

## 🚀 What's Next

### **v0.7.0 - Advanced Patterns (Q1 2026)**
- Command Palette
- Data Grid
- Tree View
- Resizable Panels
- Virtual Scrolling

### **v1.0.0 - Complete Library (Q2 2026)**
- Full Radix UI parity
- Advanced theming system
- Performance optimizations
- Complete documentation

## 📈 Statistics

- **57+ Components** (up from 45+)
- **1200+ Tests** (up from 1100+)
- **95% Complete** (up from 90%)
- **Zero Breaking Changes**
- **100% TDD Coverage**

## 🙏 Acknowledgments

Special thanks to the Radix-Leptos community for feedback, testing, and contributions. This release represents a significant milestone in our journey toward a complete, accessible UI component library for Leptos.

## 📚 Documentation

- [Component Documentation](https://docs.rs/radix-leptos/0.6.0)
- [Getting Started Guide](https://radix-leptos.dev/docs/getting-started)
- [Migration Guide](https://radix-leptos.dev/docs/migration)
- [Examples Repository](https://github.com/cloud-shuttle/radix-leptos-examples)

---

**Full Changelog**: https://github.com/cloud-shuttle/radix-leptos/compare/v0.5.0...v0.6.0

**Download**: [crates.io](https://crates.io/crates/radix-leptos/0.6.0) | [GitHub Releases](https://github.com/cloud-shuttle/radix-leptos/releases/tag/v0.6.0)
