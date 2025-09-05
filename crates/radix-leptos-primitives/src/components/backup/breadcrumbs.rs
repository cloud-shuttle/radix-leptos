use leptos::*;
use leptos::prelude::*;

/// Breadcrumb item structure
#[derive(Clone, Debug, PartialEq)]
pub struct BreadcrumbItem {
    pub id: String,
    pub label: String,
    pub href: Option<String>,
    pub icon: Option<String>,
    pub _disabled: bool,
    pub _current: bool,
}

impl BreadcrumbItem {
    pub fn new(id: String, label: String) -> Self {
        Self {
            id,
            label,
            href: None,
            icon: None,
            disabled: false,
            current: false,
        }
    }

    pub fn with_href(mut self, href: String) -> Self {
        self.href = Some(href);
        self
    }

    pub fn with_icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_disabled(mut self, _disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_current(mut self, _current: bool) -> Self {
        self.current = current;
        self
    }
}

/// Breadcrumb separator
#[derive(Clone, Debug, PartialEq)]
pub enum BreadcrumbSeparator {
    Slash,
    Chevron,
    Arrow,
    Custom(String),
}

impl BreadcrumbSeparator {
    pub fn as_str(&self) -> String {
        match self {
            BreadcrumbSeparator::Slash => "/".to_string(),
            BreadcrumbSeparator::Chevron => "›".to_string(),
            BreadcrumbSeparator::Arrow => "→".to_string(),
            BreadcrumbSeparator::Custom(s) => s.clone(),
        }
    }
}

/// Breadcrumb context for state management
#[derive(Clone)]
pub struct BreadcrumbContext {
    pub items: Signal<Vec<BreadcrumbItem>>,
    pub separator: BreadcrumbSeparator,
    pub max_items: Option<usize>,
    pub _show_ellipsis: bool,
    pub breadcrumb_id: String,
    pub on_item_click: Option<Callback<BreadcrumbItem>>,
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

/// Main Breadcrumbs component
#[component]
pub fn Breadcrumbs(
    /// Breadcrumb items
    #[prop(optional)]
    items: Vec<BreadcrumbItem>,
    /// Separator between items
    #[prop(optional, default = BreadcrumbSeparator::Slash)]
    separator: BreadcrumbSeparator,
    /// Maximum number of items to show
    #[prop(optional)]
    max_items: Option<usize>,
    /// Whether to show ellipsis for truncated items
    #[prop(optional, default = true)]
    _show_ellipsis: bool,
    /// Item click event handler
    #[prop(optional)]
    on_item_click: Option<Callback<BreadcrumbItem>>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child content (breadcrumb items, etc.)
    children: Children,
) -> impl IntoView {
    let _breadcrumb_id = generate_id("breadcrumbs");
    
    // Reactive state
    let (items_signal, _set_items_signal) = signal(items);
    
    // Create context
    let context = BreadcrumbContext {
        items: items_signal.into(),
        separator: separator.clone(),
        max_items,
        show_ellipsis,
        breadcrumb_id: breadcrumb_id.clone(),
        on_item_click: on_item_click.clone(),
    };
    
    // Build base classes
    let base_classes = "radix-breadcrumbs";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Provide the context
    provide_context(context);
    
    view! {
        <nav
            id=breadcrumb_id
            class=combined_class
            data-separator=separator.as_str()
            data-max-items=max_items.unwrap_or_default()
            data-show-ellipsis=show_ellipsis
            role="navigation"
            aria-label="Breadcrumb"
        >
            {children()}
        </nav>
    }
}

/// BreadcrumbList component for the breadcrumb items container
#[component]
pub fn BreadcrumbList(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (breadcrumb items)
    children: Children,
) -> impl IntoView {
    let _context = use_context::<BreadcrumbContext>().expect("BreadcrumbList must be used within Breadcrumbs");
    let _list_id = generate_id("breadcrumb-list");
    
    // Build base classes
    let base_classes = "radix-breadcrumb-list";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <ol
            id=list_id
            class=combined_class
            style=style.unwrap_or_default()
            role="list"
        >
            {children()}
        </ol>
    }
}

/// BreadcrumbItem component for individual breadcrumb items
#[component]
pub fn BreadcrumbItem(
    /// The breadcrumb item this component represents
    #[prop(optional)]
    item: Option<BreadcrumbItem>,
    /// Whether this item is current
    #[prop(optional)]
    current: Option<bool>,
    /// Whether this item is disabled
    #[prop(optional)]
    disabled: Option<bool>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<BreadcrumbContext>().expect("BreadcrumbItem must be used within Breadcrumbs");
    let _item_id = generate_id("breadcrumb-item");
    
    let item_for_click = item.clone();
    let item_for_current = item.clone();
    let item_for_disabled = item.clone();
    
    let handle_click = move |event: web_sys::MouseEvent| {
        if let Some(item) = item_for_click.clone() {
            if !item.disabled {
                // Call the click handler
                if let Some(callback) = context.on_item_click.clone() {
                    callback.run(item);
                }
            } else {
                event.prevent_default();
            }
        }
    };
    
    // Determine if this item is current
    let is_current = Memo::new(move |_| {
        if let Some(current) = current {
            current
        } else if let Some(item) = item_for_current.as_ref() {
            item.current
        } else {
            false
        }
    });
    
    // Determine if this item is disabled
    let is_disabled = Memo::new(move |_| {
        if let Some(disabled) = disabled {
            disabled
        } else if let Some(item) = item_for_disabled.as_ref() {
            item.disabled
        } else {
            false
        }
    });
    
    // Build base classes
    let base_classes = "radix-breadcrumb-item";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <li
            id=item_id
            class=combined_class
            style=style.unwrap_or_default()
            data-current=is_current.get()
            data-disabled=is_disabled.get()
            role="listitem"
        >
                            <span
                    class=if is_current.get() { "radix-breadcrumb-current" } else { "radix-breadcrumb-link" }
                    data-current=is_current.get()
                    data-disabled=is_disabled.get()
                    role=if is_current.get() { "none" } else { "link" }
                    tabindex=if is_current.get() || is_disabled.get() { "-1" } else { "0" }
                    aria-current=if is_current.get() { "page" } else { "false" }
                    aria-disabled=is_disabled.get()
                    on:click=handle_click
                >
                    {children()}
                </span>
        </li>
    }
}

/// BreadcrumbLink component for breadcrumb links
#[component]
pub fn BreadcrumbLink(
    /// Link text
    #[prop(optional)]
    text: Option<String>,
    /// Link href
    #[prop(optional)]
    href: Option<String>,
    /// Link icon
    #[prop(optional)]
    icon: Option<String>,
    /// Whether this link is current
    #[prop(optional)]
    current: Option<bool>,
    /// Whether this link is disabled
    #[prop(optional)]
    disabled: Option<bool>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<BreadcrumbContext>().expect("BreadcrumbLink must be used within Breadcrumbs");
    let _link_id = generate_id("breadcrumb-link");
    let link_id_clone = link_id.clone();
    let text_clone = text.clone();
    let href_clone = href.clone();
    let icon_clone = icon.clone();
    let current_clone = current;
    let disabled_clone = disabled;
    
    let handle_click = move |event: web_sys::MouseEvent| {
        if disabled_clone.unwrap_or(false) {
            event.prevent_default();
            return;
        }
        
        // Call the click handler if provided
        if let Some(callback) = context.on_item_click.clone() {
            let item = BreadcrumbItem::new(
                link_id_clone.clone(),
                text_clone.clone().unwrap_or_default(),
            ).with_href(href_clone.clone().unwrap_or_default())
             .with_icon(icon_clone.clone().unwrap_or_default())
             .with_current(current_clone.unwrap_or(false))
             .with_disabled(disabled_clone.unwrap_or(false));
            
            callback.run(item);
        }
    };
    

    
    // Build base classes
    let base_classes = "radix-breadcrumb-link";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <a
            id=link_id
            href=href.unwrap_or_default()
            class=combined_class
            style=style.unwrap_or_default()
            data-current=current.unwrap_or(false)
            data-disabled=disabled.unwrap_or(false)
            role="link"
            tabindex=if disabled.unwrap_or(false) { "-1" } else { "0" }
            aria-disabled=disabled.unwrap_or(false)
            aria-current=if current.unwrap_or(false) { "page" } else { "false" }
            on:click=handle_click
        >
            {icon.map(|icon_text| view! {
                <span class="radix-breadcrumb-icon">{icon_text}</span>
            })}
            {text.map(|link_text| view! {
                <span class="radix-breadcrumb-text">{link_text}</span>
            })}
            {children()}
        </a>
    }
}

/// BreadcrumbSeparator component for separators between items
#[component]
pub fn BreadcrumbSeparator(
    /// Custom separator text
    #[prop(optional)]
    separator: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<BreadcrumbContext>().expect("BreadcrumbSeparator must be used within Breadcrumbs");
    let _separator_id = generate_id("breadcrumb-separator");
    
    // Build base classes
    let base_classes = "radix-breadcrumb-separator";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Get separator text
    let separator_text = separator.unwrap_or_else(|| context.separator.as_str());
    
    view! {
        <li
            id=separator_id
            class=combined_class
            style=style.unwrap_or_default()
            role="separator"
            aria-hidden="true"
        >
            <span class="radix-breadcrumb-separator-text">{separator_text}</span>
            {children()}
        </li>
    }
}

/// BreadcrumbEllipsis component for truncated items
#[component]
pub fn BreadcrumbEllipsis(
    /// Ellipsis text
    #[prop(optional)]
    text: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let _ellipsis_id = generate_id("breadcrumb-ellipsis");
    
    // Build base classes
    let base_classes = "radix-breadcrumb-ellipsis";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <li
            id=ellipsis_id
            class=combined_class
            style=style.unwrap_or_default()
            role="separator"
            aria-hidden="true"
        >
            <span class="radix-breadcrumb-ellipsis-text">
                {text.unwrap_or_else(|| "…".to_string())}
            </span>
            {children()}
        </li>
    }
}
