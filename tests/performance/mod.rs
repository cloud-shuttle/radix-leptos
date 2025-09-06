//! Performance tests for bundle optimization and runtime performance
//! 
//! This module contains performance tests that verify:
//! - Bundle size optimization (<400KB target)
//! - Build time optimization (<0.5s target)
//! - Runtime performance
//! - Memory efficiency
//! - Component rendering speed
//! - State update performance
//! - Event handling performance

pub mod component_benchmarks;
pub mod component_performance;
pub mod bundle_optimization;

// Re-export test utilities
pub use component_benchmarks::*;
pub use component_performance::*;
pub use bundle_optimization::*;
