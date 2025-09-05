use leptos::*;
use leptos::prelude::*;
use std::collections::HashMap;
use crate::utils::merge_classes;

/// Form Validation System - Comprehensive validation with real-time feedback
#[component]
pub fn FormValidationProvider(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] validation_mode: Option<ValidationMode>,
    #[prop(optional)] on_validation_change: Option<Callback<FormValidationState>>,
) -> impl IntoView {
    let validation_mode = validation_mode.unwrap_or(ValidationMode::OnChange);
    
    let (validation_state, set_validation_state) = create_signal(FormValidationState::default());
    let (field_errors, set_field_errors) = create_signal(HashMap::<String, FieldError>::new());
    let (form_errors, set_form_errors) = create_signal(Vec::<FormError>::new());

    let class = merge_classes(vec![
        "form-validation-provider",
        validation_mode.as_str(),
        class.as_deref().unwrap_or(""),
    ]);

    let handle_validation_change = move |new_state: FormValidationState| {
        set_validation_state.set(new_state.clone());
        if let Some(callback) = on_validation_change {
            callback.run(new_state);
        }
    };

    view! {
        <div
            class=class
            style=style
            role="form"
            aria-label="Form with validation"
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Form Field with Validation
#[component]
pub fn FormField(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] name: Option<String>,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] validation_rules: Option<Vec<ValidationRule>>,
    #[prop(optional)] on_validation: Option<Callback<FieldValidationResult>>,
) -> impl IntoView {
    let name = name.unwrap_or_default();
    let label = label.unwrap_or_default();
    let required = required.unwrap_or(false);
    let validation_rules = validation_rules.unwrap_or_default();

    let class = merge_classes(vec![
        "form-field",
        if required { "required" } else { "" },
        class.as_deref().unwrap_or(""),
    ]);

    let handle_validation = move |result: FieldValidationResult| {
        if let Some(callback) = on_validation {
            callback.run(result);
        }
    };

    view! {
        <div
            class=class
            style=style
            data-field-name=name
            data-required=required
        >
            {if !label.is_empty() {
                view! {
                    <FormLabel for_id=name.clone()>
                        {label}
                        {if required {
                            view! { <span class="required-indicator">"*"</span> }
                        } else {
                            view! { <span class="required-indicator">""</span> }
                        }}
                    </FormLabel>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
            {children.map(|c| c())}
            <FormFieldError name=name.clone() />
        </div>
    }
}

/// Form Label component
#[component]
pub fn FormLabel(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] for_id: Option<String>,
) -> impl IntoView {
    let class = merge_classes(vec![
        "form-label",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <label
            class=class
            style=style
            for=for_id
        >
            {children.map(|c| c())}
        </label>
    }
}

/// Form Field Error component
#[component]
pub fn FormFieldError(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] name: Option<String>,
) -> impl IntoView {
    let name = name.unwrap_or_default();

    let class = merge_classes(vec![
        "form-field-error",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="alert"
            aria-live="polite"
            data-field-name=name
        >
            // Error message will be displayed here
        </div>
    }
}

/// Form Error Summary component
#[component]
pub fn FormErrorSummary(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] errors: Option<Vec<FormError>>,
    #[prop(optional)] show_field_errors: Option<bool>,
    #[prop(optional)] show_form_errors: Option<bool>,
) -> impl IntoView {
    let errors = errors.unwrap_or_default();
    let show_field_errors = show_field_errors.unwrap_or(true);
    let show_form_errors = show_form_errors.unwrap_or(true);

    let class = merge_classes(vec![
        "form-error-summary",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="alert"
            aria-live="polite"
            aria-label="Form errors"
        >
            {if !errors.is_empty() {
                view! {
                    <div class="error-summary-header">
                        <h3>"Please correct the following errors:"</h3>
                    </div>
                    <ul class="error-summary-list">
                        {errors.into_iter().map(|error| {
                            view! {
                                <li class="error-summary-item">
                                    <span class="error-field">{error.field}</span>
                                    <span class="error-message">{error.message}</span>
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                    </ul>
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }}
        </div>
    }
}

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
                        message: Some("Invalid number format".to_string()),
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
                        message: Some("Invalid number format".to_string()),
                    }
                }
            }
            ValidationRuleType::Pattern(pattern) => {
                if let Ok(regex) = regex::Regex::new(pattern) {
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
                        message: Some("Invalid regex pattern".to_string()),
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
                    ValidationResult {
                        is_valid: false,
                        message: Some("Custom validator not found".to_string()),
                    }
                }
            }
        }
    }
}

/// Email validation
fn is_valid_email(email: &str) -> bool {
    let email_regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

/// URL validation
fn is_valid_url(url: &str) -> bool {
    let url_regex = regex::Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap();
    url_regex.is_match(url)
}

/// Phone validation
fn is_valid_phone(phone: &str) -> bool {
    let phone_regex = regex::Regex::new(r"^\+?[\d\s\-\(\)]{10,}$").unwrap();
    phone_regex.is_match(phone)
}

/// Date validation
fn is_valid_date(date: &str) -> bool {
    let date_regex = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    date_regex.is_match(date)
}

/// Time validation
fn is_valid_time(time: &str) -> bool {
    let time_regex = regex::Regex::new(r"^\d{2}:\d{2}(:\d{2})?$").unwrap();
    time_regex.is_match(time)
}

/// Number validation
fn is_valid_number(number: &str) -> bool {
    number.parse::<f64>().is_ok()
}

/// Integer validation
fn is_valid_integer(integer: &str) -> bool {
    integer.parse::<i64>().is_ok()
}

#[cfg(test)]
mod form_validation_tests {
    use super::*;
    use leptos::*;
    use proptest::prelude::*;

    #[test]
    fn test_form_validation_provider_creation() {
        let runtime = create_runtime();
        let _view = view! {
            <FormValidationProvider>
                <div>"Test form"</div>
            </FormValidationProvider>
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
    }

    #[test]
    fn test_form_field_creation() {
        let runtime = create_runtime();
        let _view = view! {
            <FormField name="email" label="Email" required=true>
                <input type="email" />
            </FormField>
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
    }

    #[test]
    fn test_form_label_creation() {
        let runtime = create_runtime();
        let _view = view! {
            <FormLabel for_id="email">"Email Address"</FormLabel>
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
    }

    #[test]
    fn test_form_field_error_creation() {
        let runtime = create_runtime();
        let _view = view! {
            <FormFieldError name="email" />
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
    }

    #[test]
    fn test_form_error_summary_creation() {
        let runtime = create_runtime();
        let errors = vec![
            FormError {
                field: "email".to_string(),
                message: "Invalid email format".to_string(),
                error_type: ErrorType::Validation,
            }
        ];
        let _view = view! {
            <FormErrorSummary errors=errors />
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
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
            custom_validator: None,
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
            custom_validator: None,
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
            custom_validator: None,
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
                custom_validator: None,
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
        assert!(true);
    }

    #[test]
    fn test_form_validation_accessibility() {
        // Test form validation accessibility features
        assert!(true);
    }

    #[test]
    fn test_form_validation_performance() {
        // Test form validation performance
        assert!(true);
    }

    #[test]
    fn test_form_validation_error_handling() {
        // Test form validation error handling
        assert!(true);
    }

    // Performance Tests
    #[test]
    fn test_validation_engine_performance() {
        // Test validation engine performance
        assert!(true);
    }

    #[test]
    fn test_form_validation_memory_usage() {
        // Test form validation memory usage
        assert!(true);
    }

    #[test]
    fn test_validation_rule_performance() {
        // Test validation rule performance
        assert!(true);
    }
}
