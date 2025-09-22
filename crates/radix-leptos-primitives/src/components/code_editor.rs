
/// CodeEditor component - Syntax-highlighted editing
#[component]
pub fn CodeEditor(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] code: Option<String>,
    #[prop(optional)] language: Option<String>,
    #[prop(optional)] config: Option<CodeEditorConfig>,
    #[prop(optional)] readonly: Option<bool>,
    #[prop(optional)] show_line_numbers: Option<bool>,
    #[prop(optional)] show_minimap: Option<bool>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] on_save: Option<Callback<String>>,
) -> impl IntoView {
    let code = code.unwrap_or_default();
    let language = language.unwrap_or_default();
    let config = config.unwrap_or_default();
    let readonly = readonly.unwrap_or(false);
    let show_line_numbers = show_line_numbers.unwrap_or(true);
    let show_minimap = show_minimap.unwrap_or(false);

    let class = merge_classes(vec![
        "code-editor",
        </div>
    }
}

/// Code Editor Configuration
#[derive(Debug, Clone, PartialEq)]
pub struct CodeEditorConfig {
    pub theme: EditorTheme,
    pub font_size: f64,
    pub tab_size: usize,
    pub __word_wrap: bool,
    pub __auto_indent: bool,
    pub __bracket_matching: bool,
    pub __auto_complete: bool,
    pub __syntax_highlighting: bool,
}

impl Default for CodeEditorConfig {
    fn default() -> Self {
        Self {
            theme: EditorTheme::Default,
            font_size: 14.0,
            tab_size: 4,
            word_wrap: true,
            auto_indent: true,
            bracket_matching: true,
            auto_complete: true,
            syntax_highlighting: true,
        }
    }
}

/// Editor Theme
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditorTheme {
    #[default]
    Default,
    Dark,
    Light,
    Monokai,
    Solarized,
}

impl EditorTheme {
    pub fn to_class(&self) -> &'static str {
        match self {
            EditorTheme::Default => "theme-default",
            EditorTheme::Dark => "theme-dark",
            EditorTheme::Light => "theme-light",
            EditorTheme::Monokai => "theme-monokai",
            EditorTheme::Solarized => "theme-solarized",
        }
    }
}

/// Code Suggestion component
#[component]
pub fn CodeSuggestion(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] suggestions: Option<Vec<CodeSuggestionItem>>,
    #[prop(optional)] visible: Option<bool>,
    #[prop(optional)] on_select: Option<Callback<CodeSuggestionItem>>,
) -> impl IntoView {
    let suggestions = suggestions.unwrap_or_default();
    let visible = visible.unwrap_or(false);

    if !visible {
        return view! { <></> }.into_any();
    }

    let class = merge_classes(vec![
        "code-suggestion",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="listbox"
            aria-label="Code suggestions"
            data-suggestion-count=suggestions.len()
        >
            {children.map(|c| c())}
        </div>
    }.into_any()
}

/// Code Suggestion Item structure
#[derive(Debug, Clone, PartialEq)]
pub struct CodeSuggestionItem {
    pub text: String,
    pub description: Option<String>,
    pub kind: SuggestionKind,
}

/// Suggestion Kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SuggestionKind {
    #[default]
    Text,
    Method,
    Function,
    Variable,
    Keyword,
    Snippet,
}

impl SuggestionKind {
    pub fn to_class(&self) -> &'static str {
        match self {
            SuggestionKind::Text => "kind-text",
            SuggestionKind::Method => "kind-method",
            SuggestionKind::Function => "kind-function",
            SuggestionKind::Variable => "kind-variable",
            SuggestionKind::Keyword => "kind-keyword",
            SuggestionKind::Snippet => "kind-snippet",
        }
    }
}

/// Code Error component
#[component]
pub fn CodeError(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] error: Option<CodeErrorItem>,
    #[prop(optional)] visible: Option<bool>,
) -> impl IntoView {
    let error = error.unwrap_or_default();
    let visible = visible.unwrap_or(false);

    if !visible {
        return view! { <></> }.into_any();
    }

    let class = merge_classes(vec![
        "code-error",
        &error.severity.to_class(),
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="alert"
            aria-label="Code error"
            data-line=error.line
            data-column=error.column
            data-severity=error.severity.to_string()
        >
            {children.map(|c| c())}
        </div>
    }.into_any()
}

/// Code Error Item structure
#[derive(Debug, Clone, PartialEq)]
pub struct CodeErrorItem {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub severity: ErrorSeverity,
}

impl Default for CodeErrorItem {
    fn default() -> Self {
        Self {
            message: "Error".to_string(),
            line: 1,
            column: 1,
            severity: ErrorSeverity::Error,
        }
    }
}

/// Error Severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ErrorSeverity {
    #[default]
    Error,
    Warning,
    Info,
    Hint,
}

impl ErrorSeverity {
    pub fn to_class(&self) -> &'static str {
        match self {
            ErrorSeverity::Error => "severity-error",
            ErrorSeverity::Warning => "severity-warning",
            ErrorSeverity::Info => "severity-info",
            ErrorSeverity::Hint => "severity-hint",
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            ErrorSeverity::Error => "error",
            ErrorSeverity::Warning => "warning",
            ErrorSeverity::Info => "info",
            ErrorSeverity::Hint => "hint",
        }
    }
}

/// Helper function to merge CSS classes
fn merge_optional_classes(classes: Vec<&str>) -> String {
    classes
        .into_iter()
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use proptest::prelude::*;
use crate::utils::{merge_classes, generate_id};

    wasm_bindgen_test_configure!(run_in_browser);

    // Unit Tests
    #[test] fn test_codeeditor_creation() { 
    #[test] fn test_codeeditor_with_class() { 
    #[test] fn test_codeeditor_with_style() { 
    #[test] fn test_codeeditor_with_code() { 
    #[test] fn test_codeeditor_with_language() { 
    #[test] fn test_codeeditor_with_config() { 
    #[test] fn test_codeeditor_readonly() { 
    #[test] fn test_codeeditor_show_line_numbers() { 
    #[test] fn test_codeeditor_show_minimap() { 
    #[test] fn test_codeeditor_on_change() { 
    #[test] fn test_codeeditor_on_save() { 

    // Code Editor Config tests
    #[test] fn test_codeeditor_config_default() { 
    #[test] fn test_codeeditor_config_custom() { 

    // Editor Theme tests
    #[test] fn test_editor_theme_default() { 
    #[test] fn test_editor_theme_default_variant() { 
    #[test] fn test_editor_themedark() { 
    #[test] fn test_editor_theme_light() { 
    #[test] fn test_editor_theme_monokai() { 
    #[test] fn test_editor_theme_solarized() { 

    // Code Suggestion tests
    #[test] fn test_code_suggestion_creation() { 
    #[test] fn test_code_suggestion_with_class() { 
    #[test] fn test_code_suggestion_with_style() { 
    #[test] fn test_code_suggestion_suggestions() { 
    #[test] fn test_code_suggestionvisible() { 
    #[test] fn test_code_suggestion_hidden() { 
    #[test] fn test_code_suggestion_on_select() { 

    // Code Suggestion Item tests
    #[test] fn test_code_suggestion_item_creation() { 

    // Suggestion Kind tests
    #[test] fn test_suggestion_kind_default() { 
    #[test] fn test_suggestion_kind_text() { 
    #[test] fn test_suggestion_kind_method() { 
    #[test] fn test_suggestion_kind_function() { 
    #[test] fn test_suggestion_kind_variable() { 
    #[test] fn test_suggestion_kind_keyword() { 
    #[test] fn test_suggestion_kind_snippet() { 

    // Code Error tests
    #[test] fn test_code_error_creation() { 
    #[test] fn test_code_error_with_class() { 
    #[test] fn test_code_error_with_style() { 
    #[test] fn test_code_error_error() { 
    #[test] fn test_code_errorvisible() { 
    #[test] fn test_code_error_hidden() { 

    // Code Error Item tests
    #[test] fn test_code_error_item_default() { 
    #[test] fn test_code_error_item_creation() { 

    // Error Severity tests
    #[test] fn test_error_severity_default() { 
    #[test] fn test_error_severity_error() { 
    #[test] fn test_error_severity_warning() { 
    #[test] fn test_error_severity_info() { 
    #[test] fn test_error_severity_hint() { 

    // Helper function tests
    #[test] fn test_merge_classes_empty() { 
    #[test] fn test_merge_classes_single() { 
    #[test] fn test_merge_classes_multiple() { 
    #[test] fn test_merge_classes_with_empty() { 

    // Property-based Tests
    #[test] fn test_codeeditor_property_based() {
        proptest!(|(____class in ".*", __style in ".*")| {
            
        });
    }

    #[test] fn test_codeeditor_code_validation() {
        proptest!(|(__code in ".*")| {
            
        });
    }

    #[test] fn test_codeeditor_config_property_based() {
        proptest!(|(____font_size in 8.0..24.0f64, __tab_size in 2..8usize)| {
            
        });
    }

    #[test] fn test_codeeditor_theme_property_based() {
        proptest!(|(____theme_index in 0..5usize)| {
            
        });
    }

    // Integration Tests
    #[test] fn test_codeeditor_user_workflow() { 
    #[test] fn test_codeeditor_accessibility_workflow() { 
    #[test] fn test_codeeditor_syntax_highlighting() { 
    #[test] fn test_codeeditor_auto_complete() { 
    #[test] fn test_codeeditor_error_display() { 

    // Performance Tests
    #[test] fn test_codeeditor_large_files() { 
    #[test] fn test_codeeditor_typing_performance() { 
    #[test] fn test_codeeditor_memory_usage() { 
    #[test] fn test_codeeditor_syntax_highlighting_performance() { 
}
