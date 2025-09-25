//! # Radix-Leptos Primitives
//!
//! Primitive UI components built on top of radix-leptos-core.
//! These components provide the building blocks for accessible UI libraries.

pub mod components;
pub mod theming;
pub mod utils;
pub mod performance;

// Re-export all components at the crate root
pub use components::*;
pub use theming::*;
