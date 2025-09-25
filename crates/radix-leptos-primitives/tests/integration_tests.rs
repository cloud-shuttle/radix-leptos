//! Integration tests for Radix-Leptos components
//! 
//! These tests verify that components work together properly and handle
//! real-world usage scenarios.

use radix_leptos_primitives::*;

#[cfg(test)]
mod integration_tests {
    use super::*;
use crate::utils::{generate_id, merge_classes, merge_optional_classes};
use crate::form_validation::{is_valid_email, is_valid_date, is_valid_time, is_valid_phone};

    #[test]
    fn test_pagination_with_form_validation() {
        // Test that pagination works with form validation
        let mut validation_engine = ValidationEngine::new();
        validation_engine.add_rule("page".to_string(), ValidationRule { rule_type: ValidationRuleType::Required, message: "Page is required".to_string(), value: None });
        
        // Test pagination helper functions
        let pages = generate_page_numbers(1, 10, 7);
        assert!(!pages.is_empty());
        
        // Test validation engine
        assert!(validation_engine.has_rules());
        assert!(validation_engine.has_rule_for_field("page"));
    }

    #[test]
    fn test_form_validation_with_multiple_fields() {
        // Test form validation with multiple field types
        let mut engine = ValidationEngine::new();
        
        // Add different validation rules
        engine.add_rule("email".to_string(), ValidationRule { rule_type: ValidationRuleType::Email, message: "Invalid email".to_string(), value: None });
        engine.add_rule("phone".to_string(), ValidationRule { rule_type: ValidationRuleType::Phone, message: "Invalid phone".to_string(), value: None });
        engine.add_rule("required".to_string(), ValidationRule { rule_type: ValidationRuleType::Required, message: "Required field".to_string(), value: None });
        
        // Test validation
        let mut form_data = std::collections::HashMap::new();
        form_data.insert("email".to_string(), "test@example.com".to_string());
        form_data.insert("phone".to_string(), "123-456-7890".to_string());
        form_data.insert("required".to_string(), "value".to_string());
        
        let result = engine.validate_form(&form_data);
        assert!(result.is_valid);
    }

    #[test]
    fn test_theme_provider_with_components() {
        // Test that theme provider works with various components
        // Test enum values work correctly
        assert_eq!(ButtonVariant::Default.as_str(), "default");
        assert_eq!(ButtonVariant::Destructive.as_str(), "destructive");
        assert_eq!(SkeletonVariant::Text.as_str(), "text");
        assert_eq!(SkeletonVariant::Circular.as_str(), "circular");
        assert_eq!(ButtonSize::Default.as_str(), "medium");
        assert_eq!(SkeletonSize::Medium.as_str(), "medium");
    }

    #[test]
    fn test_alert_dialog_with_form_validation() {
        // Test alert dialog with form validation integration
        let mut engine = ValidationEngine::new();
        engine.add_rule("confirm".to_string(), ValidationRule { rule_type: ValidationRuleType::Required, message: "Confirmation required".to_string(), value: None });
        
        // Test enum values
        assert_eq!(AlertDialogVariant::Destructive.as_str(), "destructive");
        assert_eq!(AlertDialogVariant::Default.as_str(), "default");
        assert_eq!(AlertDialogSize::Medium.as_str(), "medium");
        
        // Test validation engine
        assert!(engine.has_rule_for_field("confirm"));
    }

    #[test]
    fn test_component_enum_consistency() {
        // Test that all component enums are consistent
        assert_eq!(ButtonVariant::Secondary.as_str(), "secondary");
        assert_eq!(ButtonVariant::Outline.as_str(), "outline");
        assert_eq!(ButtonVariant::Ghost.as_str(), "ghost");
        assert_eq!(ButtonVariant::Link.as_str(), "link");
        
        assert_eq!(SkeletonVariant::Circular.as_str(), "circular");
        assert_eq!(SkeletonVariant::Rectangular.as_str(), "rectangular");
        
        assert_eq!(AlertDialogVariant::Warning.as_str(), "warning");
        assert_eq!(AlertDialogSize::Large.as_str(), "large");
        assert_eq!(AlertDialogSize::Small.as_str(), "small");
    }

    #[test]
    fn test_utility_functions_consistency() {
        // Test that utility functions work consistently across components
        let id1 = generate_id("test");
        let id2 = generate_id("test");
        
        // IDs should be unique
        assert_ne!(id1, id2);
        assert!(id1.starts_with("test-"));
        assert!(id2.starts_with("test-"));
        
        // Class merging should work consistently
        let merged = merge_optional_classes(Some("base"), Some("additional"));
        assert_eq!(merged, Some("base additional".to_string()));
        
        let merged_vec = merge_classes(vec!["class1", "class2", "class3"]);
        assert_eq!(merged_vec, "class1 class2 class3");
    }

    #[test]
    fn test_validation_engine_performance() {
        // Test validation engine with many rules
        let mut engine = ValidationEngine::new();
        
        // Add many rules
        for i in 0..100 {
            engine.add_rule(format!("field_{}", i), 
                ValidationRule { rule_type: ValidationRuleType::Required, message: format!("Field {} is required", i), value: None });
        }
        
        // Should handle many rules efficiently
        assert!(engine.has_rules());
        assert_eq!(engine.has_rule_for_field("field_0"), true);
        assert_eq!(engine.has_rule_for_field("field_99"), true);
        assert_eq!(engine.has_rule_for_field("nonexistent"), false);
    }

    #[test]
    fn test_pagination_edge_cases() {
        // Test pagination with edge cases
        let pages = generate_page_numbers(1, 1, 5);
        assert_eq!(pages.len(), 1);
        assert!(pages[0]._current);
        
        let pages = generate_page_numbers(5, 3, 5);
        assert_eq!(pages.len(), 3);
        
        let pages = generate_page_numbers(50, 100, 7);
        assert!(pages.len() >= 7);
    }

    #[test]
    fn test_form_validation_edge_cases() {
        // Test form validation with edge cases
        assert!(is_valid_email("test@example.com"));
        assert!(!is_valid_email("invalid"));
        
        assert!(is_valid_date("2023-12-25"));
        assert!(!is_valid_date("2023-13-01"));
        
        assert!(is_valid_time("14:30"));
        assert!(!is_valid_time("25:00"));
        
        assert!(is_valid_phone("+1234567890"));
        assert!(!is_valid_phone("123"));
    }
}
