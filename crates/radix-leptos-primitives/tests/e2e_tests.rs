//! End-to-end tests for Radix-Leptos components
//! 
//! These tests simulate real user workflows and interactions.

use radix_leptos_primitives::*;
use leptos::prelude::*;

#[cfg(test)]
mod e2e_tests {
    use super::*;
use crate::utils::{generate_id, merge_classes, merge_optional_classes};
use crate::form_validation::{is_valid_email, is_valid_date, is_valid_time, is_valid_phone};

    #[test]
    fn test_user_registration_workflow() {
        // Simulate a user registration workflow
        let mut validation_engine = ValidationEngine::new();
        
        // Add validation rules for registration
        validation_engine.add_rule("email".to_string(), ValidationRule { rule_type: ValidationRuleType::Email, message: "Invalid email".to_string(), value: None });
        validation_engine.add_rule("password".to_string(), ValidationRule { rule_type: ValidationRuleType::Required, message: "Password required".to_string(), value: None });
        validation_engine.add_rule("confirm_password".to_string(), ValidationRule { rule_type: ValidationRuleType::Required, message: "Confirm password".to_string(), value: None });
        
        // Simulate user input
        let mut form_data = std::collections::HashMap::new();
        form_data.insert("email".to_string(), "user@example.com".to_string());
        form_data.insert("password".to_string(), "securepassword123".to_string());
        form_data.insert("confirm_password".to_string(), "securepassword123".to_string());
        
        // Validate the form
        let validation_result = validation_engine.validate_form(&form_data);
        assert!(validation_result.is_valid);
        
        // Show success dialog
        let success_dialog = AlertDialog::new()
            .with_variant(AlertDialogVariant::Default)
            .with_title("Registration Successful")
            .with_description("Welcome! Your account has been created.")
            .with_open(true);
        
        assert!(success_dialog.open);
        assert_eq!(success_dialog.title, Some("Registration Successful".to_string()));
    }

    #[test]
    fn test_data_table_with_pagination_workflow() {
        // Simulate browsing a data table with pagination
        let mut pagination = Pagination::new()
            .with_current_page(1)
            .with_total_pages(10);
        
        // User clicks next page
        pagination = pagination.with_current_page(2);
        assert_eq!(pagination.current_page, 2);
        
        // User jumps to last page
        pagination = pagination.with_current_page(10);
        assert_eq!(pagination.current_page, 10);
        
        // User goes back to first page
        pagination = pagination.with_current_page(1);
        assert_eq!(pagination.current_page, 1);
        
        // Show loading state while data loads
        let loading_skeleton = Skeleton::new()
            .with_variant(SkeletonVariant::Text)
            .with_lines(5);
        
        assert_eq!(loading_skeleton.lines, 5);
    }

    #[test]
    fn test_form_validation_error_workflow() {
        // Simulate form validation errors
        let mut validation_engine = ValidationEngine::new();
        validation_engine.add_rule("email".to_string(), ValidationRule { rule_type: ValidationRuleType::Email, message: "Invalid email format".to_string(), value: None });
        validation_engine.add_rule("age".to_string(), ValidationRule { rule_type: ValidationRuleType::Number, message: "Age must be a number".to_string(), value: None });
        
        // User submits invalid data
        let mut form_data = std::collections::HashMap::new();
        form_data.insert("email".to_string(), "invalid-email".to_string());
        form_data.insert("age".to_string(), "not-a-number".to_string());
        
        // Validation should fail
        let validation_result = validation_engine.validate_form(&form_data);
        assert!(!validation_result.is_valid);
        assert!(!validation_result.field_errors.is_empty());
        
        // Show error dialog
        let error_dialog = AlertDialog::new()
            .with_variant(AlertDialogVariant::Destructive)
            .with_title("Validation Errors")
            .with_description("Please fix the errors below and try again.")
            .with_open(true);
        
        assert!(error_dialog.open);
        assert_eq!(error_dialog.variant, AlertDialogVariant::Destructive);
    }

    #[test]
    fn test_theme_switching_workflow() {
        // Simulate user switching themes
        let mut theme_provider = ThemeProvider::new()
            .with_theme(Theme::Light);
        
        // User switches to dark theme
        theme_provider = theme_provider.with_theme(Theme::Dark);
        assert_eq!(theme_provider.theme, Theme::Dark);
        
        // User switches to high contrast
        theme_provider = theme_provider.with_variant(ThemeVariant::HighContrast);
        assert_eq!(theme_provider.variant, ThemeVariant::HighContrast);
        
        // Components should adapt to theme
        let button = Button::new()
            .with_variant(ButtonVariant::Default)
            .with_size(ButtonSize::Default);
        
        let skeleton = Skeleton::new()
            .with_variant(SkeletonVariant::Text)
            .with_size(SkeletonSize::Medium);
        
        // All components should work with the theme
        assert_eq!(button.variant, ButtonVariant::Default);
        assert_eq!(skeleton.variant, SkeletonVariant::Text);
    }

    #[test]
    fn test_multi_step_form_workflow() {
        // Simulate a multi-step form workflow
        let mut validation_engine = ValidationEngine::new();
        
        // Step 1: Personal Information
        validation_engine.add_rule("name".to_string(), ValidationRule { rule_type: ValidationRuleType::Required, message: "Name required".to_string(), value: None });
        validation_engine.add_rule("email".to_string(), ValidationRule { rule_type: ValidationRuleType::Email, message: "Invalid email".to_string(), value: None });
        
        let mut step1_data = std::collections::HashMap::new();
        step1_data.insert("name".to_string(), "John Doe".to_string());
        step1_data.insert("email".to_string(), "john@example.com".to_string());
        
        let step1_result = validation_engine.validate_form(&step1_data);
        assert!(step1_result.is_valid);
        
        // Step 2: Additional Information
        validation_engine.add_rule("phone".to_string(), ValidationRule { rule_type: ValidationRuleType::Phone, message: "Invalid phone".to_string(), value: None });
        validation_engine.add_rule("address".to_string(), ValidationRule { rule_type: ValidationRuleType::Required, message: "Address required".to_string(), value: None });
        
        let mut step2_data = step1_data.clone();
        step2_data.insert("phone".to_string(), "123-456-7890".to_string());
        step2_data.insert("address".to_string(), "123 Main St".to_string());
        
        let step2_result = validation_engine.validate_form(&step2_data);
        assert!(step2_result.is_valid);
        
        // Final confirmation
        let confirmation_dialog = AlertDialog::new()
            .with_variant(AlertDialogVariant::Default)
            .with_title("Form Complete")
            .with_description("Thank you for submitting your information!")
            .with_open(true);
        
        assert!(confirmation_dialog.open);
    }

    #[test]
    fn test_search_and_filter_workflow() {
        // Simulate search and filter functionality
        let mut pagination = Pagination::new()
            .with_current_page(1)
            .with_total_pages(5);
        
        // User searches and gets results
        let search_results = 25; // 25 results found
        let results_per_page = 5;
        let total_pages = (search_results + results_per_page - 1) / results_per_page;
        
        pagination = pagination.with_total_pages(total_pages);
        assert_eq!(pagination.total_pages, 5);
        
        // User navigates through results
        for page in 1..=5 {
            pagination = pagination.with_current_page(page);
            assert_eq!(pagination.current_page, page);
            
            // Show loading state for each page
            let loading_skeleton = Skeleton::new()
                .with_variant(SkeletonVariant::Rectangular)
                .with_width("100%")
                .with_height("200px");
            
            assert_eq!(loading_skeleton.variant, SkeletonVariant::Rectangular);
        }
    }

    #[test]
    fn test_error_recovery_workflow() {
        // Simulate error recovery workflow
        let mut validation_engine = ValidationEngine::new();
        validation_engine.add_rule("required_field".to_string(), ValidationRule { rule_type: ValidationRuleType::Required, message: "This field is required".to_string(), value: None });
        
        // Initial submission with error
        let mut form_data = std::collections::HashMap::new();
        form_data.insert("required_field".to_string(), "".to_string());
        
        let initial_result = validation_engine.validate_form(&form_data);
        assert!(!initial_result.is_valid);
        
        // Show error dialog
        let error_dialog = AlertDialog::new()
            .with_variant(AlertDialogVariant::Destructive)
            .with_title("Error")
            .with_description("Please fix the errors and try again.")
            .with_open(true);
        
        assert!(error_dialog.open);
        
        // User fixes the error
        form_data.insert("required_field".to_string(), "Fixed value".to_string());
        
        let fixed_result = validation_engine.validate_form(&form_data);
        assert!(fixed_result.is_valid);
        
        // Show success dialog
        let success_dialog = AlertDialog::new()
            .with_variant(AlertDialogVariant::Default)
            .with_title("Success")
            .with_description("Form submitted successfully!")
            .with_open(true);
        
        assert!(success_dialog.open);
    }

    #[test]
    fn test_accessibility_workflow() {
        // Simulate accessibility-focused user workflow
        let button = Button::new()
            
            .with_aria_described_by("submit-description")
            .with_disabled(false);
        
        // Button should be accessible
        assert_eq!(button.aria_label, Some("Submit form".to_string()));
        assert_eq!(button.aria_described_by, Some("submit-description".to_string()));
        assert!(!button.disabled);
        
        // Form validation with accessibility
        let mut validation_engine = ValidationEngine::new();
        validation_engine.add_rule("email".to_string(), ValidationRule { rule_type: ValidationRuleType::Email, message: "Please enter a valid email address".to_string(), value: None });
        
        let mut form_data = std::collections::HashMap::new();
        form_data.insert("email".to_string(), "user@example.com".to_string());
        
        let result = validation_engine.validate_form(&form_data);
        assert!(result.is_valid);
        
        // Accessible dialog
        let dialog = AlertDialog::new()
            .with_title("Form Submitted")
            .with_description("Your form has been submitted successfully.")
            .with_open(true);
        
        assert!(dialog.open);
        assert_eq!(dialog.title, Some("Form Submitted".to_string()));
    }

    #[test]
    fn test_performance_workflow() {
        // Simulate performance-critical workflow
        let start_time = std::time::Instant::now();
        
        // Create many components quickly
        let mut components = Vec::new();
        for i in 0..1000 {
            let button = Button::new()
                .with_variant(ButtonVariant::Default)
                .with_size(ButtonSize::Default)
                .with_class(&format!("button-{}", i));
            
            let skeleton = Skeleton::new()
                .with_variant(SkeletonVariant::Text)
                .with_lines(3)
                .with_class(&format!("skeleton-{}", i));
            
            components.push((button, skeleton));
        }
        
        let creation_time = start_time.elapsed();
        
        // Should create components quickly (less than 100ms for 1000 components)
        assert!(creation_time.as_millis() < 100);
        assert_eq!(components.len(), 1000);
        
        // All components should be properly configured
        for (i, (button, skeleton)) in components.iter().enumerate() {
            assert_eq!(button.variant, ButtonVariant::Default);
            assert_eq!(skeleton.variant, SkeletonVariant::Text);
            assert_eq!(skeleton.lines, 3);
        }
    }
}
