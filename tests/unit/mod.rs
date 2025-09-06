//! Unit tests for Radix-Leptos components
//! 
//! This module contains comprehensive unit tests following TDD principles:
//! - RED: Write failing test first
//! - GREEN: Make test pass with minimal code
//! - REFACTOR: Improve code while keeping tests green

pub mod test_components;
pub mod test_theming;
pub mod tdd_component_tests;

// Re-export test utilities
pub use test_components::*;
pub use test_theming::*;
pub use tdd_component_tests::*;
