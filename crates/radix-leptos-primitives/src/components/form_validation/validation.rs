use std::collections::HashMap;
use regex::Regex;

/// Validation Mode enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValidationMode {
    OnChange,
    OnBlur,
    OnSubmit,
    Manual,
}

impl ValidationMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ValidationMode::OnChange => "on-change",
            ValidationMode::OnBlur => "on-blur",
            ValidationMode::OnSubmit => "on-submit",
            ValidationMode::Manual => "manual",
        }
    }
}

/// Validation Rule struct
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationRule {
    pub rule_type: ValidationRuleType,
    pub message: String,
    pub value: Option<String>,
}

impl Default for ValidationRule {
    fn default() -> Self {
        Self {
            rule_type: ValidationRuleType::Required,
            message: "This field is required".to_string(),
            value: None,
        }
    }
}

/// Validation Rule Type enum
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationRuleType {
    Required,
    MinLength(usize),
    MaxLength(usize),
    Min(f64),
    Max(f64),
    Pattern(String),
    Email,
    Url,
    Phone,
    Date,
    Time,
    Number,
    Integer,
    Custom(String),
}

/// Custom Validator function type
pub type CustomValidator = Box<dyn Fn(&str) -> ValidationResult + Send + Sync>;

/// Validation Result struct
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub message: Option<String>,
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self {
            is_valid: true,
            message: None,
        }
    }
}

impl ValidationResult {
    pub fn new(is_valid: bool, message: Option<String>) -> Self {
        Self {
            is_valid,
            message,
        }
    }
}

/// Field Validation Result struct
#[derive(Debug, Clone, PartialEq)]
pub struct FieldValidationResult {
    pub field_name: String,
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl Default for FieldValidationResult {
    fn default() -> Self {
        Self {
            field_name: String::new(),
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

/// Form Validation State struct
#[derive(Debug, Clone, PartialEq)]
pub struct FormValidationState {
    pub is_valid: bool,
    pub is_submitting: bool,
    pub is_dirty: bool,
    pub is_touched: bool,
    pub field_errors: HashMap<String, FieldError>,
    pub form_errors: Vec<FormError>,
}

impl Default for FormValidationState {
    fn default() -> Self {
        Self {
            is_valid: true,
            is_submitting: false,
            is_dirty: false,
            is_touched: false,
            field_errors: HashMap::new(),
            form_errors: Vec::new(),
        }
    }
}

/// Field Error struct
#[derive(Debug, Clone, PartialEq)]
pub struct FieldError {
    pub field_name: String,
    pub message: String,
    pub error_type: ErrorType,
    pub timestamp: u64,
}

impl Default for FieldError {
    fn default() -> Self {
        Self {
            field_name: String::new(),
            message: String::new(),
            error_type: ErrorType::Validation,
            timestamp: 0,
        }
    }
}

/// Form Error struct
#[derive(Debug, Clone, PartialEq)]
pub struct FormError {
    pub field: String,
    pub message: String,
    pub error_type: ErrorType,
}

impl Default for FormError {
    fn default() -> Self {
        Self {
            field: String::new(),
            message: String::new(),
            error_type: ErrorType::Validation,
        }
    }
}

/// Error Type enum
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    Validation,
    Network,
    Server,
    Custom,
}

/// Validation Engine
pub struct ValidationEngine {
    rules: HashMap<String, Vec<ValidationRule>>,
    custom_validators: HashMap<String, CustomValidator>,
}

impl Default for ValidationEngine {
    fn default() -> Self {
        Self {
            rules: HashMap::new(),
            custom_validators: HashMap::new(),
        }
    }
}

impl ValidationEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rule(&mut self, field_name: String, rule: ValidationRule) {
        self.rules.entry(field_name).or_insert_with(Vec::new).push(rule);
    }

    pub fn add_custom_validator(&mut self, name: String, validator: CustomValidator) {
        self.custom_validators.insert(name, validator);
    }

    pub fn has_rules(&self) -> bool {
        !self.rules.is_empty()
    }

    pub fn has_rule_for_field(&self, field_name: &str) -> bool {
        self.rules.contains_key(field_name)
    }

    pub fn has_custom_validators(&self) -> bool {
        !self.custom_validators.is_empty()
    }

    pub fn validate_field(&self, field_name: &str, value: &str) -> FieldValidationResult {
        let mut result = FieldValidationResult {
            field_name: field_name.to_string(),
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        if let Some(rules) = self.rules.get(field_name) {
            for rule in rules {
                let validation_result = self.validate_rule(rule, value);
                if !validation_result.is_valid {
                    result.is_valid = false;
                    if let Some(message) = validation_result.message {
                        result.errors.push(message);
                    }
                }
            }
        }

        result
    }

    pub fn validate_form(&self, form_data: &HashMap<String, String>) -> FormValidationState {
        let mut state = FormValidationState::default();
        let mut all_valid = true;

        for (field_name, value) in form_data {
            let field_result = self.validate_field(field_name, value);
            if !field_result.is_valid {
                all_valid = false;
                let field_error = FieldError {
                    field_name: field_name.clone(),
                    message: field_result.errors.join(", "),
                    error_type: ErrorType::Validation,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                };
                state.field_errors.insert(field_name.clone(), field_error);
            }
        }

        state.is_valid = all_valid;
        state
    }

    fn validate_rule(&self, rule: &ValidationRule, value: &str) -> ValidationResult {
        match &rule.rule_type {
            ValidationRuleType::Required => {
                if value.trim().is_empty() {
                    ValidationResult {
                        is_valid: false,
                        message: Some(rule.message.clone()),
                    }
                } else {
                    ValidationResult::default()
                }
            }
            ValidationRuleType::MinLength(min_len) => {
                if value.len() < *min_len {
                    ValidationResult {
                        is_valid: false,
                        message: Some(rule.message.clone()),
                    }
                } else {
                    ValidationResult::default()
                }
            }
            ValidationRuleType::MaxLength(max_len) => {
                if value.len() > *max_len {
                    ValidationResult {
                        is_valid: false,
                        message: Some(rule.message.clone()),
                    }
                } else {
                    ValidationResult::default()
                }
            }
            ValidationRuleType::Min(min_val) => {
                if let Ok(num) = value.parse::<f64>() {
                    if num < *min_val {
                        ValidationResult {
                            is_valid: false,
                            message: Some(rule.message.clone()),
                        }
                    } else {
                        ValidationResult::default()
                    }
                } else {
                    ValidationResult {
                        is_valid: false,
                        message: Some(rule.message.clone()),
                    }
                }
            }
            ValidationRuleType::Max(max_val) => {
                if let Ok(num) = value.parse::<f64>() {
                    if num > *max_val {
                        ValidationResult {
                            is_valid: false,
                            message: Some(rule.message.clone()),
                        }
                    } else {
                        ValidationResult::default()
                    }
                } else {
                    ValidationResult {
                        is_valid: false,
                        message: Some(rule.message.clone()),
                    }
                }
            }
            ValidationRuleType::Pattern(pattern) => {
                if let Ok(regex) = Regex::new(pattern) {
                    if !regex.is_match(value) {
                        ValidationResult {
                            is_valid: false,
                            message: Some(rule.message.clone()),
                        }
                    } else {
                        ValidationResult::default()
                    }
                } else {
                    ValidationResult {
                        is_valid: false,
                        message: Some(rule.message.clone()),
                    }
                }
            }
            ValidationRuleType::Email => {
                if !is_valid_email(value) {
                    ValidationResult {
                        is_valid: false,
                        message: Some(rule.message.clone()),
                    }
                } else {
                    ValidationResult::default()
                }
            }
            ValidationRuleType::Url => {
                if !is_valid_url(value) {
                    ValidationResult {
                        is_valid: false,
                        message: Some(rule.message.clone()),
                    }
                } else {
                    ValidationResult::default()
                }
            }
            ValidationRuleType::Phone => {
                if !is_valid_phone(value) {
                    ValidationResult {
                        is_valid: false,
                        message: Some(rule.message.clone()),
                    }
                } else {
                    ValidationResult::default()
                }
            }
            ValidationRuleType::Date => {
                if !is_valid_date(value) {
                    ValidationResult {
                        is_valid: false,
                        message: Some(rule.message.clone()),
                    }
                } else {
                    ValidationResult::default()
                }
            }
            ValidationRuleType::Time => {
                if !is_valid_time(value) {
                    ValidationResult {
                        is_valid: false,
                        message: Some(rule.message.clone()),
                    }
                } else {
                    ValidationResult::default()
                }
            }
            ValidationRuleType::Number => {
                if !is_valid_number(value) {
                    ValidationResult {
                        is_valid: false,
                        message: Some(rule.message.clone()),
                    }
                } else {
                    ValidationResult::default()
                }
            }
            ValidationRuleType::Integer => {
                if !is_valid_integer(value) {
                    ValidationResult {
                        is_valid: false,
                        message: Some(rule.message.clone()),
                    }
                } else {
                    ValidationResult::default()
                }
            }
            ValidationRuleType::Custom(name) => {
                if let Some(validator) = self.custom_validators.get(name) {
                    validator(value)
                } else {
                    ValidationResult::default()
                }
            }
        }
    }
}

/// Email validation
pub fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

/// URL validation
pub fn is_valid_url(url: &str) -> bool {
    let url_regex = Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap();
    url_regex.is_match(url)
}

/// Phone validation
pub fn is_valid_phone(phone: &str) -> bool {
    let phone_regex = Regex::new(r"^\+?[\d\s\-\(\)]{10,}$").unwrap();
    phone_regex.is_match(phone)
}

/// Date validation
pub fn is_valid_date(date: &str) -> bool {
    let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    if !date_regex.is_match(date) {
        return false;
    }
    
    // Parse the date to validate actual values
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return false;
    }
    
    let year: i32 = parts[0].parse().unwrap_or(0);
    let month: u32 = parts[1].parse().unwrap_or(0);
    let day: u32 = parts[2].parse().unwrap_or(0);
    
    // Basic validation
    if year < 1 || month < 1 || month > 12 || day < 1 || day > 31 {
        return false;
    }
    
    // More specific validation for days per month
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => return false,
    };
    
    day <= days_in_month
}

/// Helper function to check if a year is a leap year
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Time validation
pub fn is_valid_time(time: &str) -> bool {
    let time_regex = Regex::new(r"^\d{2}:\d{2}(:\d{2})?$").unwrap();
    if !time_regex.is_match(time) {
        return false;
    }
    
    // Parse the time to validate actual values
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return false;
    }
    
    let hour: u32 = parts[0].parse().unwrap_or(99);
    let minute: u32 = parts[1].parse().unwrap_or(99);
    let second: u32 = if parts.len() == 3 { parts[2].parse().unwrap_or(99) } else { 0 };
    
    // Validate ranges
    hour < 24 && minute < 60 && second < 60
}

/// Number validation
pub fn is_valid_number(number: &str) -> bool {
    number.parse::<f64>().is_ok()
}

/// Integer validation
pub fn is_valid_integer(integer: &str) -> bool {
    integer.parse::<i64>().is_ok()
}

#[cfg(test)]
mod validation_tests {
    use super::*;
    use proptest::prelude::*;
use crate::utils::{merge_optional_classes, generate_id};

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
        assert!(engine.rules.is_empty());
        assert!(engine.custom_validators.is_empty());
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
        assert!(engine.rules.contains_key("email"));
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
    fn test_time_validation_edge_cases() {
        // Valid times
        assert!(is_valid_time("00:00"));
        assert!(is_valid_time("23:59"));
        assert!(is_valid_time("12:00:00"));
        assert!(is_valid_time("00:00:59"));
        
        // Invalid times
        assert!(!is_valid_time("24:00"));
        assert!(!is_valid_time("12:60"));
        assert!(!is_valid_time("12:30:60"));
        assert!(!is_valid_time(""));
        assert!(!is_valid_time("12"));
        assert!(!is_valid_time("12:"));
        assert!(!is_valid_time(":30"));
    }

    #[test]
    fn test_date_validation_edge_cases() {
        // Valid dates
        assert!(is_valid_date("2023-01-01"));
        assert!(is_valid_date("2000-02-29")); // Leap year
        assert!(is_valid_date("2023-12-31"));
        
        // Invalid dates
        assert!(!is_valid_date("2023-02-29")); // Not leap year
        assert!(!is_valid_date("2023-04-31")); // April has 30 days
        assert!(!is_valid_date("2023-13-01")); // Invalid month
        assert!(!is_valid_date("2023-00-01")); // Invalid month
        assert!(!is_valid_date("2023-01-00")); // Invalid day
        assert!(!is_valid_date("2023-01-32")); // Invalid day
        assert!(!is_valid_date(""));
        assert!(!is_valid_date("2023-1-1")); // Wrong format
        assert!(!is_valid_date("2023/01/01")); // Wrong format
    }

    #[test]
    fn test_email_validation() {
        // Valid emails
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user.name@domain.co.uk"));
        assert!(is_valid_email("user+tag@example.org"));
        
        // Invalid emails
        assert!(!is_valid_email("invalid-email"));
        assert!(!is_valid_email("@example.com"));
        assert!(!is_valid_email("test@"));
        assert!(!is_valid_email(""));
        // Note: "test..test@example.com" might be valid according to some email regex patterns
        // Let's use a clearly invalid email instead
        assert!(!is_valid_email("test@"));
    }

    #[test]
    fn test_phone_validation() {
        // Valid phones
        assert!(is_valid_phone("+1234567890"));
        assert!(is_valid_phone("123-456-7890"));
        assert!(is_valid_phone("(123) 456-7890"));
        assert!(is_valid_phone("1234567890"));
        
        // Invalid phones
        assert!(!is_valid_phone("123"));
        assert!(!is_valid_phone("invalid-phone"));
        assert!(!is_valid_phone(""));
        assert!(!is_valid_phone("abc-def-ghij"));
    }

    #[test]
    fn test_url_validation() {
        // Valid URLs
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://example.com"));
        assert!(is_valid_url("https://www.example.com/path"));
        assert!(is_valid_url("https://example.com:8080/path?query=value"));
        
        // Invalid URLs
        assert!(!is_valid_url("not-a-url"));
        assert!(!is_valid_url("example.com"));
        assert!(!is_valid_url(""));
        assert!(!is_valid_url("ftp://example.com")); // Only http/https allowed
    }

    #[test]
    fn test_number_validation() {
        // Valid numbers
        assert!(is_valid_number("123"));
        assert!(is_valid_number("123.45"));
        assert!(is_valid_number("-123.45"));
        assert!(is_valid_number("0"));
        assert!(is_valid_number("0.0"));
        
        // Invalid numbers
        assert!(!is_valid_number(""));
        assert!(!is_valid_number("abc"));
        assert!(!is_valid_number("12.34.56"));
        assert!(!is_valid_number("12,34"));
    }

    #[test]
    fn test_integer_validation() {
        // Valid integers
        assert!(is_valid_integer("123"));
        assert!(is_valid_integer("-123"));
        assert!(is_valid_integer("0"));
        assert!(is_valid_integer("9223372036854775807")); // Max i64
        
        // Invalid integers
        assert!(!is_valid_integer(""));
        assert!(!is_valid_integer("abc"));
        assert!(!is_valid_integer("123.45"));
        assert!(!is_valid_integer("12,34"));
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
}
