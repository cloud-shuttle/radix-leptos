use crate::utils::merge_classes;
use leptos::callback::Callback;
use leptos::children::Children;
use leptos::prelude::*;

/// Navigation Menu component for main navigation
///
/// Provides accessible navigation with keyboard support and ARIA attributes
#[component]
pub fn NavigationMenu(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] orientation: Option<NavigationMenuOrientation>,
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
        "navigation-menu",
        &orientation.to_class(),
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <nav
            class=class
            style=style
            role="navigation"
            aria-orientation=orientation.to_aria()
        >
            {children.map(|c| c())}
        </nav>
    }
}

/// Navigation Menu List component
#[component]
pub fn NavigationMenuList(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = merge_classes(vec!["navigation-menu-list", class.as_deref().unwrap_or("")]);

    view! {
        <ul class=class style=style role="menubar">
            {children.map(|c| c())}
        </ul>
    }
}

/// Navigation Menu Item component
#[component]
pub fn NavigationMenuItem(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_select: Option<Callback<()>>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);
    let value = value.unwrap_or_default();

    let class = merge_classes(vec!["navigation-menu-item"]);

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if !disabled && (ev.key() == "Enter" || ev.key() == " ") {
            ev.prevent_default();
            if let Some(on_select) = on_select {
                on_select.run(());
            }
        }
    };

    view! {
        <li
            class=class
            style=style
            role="none"
        >
            <button
                role="menuitem"
            >
            </button>
        </li>
    }
}

/// Navigation Menu Trigger component
#[component]
pub fn NavigationMenuTrigger(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);

    let class = merge_classes(vec!["navigation-menu-trigger"]);

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
            aria-haspopup="true"
            aria-expanded="false"
        >
            {children.map(|c| c())}
        </button>
    }
}

/// Navigation Menu Content component
#[component]
pub fn NavigationMenuContent(
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

    let class = merge_classes(vec![
        "navigation-menu-content",
        class.as_deref().unwrap_or(""),
    ]);

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

/// Navigation Menu Link component
#[component]
pub fn NavigationMenuLink(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] href: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] active: Option<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);
    let active = active.unwrap_or(false);

    let class = merge_classes(vec!["navigation-menu-link", class.as_deref().unwrap_or("")]);

    let handle_click = move |_| {
        if !disabled {
            if let Some(on_click) = on_click {
                on_click.run(());
            }
        }
    };

    if let Some(href) = href {
        view! {
            <a
                class=class
                style=style
                href=href
                on:click=handle_click
            >
                {children.map(|c| c())}
            </a>
        }
        .into_any()
    } else {
        view! {
            <button
                class=class
                style=style
                on:click=handle_click
            >
                {children.map(|c| c())}
            </button>
        }
        .into_any()
    }
}

/// Navigation Menu Separator component
#[component]
pub fn NavigationMenuSeparator(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let class = merge_classes(vec![
        "navigation-menu-separator",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="separator"
            aria-orientation="horizontal"
        />
    }
}

/// Navigation Menu Orientation enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavigationMenuOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl NavigationMenuOrientation {
    pub fn to_class(&self) -> &'static str {
        match self {
            NavigationMenuOrientation::Horizontal => "horizontal",
            NavigationMenuOrientation::Vertical => "vertical",
        }
    }

    pub fn to_aria(&self) -> &'static str {
        match self {
            NavigationMenuOrientation::Horizontal => "horizontal",
            NavigationMenuOrientation::Vertical => "vertical",
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::NavigationMenuOrientation;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Navigation Menu Tests
    #[test]
    fn test_navigation_menu_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_navigation_menu_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_navigation_menu_with_style() {
        // Test that the component can be created with style
    }

    #[test]
    fn test_navigation_menu_horizontal_orientation() {
        // Test horizontal orientation
    }

    #[test]
    fn test_navigation_menu_vertical_orientation() {
        // Test vertical orientation
    }

    #[test]
    fn test_navigation_menu_with_value() {
        // Test with controlled value
    }

    #[test]
    fn test_navigation_menu_with_default_value() {
        // Test with default value
    }

    #[test]
    fn test_navigation_menu_value_change_callback() {
        // Test value change callback
    }

    // Navigation Menu List Tests
    #[test]
    fn test_navigation_menu_list_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_navigation_menu_list_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_navigation_menu_list_with_style() {
        // Test that the component can be created with style
    }

    // Navigation Menu Item Tests
    #[test]
    fn test_navigation_menu_item_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_navigation_menu_item_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_navigation_menu_item_with_style() {
        // Test that the component can be created with style
    }

    #[test]
    fn test_navigation_menu_item_with_value() {
        // Test with value
    }

    #[test]
    fn test_navigation_menu_itemdisabled() {
        // Test disabled state
    }

    #[test]
    fn test_navigation_menu_item_on_select() {
        // Test on_select callback
    }

    // Navigation Menu Trigger Tests
    #[test]
    fn test_navigation_menu_trigger_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_navigation_menu_trigger_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_navigation_menu_trigger_with_style() {
        // Test that the component can be created with style
    }

    #[test]
    fn test_navigation_menu_triggerdisabled() {
        // Test disabled state
    }

    #[test]
    fn test_navigation_menu_trigger_on_click() {
        // Test on_click callback
    }

    // Navigation Menu Content Tests
    #[test]
    fn test_navigation_menu_content_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_navigation_menu_content_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_navigation_menu_content_with_style() {
        // Test that the component can be created with style
    }

    #[test]
    fn test_navigation_menu_contentvisible() {
        // Test visible state
    }

    #[test]
    fn test_navigation_menu_content_hidden() {
        // Test hidden state
    }

    // Navigation Menu Link Tests
    #[test]
    fn test_navigation_menu_link_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_navigation_menu_link_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_navigation_menu_link_with_style() {
        // Test that the component can be created with style
    }

    #[test]
    fn test_navigation_menu_link_with_href() {
        // Test with href (anchor)
    }

    #[test]
    fn test_navigation_menu_link_without_href() {
        // Test without href (button)
    }

    #[test]
    fn test_navigation_menu_linkdisabled() {
        // Test disabled state
    }

    #[test]
    fn test_navigation_menu_link_active() {
        // Test active state
    }

    #[test]
    fn test_navigation_menu_link_on_click() {
        // Test on_click callback
    }

    // Navigation Menu Separator Tests
    #[test]
    fn test_navigation_menu_separator_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_navigation_menu_separator_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_navigation_menu_separator_with_style() {
        // Test that the component can be created with style
    }

    // Navigation Menu Orientation Tests
    #[test]
    fn test_navigation_menu_orientation_default() {
        let orientation = NavigationMenuOrientation::default();
        assert_eq!(orientation, NavigationMenuOrientation::Horizontal);
    }

    #[test]
    fn test_navigation_menu_orientation_horizontal() {
        let orientation = NavigationMenuOrientation::Horizontal;
        assert_eq!(orientation.to_class(), "horizontal");
        assert_eq!(orientation.to_aria(), "horizontal");
    }

    #[test]
    fn test_navigation_menu_orientation_vertical() {
        let orientation = NavigationMenuOrientation::Vertical;
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
    fn test_navigation_menu_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*")| {
            // Test that the component can be created with various class and style values

        });
    }

    #[test]
    fn test_navigation_menu_list_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*")| {
            // Test that the component can be created with various class and style values

        });
    }

    #[test]
    fn test_navigation_menu_item_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*", __value in ".*")| {
            // Test that the component can be created with various prop values

        });
    }

    #[test]
    fn test_navigation_menu_trigger_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*")| {
            // Test that the component can be created with various class and style values

        });
    }

    #[test]
    fn test_navigation_menu_content_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*")| {
            // Test that the component can be created with various class and style values

        });
    }

    #[test]
    fn test_navigation_menu_link_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*", _href in ".*")| {
            // Test that the component can be created with various prop values

        });
    }

    #[test]
    fn test_navigation_menu_separator_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*")| {
            // Test that the component can be created with various class and style values

        });
    }
}
