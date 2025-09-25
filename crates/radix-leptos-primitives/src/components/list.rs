use leptos::children::Children;
use leptos::context::use_context;
use leptos::prelude::*;
use crate::utils::{merge_optional_classes, generate_id};

/// List item information
#[derive(Clone, Debug, PartialEq)]
pub struct ListItem<T: Send + Sync + 'static> {
    pub id: String,
    pub data: T,
    pub disabled: bool,
    pub selected: bool,
    pub _focused: bool,
}

impl<T: Send + Sync + 'static> ListItem<T> {
    pub fn new(id: String, data: T) -> Self {
        Self {
            id,
            data,
            disabled: false,
            selected: false,
            _focused: false,
        }
    }

    pub fn withdisabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn withselected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn withfocused(mut self, focused: bool) -> Self {
        self._focused = focused;
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
    pub _multi_select: bool,
    pub list_id: String,
    pub on_selection_change: Option<Callback<Vec<String>>>,
    pub on_item_click: Option<Callback<ListItem<T>>>,
    pub on_item_focus: Option<Callback<ListItem<T>>>,
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
    let (selected_items_signal, _setselected_items_signal) =
        signal(selected_items.unwrap_or_default());
    let (focused_item_signal, _setfocused_item_signal) = signal(focused_item);

    // Create context
    let context = ListContext {
        items: items_signal.into(),
        selected_items: selected_items_signal.into(),
        focused_item: focused_item_signal.into(),
        size: size.clone(),
        variant: variant.clone(),
        _multi_select: multi_select,
        list_id: list_id.clone(),
        on_selection_change,
        on_item_click,
        on_item_focus,
    };

    // Build base classes
    let base_classes = "radix-list";
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
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
                let mut currentselected = context.selected_items.get();
                let item_id = item.id.clone();

                if context._multi_select {
                    if currentselected.contains(&item_id) {
                        currentselected.retain(|id| id != &item_id);
                    } else {
                        currentselected.push(item_id);
                    }
                } else {
                    currentselected = vec![item_id];
                }

                // Call the selection change handler
                if let Some(callback) = context.on_selection_change {
                    callback.run(currentselected);
                }

                // Call the item click handler
                if let Some(callback) = context.on_item_click {
                    callback.run(item);
                }
            }
        }
    };

    let item_for_focus = item.clone();
    let handle_focus = move |_event: web_sys::FocusEvent| {
        if let Some(item) = item_for_focus.clone() {
            if let Some(callback) = context.on_item_focus {
                callback.run(item);
            }
        }
    };

    let item_forcurrent = item.clone();
    let item_fordisabled = item.clone();
    let item_forselected = item.clone();

    // Determine if this item is current
    let iscurrent = Memo::new(move |_| {
        if let Some(focused) = focused {
            focused
        } else if let Some(item) = item_forcurrent.as_ref() {
            item._focused
        } else {
            false
        }
    });

    // Determine if this item is disabled
    let isdisabled = Memo::new(move |_| {
        if let Some(disabled) = disabled {
            disabled
        } else if let Some(item) = item_fordisabled.as_ref() {
            item.disabled
        } else {
            false
        }
    });

    // Determine if this item is selected
    let isselected = Memo::new(move |_| {
        if let Some(selected) = selected {
            selected
        } else if let Some(item) = item_forselected.as_ref() {
            item.selected
        } else {
            false
        }
    });

    // Build base classes
    let base_classes = "radix-list-item";
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());

    view! {
        <div
            id=item_id
            class=combined_class
            style=style.unwrap_or_default()
            data-disabled=isdisabled.get()
            data-selected=isselected.get()
            data-current=iscurrent.get()
            role="option"
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
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
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
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
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
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
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
                view! { <span class="radix-list-empty-message">{String::new()}</span> }
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
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
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
                view! { <span class="radix-list-empty-message">{String::new()}</span> }
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
pub fn createdisabled_list_item<T: Send + Sync + 'static>(id: &str, data: T) -> ListItem<T> {
    ListItem::new(id.to_string(), data).withdisabled(true)
}

/// Helper function to create a selected list item
pub fn createselected_list_item<T: Send + Sync + 'static>(id: &str, data: T) -> ListItem<T> {
    ListItem::new(id.to_string(), data).withselected(true)
}
