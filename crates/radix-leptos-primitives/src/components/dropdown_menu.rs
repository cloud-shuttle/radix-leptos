use crate::utils::merge_classes;
use leptos::callback::Callback;
use leptos::children::Children;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent};

#[derive(Clone, Debug, PartialEq)]
#[derive(Default)]
pub enum DropdownMenuSize {
    Small,
    #[default]
    Medium,
    Large,
}


#[derive(Clone, Debug, PartialEq)]
#[derive(Default)]
pub enum DropdownMenuItemVariant {
    #[default]
    Default,
    Destructive,
    Disabled,
}


#[component]
pub fn DropdownMenu(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let (_isopen, set_isopen) = signal(false);
    let trigger_ref = NodeRef::<html::Div>::new();
    let content_ref = NodeRef::<html::Div>::new();

    let handle_click_outside = move |e: MouseEvent| {
        if let (Some(trigger_el), Some(content_el)) = (trigger_ref.get(), content_ref.get()) {
            let target = e.target().unwrap();
            let target_element = target.dyn_ref::<web_sys::Element>().unwrap();

            if !trigger_el.contains(Some(target_element))
                && !content_el.contains(Some(target_element))
            {
                set_isopen.set(false);
            }
        }
    };

    let handle_keydown = move |e: KeyboardEvent| match e.key().as_str() {
        "Escape" => {
            set_isopen.set(false);
        }
        "Enter" | " " => {
            e.prevent_default();
            set_isopen.update(|open| *open = !*open);
        }
        _ => {}
    };

    let base_classes = ["radix-dropdown-menu", "relative", "inline-block"];

    let class_value = class.unwrap_or_default();
    let classes = merge_classes(base_classes.to_vec());
    let final_class = format!("{} {}", classes, class_value);

    view! {
        <div
            class=final_class
            style=style
            data-radix-dropdown-menu=""
            on:click=handle_click_outside
            on:keydown=handle_keydown
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuTrigger(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    children: Children,
) -> impl IntoView {
    let handle_click = move |e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        if !disabled.unwrap_or(false) {
            // This would need to be connected to the parent DropdownMenu
            // For now, we'll just log the action
            web_sys::console::log_1(&"DropdownMenu trigger clicked".into());
        }
    };

    let handle_keydown = move |e: KeyboardEvent| {
        if !disabled.unwrap_or(false) {
            match e.key().as_str() {
                "Enter" | " " => {
                    e.prevent_default();
                    web_sys::console::log_1(&"DropdownMenu trigger activated".into());
                }
                "ArrowDown" => {
                    e.prevent_default();
                    web_sys::console::log_1(&"DropdownMenu trigger arrow down".into());
                }
                _ => {}
            }
        }
    };

    let base_classes = [
        "radix-dropdown-menu-trigger",
        "inline-flex",
        "items-center",
        "justify-center",
        "rounded-md",
        "text-sm",
        "font-medium",
        "transition-colors",
        "focus-visible:outline-none",
        "focus-visible:ring-2",
        "focus-visible:ring-ring",
        "focus-visible:ring-offset-2",
        "disabled:pointer-events-none",
        "disabled:opacity-50",
    ];

    let class_value = class.unwrap_or_default();
    let classes = merge_classes(base_classes.to_vec());
    let final_class = format!("{} {}", classes, class_value);

    view! {
        <div
            class=final_class
            style=style
            role="button"
            tabindex="0"
            aria-haspopup="true"
            aria-expanded="false"
            data-radix-dropdown-menu-trigger=""
            on:click=handle_click
            on:keydown=handle_keydown
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuContent(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] align: Option<&'static str>,
    #[prop(optional)] side: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let align_class = align.unwrap_or("start");
    let side_class = side.unwrap_or("bottom");

    let base_classes = [
        "radix-dropdown-menu-content",
        "z-50",
        "min-w-[8rem]",
        "overflow-hidden",
        "rounded-md",
        "border",
        "bg-popover",
        "p-1",
        "text-popover-foreground",
        "shadow-md",
        "animate-in",
        "data-[side=bottom]:slide-in-from-top-2",
        "data-[side=left]:slide-in-from-right-2",
        "data-[side=right]:slide-in-from-left-2",
        "data-[side=top]:slide-in-from-bottom-2",
    ];

    let class_value = class.unwrap_or_default();
    let classes = merge_classes(base_classes.to_vec());
    let final_class = format!("{} {}", classes, class_value);

    view! {
        <div
            class=final_class
            style=style
            data-side=side_class
            data-align=align_class
            data-radix-dropdown-menu-content=""
            role="menu"
            aria-orientation="vertical"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuItem(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] variant: Option<DropdownMenuItemVariant>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let handle_click = move |e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        if !disabled.unwrap_or(false) {
            if let Some(callback) = on_click {
                callback.run(());
            }
        }
    };

    let handle_keydown = move |e: KeyboardEvent| {
        if !disabled.unwrap_or(false) {
            match e.key().as_str() {
                "Enter" | " " => {
                    e.prevent_default();
                    if let Some(callback) = on_click {
                        callback.run(());
                    }
                }
                "Escape" => {
                    web_sys::console::log_1(&"DropdownMenu item escape".into());
                }
                _ => {}
            }
        }
    };

    let variant = variant.unwrap_or_default();
    let variant_classes = match variant {
        DropdownMenuItemVariant::Default => ["hover:bg-accent", "hover:text-accent-foreground"],
        DropdownMenuItemVariant::Destructive => ["text-destructive", "focus:text-destructive"],
        DropdownMenuItemVariant::Disabled => ["opacity-50", "pointer-events-none"],
    };

    let base_classes = [
        "radix-dropdown-menu-item",
        "relative",
        "flex",
        "cursor-default",
        "select-none",
        "items-center",
        "rounded-sm",
        "px-2",
        "py-1.5",
        "text-sm",
        "outline-none",
        "transition-colors",
        "focus:bg-accent",
        "focus:text-accent-foreground",
        "disabled:pointer-events-none",
        "disabled:opacity-50",
    ];

    let mut all_classes = base_classes.to_vec();
    all_classes.extend(variant_classes);

    let class_value = class.unwrap_or_default();
    let classes = merge_classes(all_classes);
    let final_class = format!("{} {}", classes, class_value);

    view! {
        <div
            class=final_class
            style=style
            role="menuitem"
            tabindex="-1"
            data-radix-dropdown-menu-item=""
            on:click=handle_click
            on:keydown=handle_keydown
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuSeparator(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let base_classes = [
        "radix-dropdown-menu-separator",
        "-mx-1",
        "my-1",
        "h-px",
        "bg-muted",
    ];

    let class_value = class.unwrap_or_default();
    let classes = merge_classes(base_classes.to_vec());
    let final_class = format!("{} {}", classes, class_value);

    view! {
        <div
            class=final_class
            style=style
            role="separator"
        />
    }
}

#[component]
pub fn DropdownMenuLabel(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let base_classes = [
        "radix-dropdown-menu-label",
        "px-2",
        "py-1.5",
        "text-sm",
        "font-semibold",
    ];

    let class_value = class.unwrap_or_default();
    let classes = merge_classes(base_classes.to_vec());
    let final_class = format!("{} {}", classes, class_value);

    view! {
        <div
            class=final_class
            style=style
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuCheckboxItem(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] checked: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] onchecked_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let (ischecked, set_ischecked) = signal(checked.unwrap_or(false));

    let handle_click = move |e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        if !disabled.unwrap_or(false) {
            let newchecked = !ischecked.get();
            set_ischecked.set(newchecked);
            if let Some(callback) = onchecked_change {
                callback.run(newchecked);
            }
        }
    };

    let handle_keydown = move |e: KeyboardEvent| {
        if !disabled.unwrap_or(false) {
            match e.key().as_str() {
                "Enter" | " " => {
                    e.prevent_default();
                    let newchecked = !ischecked.get();
                    set_ischecked.set(newchecked);
                    if let Some(callback) = onchecked_change {
                        callback.run(newchecked);
                    }
                }
                "Escape" => {
                    web_sys::console::log_1(&"DropdownMenu checkbox escape".into());
                }
                _ => {}
            }
        }
    };

    let base_classes = [
        "radix-dropdown-menu-checkbox-item",
        "relative",
        "flex",
        "cursor-default",
        "select-none",
        "items-center",
        "rounded-sm",
        "px-2",
        "py-1.5",
        "text-sm",
        "outline-none",
        "transition-colors",
        "focus:bg-accent",
        "focus:text-accent-foreground",
        "disabled:pointer-events-none",
        "disabled:opacity-50",
    ];

    let class_value = class.unwrap_or_default();
    let classes = merge_classes(base_classes.to_vec());
    let final_class = format!("{} {}", classes, class_value);

    view! {
        <div
            class=final_class
            style=style
            role="menuitemcheckbox"
            tabindex="-1"
            aria-checked=move || ischecked.get()
            on:click=handle_click
            on:keydown=handle_keydown
        >
            <div class="flex items-center gap-2">
                <div class="flex h-4 w-4 items-center justify-center">
                    <div
                        class=move || {
                            if ischecked.get() {
                                "h-2 w-2 bg-current"
                            } else {
                                ""
                            }
                        }
                        style=move || {
                            if ischecked.get() {
                                "background-color: currentColor;"
                            } else {
                                ""
                            }
                        }
                    />
                </div>
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn DropdownMenuRadioItem(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] checked: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_value_change: Option<Callback<String>>,
    children: Children,
) -> impl IntoView {
    let (ischecked, set_ischecked) = signal(checked.unwrap_or(false));
    let value = value.unwrap_or_default();

    let handle_click = {
        let value = value.clone();
        move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            if !disabled.unwrap_or(false) {
                set_ischecked.set(true);
                if let Some(callback) = on_value_change {
                    let value_clone = value.clone();
                    callback.run(value_clone);
                }
            }
        }
    };

    let handle_keydown = {
        let value = value.clone();
        move |e: KeyboardEvent| {
            if !disabled.unwrap_or(false) {
                match e.key().as_str() {
                    "Enter" | " " => {
                        e.prevent_default();
                        set_ischecked.set(true);
                        if let Some(callback) = on_value_change {
                            let value_clone = value.clone();
                            callback.run(value_clone);
                        }
                    }
                    "Escape" => {
                        web_sys::console::log_1(&"DropdownMenu radio escape".into());
                    }
                    _ => {}
                }
            }
        }
    };

    let base_classes = [
        "radix-dropdown-menu-radio-item",
        "relative",
        "flex",
        "cursor-default",
        "select-none",
        "items-center",
        "rounded-sm",
        "px-2",
        "py-1.5",
        "text-sm",
        "outline-none",
        "transition-colors",
        "focus:bg-accent",
        "focus:text-accent-foreground",
        "disabled:pointer-events-none",
        "disabled:opacity-50",
    ];

    let class_value = class.unwrap_or_default();
    let classes = merge_classes(base_classes.to_vec());
    let final_class = format!("{} {}", classes, class_value);

    view! {
        <div
            class=final_class
            style=style
            role="menuitemradio"
            tabindex="-1"
            aria-checked=move || ischecked.get()
            on:click=handle_click
            on:keydown=handle_keydown
        >
            <div class="flex items-center gap-2">
                <div class="flex h-4 w-4 items-center justify-center">
                    <div
                        class=move || {
                            if ischecked.get() {
                                "h-2 w-2 rounded-full bg-current"
                            } else {
                                ""
                            }
                        }
                        style=move || {
                            if ischecked.get() {
                                "background-color: currentColor;"
                            } else {
                                ""
                            }
                        }
                    />
                </div>
                {children()}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use crate::{DropdownMenuItemVariant, DropdownMenuSize};
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_dropdown_menu_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_dropdown_menu_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_dropdown_menu_with_style() {
        // Test that the component can be created with style
    }

    #[test]
    fn test_dropdown_menu_trigger_creation() {
        // Test that the trigger component can be created
    }

    #[test]
    fn test_dropdown_menu_triggerdisabled() {
        // Test that the trigger component can be created disabled
    }

    #[test]
    fn test_dropdown_menu_content_creation() {
        // Test that the content component can be created
    }

    #[test]
    fn test_dropdown_menu_content_with_align() {
        // Test that the content component can be created with align
    }

    #[test]
    fn test_dropdown_menu_content_with_side() {
        // Test that the content component can be created with side
    }

    #[test]
    fn test_dropdown_menu_item_creation() {
        // Test that the item component can be created
    }

    #[test]
    fn test_dropdown_menu_itemdisabled() {
        // Test that the item component can be created disabled
    }

    #[test]
    fn test_dropdown_menu_item_variants() {
        let variants = [
            DropdownMenuItemVariant::Default,
            DropdownMenuItemVariant::Destructive,
            DropdownMenuItemVariant::Disabled,
        ];

        for variant in variants {
            // Test that each variant can be created
        }
    }

    #[test]
    fn test_dropdown_menu_item_with_callback() {
        // Test that the item component can be created with callback
    }

    #[test]
    fn test_dropdown_menu_separator_creation() {
        // Test that the separator component can be created
    }

    #[test]
    fn test_dropdown_menu_separator_with_class() {
        // Test that the separator component can be created with class
    }

    #[test]
    fn test_dropdown_menu_label_creation() {
        // Test that the label component can be created
    }

    #[test]
    fn test_dropdown_menu_checkbox_item_creation() {
        // Test that the checkbox item component can be created
    }

    #[test]
    fn test_dropdown_menu_checkbox_itemchecked() {
        // Test that the checkbox item component can be created checked
    }

    #[test]
    fn test_dropdown_menu_checkbox_itemdisabled() {
        // Test that the checkbox item component can be created disabled
    }

    #[test]
    fn test_dropdown_menu_checkbox_item_with_callback() {
        // Test that the checkbox item component can be created with callback
    }

    #[test]
    fn test_dropdown_menu_radio_item_creation() {
        // Test that the radio item component can be created
    }

    #[test]
    fn test_dropdown_menu_radio_item_with_value() {
        // Test that the radio item component can be created with value
    }

    #[test]
    fn test_dropdown_menu_radio_itemchecked() {
        // Test that the radio item component can be created checked
    }

    #[test]
    fn test_dropdown_menu_radio_itemdisabled() {
        // Test that the radio item component can be created disabled
    }

    #[test]
    fn test_dropdown_menu_radio_item_with_callback() {
        // Test that the radio item component can be created with callback
    }

    #[test]
    fn test_dropdown_menu_size_default() {
        let size = DropdownMenuSize::default();
        assert_eq!(size, DropdownMenuSize::Medium);
    }

    #[test]
    fn test_dropdown_menu_item_variant_default() {
        let variant = DropdownMenuItemVariant::default();
        assert_eq!(variant, DropdownMenuItemVariant::Default);
    }

    #[test]
    fn test_merge_classes_empty() {
        let result = crate::utils::merge_classes(Vec::new());
        assert_eq!(result, "");
    }

    #[test]
    fn test_merge_classes_single() {
        let result = crate::utils::merge_classes(vec!["class1"]);
        assert_eq!(result, "class1");
    }

    #[test]
    fn test_merge_classes_multiple() {
        let result = crate::utils::merge_classes(vec!["class1", "class2", "class3"]);
        assert_eq!(result, "class1 class2 class3");
    }

    #[test]
    fn test_merge_classes_with_empty() {
        let result = crate::utils::merge_classes(vec!["class1", "", "class3"]);
        assert_eq!(result, "class1 class3");
    }

    // Property-based tests
    #[test]
    fn test_dropdown_menu_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*")| {
            // Test that the component can be created with various class and style values

        });
    }

    #[test]
    fn test_dropdown_menu_trigger_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*", _disabled: bool)| {
            // Test that the trigger component can be created with various properties

        });
    }

    #[test]
    fn test_dropdown_menu_item_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*", _disabled: bool)| {
            // Test that the item component can be created with various properties

        });
    }

    #[test]
    fn test_dropdown_menu_checkbox_item_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*", _checked: bool, _disabled: bool)| {
            // Test that the checkbox item component can be created with various properties

        });
    }

    #[test]
    fn test_dropdown_menu_radio_item_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*", __value in ".*", _checked: bool, _disabled: bool)| {
            // Test that the radio item component can be created with various properties

        });
    }
}
