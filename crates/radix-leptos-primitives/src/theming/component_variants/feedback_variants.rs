use crate::utils::merge_classes;
use leptos::callback::Callback;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use super::input_variants::{SizeVariant, StyleVariant};

/// Badge variants configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BadgeVariants {
    pub sizes: Vec<SizeVariant>,
    pub styles: Vec<StyleVariant>,
    pub shapes: Vec<ShapeVariant>,
}

impl Default for BadgeVariants {
    fn default() -> Self {
        Self {
            sizes: [SizeVariant::Small, SizeVariant::Medium, SizeVariant::Large].to_vec(),
            styles: [
                StyleVariant::Default,
                StyleVariant::Primary,
                StyleVariant::Secondary,
                StyleVariant::Success,
                StyleVariant::Warning,
                StyleVariant::Error,
            ]
            .to_vec(),
            shapes: [
                ShapeVariant::Rounded,
                ShapeVariant::Pill,
                ShapeVariant::Square,
            ]
            .to_vec(),
        }
    }
}

/// Alert variants configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertVariants {
    pub sizes: Vec<SizeVariant>,
    pub styles: Vec<StyleVariant>,
    pub types: Vec<AlertTypeVariant>,
}

impl Default for AlertVariants {
    fn default() -> Self {
        Self {
            sizes: [SizeVariant::Small, SizeVariant::Medium, SizeVariant::Large].to_vec(),
            styles: [
                StyleVariant::Default,
                StyleVariant::Outlined,
                StyleVariant::Filled,
            ]
            .to_vec(),
            types: [
                AlertTypeVariant::Info,
                AlertTypeVariant::Success,
                AlertTypeVariant::Warning,
                AlertTypeVariant::Error,
            ]
            .to_vec(),
        }
    }
}

/// Shape variant enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ShapeVariant {
    Rounded,
    Pill,
    Square,
}

impl ShapeVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            ShapeVariant::Rounded => "rounded",
            ShapeVariant::Pill => "pill",
            ShapeVariant::Square => "square",
        }
    }
}

/// Alert type variant enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AlertTypeVariant {
    Info,
    Success,
    Warning,
    Error,
}

impl AlertTypeVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertTypeVariant::Info => "info",
            AlertTypeVariant::Success => "success",
            AlertTypeVariant::Warning => "warning",
            AlertTypeVariant::Error => "error",
        }
    }
}

#[cfg(test)]
mod feedback_variants_tests {
    use super::*;

    #[test]
    fn test_badge_variants_default() {
        let badge_variants = BadgeVariants::default();
        assert!(badge_variants.sizes.contains(&SizeVariant::Small));
        assert!(badge_variants.sizes.contains(&SizeVariant::Medium));
        assert!(badge_variants.sizes.contains(&SizeVariant::Large));
        assert!(badge_variants.shapes.contains(&ShapeVariant::Rounded));
        assert!(badge_variants.shapes.contains(&ShapeVariant::Pill));
        assert!(badge_variants.shapes.contains(&ShapeVariant::Square));
    }

    #[test]
    fn test_alert_variants_default() {
        let alert_variants = AlertVariants::default();
        assert!(alert_variants.sizes.contains(&SizeVariant::Small));
        assert!(alert_variants.sizes.contains(&SizeVariant::Medium));
        assert!(alert_variants.sizes.contains(&SizeVariant::Large));
        assert!(alert_variants.types.contains(&AlertTypeVariant::Info));
        assert!(alert_variants.types.contains(&AlertTypeVariant::Success));
        assert!(alert_variants.types.contains(&AlertTypeVariant::Warning));
        assert!(alert_variants.types.contains(&AlertTypeVariant::Error));
    }

    #[test]
    fn test_shape_variant_enum() {
        assert_eq!(ShapeVariant::Rounded.as_str(), "rounded");
        assert_eq!(ShapeVariant::Pill.as_str(), "pill");
        assert_eq!(ShapeVariant::Square.as_str(), "square");
    }

    #[test]
    fn test_alert_type_variant_enum() {
        assert_eq!(AlertTypeVariant::Info.as_str(), "info");
        assert_eq!(AlertTypeVariant::Success.as_str(), "success");
        assert_eq!(AlertTypeVariant::Warning.as_str(), "warning");
        assert_eq!(AlertTypeVariant::Error.as_str(), "error");
    }
}
