use leptos::*;
use leptos::prelude::*;

/// Alert variant for different message types
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlertVariant {
    Default,
    Success,
    Error,
    Warning,
    Info,
}

/// Alert size variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlertSize {
    Small,
    Medium,
    Large,
}

/// Merge CSS classes with proper spacing
fn merge_classes(classes: &[&str]) -> String {
    classes.iter().filter(|&&c| !c.is_empty()).map(|&s| s).collect::<Vec<&str>>().join(" ")
}

/// Root Alert component
#[component]
pub fn Alert(
    /// Alert variant
    #[prop(optional, default = AlertVariant::Default)]
    variant: AlertVariant,
    /// Alert size
    #[prop(optional, default = AlertSize::Medium)]
    size: AlertSize,
    /// Whether the alert is dismissible
    #[prop(optional, default = false)]
    dismissible: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Dismiss handler
    #[prop(optional)]
    on_dismiss: Option<Callback<web_sys::MouseEvent>>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let variant_class = move || {
        match variant {
            AlertVariant::Default => "radix-alert--variant-default",
            AlertVariant::Success => "radix-alert--variant-success",
            AlertVariant::Error => "radix-alert--variant-error",
            AlertVariant::Warning => "radix-alert--variant-warning",
            AlertVariant::Info => "radix-alert--variant-info",
        }
    };
    
    let size_class = move || {
        match size {
            AlertSize::Small => "radix-alert--size-small",
            AlertSize::Medium => "radix-alert--size-medium",
            AlertSize::Large => "radix-alert--size-large",
        }
    };
    
    let variant_icon = move || {
        match variant {
            AlertVariant::Default => "ℹ️",
            AlertVariant::Success => "✅",
            AlertVariant::Error => "❌",
            AlertVariant::Warning => "⚠️",
            AlertVariant::Info => "ℹ️",
        }
    };
    
    let handle_dismiss = move |e: web_sys::MouseEvent| {
        if let Some(callback) = on_dismiss {
            callback.run(e);
        }
    };
    
    let class_value = class.unwrap_or_default();
    let children_view = children();
    let icon_clone = variant_icon();
    
    view! {
        <div
            class=merge_classes(&["radix-alert", &variant_class(), &size_class(), &class_value])
            role="alert"
            aria-live="polite"
        >
            <div class="radix-alert-content">
                <div class="radix-alert-icon">
                    {icon_clone}
                </div>
                <div class="radix-alert-body">
                    {children_view}
                </div>
                <button
                    class=move || {
                        if dismissible {
                            "radix-alert-close"
                        } else {
                            "radix-alert-close radix-alert-close--hidden"
                        }
                    }
                    disabled=move || !dismissible
                    on:click=handle_dismiss
                    aria-label="Close alert"
                >
                    "×"
                </button>
            </div>
        </div>
    }
}

/// Alert title component
#[component]
pub fn AlertTitle(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();
    let children_view = children();
    
    view! {
        <h4 class=merge_classes(&["radix-alert-title", &class_value])>
            {children_view}
        </h4>
    }
}

/// Alert description component
#[component]
pub fn AlertDescription(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();
    let children_view = children();
    
    view! {
        <div class=merge_classes(&["radix-alert-description", &class_value])>
            {children_view}
        </div>
    }
}

/// Alert action component
#[component]
pub fn AlertAction(
    /// Whether the action is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let handle_click = move |e: web_sys::MouseEvent| {
        if !disabled {
            if let Some(callback) = on_click {
                callback.run(e);
            }
        }
    };
    
    let class_value = class.unwrap_or_default();
    let children_view = children();
    
    view! {
        <button
            class=merge_classes(&["radix-alert-action", &class_value])
            disabled=disabled
            on:click=handle_click
        >
            {children_view}
        </button>
    }
}
