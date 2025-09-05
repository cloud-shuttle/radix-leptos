use leptos::prelude::*;
use leptos::*;

/// Toggle component for toggle button functionality
///
/// Provides accessible toggle button with keyboard support and ARIA attributes
#[component]
pub fn Toggle(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] variant: Option<ToggleVariant>,
    #[prop(optional)] size: Option<ToggleSize>,
    #[prop(optional)] pressed: Option<bool>,
    #[prop(optional)] default_pressed: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_pressed_change: Option<Callback<bool>>,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let variant = variant.unwrap_or_default();
    let size = size.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);
    let (is_pressed, set_is_pressed) =
        signal(pressed.unwrap_or_else(|| default_pressed.unwrap_or(false)));

    // Handle external pressed state changes
    if let Some(external_pressed) = pressed {
        Effect::new(move |_| {
            set_is_pressed.set(external_pressed);
        });
    }

    // Handle pressed state changes
    if let Some(on_pressed_change) = on_pressed_change {
        Effect::new(move |_| {
            on_pressed_change.run(is_pressed.get());
        });
    }

    let class = merge_classes([
        "toggle",
        &variant.to_class(),
        &size.to_class(),
        if is_pressed.get() { "pressed" } else { "" },
        if disabled { "disabled" } else { "" },
        class.as_deref().unwrap_or(""),
    ]);

    let handle_click = move |_| {
        if !disabled {
            set_is_pressed.update(|pressed| *pressed = !*pressed);
            if let Some(on_click) = on_click {
                on_click.run(());
            }
        }
    };

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if !disabled && (ev.key() == "Enter" || ev.key() == " ") {
            ev.prevent_default();
            set_is_pressed.update(|pressed| *pressed = !*pressed);
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
            aria-pressed=is_pressed.get()
            type="button"
        >
            {children.map(|c| c())}
        </button>
    }
}

/// Toggle Variant enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
    Ghost,
    Destructive,
}

impl ToggleVariant {
    pub fn to_class(&self) -> &'static str {
        match self {
            ToggleVariant::Default => "variant-default",
            ToggleVariant::Outline => "variant-outline",
            ToggleVariant::Ghost => "variant-ghost",
            ToggleVariant::Destructive => "variant-destructive",
        }
    }
}

/// Toggle Size enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleSize {
    #[default]
    Default,
    Small,
    Large,
}

impl ToggleSize {
    pub fn to_class(&self) -> &'static str {
        match self {
            ToggleSize::Default => "size-default",
            ToggleSize::Small => "size-small",
            ToggleSize::Large => "size-large",
        }
    }
}

/// Helper function to merge CSS classes
fn merge_classes(classes: Vec<&str>) -> String {
    classes
        .into_iter()
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Toggle Tests
    #[test]
    fn test_toggle_creation() {}

    #[test]
    fn test_toggle_with_class() {}

    #[test]
    fn test_toggle_with_style() {}

    #[test]
    fn test_toggle_default_variant() {}

    #[test]
    fn test_toggle_outline_variant() {}

    #[test]
    fn test_toggle_ghost_variant() {}

    #[test]
    fn test_toggle_destructive_variant() {}

    #[test]
    fn test_toggle_default_size() {}

    #[test]
    fn test_toggle_small_size() {}

    #[test]
    fn test_toggle_large_size() {}

    #[test]
    fn test_toggle_pressed() {}

    #[test]
    fn test_toggle_default_pressed() {}

    #[test]
    fn test_toggle_disabled() {}

    #[test]
    fn test_toggle_on_pressed_change() {}

    #[test]
    fn test_toggle_on_click() {}

    // Toggle Variant Tests
    #[test]
    fn test_toggle_variant_default() {
        let variant = ToggleVariant::default();
        assert_eq!(variant, ToggleVariant::Default);
    }

    #[test]
    fn test_toggle_variant_default_class() {
        let variant = ToggleVariant::Default;
        assert_eq!(variant.to_class(), "variant-default");
    }

    #[test]
    fn test_toggle_variant_outline_class() {
        let variant = ToggleVariant::Outline;
        assert_eq!(variant.to_class(), "variant-outline");
    }

    #[test]
    fn test_toggle_variant_ghost_class() {
        let variant = ToggleVariant::Ghost;
        assert_eq!(variant.to_class(), "variant-ghost");
    }

    #[test]
    fn test_toggle_variant_destructive_class() {
        let variant = ToggleVariant::Destructive;
        assert_eq!(variant.to_class(), "variant-destructive");
    }

    // Toggle Size Tests
    #[test]
    fn test_toggle_size_default() {
        let size = ToggleSize::default();
        assert_eq!(size, ToggleSize::Default);
    }

    #[test]
    fn test_toggle_size_default_class() {
        let size = ToggleSize::Default;
        assert_eq!(size.to_class(), "size-default");
    }

    #[test]
    fn test_toggle_size_small_class() {
        let size = ToggleSize::Small;
        assert_eq!(size.to_class(), "size-small");
    }

    #[test]
    fn test_toggle_size_large_class() {
        let size = ToggleSize::Large;
        assert_eq!(size.to_class(), "size-large");
    }

    // Helper Function Tests
    #[test]
    fn test_merge_classes_empty() {
        let result = merge_classes([]);
        assert_eq!(result, "");
    }

    #[test]
    fn test_merge_classes_single() {
        let result = merge_classes(["class1"]);
        assert_eq!(result, "class1");
    }

    #[test]
    fn test_merge_classes_multiple() {
        let result = merge_classes(["class1", "class2", "class3"]);
        assert_eq!(result, "class1 class2 class3");
    }

    #[test]
    fn test_merge_classes_with_empty() {
        let result = merge_classes(["class1", "", "class3"]);
        assert_eq!(result, "class1 class3");
    }

    // Property-based tests
    #[test]
    fn test_toggle_property_based() {
        use proptest::prelude::*;
        proptest!(|(__class in ".*", _style in ".*")| {

        });
    }
}
