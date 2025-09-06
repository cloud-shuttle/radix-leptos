//! Accessibility tests for WCAG 2.1 AA compliance
//! 
//! This module contains comprehensive accessibility tests that verify:
//! - WCAG 2.1 AA compliance
//! - Keyboard navigation
//! - Screen reader compatibility
//! - Focus management
//! - ARIA attributes
//! - Color contrast
//! - Semantic markup

pub mod wcag_compliance;
pub mod wcag_comprehensive;

// Re-export test utilities
pub use wcag_compliance::*;
pub use wcag_comprehensive::*;
