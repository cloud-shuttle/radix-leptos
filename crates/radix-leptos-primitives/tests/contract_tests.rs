//! Contract tests for Radix-Leptos components
//! 
//! These tests verify API contracts and accessibility compliance.

use radix_leptos_primitives::*;
use leptos::prelude::*;

#[cfg(test)]
mod contract_tests {
    use super::*;
use crate::utils::{generate_id, merge_classes, merge_optional_classes};
use crate::form_validation::{is_valid_email, is_valid_date, is_valid_time, is_valid_phone};

    #[test]
    fn test_button_api_contract() {
        // Test Button API contract
        let button = Button::new();
        
        // Default values should be consistent
        assert_eq!(button.variant, ButtonVariant::Default);
        assert_eq!(button.size, ButtonSize::Default);
        assert!(!button.disabled);
        assert!(button.class.is_none());
        assert!(button.style.is_none());
        
        // Builder pattern should work
        let custom_button = Button::new()
            .with_variant(ButtonVariant::Default)
            .with_size(ButtonSize::Large)
            .with_disabled(true)
            .with_class("custom")
            ;
        
        assert_eq!(custom_button.variant, ButtonVariant::Default);
        assert_eq!(custom_button.size, ButtonSize::Large);
        assert!(custom_button.disabled);
        assert_eq!(custom_button.class, Some("custom".to_string()));
        assert_eq!(custom_button.style, Some("color: red".to_string()));
    }

    #[test]
    fn test_skeleton_api_contract() {
        // Test Skeleton API contract
        let skeleton = Skeleton::new();
        
        // Default values should be consistent
        assert_eq!(skeleton.variant, SkeletonVariant::Text);
        assert_eq!(skeleton.size, SkeletonSize::Medium);
        assert_eq!(skeleton.lines, 1);
        assert!(skeleton.width.is_none());
        assert!(skeleton.height.is_none());
        assert!(skeleton.class.is_none());
        assert!(skeleton.style.is_none());
        
        // Builder pattern should work
        let custom_skeleton = Skeleton::new()
            .with_variant(SkeletonVariant::Circular)
            .with_size(SkeletonSize::Large)
            .with_lines(3)
            .with_width("100px")
            .with_height("100px")
            .with_class("custom")
            ;
        
        assert_eq!(custom_skeleton.variant, SkeletonVariant::Circular);
        assert_eq!(custom_skeleton.size, SkeletonSize::Large);
        assert_eq!(custom_skeleton.lines, 3);
        assert_eq!(custom_skeleton.width, Some("100px".to_string()));
        assert_eq!(custom_skeleton.height, Some("100px".to_string()));
        assert_eq!(custom_skeleton.class, Some("custom".to_string()));
        assert_eq!(custom_skeleton.style, Some("border-radius: 50%".to_string()));
    }

    #[test]
    fn test_alert_dialog_api_contract() {
        // Test AlertDialog API contract
        let dialog = AlertDialog::new();
        
        // Default values should be consistent
        assert_eq!(dialog.variant, AlertDialogVariant::Default);
        assert_eq!(dialog.size, AlertDialogSize::Medium);
        assert!(!dialog.open);
        assert!(dialog.title.is_none());
        assert!(dialog.description.is_none());
        assert!(dialog.class.is_none());
        assert!(dialog.style.is_none());
        
        // Builder pattern should work
        let custom_dialog = AlertDialog::new()
            .with_variant(AlertDialogVariant::Destructive)
            .with_size(AlertDialogSize::Large)
            .with_open(true)
            .with_title("Custom Title")
            .with_description("Custom Description")
            .with_class("custom")
            ;
        
        assert_eq!(custom_dialog.variant, AlertDialogVariant::Destructive);
        assert_eq!(custom_dialog.size, AlertDialogSize::Large);
        assert!(custom_dialog.open);
        assert_eq!(custom_dialog.title, Some("Custom Title".to_string()));
        assert_eq!(custom_dialog.description, Some("Custom Description".to_string()));
        assert_eq!(custom_dialog.class, Some("custom".to_string()));
        assert_eq!(custom_dialog.style, Some("z-index: 1000".to_string()));
    }

    #[test]
    fn test_pagination_api_contract() {
        // Test Pagination API contract
        let pagination = Pagination::new();
        
        // Default values should be consistent
        assert_eq!(pagination.current_page, 1);
        assert_eq!(pagination.total_pages, 1);
        assert_eq!(pagination.size, PaginationSize::Medium);
        assert!(pagination.class.is_none());
        assert!(pagination.style.is_none());
        
        // Builder pattern should work
        let custom_pagination = Pagination::new()
            .with_current_page(5)
            .with_total_pages(20)
            .with_size(PaginationSize::Large)
            .with_class("custom")
            ;
        
        assert_eq!(custom_pagination.current_page, 5);
        assert_eq!(custom_pagination.total_pages, 20);
        assert_eq!(custom_pagination.size, PaginationSize::Large);
        assert_eq!(custom_pagination.class, Some("custom".to_string()));
        assert_eq!(custom_pagination.style, Some("margin: 20px".to_string()));
    }

    #[test]
    fn test_validation_engine_api_contract() {
        // Test ValidationEngine API contract
        let engine = ValidationEngine::new();
        
        // Default state should be consistent
        assert!(!engine.has_rules());
        assert!(!engine.has_custom_validators());
        
        // Adding rules should work
        let mut engine = engine;
        engine.add_rule("test".to_string(), ValidationRule { rule_type: ValidationRuleType::Required, message: "Test message".to_string(), value: None });
        
        assert!(engine.has_rules());
        assert!(engine.has_rule_for_field("test"));
        assert!(!engine.has_rule_for_field("nonexistent"));
    }

    #[test]
    fn test_theme_provider_api_contract() {
        // Test ThemeProvider API contract
        let theme = ThemeProvider::new();
        
        // Default values should be consistent
        assert_eq!(theme.theme, Theme::Light);
        assert_eq!(theme.variant, ThemeVariant::Default);
        assert!(theme.class.is_none());
        assert!(theme.style.is_none());
        
        // Builder pattern should work
        let custom_theme = ThemeProvider::new()
            .with_theme(Theme::Dark)
            .with_variant(ThemeVariant::HighContrast)
            .with_class("custom-theme")
            ;
        
        assert_eq!(custom_theme.theme, Theme::Dark);
        assert_eq!(custom_theme.variant, ThemeVariant::HighContrast);
        assert_eq!(custom_theme.class, Some("custom-theme".to_string()));
        assert_eq!(custom_theme.style, Some("background: black".to_string()));
    }

    #[test]
    fn test_utility_functions_api_contract() {
        // Test utility functions API contract
        
        // generate_id should always return unique IDs
        let id1 = generate_id("test");
        let id2 = generate_id("test");
        let id3 = generate_id("different");
        
        assert_ne!(id1, id2);
        assert_ne!(id1, id3);
        assert_ne!(id2, id3);
        assert!(id1.starts_with("test-"));
        assert!(id2.starts_with("test-"));
        assert!(id3.starts_with("different-"));
        
        // merge_optional_classes should handle all combinations
        assert_eq!(merge_optional_classes(Some("a"), Some("b")), Some("a b".to_string()));
        assert_eq!(merge_optional_classes(Some("a"), None), Some("a".to_string()));
        assert_eq!(merge_optional_classes(None, Some("b")), Some("b".to_string()));
        assert_eq!(merge_optional_classes(None, None), None);
        
        // merge_classes should handle empty and multiple classes
        assert_eq!(merge_classes(Vec::new()), "");
        assert_eq!(merge_classes(vec!["a"]), "a");
        assert_eq!(merge_classes(vec!["a", "b"]), "a b");
        assert_eq!(merge_classes(vec!["a", "b", "c"]), "a b c");
    }

    #[test]
    fn test_enum_consistency() {
        // Test that all enums have consistent implementations
        
        // ButtonVariant should have as_str method
        assert_eq!(ButtonVariant::Default.as_str(), "default");
        assert_eq!(ButtonVariant::Destructive.as_str(), "destructive");
        assert_eq!(ButtonVariant::Secondary.as_str(), "secondary");
        
        // ButtonSize should have as_str method
        assert_eq!(ButtonSize::Default.as_str(), "default");
        assert_eq!(ButtonSize::Small.as_str(), "small");
        assert_eq!(ButtonSize::Default.as_str(), "medium");
        assert_eq!(ButtonSize::Large.as_str(), "large");
        
        // SkeletonVariant should have as_str method
        assert_eq!(SkeletonVariant::Text.as_str(), "text");
        assert_eq!(SkeletonVariant::Circular.as_str(), "circular");
        assert_eq!(SkeletonVariant::Rectangular.as_str(), "rectangular");
        
        // SkeletonSize should have as_str method
        assert_eq!(SkeletonSize::Small.as_str(), "small");
        assert_eq!(SkeletonSize::Medium.as_str(), "medium");
        assert_eq!(SkeletonSize::Large.as_str(), "large");
        assert_eq!(SkeletonSize::ExtraLarge.as_str(), "xl");
    }

    #[test]
    fn test_accessibility_contracts() {
        // Test accessibility-related contracts
        
        // Components should support aria attributes
        let button = Button::new()
            
            .with_aria_described_by("button-description");
        
        assert_eq!(button.aria_label, Some("Click me".to_string()));
        assert_eq!(button.aria_described_by, Some("button-description".to_string()));
        
        // Components should support disabled state
        let disabled_button = Button::new().with_disabled(true);
        assert!(disabled_button.disabled);
        
        // Components should support custom IDs
        let custom_id = generate_id("button");
        let button_with_id = Button::new().with_id(custom_id.clone());
        assert_eq!(button_with_id.id, Some(custom_id));
    }

    #[test]
    fn test_validation_rule_contracts() {
        // Test ValidationRule API contract
        let rule = ValidationRule { rule_type: ValidationRuleType::Required, message: "Required field".to_string(), value: None };
        
        assert_eq!(rule.rule_type, ValidationRuleType::Required);
        assert_eq!(rule.message, "Required field");
        
        // Default rule should be consistent
        let default_rule = ValidationRule::default();
        assert_eq!(default_rule.rule_type, ValidationRuleType::Required);
        assert_eq!(default_rule.message, "This field is required");
    }

    #[test]
    fn test_validation_result_contracts() {
        // Test ValidationResult API contract
        let result = ValidationResult::new(true, Some("Success".to_string()));
        
        assert!(result.is_valid);
        assert_eq!(result.message, Some("Success".to_string()));
        
        // Default result should be consistent
        let default_result = ValidationResult::default();
        assert!(default_result.is_valid);
        assert!(default_result.message.is_none());
    }
}
