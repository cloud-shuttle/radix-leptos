use leptos::*;
use leptos::prelude::*;
use leptos::context::Provider;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Toast variant for different notification types
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToastVariant {
    Default,
    Success,
    Error,
    Warning,
    Info,
}

/// Toast position on screen
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToastPosition {
    TopLeft,
    TopRight,
    TopCenter,
    BottomLeft,
    BottomRight,
    BottomCenter,
}

/// Toast size variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToastSize {
    Small,
    Medium,
    Large,
}

/// Individual toast item
#[derive(Clone)]
pub struct ToastItem {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub variant: ToastVariant,
    pub duration: Option<u32>, // milliseconds, None for persistent
    pub created_at: std::time::Instant,
}

/// Toast context for managing toast state across components
#[derive(Clone)]
struct ToastContext {
    toasts: ReadSignal<HashMap<String, ToastItem>>,
    set_toasts: WriteSignal<HashMap<String, ToastItem>>,
    position: ToastPosition,
    max_toasts: usize,
}

/// Generate a unique ID for toast items
fn generate_toast_id() -> String {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("toast-{}", id)
}

/// Merge CSS classes with proper spacing
fn merge_classes(classes: &[&str]) -> String {
    classes.iter().filter(|&&c| !c.is_empty()).map(|&s| s).collect::<Vec<&str>>().join(" ")
}

/// Create a new toast item
pub fn create_toast(
    title: impl Into<String>,
    description: Option<impl Into<String>>,
    variant: ToastVariant,
    duration: Option<u32>,
) -> ToastItem {
    ToastItem {
        id: generate_toast_id(),
        title: title.into(),
        description: description.map(|d| d.into()),
        variant,
        duration,
        created_at: std::time::Instant::now(),
    }
}

/// Root component for Toast system - provides context and state management
#[component]
pub fn ToastRoot(
    /// Default position for toasts
    #[prop(optional, default = ToastPosition::TopRight)]
    position: ToastPosition,
    /// Maximum number of toasts to show at once
    #[prop(optional, default = 5)]
    max_toasts: usize,
    /// Child components
    children: Children,
) -> impl IntoView {
    let (toasts_signal, set_toasts_signal) = signal(HashMap::<String, ToastItem>::new());
    
    let context = ToastContext {
        toasts: toasts_signal,
        set_toasts: set_toasts_signal,
        position,
        max_toasts,
    };
    
    view! {
        <Provider value=context>
            {children()}
        </Provider>
    }
}

/// Toast provider component that renders the toast container
#[component]
pub fn ToastProvider() -> impl IntoView {
    let context = use_context::<ToastContext>()
        .expect("ToastProvider must be used within ToastRoot");
    
    let toasts = move || context.toasts.get();
    let position = move || context.position;
    
    // Auto-dismiss toasts with duration
    Effect::new(move |_| {
        let current_toasts = toasts();
        let mut updated_toasts = current_toasts.clone();
        let mut to_remove = Vec::new();
        
        for (id, toast) in &current_toasts {
            if let Some(duration) = toast.duration {
                if toast.created_at.elapsed().as_millis() > duration as u128 {
                    to_remove.push(id.clone());
                }
            }
        }
        
        for id in to_remove {
            updated_toasts.remove(&id);
        }
        
        if updated_toasts.len() != current_toasts.len() {
            context.set_toasts.set(updated_toasts);
        }
    });
    
    let position_class = move || {
        match position() {
            ToastPosition::TopLeft => "radix-toast--position-top-left",
            ToastPosition::TopRight => "radix-toast--position-top-right",
            ToastPosition::TopCenter => "radix-toast--position-top-center",
            ToastPosition::BottomLeft => "radix-toast--position-bottom-left",
            ToastPosition::BottomRight => "radix-toast--position-bottom-right",
            ToastPosition::BottomCenter => "radix-toast--position-bottom-center",
        }
    };
    
    let toasts_vec = move || {
        toasts().into_iter().collect::<Vec<_>>()
    };
    
    view! {
        <div class=merge_classes(&["radix-toast-provider", &position_class()])>
            <For
                each=toasts_vec
                key=|(id, _)| id.clone()
                children=move |(id, toast)| {
                    let context_clone = context.clone();
                    let toast_clone = toast.clone();
                    
                    let handle_dismiss = Callback::new(move |_: web_sys::MouseEvent| {
                        let mut current_toasts = context_clone.toasts.get();
                        current_toasts.remove(&id);
                        context_clone.set_toasts.set(current_toasts);
                    });
                    
                    view! {
                        <ToastItemComponent
                            toast=toast_clone
                            on_dismiss=handle_dismiss
                        />
                    }
                }
            />
        </div>
    }
}

/// Individual toast item component
#[component]
pub fn ToastItemComponent(
    /// The toast item to display
    toast: ToastItem,
    /// Dismiss handler
    #[prop(optional)]
    on_dismiss: Option<Callback<web_sys::MouseEvent>>,
) -> impl IntoView {
    let variant_class = move || {
        match toast.variant {
            ToastVariant::Default => "radix-toast--variant-default",
            ToastVariant::Success => "radix-toast--variant-success",
            ToastVariant::Error => "radix-toast--variant-error",
            ToastVariant::Warning => "radix-toast--variant-warning",
            ToastVariant::Info => "radix-toast--variant-info",
        }
    };
    
    let variant_icon = move || {
        match toast.variant {
            ToastVariant::Default => "📢",
            ToastVariant::Success => "✅",
            ToastVariant::Error => "❌",
            ToastVariant::Warning => "⚠️",
            ToastVariant::Info => "ℹ️",
        }
    };
    
    let handle_dismiss = move |e: web_sys::MouseEvent| {
        if let Some(callback) = on_dismiss {
            callback.run(e);
        }
    };
    
    let title_clone = toast.title.clone();
    let description_clone = toast.description.clone();
    let icon_clone = variant_icon();
    
    view! {
        <div
            class=merge_classes(&["radix-toast-item", &variant_class()])
            role="alert"
            aria-live="polite"
        >
            <div class="radix-toast-item-content">
                <div class="radix-toast-item-icon">
                    {icon_clone}
                </div>
                <div class="radix-toast-item-body">
                    <div class="radix-toast-item-header">
                        <h4 class="radix-toast-item-title">
                            {title_clone}
                        </h4>
                        <button
                            class="radix-toast-item-close"
                            on:click=handle_dismiss
                            aria-label="Close notification"
                        >
                            "×"
                        </button>
                    </div>
                    {move || {
                        if let Some(desc) = &description_clone {
                            let desc_clone = desc.clone();
                            view! {
                                <p class="radix-toast-item-description">
                                    {desc_clone}
                                </p>
                            }
                        } else {
                            view! {
                                <p class="radix-toast-item-description">{String::new()}</p>
                            }
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

/// Hook to use toast functionality
pub fn use_toast() -> impl Fn(ToastItem) + Clone {
    let context = use_context::<ToastContext>()
        .expect("use_toast must be used within ToastRoot");
    
    move |toast: ToastItem| {
        let mut current_toasts = context.toasts.get();
        
        // Remove oldest toast if at max capacity
        if current_toasts.len() >= context.max_toasts {
            let oldest_id = current_toasts
                .iter()
                .min_by_key(|(_, t)| t.created_at)
                .map(|(id, _)| id.clone());
            
            if let Some(id) = oldest_id {
                current_toasts.remove(&id);
            }
        }
        
        current_toasts.insert(toast.id.clone(), toast);
        context.set_toasts.set(current_toasts);
    }
}

/// Toast action component for triggering toasts
#[component]
pub fn ToastAction(
    /// Toast item to show
    toast: ToastItem,
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
    let show_toast = use_toast();
    let toast_clone = toast.clone();
    
    let handle_click = move |e: web_sys::MouseEvent| {
        if !disabled {
            show_toast(toast_clone.clone());
            if let Some(callback) = on_click {
                callback.run(e);
            }
        }
    };
    
    let class_value = class.unwrap_or_default();
    let children_view = children();
    
    view! {
        <button
            class=class_value
            disabled=disabled
            on:click=handle_click
        >
            {children_view}
        </button>
    }
}

/// Toast viewport component for rendering toasts
#[component]
pub fn ToastViewport() -> impl IntoView {
    view! {
        <ToastProvider />
    }
}
