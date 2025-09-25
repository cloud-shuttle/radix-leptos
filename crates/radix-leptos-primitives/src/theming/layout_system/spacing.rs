use serde::{Deserialize, Serialize};

/// Spacing system configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpacingSystem {
    pub base_unit: f64,
    pub scale: Vec<f64>,
    pub directions: Vec<SpacingDirection>,
}

impl Default for SpacingSystem {
    fn default() -> Self {
        Self {
            base_unit: 4.0, // 4px base unit
            scale: [
                0.0, 0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 2.0, 2.5, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0,
                12.0, 16.0, 20.0, 24.0, 32.0, 40.0, 48.0, 56.0, 64.0,
            ]
            .to_vec(),
            directions: [
                SpacingDirection::All,
                SpacingDirection::Horizontal,
                SpacingDirection::Vertical,
                SpacingDirection::Top,
                SpacingDirection::Right,
                SpacingDirection::Bottom,
                SpacingDirection::Left,
            ]
            .to_vec(),
        }
    }
}

/// Spacing direction enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SpacingDirection {
    All,
    Horizontal,
    Vertical,
    Top,
    Right,
    Bottom,
    Left,
}

impl SpacingDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            SpacingDirection::All => "all",
            SpacingDirection::Horizontal => "horizontal",
            SpacingDirection::Vertical => "vertical",
            SpacingDirection::Top => "top",
            SpacingDirection::Right => "right",
            SpacingDirection::Bottom => "bottom",
            SpacingDirection::Left => "left",
        }
    }
}

/// Utility functions for spacing calculations
pub mod spacing_utils {
    use super::*;

    /// Calculate spacing value based on scale index
    pub fn calculate_spacing(spacing_system: &SpacingSystem, scale_index: usize) -> f64 {
        if scale_index < spacing_system.scale.len() {
            spacing_system.base_unit * spacing_system.scale[scale_index]
        } else {
            spacing_system.base_unit * spacing_system.scale.last().unwrap_or(&1.0)
        }
    }

    /// Get all available spacing values
    pub fn get_spacing_values(spacing_system: &SpacingSystem) -> Vec<f64> {
        spacing_system
            .scale
            .iter()
            .map(|&scale| spacing_system.base_unit * scale)
            .collect()
    }

    /// Get spacing value for a specific direction
    pub fn get_direction_spacing(
        spacing_system: &SpacingSystem,
        direction: SpacingDirection,
        scale_index: usize,
    ) -> f64 {
        calculate_spacing(spacing_system, scale_index)
    }

    /// Validate spacing configuration
    pub fn validate_spacing_system(spacing_system: &SpacingSystem) -> bool {
        spacing_system.base_unit > 0.0
            && !spacing_system.scale.is_empty()
            && !spacing_system.directions.is_empty()
    }
}

#[cfg(test)]
mod spacing_tests {
    use super::*;

    #[test]
    fn test_spacing_system_default() {
        let spacing = SpacingSystem::default();
        assert_eq!(spacing.base_unit, 4.0);
        assert!(spacing.scale.contains(&0.0));
        assert!(spacing.scale.contains(&1.0));
        assert!(spacing.scale.contains(&4.0));
        assert!(spacing.scale.contains(&64.0));
        assert!(spacing.directions.contains(&SpacingDirection::All));
        assert!(spacing.directions.contains(&SpacingDirection::Horizontal));
        assert!(spacing.directions.contains(&SpacingDirection::Vertical));
    }

    #[test]
    fn test_spacing_direction_enum() {
        assert_eq!(SpacingDirection::All.as_str(), "all");
        assert_eq!(SpacingDirection::Horizontal.as_str(), "horizontal");
        assert_eq!(SpacingDirection::Vertical.as_str(), "vertical");
        assert_eq!(SpacingDirection::Top.as_str(), "top");
        assert_eq!(SpacingDirection::Right.as_str(), "right");
        assert_eq!(SpacingDirection::Bottom.as_str(), "bottom");
        assert_eq!(SpacingDirection::Left.as_str(), "left");
    }

    #[test]
    fn test_calculate_spacing() {
        let spacing_system = SpacingSystem::default();
        let spacing_0 = spacing_utils::calculate_spacing(&spacing_system, 0);
        let spacing_1 = spacing_utils::calculate_spacing(&spacing_system, 1);
        let spacing_4 = spacing_utils::calculate_spacing(&spacing_system, 4);

        assert_eq!(spacing_0, 0.0); // 4.0 * 0.0
        assert_eq!(spacing_1, 1.0); // 4.0 * 0.25
        assert_eq!(spacing_4, 4.0); // 4.0 * 1.0
    }

    #[test]
    fn test_get_spacing_values() {
        let spacing_system = SpacingSystem::default();
        let values = spacing_utils::get_spacing_values(&spacing_system);
        
        assert_eq!(values.len(), spacing_system.scale.len());
        assert_eq!(values[0], 0.0);
        assert_eq!(values[1], 1.0);
        assert_eq!(values[4], 4.0);
    }

    #[test]
    fn test_get_direction_spacing() {
        let spacing_system = SpacingSystem::default();
        let spacing = spacing_utils::get_direction_spacing(
            &spacing_system,
            SpacingDirection::Top,
            4,
        );
        
        assert_eq!(spacing, 4.0);
    }

    #[test]
    fn test_validate_spacing_system() {
        let valid_spacing = SpacingSystem::default();
        assert!(spacing_utils::validate_spacing_system(&valid_spacing));

        let invalid_spacing = SpacingSystem {
            base_unit: 0.0,
            scale: Vec::new(),
            directions: Vec::new(),
        };
        assert!(!spacing_utils::validate_spacing_system(&invalid_spacing));
    }

    // Property-based tests
    #[test]
    fn test_spacing_direction_property_based() {
        use proptest::prelude::*;
        
        proptest!(|(direction in prop::sample::select(&[
            SpacingDirection::All,
            SpacingDirection::Horizontal,
            SpacingDirection::Vertical,
            SpacingDirection::Top,
            SpacingDirection::Right,
            SpacingDirection::Bottom,
            SpacingDirection::Left,
        ]))| {
            let direction_str = direction.as_str();
            assert!(!direction_str.is_empty());
        });
    }
}
