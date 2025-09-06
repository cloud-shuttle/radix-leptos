//! Comprehensive test suite for Radix-Leptos
//! 
//! This module contains all test categories:
//! - Unit tests for individual components
//! - Integration tests for component interactions
//! - Accessibility tests for WCAG 2.1 AA compliance
//! - Performance tests for bundle optimization

pub mod unit;
pub mod integration;
pub mod accessibility;
pub mod performance;

// Re-export test utilities
pub use unit::*;
pub use integration::*;
pub use accessibility::*;
pub use performance::*;
