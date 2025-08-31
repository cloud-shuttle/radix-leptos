use leptos::*;
use leptos::prelude::*;

/// Button component with proper accessibility and styling variants
///
/// The Button component provides accessible button functionality with
/// proper ARIA attributes, keyboard navigation, and flexible styling.
///
/// # Features
/// - Proper button semantics and accessibility
/// - Multiple variants (default, destructive, outline, secondary, ghost, link)
/// - Multiple sizes (default, sm, lg, icon)
/// - Disabled state handling
/// - Loading state support
/// - Click and keyboard event handling
///
/// # Example
///
/// ```rust
/// use leptos::*;
/// use radix_leptos_primitives::*;
///
/// #[component]
/// fn MyComponent() -> impl IntoView {
///     let (count, set_count) = create_signal(0);
///
///     view! {
///         <Button 
///             variant=ButtonVariant::Default
///             size=ButtonSize::Default
///             on_click=move |_| set_count.update(|c| *c += 1)
///         >
///             "Click me! Count: " {move || count.get()}
///         </Button>
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Default,
    Small,
    Large,
    Icon,
}

impl ButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonVariant::Default => "default",
            ButtonVariant::Destructive => "destructive", 
            ButtonVariant::Outline => "outline",
            ButtonVariant::Secondary => "secondary",
            ButtonVariant::Ghost => "ghost",
            ButtonVariant::Link => "link",
        }
    }
}

impl ButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonSize::Default => "default",
            ButtonSize::Small => "sm", 
            ButtonSize::Large => "lg",
            ButtonSize::Icon => "icon",
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

/// Button component with accessibility and variant support
#[component]
pub fn Button(
    /// Button styling variant
    #[prop(optional, default = ButtonVariant::Default)]
    variant: ButtonVariant,
    /// Button size
    #[prop(optional, default = ButtonSize::Default)]
    size: ButtonSize,
    /// Whether the button is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Whether the button is in a loading state
    #[prop(optional, default = false)]
    loading: bool,
    /// Button type attribute (button, submit, reset)
    #[prop(optional, into)]
    button_type: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Click event handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Focus event handler
    #[prop(optional)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Blur event handler
    #[prop(optional)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let button_id = generate_id("button");
    
    // Build data attributes for styling
    let data_variant = variant.as_str();
    let data_size = size.as_str();
    
    // Merge classes with data attributes for CSS targeting
    let base_classes = "radix-button";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Handle click events
    let handle_click = move |e: web_sys::MouseEvent| {
        if !disabled && !loading {
                            if let Some(on_click) = on_click {
                    on_click.run(e);
                }
        }
    };
    
    // Handle focus events  
    let handle_focus = move |e: web_sys::FocusEvent| {
                    if let Some(on_focus) = on_focus {
                on_focus.run(e);
            }
    };
    
    // Handle blur events
    let handle_blur = move |e: web_sys::FocusEvent| {
                    if let Some(on_blur) = on_blur {
                on_blur.run(e);
            }
    };
    
    view! {
        <button
            id=button_id
            class=combined_class
            style=style
            type=button_type.unwrap_or_else(|| "button".to_string())
            disabled=disabled || loading
            data-variant=data_variant
            data-size=data_size
            data-loading=loading
            aria-disabled=disabled || loading
            on:click=handle_click
            on:focus=handle_focus
            on:blur=handle_blur
        >
            <Show when=move || loading>
                <span class="button-spinner" aria-hidden="true">
                    "⟳"
                </span>
            </Show>
            <span class=move || if loading { "button-content loading" } else { "button-content" }>
                {children()}
            </span>
        </button>
    }
}
