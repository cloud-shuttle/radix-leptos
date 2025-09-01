use leptos::*;
use leptos::prelude::*;

/// List item information
#[derive(Clone, Debug, PartialEq)]
pub struct ListItem<T: Send + Sync + 'static> {
    pub id: String,
    pub data: T,
    pub disabled: bool,
    pub selected: bool,
    pub focused: bool,
}

impl<T: Send + Sync + 'static> ListItem<T> {
    pub fn new(id: String, data: T) -> Self {
        Self {
            id,
            data,
            disabled: false,
            selected: false,
            focused: false,
        }
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn with_focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }
}

/// List size variants
#[derive(Clone, Debug, PartialEq)]
pub enum ListSize {
    Small,
    Medium,
    Large,
}

impl ListSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ListSize::Small => "small",
            ListSize::Medium => "medium",
            ListSize::Large => "large",
        }
    }
}

/// List variant styles
#[derive(Clone, Debug, PartialEq)]
pub enum ListVariant {
    Default,
    Bordered,
    Striped,
    Compact,
}

impl ListVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            ListVariant::Default => "default",
            ListVariant::Bordered => "bordered",
            ListVariant::Striped => "striped",
            ListVariant::Compact => "compact",
        }
    }
}

/// List context for state management
#[derive(Clone)]
pub struct ListContext<T: Send + Sync + 'static> {
    pub items: Signal<Vec<ListItem<T>>>,
    pub selected_items: Signal<Vec<String>>,
    pub focused_item: Signal<Option<String>>,
    pub size: ListSize,
    pub variant: ListVariant,
    pub multi_select: bool,
    pub list_id: String,
    pub on_selection_change: Option<Callback<Vec<String>>>,
    pub on_item_click: Option<Callback<ListItem<T>>>,
    pub on_item_focus: Option<Callback<ListItem<T>>>,
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

/// Main List component
#[component]
pub fn List<T: Clone + Send + Sync + 'static>(
    /// List items
    #[prop(optional)]
    items: Option<Vec<ListItem<T>>>,
    /// Selected item IDs
    #[prop(optional)]
    selected_items: Option<Vec<String>>,
    /// Currently focused item ID
    #[prop(optional)]
    focused_item: Option<String>,
    /// List size
    #[prop(optional, default = ListSize::Medium)]
    size: ListSize,
    /// List variant
    #[prop(optional, default = ListVariant::Default)]
    variant: ListVariant,
    /// Whether to allow multiple selection
    #[prop(optional, default = false)]
    multi_select: bool,
    /// Selection change event handler
    #[prop(optional)]
    on_selection_change: Option<Callback<Vec<String>>>,
    /// Item click event handler
    #[prop(optional)]
    on_item_click: Option<Callback<ListItem<T>>>,
    /// Item focus event handler
    #[prop(optional)]
    on_item_focus: Option<Callback<ListItem<T>>>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child content (list items, etc.)
    children: Children,
) -> impl IntoView {
    let list_id = generate_id("list");
    
    // Reactive state
    let (items_signal, _set_items_signal) = signal(items.unwrap_or_default());
    let (selected_items_signal, _set_selected_items_signal) = signal(selected_items.unwrap_or_default());
    let (focused_item_signal, _set_focused_item_signal) = signal(focused_item);
    
    // Create context
    let context = ListContext {
        items: items_signal.into(),
        selected_items: selected_items_signal.into(),
        focused_item: focused_item_signal.into(),
        size: size.clone(),
        variant: variant.clone(),
        multi_select,
        list_id: list_id.clone(),
        on_selection_change,
        on_item_click,
        on_item_focus,
    };
    
    // Build base classes
    let base_classes = "radix-list";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Provide the context
    provide_context(context);
    
    view! {
        <div
            id=list_id
            class=combined_class
            data-size=size.as_str()
            data-variant=variant.as_str()
            data-multi-select=multi_select
            role="listbox"
            aria-multiselectable=multi_select
        >
            {children()}
        </div>
    }
}

/// ListItem component for individual list items
#[component]
pub fn ListItem<T: Clone + Send + Sync + 'static>(
    /// The list item this component represents
    #[prop(optional)]
    item: Option<ListItem<T>>,
    /// Whether this item is disabled
    #[prop(optional)]
    disabled: Option<bool>,
    /// Whether this item is selected
    #[prop(optional)]
    selected: Option<bool>,
    /// Whether this item is focused
    #[prop(optional)]
    focused: Option<bool>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<ListContext<T>>().expect("ListItem must be used within List");
    let item_id = generate_id("list-item");
    
    let item_clone = item.clone();
    let handle_click = move |event: web_sys::MouseEvent| {
        event.prevent_default();
        
        if let Some(item) = item_clone.clone() {
            if !item.disabled {
                // Handle selection
                let mut current_selected = context.selected_items.get();
                let item_id = item.id.clone();
                
                if context.multi_select {
                    if current_selected.contains(&item_id) {
                        current_selected.retain(|id| id != &item_id);
                    } else {
                        current_selected.push(item_id);
                    }
                } else {
                    current_selected = vec![item_id];
                }
                
                // Call the selection change handler
                if let Some(callback) = context.on_selection_change.clone() {
                    callback.run(current_selected);
                }
                
                // Call the item click handler
                if let Some(callback) = context.on_item_click.clone() {
                    callback.run(item);
                }
            }
        }
    };
    
    let item_for_focus = item.clone();
    let handle_focus = move |_event: web_sys::FocusEvent| {
        if let Some(item) = item_for_focus.clone() {
            if let Some(callback) = context.on_item_focus.clone() {
                callback.run(item);
            }
        }
    };
    
    let item_for_current = item.clone();
    let item_for_disabled = item.clone();
    let item_for_selected = item.clone();
    
    // Determine if this item is current
    let is_current = Memo::new(move |_| {
        if let Some(focused) = focused {
            focused
        } else if let Some(item) = item_for_current.as_ref() {
            item.focused
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
    
    // Determine if this item is selected
    let is_selected = Memo::new(move |_| {
        if let Some(selected) = selected {
            selected
        } else if let Some(item) = item_for_selected.as_ref() {
            item.selected
        } else {
            false
        }
    });
    
    // Build base classes
    let base_classes = "radix-list-item";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=item_id
            class=combined_class
            style=style.unwrap_or_default()
            data-disabled=is_disabled.get()
            data-selected=is_selected.get()
            data-current=is_current.get()
            role="option"
            tabindex=if is_disabled.get() { "-1" } else { "0" }
            aria-disabled=is_disabled.get()
            aria-selected=is_selected.get()
            aria-current=if is_current.get() { "true" } else { "false" }
            on:click=handle_click
            on:focus=handle_focus
        >
            {children()}
        </div>
    }
}

/// ListHeader component for list headers
#[component]
pub fn ListHeader(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let header_id = generate_id("list-header");
    
    // Build base classes
    let base_classes = "radix-list-header";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=header_id
            class=combined_class
            style=style.unwrap_or_default()
            role="presentation"
        >
            {children()}
        </div>
    }
}

/// ListFooter component for list footers
#[component]
pub fn ListFooter(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let footer_id = generate_id("list-footer");
    
    // Build base classes
    let base_classes = "radix-list-footer";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=footer_id
            class=combined_class
            style=style.unwrap_or_default()
            role="presentation"
        >
            {children()}
        </div>
    }
}

/// ListEmpty component for empty state
#[component]
pub fn ListEmpty(
    /// Empty state message
    #[prop(optional)]
    message: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let empty_id = generate_id("list-empty");
    
    // Build base classes
    let base_classes = "radix-list-empty";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=empty_id
            class=combined_class
            style=style.unwrap_or_default()
            role="status"
            aria-live="polite"
        >
            {if let Some(msg) = message {
                view! {
                    <span class="radix-list-empty-message">{msg}</span>
                }
            } else {
                view! {
                    <span class="radix-list-empty-message">{"No items found".to_string()}</span>
                }
            }}
            {children()}
        </div>
    }
}

/// ListLoading component for loading state
#[component]
pub fn ListLoading(
    /// Loading message
    #[prop(optional)]
    message: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let loading_id = generate_id("list-loading");
    
    // Build base classes
    let base_classes = "radix-list-loading";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=loading_id
            class=combined_class
            style=style.unwrap_or_default()
            role="status"
            aria-live="polite"
            aria-label="Loading"
        >
            {if let Some(msg) = message {
                view! {
                    <span class="radix-list-loading-message">{msg}</span>
                }
            } else {
                view! {
                    <span class="radix-list-loading-message">{"Loading...".to_string()}</span>
                }
            }}
            {children()}
        </div>
    }
}

/// Helper function to create a simple list item
pub fn create_list_item<T: Send + Sync + 'static>(id: &str, data: T) -> ListItem<T> {
    ListItem::new(id.to_string(), data)
}

/// Helper function to create a disabled list item
pub fn create_disabled_list_item<T: Send + Sync + 'static>(id: &str, data: T) -> ListItem<T> {
    ListItem::new(id.to_string(), data).with_disabled(true)
}

/// Helper function to create a selected list item
pub fn create_selected_list_item<T: Send + Sync + 'static>(id: &str, data: T) -> ListItem<T> {
    ListItem::new(id.to_string(), data).with_selected(true)
}
