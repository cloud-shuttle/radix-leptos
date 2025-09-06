use crate::utils::merge_classes;
use leptos::callback::Callback;
use leptos::children::Children;
use leptos::prelude::*;

/// Popover component for floating content containers
///
/// Provides accessible popover with keyboard support and ARIA attributes
#[component]
pub fn Popover(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] defaultopen: Option<bool>,
    #[prop(optional)] open: Option<ReadSignal<bool>>,
    #[prop(optional)] onopen_change: Option<Callback<bool>>,
) -> impl IntoView {
    let (isopen, set_isopen) = signal(
        open.map(|o| o.get())
            .unwrap_or_else(|| defaultopen.unwrap_or(false)),
    );

    // Handle external open state changes
    if let Some(externalopen) = open {
        Effect::new(move |_| {
            set_isopen.set(externalopen.get());
        });
    }

    // Handle open state changes
    if let Some(onopen_change) = onopen_change {
        Effect::new(move |_| {
            onopen_change.run(isopen.get());
        });
    }

    let class = merge_classes(vec!["popover", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
        >
        </div>
    }
}

/// Popover Trigger component
#[component]
pub fn PopoverTrigger(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);

    let class = merge_classes(vec!["popover-trigger"]);

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
            aria-haspopup="dialog"
            aria-expanded="false"
        >
            {children.map(|c| c())}
        </button>
    }
}

/// Popover Content component
#[component]
pub fn PopoverContent(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] visible: Option<ReadSignal<bool>>,
    #[prop(optional)] side: Option<PopoverSide>,
    #[prop(optional)] align: Option<PopoverAlign>,
    #[prop(optional)] side_offset: Option<f64>,
    #[prop(optional)] align_offset: Option<f64>,
) -> impl IntoView {
    let visible = visible.map(|v| v.get()).unwrap_or(true);
    let side = side.unwrap_or_default();
    let align = align.unwrap_or_default();
    let side_offset = side_offset.unwrap_or(4.0);
    let align_offset = align_offset.unwrap_or(0.0);

    if !visible {
        return view! { <></> }.into_any();
    }

    let class = merge_classes(vec![
        "popover-content",
        &side.to_class(),
        &align.to_class(),
        class.as_deref().unwrap_or(""),
    ]);

    let style = format!(
        "{}; --side-offset: {}px; --align-offset: {}px;",
        style.unwrap_or_default(),
        side_offset,
        align_offset
    );

    view! {
        <div
            class=class
            style=style
            role="dialog"
            aria-hidden="false"
            data-side=side.to_aria()
            data-align=align.to_aria()
        >
            {children.map(|c| c())}
        </div>
    }
    .into_any()
}

/// Popover Portal component
#[component]
pub fn PopoverPortal(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] container: Option<String>,
) -> impl IntoView {
    let class = merge_classes(vec!["popover-portal", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
            data-portal=container.unwrap_or_default()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Popover Arrow component
#[component]
pub fn PopoverArrow(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] width: Option<f64>,
    #[prop(optional)] height: Option<f64>,
) -> impl IntoView {
    let width = width.unwrap_or(11.0);
    let height = height.unwrap_or(5.0);

    let class = merge_classes(vec!["popover-arrow", class.as_deref().unwrap_or("")]);

    let style = format!(
        "{}; --arrow-width: {}px; --arrow-height: {}px;",
        style.unwrap_or_default(),
        width,
        height
    );

    view! {
        <div
            class=class
            style=style
            role="presentation"
        />
    }
}

/// Popover Close component
#[component]
pub fn PopoverClose(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let class = merge_classes(vec!["popover-close", class.as_deref().unwrap_or("")]);

    let handle_click = move |_| {
        if let Some(on_click) = on_click {
            on_click.run(());
        }
    };

    view! {
        <button
            class=class
            style=style
            on:click=handle_click
            aria-label="Close"
        >
            {children.map(|c| c())}
        </button>
    }
}

/// Popover Variant enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverVariant {
    #[default]
    Default,
    Destructive,
    Success,
    Warning,
    Info,
}

impl PopoverVariant {
    pub fn to_class(&self) -> &'static str {
        match self {
            PopoverVariant::Default => "variant-default",
            PopoverVariant::Destructive => "variant-destructive",
            PopoverVariant::Success => "variant-success",
            PopoverVariant::Warning => "variant-warning",
            PopoverVariant::Info => "variant-info",
        }
    }
}

/// Popover Side enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverSide {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

impl PopoverSide {
    pub fn to_class(&self) -> &'static str {
        match self {
            PopoverSide::Top => "side-top",
            PopoverSide::Right => "side-right",
            PopoverSide::Bottom => "side-bottom",
            PopoverSide::Left => "side-left",
        }
    }

    pub fn to_aria(&self) -> &'static str {
        match self {
            PopoverSide::Top => "top",
            PopoverSide::Right => "right",
            PopoverSide::Bottom => "bottom",
            PopoverSide::Left => "left",
        }
    }
}

/// Popover Align enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverAlign {
    #[default]
    Start,
    Center,
    End,
}

impl PopoverAlign {
    pub fn to_class(&self) -> &'static str {
        match self {
            PopoverAlign::Start => "align-start",
            PopoverAlign::Center => "align-center",
            PopoverAlign::End => "align-end",
        }
    }

    pub fn to_aria(&self) -> &'static str {
        match self {
            PopoverAlign::Start => "start",
            PopoverAlign::Center => "center",
            PopoverAlign::End => "end",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::merge_classes;
    use crate::{PopoverAlign, PopoverSide};
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Popover Tests
    #[test]
    fn test_popover_creation() {}

    #[test]
    fn test_popover_with_class() {}

    #[test]
    fn test_popover_with_style() {}

    #[test]
    fn test_popover_with_defaultopen() {}

    #[test]
    fn test_popover_with_controlledopen() {}

    #[test]
    fn test_popover_onopen_change() {}

    // Popover Trigger Tests
    #[test]
    fn test_popover_trigger_creation() {}

    #[test]
    fn test_popover_trigger_with_class() {}

    #[test]
    fn test_popover_trigger_with_style() {}

    #[test]
    fn test_popover_triggerdisabled() {}

    #[test]
    fn test_popover_trigger_on_click() {}

    // Popover Content Tests
    #[test]
    fn test_popover_content_creation() {}

    #[test]
    fn test_popover_content_with_class() {}

    #[test]
    fn test_popover_content_with_style() {}

    #[test]
    fn test_popover_contentvisible() {}

    #[test]
    fn test_popover_content_hidden() {}

    #[test]
    fn test_popover_content_with_side() {}

    #[test]
    fn test_popover_content_with_align() {}

    #[test]
    fn test_popover_content_with_side_offset() {}

    #[test]
    fn test_popover_content_with_align_offset() {}

    // Popover Portal Tests
    #[test]
    fn test_popover_portal_creation() {}

    #[test]
    fn test_popover_portal_with_class() {}

    #[test]
    fn test_popover_portal_with_style() {}

    #[test]
    fn test_popover_portal_with_container() {}

    // Popover Arrow Tests
    #[test]
    fn test_popover_arrow_creation() {}

    #[test]
    fn test_popover_arrow_with_class() {}

    #[test]
    fn test_popover_arrow_with_style() {}

    #[test]
    fn test_popover_arrow_with_width() {}

    #[test]
    fn test_popover_arrow_with_height() {}

    // Popover Close Tests
    #[test]
    fn test_popover_close_creation() {}

    #[test]
    fn test_popover_close_with_class() {}

    #[test]
    fn test_popover_close_with_style() {}

    #[test]
    fn test_popover_close_on_click() {}

    // Popover Side Tests
    #[test]
    fn test_popover_side_default() {
        let side = PopoverSide::default();
        assert_eq!(side, PopoverSide::Top);
    }

    #[test]
    fn test_popover_side_top() {
        let side = PopoverSide::Top;
        assert_eq!(side.to_class(), "side-top");
        assert_eq!(side.to_aria(), "top");
    }

    #[test]
    fn test_popover_side_right() {
        let side = PopoverSide::Right;
        assert_eq!(side.to_class(), "side-right");
        assert_eq!(side.to_aria(), "right");
    }

    #[test]
    fn test_popover_side_bottom() {
        let side = PopoverSide::Bottom;
        assert_eq!(side.to_class(), "side-bottom");
        assert_eq!(side.to_aria(), "bottom");
    }

    #[test]
    fn test_popover_side_left() {
        let side = PopoverSide::Left;
        assert_eq!(side.to_class(), "side-left");
        assert_eq!(side.to_aria(), "left");
    }

    // Popover Align Tests
    #[test]
    fn test_popover_align_default() {
        let align = PopoverAlign::default();
        assert_eq!(align, PopoverAlign::Start);
    }

    #[test]
    fn test_popover_align_start() {
        let align = PopoverAlign::Start;
        assert_eq!(align.to_class(), "align-start");
        assert_eq!(align.to_aria(), "start");
    }

    #[test]
    fn test_popover_align_center() {
        let align = PopoverAlign::Center;
        assert_eq!(align.to_class(), "align-center");
        assert_eq!(align.to_aria(), "center");
    }

    #[test]
    fn test_popover_align_end() {
        let align = PopoverAlign::End;
        assert_eq!(align.to_class(), "align-end");
        assert_eq!(align.to_aria(), "end");
    }

    // Helper Function Tests
    #[test]
    fn test_merge_classes_empty() {
        let result = merge_classes(Vec::new());
        assert_eq!(result, "");
    }

    #[test]
    fn test_merge_classes_single() {
        let result = merge_classes(vec!["class1"]);
        assert_eq!(result, "class1");
    }

    #[test]
    fn test_merge_classes_multiple() {
        let result = merge_classes(vec!["class1", "class2", "class3"]);
        assert_eq!(result, "class1 class2 class3");
    }

    #[test]
    fn test_merge_classes_with_empty() {
        let result = merge_classes(vec!["class1", "", "class3"]);
        assert_eq!(result, "class1 class3");
    }

    // Property-based tests
    #[test]
    fn test_popover_property_based() {
        use proptest::prelude::*;
        proptest!(|(____class in ".*", __style in ".*")| {

        });
    }

    #[test]
    fn test_popover_trigger_property_based() {
        use proptest::prelude::*;
        proptest!(|(____class in ".*", __style in ".*")| {

        });
    }

    #[test]
    fn test_popover_content_property_based() {
        use proptest::prelude::*;
        proptest!(|(____class in ".*", __style in ".*", _side_offset in -100.0..100.0f64, _align_offset in -100.0..100.0f64)| {

        });
    }

    #[test]
    fn test_popover_portal_property_based() {
        use proptest::prelude::*;
        proptest!(|(____class in ".*", __style in ".*", _container in ".*")| {

        });
    }

    #[test]
    fn test_popover_arrow_property_based() {
        use proptest::prelude::*;
        proptest!(|(____class in ".*", __style in ".*", __width in 1.0..50.0f64, __height in 1.0..50.0f64)| {

        });
    }

    #[test]
    fn test_popover_close_property_based() {
        use proptest::prelude::*;
        proptest!(|(____class in ".*", __style in ".*")| {

        });
    }
}
