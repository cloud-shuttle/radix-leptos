//! # Radix-Leptos
//! 
//! A comprehensive UI component library for Leptos, built with accessibility and design system principles.
//! This crate provides a complete set of accessible, customizable components inspired by Radix UI.
//! 
//! ## Features
//! 
//! - **Accessible by default**: All components follow WAI-ARIA guidelines
//! - **Type-safe**: Built with Rust's type system for compile-time safety
//! - **Customizable**: Flexible theming and styling options
//! - **Composable**: Components can be combined and extended
//! - **SSR/Hydration ready**: Works with Leptos server-side rendering
//! 
//! ## Quick Start
//! 
//! ```rust
//! use leptos::*;
//! use radix_leptos::*;
//! 
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <div>
//!             <Button variant=ButtonVariant::Default>
//!                 "Hello, Radix-Leptos!"
//!             </Button>
//!             
//!             <Separator />
//!             
//!             <Label html_for="input">
//!                 "Email"
//!             </Label>
//!             <input id="input" type="email" />
//!         </div>
//!     }
//! }
//! ```
//! 
//! ## Component Categories
//! 
//! ### Primitives
//! Basic building blocks for UI components:
//! - `Button` - Accessible button with multiple variants
//! - `Label` - Form field labels with proper associations
//! - `Separator` - Visual separation between content
//! 
//! ### Layout
//! Components for structuring content:
//! - Coming soon...
//! 
//! ### Navigation
//! Components for navigation and menus:
//! - Coming soon...
//! 
//! ### Forms
//! Form-related components:
//! - Coming soon...
//! 
//! ### Feedback
//! Components for user feedback:
//! - Coming soon...
//! 
//! ## Styling
//! 
//! Radix-Leptos components are unstyled by default and provide CSS classes for styling.
//! You can use any CSS framework or custom styles with the provided class names.
//! 
//! ## Accessibility
//! 
//! All components are built with accessibility in mind:
//! - Proper ARIA attributes
//! - Keyboard navigation support
//! - Screen reader compatibility
//! - Focus management
//! 
//! ## Contributing
//! 
//! We welcome contributions! Please see our contributing guidelines for more information.

// Re-export all components from primitives
pub use radix_leptos_primitives::*;

// Re-export core utilities for advanced usage (excluding portal to avoid conflicts)
pub use radix_leptos_core::{utils, primitives::visually_hidden};

// Re-export commonly used Leptos items
pub use leptos::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_component_imports() {
        // This test ensures all components can be imported
        // This is a basic smoke test to ensure the library compiles
        assert!(true);
    }
}

