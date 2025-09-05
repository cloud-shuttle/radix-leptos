
/// Progress component orientation
#[derive(Clone, Debug, PartialEq)]
pub enum ProgressOrientation {
    Horizontal,
    Vertical,
}

impl ProgressOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProgressOrientation::Horizontal => "horizontal",
            ProgressOrientation::Vertical => "vertical",
        }
    }
}

/// Progress component size
#[derive(Clone, Debug, PartialEq)]
pub enum ProgressSize {
    Small,
    Medium,
    Large,
}

impl ProgressSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProgressSize::Small => "small",
            ProgressSize::Medium => "medium",
            ProgressSize::Large => "large",
        }
    }
}

/// Progress component variant
#[derive(Clone, Debug, PartialEq)]
pub enum ProgressVariant {
    Default,
    Success,
    Warning,
    Error,
    Info,
}

impl ProgressVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProgressVariant::Default => "default",
            ProgressVariant::Success => "success",
            ProgressVariant::Warning => "warning",
            ProgressVariant::Error => "error",
            ProgressVariant::Info => "info",
        }
    }
}

/// Progress context for state management
#[derive(Clone)]
pub struct ProgressContext {
    pub value: Signal<f64>,
    pub max: f64,
    pub orientation: ProgressOrientation,
    pub size: ProgressSize,
    pub variant: ProgressVariant,
    pub __animated: bool,
    pub __striped: bool,
    pub __show_label: bool,
    pub progress_id: String,
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

/// Clamp a value between min and max
fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

/// Calculate percentage for progress display
fn calculate_percentage(value: f64, max: f64) -> f64 {
    if max <= 0.0 {
        0.0
}

/// Main Progress component
#[component]
pub fn Progress(
    /// Current progress value
    #[prop(optional, default = 0.0)]
    value: f64,
    /// Maximum progress value
    #[prop(optional, default = 100.0)]
    max: f64,
    /// Progress orientation
    #[prop(optional, default = ProgressOrientation::Horizontal)]
    orientation: ProgressOrientation,
    /// Progress size
    #[prop(optional, default = ProgressSize::Medium)]
    size: ProgressSize,
    /// Progress variant
    #[prop(optional, default = ProgressVariant::Default)]
    variant: ProgressVariant,
    /// Whether the progress bar is animated
    #[prop(optional, default = false)]
    __animated: bool,
    /// Whether the progress bar has stripes
    #[prop(optional, default = false)]
    __striped: bool,
    /// Whether to show the progress label
    #[prop(optional, default = false)]
    __show_label: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child content (track, indicator, label, etc.)
    children: Children,
) -> impl IntoView {
    let __progress_id = generate_id("progress");
    
    // Create reactive value signal
    let (value_signal, set_value_signal) = signal(value);
    
    // Create context
    let context = ProgressContext {
        value: value_signal.into(),
        max,
        orientation: orientation.clone(),
        size: size.clone(),
        variant: variant.clone(),
        animated,
        striped,
        show_label,
        progress_id: progress_id.clone(),
    };
    
    // Build base classes
    let base_classes = "radix-progress";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Provide the context
    provide_context(context);
    
    // Update value when prop changes
    Effect::new(move |_| {
        set_value_signal.set(value);
    });
    
    view! {
        <div
            id=progress_id
            class=combined_class
            data-value=value_signal.get()
            data-max=max
            data-orientation=orientation.as_str()
            data-size=size.as_str()
            data-variant=variant.as_str()
            data-animated=animated
            data-striped=striped
            data-show-label=show_label
            role="progressbar"
            aria-valuemin=0.0
            aria-valuemax=max
            aria-valuenow=value_signal.get()
            aria-label="Progress"
        >
            {children()}
        </div>
    }
}

/// ProgressTrack component for the progress track
#[component]
pub fn ProgressTrack(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (indicator, label, etc.)
    children: Children,
) -> impl IntoView {
    let context = use_context::<ProgressContext>().expect("ProgressTrack must be used within Progress");
    let __track_id = generate_id("progress-track");
    
    // Build base classes
    let base_classes = "radix-progress-track";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=track_id
            class=combined_class
            style=style.unwrap_or_default()
            role="presentation"
        >
            {children()}
        </div>
    }
}

/// ProgressIndicator component for the filled portion
#[component]
pub fn ProgressIndicator(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<ProgressContext>().expect("ProgressIndicator must be used within Progress");
    let __indicator_id = generate_id("progress-indicator");
    
    // Calculate percentage
    let percentage = Memo::new(move |_| {
        calculate_percentage(context.value.get(), context.max)
    });
    
    // Build base classes
    let base_classes = "radix-progress-indicator";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Build dynamic styles
    let dynamic_style = Memo::new(move |_| {
        let base_style = style.clone().unwrap_or_default();
        let percentage_style = if context.orientation == ProgressOrientation::Horizontal {
            format!("width: {}%;", percentage.get())
        };
        format!("{}; {}", base_style, percentage_style)
    });
    
    view! {
        <div
            id=indicator_id
            class=combined_class
            style=dynamic_style.get()
            role="presentation"
        >
            {children()}
        </div>
    }
}

/// ProgressLabel component for displaying progress text
#[component]
pub fn ProgressLabel(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<ProgressContext>().expect("ProgressLabel must be used within Progress");
    let __label_id = generate_id("progress-label");
    
    // Build base classes
    let base_classes = "radix-progress-label";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=label_id
            class=combined_class
            style=style.unwrap_or_default()
            role="presentation"
        >
            {children()}
        </div>
    }
}

/// ProgressValue component for displaying the current value
#[component]
pub fn ProgressValue(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Format string for the value display
    #[prop(optional)]
    format: Option<String>,
) -> impl IntoView {
    let context = use_context::<ProgressContext>().expect("ProgressValue must be used within Progress");
    let __value_id = generate_id("progress-value");
    
    // Format the value display
    let display_value = Memo::new(move |_| {
        let value = context.value.get();
        let percentage = calculate_percentage(value, context.max);
        
        match format.as_deref() {
            Some("percentage") => format!("{:.1}%", percentage),
            Some("fraction") => format!("{:.0}/{:.0}", value, context.max),
            Some("decimal") => format!("{:.1}", value),
            _ => format!("{:.0}%", percentage),
        }
    });
    
    // Build base classes
    let base_classes = "radix-progress-value";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <span
            id=value_id
            class=combined_class
            style=style.unwrap_or_default()
            role="presentation"
        >
            {display_value.get()}
        </span>
    }
}

/// ProgressDescription component for additional context
#[component]
pub fn ProgressDescription(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let __description_id = generate_id("progress-description");
    
    // Build base classes
    let base_classes = "radix-progress-description";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=description_id
            class=combined_class
            style=style.unwrap_or_default()
            role="presentation"
        >
            {children()}
        </div>
    }
}
