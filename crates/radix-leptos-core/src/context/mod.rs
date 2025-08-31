//! # Context Management
//! 
//! Context providers and utilities for component communication.

pub mod collection;

pub use collection::*;

/// Direction for component orientation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Ltr,
    Rtl,
}

impl Direction {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "rtl" => Direction::Rtl,
            _ => Direction::Ltr,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::Ltr => "ltr",
            Direction::Rtl => "rtl",
        }
    }
}

/// Orientation for component layout  
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "vertical" => Orientation::Vertical,
            _ => Orientation::Horizontal,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            Orientation::Horizontal => "horizontal",
            Orientation::Vertical => "vertical",
        }
    }
}