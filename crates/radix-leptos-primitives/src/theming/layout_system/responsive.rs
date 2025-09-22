use serde::{Deserialize, Serialize};
use super::container::ContainerMaxWidth;

/// Breakpoint system configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakpointSystem {
    pub breakpoints: Vec<Breakpoint>,
    pub container_max_widths: Vec<ContainerMaxWidth>,
}

impl Default for BreakpointSystem {
    fn default() -> Self {
        Self {
            breakpoints: [
                Breakpoint::ExtraSmall,
                Breakpoint::Small,
                Breakpoint::Medium,
                Breakpoint::Large,
                Breakpoint::ExtraLarge,
                Breakpoint::ExtraExtraLarge,
            ]
            .to_vec(),
            container_max_widths: [
                ContainerMaxWidth::Small,
                ContainerMaxWidth::Medium,
                ContainerMaxWidth::Large,
                ContainerMaxWidth::ExtraLarge,
                ContainerMaxWidth::Fluid,
            ]
            .to_vec(),
        }
    }
}

/// Breakpoint enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Breakpoint {
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
    ExtraExtraLarge,
}

impl Breakpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            Breakpoint::ExtraSmall => "xs",
            Breakpoint::Small => "sm",
            Breakpoint::Medium => "md",
            Breakpoint::Large => "lg",
            Breakpoint::ExtraLarge => "xl",
            Breakpoint::ExtraExtraLarge => "2xl",
        }
    }

    pub fn min_width(&self) -> f64 {
        match self {
            Breakpoint::ExtraSmall => 0.0,
            Breakpoint::Small => 640.0,
            Breakpoint::Medium => 768.0,
            Breakpoint::Large => 1024.0,
            Breakpoint::ExtraLarge => 1280.0,
            Breakpoint::ExtraExtraLarge => 1536.0,
        }
    }
}

// ContainerMaxWidth is defined in the container module

/// Responsive utility functions
pub mod responsive_utils {
    use super::*;

    /// Get breakpoint for a given width
    pub fn get_breakpoint_for_width(breakpoint_system: &BreakpointSystem, width: f64) -> Breakpoint {
        let mut current_breakpoint = Breakpoint::ExtraSmall;
        
        for &breakpoint in &breakpoint_system.breakpoints {
            if width >= breakpoint.min_width() {
                current_breakpoint = breakpoint;
            } else {
                break;
            }
        }
        
        current_breakpoint
    }

    /// Check if width matches breakpoint
    pub fn matches_breakpoint(breakpoint: Breakpoint, width: f64) -> bool {
        width >= breakpoint.min_width()
    }

    /// Get container max width for breakpoint
    pub fn get_container_max_width_for_breakpoint(
        breakpoint_system: &BreakpointSystem,
        breakpoint: Breakpoint,
    ) -> Option<ContainerMaxWidth> {
        // Simple mapping logic - can be made more sophisticated
        match breakpoint {
            Breakpoint::ExtraSmall => Some(ContainerMaxWidth::Small),
            Breakpoint::Small => Some(ContainerMaxWidth::Small),
            Breakpoint::Medium => Some(ContainerMaxWidth::Medium),
            Breakpoint::Large => Some(ContainerMaxWidth::Large),
            Breakpoint::ExtraLarge => Some(ContainerMaxWidth::ExtraLarge),
            Breakpoint::ExtraExtraLarge => Some(ContainerMaxWidth::ExtraLarge),
        }
    }

    /// Generate responsive CSS media queries
    pub fn generate_media_query(breakpoint: Breakpoint) -> String {
        format!("@media (min-width: {}px)", breakpoint.min_width())
    }

    /// Validate breakpoint system
    pub fn validate_breakpoint_system(breakpoint_system: &BreakpointSystem) -> bool {
        !breakpoint_system.breakpoints.is_empty() && !breakpoint_system.container_max_widths.is_empty()
    }
}

#[cfg(test)]
mod responsive_tests {
    use super::*;

    #[test]
    fn test_breakpoint_system_default() {
        let breakpoints = BreakpointSystem::default();
        assert!(breakpoints.breakpoints.contains(&Breakpoint::ExtraSmall));
        assert!(breakpoints.breakpoints.contains(&Breakpoint::Small));
        assert!(breakpoints.breakpoints.contains(&Breakpoint::Medium));
        assert!(breakpoints.breakpoints.contains(&Breakpoint::Large));
        assert!(breakpoints.breakpoints.contains(&Breakpoint::ExtraLarge));
        assert!(breakpoints
            .breakpoints
            .contains(&Breakpoint::ExtraExtraLarge));
        assert!(breakpoints
            .container_max_widths
            .contains(&ContainerMaxWidth::Small));
        assert!(breakpoints
            .container_max_widths
            .contains(&ContainerMaxWidth::Medium));
        assert!(breakpoints
            .container_max_widths
            .contains(&ContainerMaxWidth::Large));
        assert!(breakpoints
            .container_max_widths
            .contains(&ContainerMaxWidth::ExtraLarge));
        assert!(breakpoints
            .container_max_widths
            .contains(&ContainerMaxWidth::Fluid));
    }

    #[test]
    fn test_breakpoint_enum() {
        assert_eq!(Breakpoint::ExtraSmall.as_str(), "xs");
        assert_eq!(Breakpoint::Small.as_str(), "sm");
        assert_eq!(Breakpoint::Medium.as_str(), "md");
        assert_eq!(Breakpoint::Large.as_str(), "lg");
        assert_eq!(Breakpoint::ExtraLarge.as_str(), "xl");
        assert_eq!(Breakpoint::ExtraExtraLarge.as_str(), "2xl");
    }

    #[test]
    fn test_breakpoint_min_width() {
        assert_eq!(Breakpoint::ExtraSmall.min_width(), 0.0);
        assert_eq!(Breakpoint::Small.min_width(), 640.0);
        assert_eq!(Breakpoint::Medium.min_width(), 768.0);
        assert_eq!(Breakpoint::Large.min_width(), 1024.0);
        assert_eq!(Breakpoint::ExtraLarge.min_width(), 1280.0);
        assert_eq!(Breakpoint::ExtraExtraLarge.min_width(), 1536.0);
    }

    #[test]
    fn test_get_breakpoint_for_width() {
        let breakpoint_system = BreakpointSystem::default();
        
        assert_eq!(
            responsive_utils::get_breakpoint_for_width(&breakpoint_system, 0.0),
            Breakpoint::ExtraSmall
        );
        assert_eq!(
            responsive_utils::get_breakpoint_for_width(&breakpoint_system, 640.0),
            Breakpoint::Small
        );
        assert_eq!(
            responsive_utils::get_breakpoint_for_width(&breakpoint_system, 768.0),
            Breakpoint::Medium
        );
        assert_eq!(
            responsive_utils::get_breakpoint_for_width(&breakpoint_system, 1024.0),
            Breakpoint::Large
        );
        assert_eq!(
            responsive_utils::get_breakpoint_for_width(&breakpoint_system, 1280.0),
            Breakpoint::ExtraLarge
        );
        assert_eq!(
            responsive_utils::get_breakpoint_for_width(&breakpoint_system, 1536.0),
            Breakpoint::ExtraExtraLarge
        );
    }

    #[test]
    fn test_matches_breakpoint() {
        assert!(responsive_utils::matches_breakpoint(Breakpoint::ExtraSmall, 0.0));
        assert!(responsive_utils::matches_breakpoint(Breakpoint::Small, 640.0));
        assert!(responsive_utils::matches_breakpoint(Breakpoint::Small, 700.0));
        assert!(!responsive_utils::matches_breakpoint(Breakpoint::Small, 500.0));
    }

    #[test]
    fn test_get_container_max_width_for_breakpoint() {
        let breakpoint_system = BreakpointSystem::default();
        
        assert_eq!(
            responsive_utils::get_container_max_width_for_breakpoint(
                &breakpoint_system,
                Breakpoint::ExtraSmall
            ),
            Some(ContainerMaxWidth::Small)
        );
        assert_eq!(
            responsive_utils::get_container_max_width_for_breakpoint(
                &breakpoint_system,
                Breakpoint::Large
            ),
            Some(ContainerMaxWidth::Large)
        );
    }

    #[test]
    fn test_generate_media_query() {
        assert_eq!(
            responsive_utils::generate_media_query(Breakpoint::Small),
            "@media (min-width: 640px)"
        );
        assert_eq!(
            responsive_utils::generate_media_query(Breakpoint::Large),
            "@media (min-width: 1024px)"
        );
    }

    #[test]
    fn test_validate_breakpoint_system() {
        let valid_system = BreakpointSystem::default();
        assert!(responsive_utils::validate_breakpoint_system(&valid_system));

        let invalid_system = BreakpointSystem {
            breakpoints: vec![],
            container_max_widths: vec![],
        };
        assert!(!responsive_utils::validate_breakpoint_system(&invalid_system));
    }

    // Property-based tests
    #[test]
    fn test_breakpoint_property_based() {
        use proptest::prelude::*;
        
        proptest!(|(breakpoint in prop::sample::select(&[
            Breakpoint::ExtraSmall,
            Breakpoint::Small,
            Breakpoint::Medium,
            Breakpoint::Large,
            Breakpoint::ExtraLarge,
            Breakpoint::ExtraExtraLarge,
        ]))| {
            let breakpoint_str = breakpoint.as_str();
            let min_width = breakpoint.min_width();
            assert!(!breakpoint_str.is_empty());
            assert!(min_width >= 0.0);
        });
    }
}
