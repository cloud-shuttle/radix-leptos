
/// Text input type enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextInputType {
    Text,
    Email,
    Password,
    Number,
    Tel,
    Url,
    Search,
    Date,
    Time,
    DateTimeLocal,
    Month,
    Week,
}

impl TextInputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextInputType::Text => "text",
            TextInputType::Email => "email",
            TextInputType::Password => "password",
            TextInputType::Number => "number",
            TextInputType::Tel => "tel",
            TextInputType::Url => "url",
            TextInputType::Search => "search",
            TextInputType::Date => "date",
            TextInputType::Time => "time",
            TextInputType::DateTimeLocal => "datetime-local",
            TextInputType::Month => "month",
            TextInputType::Week => "week",
        }
    }
}

/// Generate a simple unique ID for components
fn generate_id(prefix: &str) -> String {
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("{}-{}", prefix, id)
}

/// Merge CSS classes
fn merge_classes(existing: Option<&str>, additional: Option<&str>) -> Option<String> {
    match (existing, additional) {
        (Some(a), Some(b)) => Some(format!("{} {}", a, b)),
        (Some(a), None) => Some(a.to_string()),
        (None, Some(b)) => Some(b.to_string()),
        (None, None) => None,
    }
}

/// Text Input component with proper accessibility and validation
#[component]
pub fn TextInput(
    /// Input type
    #[prop(optional, default = TextInputType::Text)]
    input_type: TextInputType,
    /// Current value
    #[prop(optional)]
    value: Option<String>,
    /// Placeholder text
    #[prop(optional)]
    placeholder: Option<String>,
    /// Whether the input is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Whether the input is required
    #[prop(optional, default = false)]
    _required: bool,
    /// Whether the input is read-only
    #[prop(optional, default = false)]
    __readonly: bool,
    /// Minimum length for validation
    #[prop(optional)]
    min_length: Option<usize>,
    /// Maximum length for validation
    #[prop(optional)]
    max_length: Option<usize>,
    /// Pattern for validation (regex)
    #[prop(optional)]
    pattern: Option<String>,
    /// Name attribute for form submission
    #[prop(optional)]
    name: Option<String>,
    /// ID attribute
    #[prop(optional)]
    id: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let input_id = id.unwrap_or_else(|| generate_id("text-input"));
    
    // Build base classes
    let base_classes = "radix-text-input";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <input
            id=input_id
            type=input_type.as_str()
            value=value.unwrap_or_default()
            placeholder=placeholder.unwrap_or_default()
            disabled=disabled
            required=required
            readonly=readonly
            minlength=min_length
            maxlength=max_length
            pattern=pattern.unwrap_or_default()
            name=name.unwrap_or_default()
            class=combined_class
            style=style.unwrap_or_default()
        />
    }
}

/// Text Input with label component for easier usage
#[component]
pub fn TextInputWithLabel(
    /// Input type
    #[prop(optional, default = TextInputType::Text)]
    input_type: TextInputType,
    /// Current value
    #[prop(optional)]
    value: Option<String>,
    /// Placeholder text
    #[prop(optional)]
    placeholder: Option<String>,
    /// Whether the input is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Whether the input is required
    #[prop(optional, default = false)]
    _required: bool,
    /// Whether the input is read-only
    #[prop(optional, default = false)]
    __readonly: bool,
    /// Name attribute for form submission
    #[prop(optional)]
    name: Option<String>,
    /// CSS classes for the input
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles for the input
    #[prop(optional)]
    style: Option<String>,
    /// Child content (label)
    children: Children,
) -> impl IntoView {
    let __input_id = generate_id("text-input");
    
    view! {
        <div style="display: flex; flex-direction: column; gap: 4px;">
            <label for=input_id.clone() class="radix-text-input-label".to_string()>
                {children()}
            </label>
            <TextInput
                input_type=input_type
                value=value.unwrap_or_default()
                placeholder=placeholder.unwrap_or_default()
                disabled=disabled
                required=required
                readonly=readonly
                name=name.unwrap_or_default()
                id=input_id
                class=class.unwrap_or_default()
                style=style.unwrap_or_default()
            />
        </div>
    }
}
