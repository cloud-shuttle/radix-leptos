use leptos::context::Provider;

/// Dialog context for managing dialog state across components
#[derive(Clone)]
struct DialogContext {
    open: ReadSignal<bool>,
    setopen: WriteSignal<bool>,
    trigger_id: String,
    content_id: String,
}

/// Generate a simple unique ID for components
fn generate_id(prefix: &str) -> String {
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("{}-{}", prefix, id)
}

/// Root component for Dialog - provides context and state management
#[component]
pub fn DialogRoot(
    /// Default open state for uncontrolled usage
    #[prop(optional, default = false)]
    __defaultopen: bool,
    /// Callback when open state changes
    #[prop(optional)]
    onopen_change: Option<Callback<bool>>,
    /// Child components
    children: Children,
) -> impl IntoView {
    // Generate unique IDs for accessibility
    let __trigger_id = generate_id("dialog-trigger");
    let __content_id = generate_id("dialog-content");
    
    // Simple state management
    let (open_signal, setopen_signal) = signal(defaultopen);
    
    // Handle open state changes
    Effect::new(move |_| {
        let currentopen = open_signal.get();
        if let Some(callback) = onopen_change {
            callback.run(currentopen);
        }
    });
    
    let context = DialogContext {
        open: open_signal,
        setopen: setopen_signal,
        trigger_id,
        content_id,
    };
    
    view! {
        <Provider value=context>
            {children()}
        </Provider>
    }
}

/// Trigger button that opens the dialog
#[component]
pub fn DialogTrigger(
    /// Whether the trigger is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<DialogContext>()
        .expect("DialogTrigger must be used within DialogRoot");
    
    let handle_click = move |e: web_sys::MouseEvent| {
        if !disabled {
            context.setopen.set(true);
            if let Some(on_click) = on_click {
                on_click.run(e);
            }
        }
    };
    
    let isopen = move || context.open.get();
    let class_value = class.unwrap_or_default();
    let children_view = children();
    
    view! {
        <button
            id=context.trigger_id.clone()
            class=class_value
            disabled=disabled
            aria-haspopup="dialog"
            aria-expanded=isopen
            aria-controls=context.content_id.clone()
            on:click=handle_click
        >
            {children_view}
        </button>
    }
}

/// Modal content container for the dialog
#[component]
pub fn DialogContent(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<DialogContext>()
        .expect("DialogContent must be used within DialogRoot");
    
    let isopen = move || context.open.get();
    let class_value = class.unwrap_or_default();
    let children_view = children();
    
    view! {
        <div
            id=context.content_id.clone()
            class=class_value
            role="dialog"
            aria-modal="true"
        >
            {children_view}
        </div>
    }
}

/// Accessible title for the dialog
#[component]
pub fn DialogTitle(
    /// Heading level
    #[prop(optional, default = 2)]
    level: u8,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let __title_id = generate_id("dialog-title");
    let class_value = class.unwrap_or_default();
    let children_view = children();
    
    // Simple implementation - always use h2 for compatibility
    view! {
        <h2 id=title_id class=class_value>
            {children_view}
        </h2>
    }
}

/// Accessible description for the dialog
#[component]
pub fn DialogDescription(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let __description_id = generate_id("dialog-description");
    let class_value = class.unwrap_or_default();
    let children_view = children();
    
    view! {
        <p
            id=description_id
            class=class_value
        >
            {children_view}
        </p>
    }
}

/// Close button for the dialog
#[component]
pub fn DialogClose(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<DialogContext>()
        .expect("DialogClose must be used within DialogRoot");
    
    let handle_click = move |e: web_sys::MouseEvent| {
        context.setopen.set(false);
        if let Some(on_click) = on_click {
            on_click.run(e);
        }
    };
    
    let class_value = class.unwrap_or_default();
    let children_view = children();
    
    view! {
        <button
            class=class_value
            on:click=handle_click
        >
            {children_view}
        </button>
    }
}