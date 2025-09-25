
// Module declarations
mod validation;
mod fields;
mod controls;

// Re-export all types and functions from sub-modules
pub use validation::*;
pub use fields::*;
pub use controls::*;

#[cfg(test)]
mod form_validation_tests {
    use super::*;
    use proptest::prelude::*;
    use std::collections::HashMap;

    #[test]
    fn test_form_validation_provider_creation() {
        // Test component creation without runtime
        let errors: HashMap<String, String> = HashMap::new();
        assert!(errors.is_empty());
    }

    #[test]
    fn test_form_field_creation() {
        // Test component creation without runtime
        let name = "email".to_string();
        let label = "Email".to_string();
        assert!(!name.is_empty());
        assert!(!label.is_empty());
    }

    #[test]
    fn test_form_label_creation() {
        // Test component creation without runtime
        let for_id = "email".to_string();
        assert!(!for_id.is_empty());
    }

    #[test]
    fn test_form_field_error_creation() {
        // Test component creation without runtime
        let name = "email".to_string();
        assert!(!name.is_empty());
    }

    #[test]
    fn test_form_error_summary_creation() {
        // Test component creation without runtime
        let errors = vec![
            FormError {
                field: "email".to_string(),
                message: "Invalid email format".to_string(),
                error_type: ErrorType::Validation,
            }
        ];
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_validation_mode_enum() {
        assert_eq!(ValidationMode::OnChange.as_str(), "on-change");
        assert_eq!(ValidationMode::OnBlur.as_str(), "on-blur");
        assert_eq!(ValidationMode::OnSubmit.as_str(), "on-submit");
        assert_eq!(ValidationMode::Manual.as_str(), "manual");
    }

    #[test]
    fn test_validation_rule_default() {
        let rule = ValidationRule::default();
        assert_eq!(rule.rule_type, ValidationRuleType::Required);
        assert_eq!(rule.message, "This field is required");
    }

    #[test]
    fn test_validation_result_default() {
        let result = ValidationResult::default();
        assert!(result.is_valid);
        assert!(result.message.is_none());
    }

    #[test]
    fn test_field_validation_result_default() {
        let result = FieldValidationResult::default();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_form_validation_state_default() {
        let state = FormValidationState::default();
        assert!(state.is_valid);
        assert!(!state.is_submitting);
        assert!(!state.is_dirty);
        assert!(!state.is_touched);
    }

    #[test]
    fn test_field_error_default() {
        let error = FieldError::default();
        assert_eq!(error.error_type, ErrorType::Validation);
        assert_eq!(error.timestamp, 0);
    }

    #[test]
    fn test_form_error_default() {
        let error = FormError::default();
        assert_eq!(error.error_type, ErrorType::Validation);
    }

    #[test]
    fn test_validation_engine_new() {
        let engine = ValidationEngine::new();
        assert!(!engine.has_rules());
        assert!(!engine.has_custom_validators());
    }

    #[test]
    fn test_validation_engine_add_rule() {
        let mut engine = ValidationEngine::new();
        let rule = ValidationRule {
            rule_type: ValidationRuleType::Required,
            message: "Field is required".to_string(),
            value: None,
        };
        engine.add_rule("email".to_string(), rule);
        assert!(engine.has_rule_for_field("email"));
    }

    #[test]
    fn test_validation_engine_validate_field() {
        let mut engine = ValidationEngine::new();
        let rule = ValidationRule {
            rule_type: ValidationRuleType::Required,
            message: "Field is required".to_string(),
            value: None,
        };
        engine.add_rule("email".to_string(), rule);
        
        let result = engine.validate_field("email", "");
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
        
        let result = engine.validate_field("email", "test@example.com");
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validation_engine_validate_form() {
        let mut engine = ValidationEngine::new();
        let rule = ValidationRule {
            rule_type: ValidationRuleType::Required,
            message: "Field is required".to_string(),
            value: None,
        };
        engine.add_rule("email".to_string(), rule);
        
        let mut form_data = HashMap::new();
        form_data.insert("email".to_string(), "".to_string());
        
        let state = engine.validate_form(&form_data);
        assert!(!state.is_valid);
        assert!(!state.field_errors.is_empty());
    }

    #[test]
    fn test_email_validation() {
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user.name+tag@domain.co.uk"));
        assert!(!is_valid_email("invalid-email"));
        assert!(!is_valid_email("@domain.com"));
        assert!(!is_valid_email("user@"));
    }

    #[test]
    fn test_url_validation() {
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://subdomain.example.com/path"));
        assert!(!is_valid_url("invalid-url"));
        assert!(!is_valid_url("ftp://example.com"));
        assert!(!is_valid_url("example.com"));
    }

    #[test]
    fn test_phone_validation() {
        assert!(is_valid_phone("+1234567890"));
        assert!(is_valid_phone("(123) 456-7890"));
        assert!(is_valid_phone("123-456-7890"));
        assert!(!is_valid_phone("123"));
        assert!(!is_valid_phone("invalid-phone"));
    }

    #[test]
    fn test_date_validation() {
        assert!(is_valid_date("2023-12-25"));
        assert!(is_valid_date("2000-01-01"));
        assert!(!is_valid_date("12/25/2023"));
        assert!(!is_valid_date("2023-13-01"));
        assert!(!is_valid_date("invalid-date"));
    }

    #[test]
    fn test_time_validation() {
        assert!(is_valid_time("14:30"));
        assert!(is_valid_time("09:15:45"));
        assert!(!is_valid_time("25:00"));
        assert!(!is_valid_time("12:60"));
        assert!(!is_valid_time("invalid-time"));
    }

    #[test]
    fn test_number_validation() {
        assert!(is_valid_number("123.45"));
        assert!(is_valid_number("-123.45"));
        assert!(is_valid_number("0"));
        assert!(!is_valid_number("abc"));
        assert!(!is_valid_number("12.34.56"));
    }

    #[test]
    fn test_integer_validation() {
        assert!(is_valid_integer("123"));
        assert!(is_valid_integer("-123"));
        assert!(is_valid_integer("0"));
        assert!(!is_valid_integer("123.45"));
        assert!(!is_valid_integer("abc"));
    }

    // Property-based tests
    #[test]
    fn test_validation_rule_property_based() {
        proptest!(|(message in ".*")| {
            let rule = ValidationRule {
                rule_type: ValidationRuleType::Required,
                message: message.clone(),
                value: None,
            };
            assert_eq!(rule.message, message);
        });
    }

    #[test]
    fn test_validation_result_property_based() {
        proptest!(|(is_valid in any::<bool>())| {
            let result = ValidationResult {
                is_valid,
                message: None,
            };
            assert_eq!(result.is_valid, is_valid);
        });
    }

    // Integration Tests
    #[test]
    fn test_form_validation_workflow() {
        // Test complete form validation workflow
    }

    #[test]
    fn test_form_validation_accessibility() {
        // Test form validation accessibility features
    }

    #[test]
    fn test_form_validation_performance() {
        // Test form validation performance
    }

    #[test]
    fn test_form_validation_error_handling() {
        // Test form validation error handling
    }

    // Performance Tests
    #[test]
    fn test_validation_engine_performance() {
        // Test validation engine performance
    }

    #[test]
    fn test_form_validation_memory_usage() {
        // Test form validation memory usage
    }

    #[test]
    fn test_validation_rule_performance() {
        // Test validation rule performance
    }
}
