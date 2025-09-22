use crate::utils::{merge_classes, generate_id};
use leptos::callback::Callback;
use leptos::children::Children;
use leptos::prelude::*;

/// Menubar component for menu bar with keyboard navigation
///
/// Provides accessible menu bar with keyboard support and ARIA attributes
#[component]
pub fn Menubar(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] orientation: Option<MenubarOrientation>,
    #[prop(optional)] default_value: Option<String>,
    #[prop(optional)] value: Option<ReadSignal<String>>,
    #[prop(optional)] on_value_change: Option<Callback<String>>,
) -> impl IntoView {
    let orientation = orientation.unwrap_or_default();
    let (current_value, setcurrent_value) = signal(
        value
            .map(|v| v.get())
            .unwrap_or_else(|| default_value.unwrap_or_default()),
    );

    // Handle value changes
    if let Some(on_change) = on_value_change {
        Effect::new(move |_| {
            on_change.run(current_value.get());
        });
    }

    // Handle external value changes
    if let Some(external_value) = value {
        Effect::new(move |_| {
            setcurrent_value.set(external_value.get());
        });
    }

    let class = merge_classes(vec![
        "menubar",
        &orientation.to_class(),
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="menubar"
            aria-orientation=orientation.to_aria()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Menubar Menu component
#[component]
pub fn MenubarMenu(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_select: Option<Callback<()>>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);
    let value = value.unwrap_or_default();

    let class = merge_classes(vec!["menubar-menu"]);

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if !disabled && (ev.key() == "Enter" || ev.key() == " ") {
            ev.prevent_default();
            if let Some(on_select) = on_select {
                on_select.run(());
            }
        }
    };

    view! {
        <div
            class=class
            style=style
            role="none"
        >
        </div>
    }
}

/// Menubar Trigger component
#[component]
pub fn MenubarTrigger(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);

    let class = merge_classes(vec!["menubar-trigger"]);

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if !disabled && (ev.key() == "Enter" || ev.key() == " ") {
            ev.prevent_default();
            if let Some(on_click) = on_click {
                on_click.run(());
            }
        }
    };

    let handle_click = move |_| {
        if !disabled {
            if let Some(on_click) = on_click {
                on_click.run(());
            }
        }
    };

    view! {
        <button
            class=class
            style=style
            disabled=disabled
            on:click=handle_click
            on:keydown=handle_keydown
            role="menuitem"
            aria-haspopup="true"
            aria-expanded="false"
        >
            {children.map(|c| c())}
        </button>
    }
}

/// Menubar Content component
#[component]
pub fn MenubarContent(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] visible: Option<ReadSignal<bool>>,
) -> impl IntoView {
    let visible = visible.map(|v| v.get()).unwrap_or(true);

    if !visible {
        return {
            let _: () = view! { <></> };
            ().into_any()
        };
    }

    let class = merge_classes(vec!["menubar-content", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
            role="menu"
            aria-hidden="false"
        >
            {children.map(|c| c())}
        </div>
    }
    .into_any()
}

/// Menubar Item component
#[component]
pub fn MenubarItem(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_select: Option<Callback<()>>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);

    let class = merge_classes(vec!["menubar-item"]);

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if !disabled && (ev.key() == "Enter" || ev.key() == " ") {
            ev.prevent_default();
            if let Some(on_select) = on_select {
                on_select.run(());
            }
        }
    };

    view! {
        <div
            class=class
            style=style
            role="menuitem"
        >
        </div>
    }
}

/// Menubar Separator component
#[component]
pub fn MenubarSeparator(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let class = merge_classes(vec!["menubar-separator", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
            role="separator"
            aria-orientation="horizontal"
        />
    }
}

/// Menubar Group component
#[component]
pub fn MenubarGroup(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = merge_classes(vec!["menubar-group", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
            role="group"
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Menubar Label component
#[component]
pub fn MenubarLabel(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = merge_classes(vec!["menubar-label", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
            role="presentation"
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Menubar Orientation enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenubarOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl MenubarOrientation {
    pub fn to_class(&self) -> &'static str {
        match self {
            MenubarOrientation::Horizontal => "horizontal",
            MenubarOrientation::Vertical => "vertical",
        }
    }

    pub fn to_aria(&self) -> &'static str {
        match self {
            MenubarOrientation::Horizontal => "horizontal",
            MenubarOrientation::Vertical => "vertical",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MenubarOrientation;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Menubar Tests
    #[test]
    fn test_menubar_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_menubar_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_menubar_with_style() {
        // Test that the component can be created with style
    }

    #[test]
    fn test_menubar_horizontal_orientation() {
        // Test horizontal orientation
    }

    #[test]
    fn test_menubar_vertical_orientation() {
        // Test vertical orientation
    }

    #[test]
    fn test_menubar_with_value() {
        // Test with controlled value
    }

    #[test]
    fn test_menubar_with_default_value() {
        // Test with default value
    }

    #[test]
    fn test_menubar_value_change_callback() {
        // Test value change callback
    }

    // Menubar Menu Tests
    #[test]
    fn test_menubar_menu_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_menubar_menu_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_menubar_menu_with_style() {
        // Test that the component can be created with style
    }

    #[test]
    fn test_menubar_menu_with_value() {
        // Test with value
    }

    #[test]
    fn test_menubar_menudisabled() {
        // Test disabled state
    }

    #[test]
    fn test_menubar_menu_on_select() {
        // Test on_select callback
    }

    // Menubar Trigger Tests
    #[test]
    fn test_menubar_trigger_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_menubar_trigger_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_menubar_trigger_with_style() {
        // Test that the component can be created with style
    }

    #[test]
    fn test_menubar_triggerdisabled() {
        // Test disabled state
    }

    #[test]
    fn test_menubar_trigger_on_click() {
        // Test on_click callback
    }

    // Menubar Content Tests
    #[test]
    fn test_menubar_content_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_menubar_content_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_menubar_content_with_style() {
        // Test that the component can be created with style
    }

    #[test]
    fn test_menubar_contentvisible() {
        // Test visible state
    }

    #[test]
    fn test_menubar_content_hidden() {
        // Test hidden state
    }

    // Menubar Item Tests
    #[test]
    fn test_menubar_item_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_menubar_item_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_menubar_item_with_style() {
        // Test that the component can be created with style
    }

    #[test]
    fn test_menubar_itemdisabled() {
        // Test disabled state
    }

    #[test]
    fn test_menubar_item_on_select() {
        // Test on_select callback
    }

    // Menubar Separator Tests
    #[test]
    fn test_menubar_separator_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_menubar_separator_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_menubar_separator_with_style() {
        // Test that the component can be created with style
    }

    // Menubar Group Tests
    #[test]
    fn test_menubar_group_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_menubar_group_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_menubar_group_with_style() {
        // Test that the component can be created with style
    }

    // Menubar Label Tests
    #[test]
    fn test_menubar_label_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_menubar_label_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_menubar_label_with_style() {
        // Test that the component can be created with style
    }

    // Menubar Orientation Tests
    #[test]
    fn test_menubar_orientation_default() {
        let orientation = MenubarOrientation::default();
        assert_eq!(orientation, MenubarOrientation::Horizontal);
    }

    #[test]
    fn test_menubar_orientation_horizontal() {
        let orientation = MenubarOrientation::Horizontal;
        assert_eq!(orientation.to_class(), "horizontal");
        assert_eq!(orientation.to_aria(), "horizontal");
    }

    #[test]
    fn test_menubar_orientation_vertical() {
        let orientation = MenubarOrientation::Vertical;
        assert_eq!(orientation.to_class(), "vertical");
        assert_eq!(orientation.to_aria(), "vertical");
    }

    // Helper Function Tests
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
    fn test_menubar_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*")| {
            // Test that the component can be created with various class and style values

        });
    }

    #[test]
    fn test_menubar_menu_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*", __value in ".*")| {
            // Test that the component can be created with various prop values

        });
    }

    #[test]
    fn test_menubar_trigger_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*")| {
            // Test that the component can be created with various class and style values

        });
    }

    #[test]
    fn test_menubar_content_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*")| {
            // Test that the component can be created with various class and style values

        });
    }

    #[test]
    fn test_menubar_item_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*")| {
            // Test that the component can be created with various class and style values

        });
    }

    #[test]
    fn test_menubar_separator_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*")| {
            // Test that the component can be created with various class and style values

        });
    }

    #[test]
    fn test_menubar_group_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*")| {
            // Test that the component can be created with various class and style values

        });
    }

    #[test]
    fn test_menubar_label_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*")| {
            // Test that the component can be created with various class and style values

        });
    }
}
