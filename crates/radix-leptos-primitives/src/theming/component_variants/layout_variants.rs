use serde::{Deserialize, Serialize};

use super::input_variants::{SizeVariant, StyleVariant};

/// Card variants configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardVariants {
    pub sizes: Vec<SizeVariant>,
    pub styles: Vec<StyleVariant>,
    pub elevations: Vec<ElevationVariant>,
}

impl Default for CardVariants {
    fn default() -> Self {
        Self {
            sizes: [SizeVariant::Small, SizeVariant::Medium, SizeVariant::Large].to_vec(),
            styles: [
                StyleVariant::Default,
                StyleVariant::Outlined,
                StyleVariant::Filled,
            ]
            .to_vec(),
            elevations: [
                ElevationVariant::None,
                ElevationVariant::Low,
                ElevationVariant::Medium,
                ElevationVariant::High,
            ]
            .to_vec(),
        }
    }
}

/// Elevation variant enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ElevationVariant {
    None,
    Low,
    Medium,
    High,
}

impl ElevationVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            ElevationVariant::None => "none",
            ElevationVariant::Low => "low",
            ElevationVariant::Medium => "medium",
            ElevationVariant::High => "high",
        }
    }
}

#[cfg(test)]
mod layout_variants_tests {
    use super::*;

    #[test]
    fn test_card_variants_default() {
        let card_variants = CardVariants::default();
        assert!(card_variants.sizes.contains(&SizeVariant::Small));
        assert!(card_variants.sizes.contains(&SizeVariant::Medium));
        assert!(card_variants.sizes.contains(&SizeVariant::Large));
        assert!(card_variants.elevations.contains(&ElevationVariant::None));
        assert!(card_variants.elevations.contains(&ElevationVariant::Low));
        assert!(card_variants.elevations.contains(&ElevationVariant::Medium));
        assert!(card_variants.elevations.contains(&ElevationVariant::High));
    }

    #[test]
    fn test_elevation_variant_enum() {
        assert_eq!(ElevationVariant::None.as_str(), "none");
        assert_eq!(ElevationVariant::Low.as_str(), "low");
        assert_eq!(ElevationVariant::Medium.as_str(), "medium");
        assert_eq!(ElevationVariant::High.as_str(), "high");
    }
}
