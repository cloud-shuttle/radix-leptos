use leptos::*;
use leptos::prelude::*;

/// Context menu size variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextMenuSize {
    Small,
    Medium,
    Large,
}

/// Context menu item variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextMenuItemVariant {
    Default,
    Destructive,
    Disabled,
}

/// Merge CSS classes with proper spacing
fn merge_classes(classes: &[&str]) -> String {
    classes.iter().filter(|&&c| !c.is_empty()).map(|&s| s).collect::<Vec<&str>>().join(" ")
}

/// Root ContextMenu component
#[component]
pub fn ContextMenu(
    /// Context menu size
    #[prop(optional, default = ContextMenuSize::Medium)]
    size: ContextMenuSize,
    /// Whether the context menu is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Context menu content
    children: Children,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    let size_class = move || {
        match size {
            ContextMenuSize::Small => "radix-context-menu--size-small",
            ContextMenuSize::Medium => "radix-context-menu--size-medium",
            ContextMenuSize::Large => "radix-context-menu--size-large",
        }
    };

    let handle_context_menu = move |e: web_sys::MouseEvent| {
        if !disabled {
            e.prevent_default();
            set_is_open.set(true);
        }
    };

    let handle_keydown = move |e: web_sys::KeyboardEvent| {
        if !disabled {
            match e.key().as_str() {
                "Escape" => {
                    set_is_open.set(false);
                }
                "Enter" | " " => {
                    if !is_open.get() {
                        set_is_open.set(true);
                    }
                }
                _ => {}
            }
        }
    };

    let class_value = class.unwrap_or_default();

    let mut base_classes = [
        "radix-context-menu",
        &size_class(),
        &class_value,
    ];

    if disabled {
        base_classes.push("radix-context-menu--disabled");
    }

    if is_open.get() {
        base_classes.push("radix-context-menu--open");
    }

    view! {
        <div
            class=merge_classes(&base_classes)
            data-radix-context-menu=""
            on:contextmenu=handle_context_menu
            on:keydown=handle_keydown
        >
            <div
                class="radix-context-menu-trigger"
                data-radix-context-menu-trigger=""
                tabindex=if disabled { -1 } else { 0 }
                role="button"
                aria-haspopup="true"
                aria-expanded=move || is_open.get()
            >
                {children()}
            </div>
            
            <div
                class="radix-context-menu-content"
                data-radix-context-menu-content=""
                role="menu"
                tabindex=-1
                data-state=move || if is_open.get() { "open" } else { "closed" }
                style=move || if is_open.get() { "" } else { "display: none;" }
            >
                <div class="radix-context-menu-viewport">
                    <div class="radix-context-menu-items">
                        // Content will be provided by children
                    </div>
                </div>
            </div>
        </div>
    }
}

/// ContextMenuTrigger component
#[component]
pub fn ContextMenuTrigger(
    /// Whether the trigger is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Trigger content
    children: Children,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();

    let mut base_classes = [
        "radix-context-menu-trigger",
        &class_value,
    ];

    if disabled {
        base_classes.push("radix-context-menu-trigger--disabled");
    }

    view! {
        <div
            class=merge_classes(&base_classes)
            data-radix-context-menu-trigger=""
            tabindex=if disabled { -1 } else { 0 }
            role="button"
            aria-haspopup="true"
        >
            {children()}
        </div>
    }
}

/// ContextMenuContent component
#[component]
pub fn ContextMenuContent(
    /// Whether the content is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Content children
    children: Children,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();

    let mut base_classes = [
        "radix-context-menu-content",
        &class_value,
    ];

    if disabled {
        base_classes.push("radix-context-menu-content--disabled");
    }

    view! {
        <div
            class=merge_classes(&base_classes)
            data-radix-context-menu-content=""
            role="menu"
            tabindex=-1
            data-state="open"
        >
            <div class="radix-context-menu-viewport">
                <div class="radix-context-menu-items">
                    {children()}
                </div>
            </div>
        </div>
    }
}

/// ContextMenuItem component
#[component]
pub fn ContextMenuItem(
    /// Menu item variant
    #[prop(optional, default = ContextMenuItemVariant::Default)]
    variant: ContextMenuItemVariant,
    /// Whether the item is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// Whether the item is selected
    #[prop(optional, default = false)]
    _selected: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Menu item content
    children: Children,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();

    let variant_class = move || {
        match variant {
            ContextMenuItemVariant::Default => "radix-context-menu-item--default",
            ContextMenuItemVariant::Destructive => "radix-context-menu-item--destructive",
            ContextMenuItemVariant::Disabled => "radix-context-menu-item--disabled",
        }
    };

    let mut base_classes = [
        "radix-context-menu-item",
        &variant_class(),
        &class_value,
    ];

    if disabled {
        base_classes.push("radix-context-menu-item--disabled");
    }

    if selected {
        base_classes.push("radix-context-menu-item--selected");
    }

    let handle_click = move |e: web_sys::MouseEvent| {
        if !disabled {
            if let Some(callback) = on_click {
                callback.run(e);
            }
        }
    };

    let handle_keydown = move |e: web_sys::KeyboardEvent| {
        if !disabled {
            match e.key().as_str() {
                "Enter" | " " => {
                    e.prevent_default();
                    if let Some(_callback) = on_click {
                        // Create a synthetic mouse event
                        // In a real implementation, we'd create a proper event
                    }
                }
                _ => {}
            }
        }
    };

    view! {
        <div
            class=merge_classes(&base_classes)
            data-radix-context-menu-item=""
            role="menuitem"
            tabindex=if disabled { -1 } else { 0 }
            aria-disabled=disabled
            aria-selected=selected
            on:click=handle_click
            on:keydown=handle_keydown
        >
            {children()}
        </div>
    }
}

/// ContextMenuSeparator component
#[component]
pub fn ContextMenuSeparator(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();

    let mut base_classes = [
        "radix-context-menu-separator",
        &class_value,
    ];

    view! {
        <div
            class=merge_classes(&base_classes)
            role="separator"
            aria-orientation="horizontal"
        />
    }
}

/// ContextMenuLabel component
#[component]
pub fn ContextMenuLabel(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Label content
    children: Children,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();

    let mut base_classes = [
        "radix-context-menu-label",
        &class_value,
    ];

    view! {
        <div
            class=merge_classes(&base_classes)
            role="menuitem"
            tabindex=-1
        >
            {children()}
        </div>
    }
}

/// ContextMenuCheckboxItem component
#[component]
pub fn ContextMenuCheckboxItem(
    /// Whether the checkbox is checked
    #[prop(optional, default = false)]
    _checked: bool,
    /// Whether the item is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Change handler
    #[prop(optional)]
    on_change: Option<Callback<bool>>,
    /// Checkbox item content
    children: Children,
) -> impl IntoView {
    let (is_checked, set_is_checked) = signal(checked);

    let class_value = class.unwrap_or_default();

    let mut base_classes = [
        "radix-context-menu-checkbox-item",
        &class_value,
    ];

    if disabled {
        base_classes.push("radix-context-menu-checkbox-item--disabled");
    }

    if is_checked.get() {
        base_classes.push("radix-context-menu-checkbox-item--checked");
    }

    let handle_click = move |e: web_sys::MouseEvent| {
        if !disabled {
            let new_checked = !is_checked.get();
            set_is_checked.set(new_checked);
            if let Some(callback) = on_change {
                callback.run(new_checked);
            }
        }
    };

    let handle_keydown = move |e: web_sys::KeyboardEvent| {
        if !disabled {
            match e.key().as_str() {
                "Enter" | " " => {
                    e.prevent_default();
                    let new_checked = !is_checked.get();
                    set_is_checked.set(new_checked);
                    if let Some(callback) = on_change {
                        callback.run(new_checked);
                    }
                }
                _ => {}
            }
        }
    };

    view! {
        <div
            class=merge_classes(&base_classes)
            role="menuitemcheckbox"
            tabindex=if disabled { -1 } else { 0 }
            aria-checked=is_checked
            aria-disabled=disabled
            on:click=handle_click
            on:keydown=handle_keydown
        >
            <div class="radix-context-menu-checkbox-item-indicator">
                <span 
                    class="radix-context-menu-checkbox-item-check"
                    style=move || if is_checked.get() { "" } else { "display: none;" }
                >
                    "✓"
                </span>
            </div>
            <div class="radix-context-menu-checkbox-item-content">
                {children()}
            </div>
        </div>
    }
}

/// ContextMenuRadioItem component
#[component]
pub fn ContextMenuRadioItem(
    /// Radio item value
    value: String,
    /// Whether the radio is checked
    #[prop(optional, default = false)]
    _checked: bool,
    /// Whether the item is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Change handler
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    /// Radio item content
    children: Children,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();

    let mut base_classes = [
        "radix-context-menu-radio-item",
        &class_value,
    ];

    if disabled {
        base_classes.push("radix-context-menu-radio-item--disabled");
    }

    if checked {
        base_classes.push("radix-context-menu-radio-item--checked");
    }

    let value_clone = value.clone();
    let handle_click = move |_e: web_sys::MouseEvent| {
        if !disabled && !checked {
            if let Some(callback) = on_change {
                callback.run(value_clone.clone());
            }
        }
    };

    let value_clone2 = value.clone();
    let handle_keydown = move |e: web_sys::KeyboardEvent| {
        if !disabled {
            match e.key().as_str() {
                "Enter" | " " => {
                    e.prevent_default();
                    if !checked {
                        if let Some(callback) = on_change {
                            callback.run(value_clone2.clone());
                        }
                    }
                }
                _ => {}
            }
        }
    };

    view! {
        <div
            class=merge_classes(&base_classes)
            role="menuitemradio"
            tabindex=if disabled { -1 } else { 0 }
            aria-checked=checked
            aria-disabled=disabled
            on:click=handle_click
            on:keydown=handle_keydown
        >
            <div class="radix-context-menu-radio-item-indicator">
                <span 
                    class="radix-context-menu-radio-item-dot"
                    style=move || if checked { "" } else { "display: none;" }
                >
                    "●"
                </span>
            </div>
            <div class="radix-context-menu-radio-item-content">
                {children()}
            </div>
        </div>
    }
}
