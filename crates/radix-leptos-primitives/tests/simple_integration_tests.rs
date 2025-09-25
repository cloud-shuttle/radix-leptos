//! Simple integration tests for Radix-Leptos components

use radix_leptos_primitives::*;
use radix_leptos_primitives::utils::{generate_id, merge_optional_classes};

#[cfg(test)]
mod simple_integration_tests {
    use super::*;
use crate::utils::{generate_id, merge_classes, merge_optional_classes};
use crate::form_validation::{is_valid_email, is_valid_date, is_valid_time, is_valid_phone};

    #[test]
    fn test_pagination_helpers() {
        // Test pagination helper functions
        let pages = generate_page_numbers(1, 10, 7);
        assert!(!pages.is_empty());
        
        let range = calculate_page_range(5, 10, 7);
        assert!(range.0 <= range.1);
    }

    #[test]
    fn test_validation_engine() {
        // Test validation engine
        let mut engine = ValidationEngine::new();
        engine.add_rule("test".to_string(), ValidationRule {
            rule_type: ValidationRuleType::Required,
            message: "Test message".to_string(),
            value: None,
        });
        
        assert!(engine.has_rules());
        assert!(engine.has_rule_for_field("test"));
    }

    #[test]
    fn test_utility_functions() {
        // Test utility functions
        let id1 = generate_id("test");
        let id2 = generate_id("test");
        
        assert_ne!(id1, id2);
        assert!(id1.starts_with("test-"));
        
        let merged = merge_optional_classes(Some("base"), Some("additional"));
        assert_eq!(merged, Some("base additional".to_string()));
    }

    #[test]
    fn test_enum_values() {
        // Test enum values
        assert_eq!(ButtonVariant::Default.as_str(), "default");
        assert_eq!(ButtonVariant::Destructive.as_str(), "destructive");
        assert_eq!(ButtonSize::Default.as_str(), "default");
        assert_eq!(SkeletonVariant::Text.as_str(), "text");
    }

    #[test]
    fn test_validation_functions() {
        // Test validation functions
        assert!(is_valid_email("test@example.com"));
        assert!(!is_valid_email("invalid"));
        
        assert!(is_valid_date("2023-12-25"));
        assert!(!is_valid_date("2023-13-01"));
        
        assert!(is_valid_time("14:30"));
        assert!(!is_valid_time("25:00"));
    }
}
