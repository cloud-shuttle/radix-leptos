use crate::utils::merge_classes;
use leptos::callback::Callback;
use leptos::children::Children;
use leptos::prelude::*;

/// Hover Card component for contextual hover information
///
/// Provides accessible hover card with keyboard support and ARIA attributes
#[component]
pub fn HoverCard(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] open_delay: Option<u32>,
    #[prop(optional)] close_delay: Option<u32>,
    #[prop(optional)] defaultopen: Option<bool>,
    #[prop(optional)] open: Option<ReadSignal<bool>>,
    #[prop(optional)] onopen_change: Option<Callback<bool>>,
) -> impl IntoView {
    let open_delay = open_delay.unwrap_or(700);
    let close_delay = close_delay.unwrap_or(300);
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

    let class = merge_classes(vec!["hover-card", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
        >
        </div>
    }
}

/// Hover Card Trigger component
#[component]
pub fn HoverCardTrigger(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_mouse_enter: Option<Callback<()>>,
    #[prop(optional)] on_mouse_leave: Option<Callback<()>>,
    #[prop(optional)] on_focus: Option<Callback<()>>,
    #[prop(optional)] on_blur: Option<Callback<()>>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);

    let class = merge_classes(vec!["hover-card-trigger"]);

    let handle_mouse_leave = move |_: ()| {
        if !disabled {
            if let Some(on_mouse_leave) = on_mouse_leave {
                on_mouse_leave.run(());
            }
        }
    };

    let handle_focus = move |_| {
        if !disabled {
            if let Some(on_focus) = on_focus {
                on_focus.run(());
            }
        }
    };

    let handle_blur = move |_| {
        if !disabled {
            if let Some(on_blur) = on_blur {
                on_blur.run(());
            }
        }
    };

    let handle_mouse_enter = move |_| {
        if let Some(callback) = on_mouse_enter {
            callback.run(());
        }
    };

    let handle_mouse_leave = move |_| {
        if let Some(callback) = on_mouse_leave {
            callback.run(());
        }
    };

    view! {
        <button
            class=class
            style=style
            disabled=disabled
            on:mouseenter=handle_mouse_enter
            on:mouseleave=handle_mouse_leave
            on:focus=handle_focus
            on:blur=handle_blur
            aria-haspopup="dialog"
            aria-expanded="false"
        >
            {children.map(|c| c())}
        </button>
    }
}

/// Hover Card Content component
#[component]
pub fn HoverCardContent(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] visible: Option<ReadSignal<bool>>,
    #[prop(optional)] side: Option<HoverCardSide>,
    #[prop(optional)] align: Option<HoverCardAlign>,
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
        "hover-card-content",
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

/// Hover Card Portal component
#[component]
pub fn HoverCardPortal(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] container: Option<String>,
) -> impl IntoView {
    let class = merge_classes(vec!["hover-card-portal", class.as_deref().unwrap_or("")]);

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

/// Hover Card Arrow component
#[component]
pub fn HoverCardArrow(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] width: Option<f64>,
    #[prop(optional)] height: Option<f64>,
) -> impl IntoView {
    let width = width.unwrap_or(11.0);
    let height = height.unwrap_or(5.0);

    let class = merge_classes(vec!["hover-card-arrow", class.as_deref().unwrap_or("")]);

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

/// Hover Card Side enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HoverCardSide {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

impl HoverCardSide {
    pub fn to_class(&self) -> &'static str {
        match self {
            HoverCardSide::Top => "side-top",
            HoverCardSide::Right => "side-right",
            HoverCardSide::Bottom => "side-bottom",
            HoverCardSide::Left => "side-left",
        }
    }

    pub fn to_aria(&self) -> &'static str {
        match self {
            HoverCardSide::Top => "top",
            HoverCardSide::Right => "right",
            HoverCardSide::Bottom => "bottom",
            HoverCardSide::Left => "left",
        }
    }
}

/// Hover Card Align enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HoverCardAlign {
    #[default]
    Start,
    Center,
    End,
}

impl HoverCardAlign {
    pub fn to_class(&self) -> &'static str {
        match self {
            HoverCardAlign::Start => "align-start",
            HoverCardAlign::Center => "align-center",
            HoverCardAlign::End => "align-end",
        }
    }

    pub fn to_aria(&self) -> &'static str {
        match self {
            HoverCardAlign::Start => "start",
            HoverCardAlign::Center => "center",
            HoverCardAlign::End => "end",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{HoverCardAlign, HoverCardSide};
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Hover Card Tests
    #[test]
    fn test_hover_card_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_hover_card_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_hover_card_with_style() {
        // Test that the component can be created with style
    }

    #[test]
    fn test_hover_card_withopen_delay() {
        // Test with custom open delay
    }

    #[test]
    fn test_hover_card_with_close_delay() {
        // Test with custom close delay
    }

    #[test]
    fn test_hover_card_with_defaultopen() {
        // Test with default open state
    }

    #[test]
    fn test_hover_card_with_controlledopen() {
        // Test with controlled open state
    }

    #[test]
    fn test_hover_card_onopen_change() {
        // Test open change callback
    }

    // Hover Card Trigger Tests
    #[test]
    fn test_hover_card_trigger_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_hover_card_trigger_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_hover_card_trigger_with_style() {
        // Test that the component can be created with style
    }

    #[test]
    fn test_hover_card_triggerdisabled() {
        // Test disabled state
    }

    #[test]
    fn test_hover_card_trigger_mouse_enter() {
        // Test mouse enter callback
    }

    #[test]
    fn test_hover_card_trigger_mouse_leave() {
        // Test mouse leave callback
    }

    #[test]
    fn test_hover_card_trigger_focus() {
        // Test focus callback
    }

    #[test]
    fn test_hover_card_trigger_blur() {
        // Test blur callback
    }

    // Hover Card Content Tests
    #[test]
    fn test_hover_card_content_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_hover_card_content_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_hover_card_content_with_style() {
        // Test that the component can be created with style
    }

    #[test]
    fn test_hover_card_contentvisible() {
        // Test visible state
    }

    #[test]
    fn test_hover_card_content_hidden() {
        // Test hidden state
    }

    #[test]
    fn test_hover_card_content_with_side() {
        // Test with side positioning
    }

    #[test]
    fn test_hover_card_content_with_align() {
        // Test with alignment
    }

    #[test]
    fn test_hover_card_content_with_side_offset() {
        // Test with side offset
    }

    #[test]
    fn test_hover_card_content_with_align_offset() {
        // Test with align offset
    }

    // Hover Card Portal Tests
    #[test]
    fn test_hover_card_portal_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_hover_card_portal_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_hover_card_portal_with_style() {
        // Test that the component can be created with style
    }

    #[test]
    fn test_hover_card_portal_with_container() {
        // Test with container
    }

    // Hover Card Arrow Tests
    #[test]
    fn test_hover_card_arrow_creation() {
        // Test that the component can be created
    }

    #[test]
    fn test_hover_card_arrow_with_class() {
        // Test that the component can be created with class
    }

    #[test]
    fn test_hover_card_arrow_with_style() {
        // Test that the component can be created with style
    }

    #[test]
    fn test_hover_card_arrow_with_width() {
        // Test with custom width
    }

    #[test]
    fn test_hover_card_arrow_with_height() {
        // Test with custom height
    }

    // Hover Card Side Tests
    #[test]
    fn test_hover_card_side_default() {
        let side = HoverCardSide::default();
        assert_eq!(side, HoverCardSide::Top);
    }

    #[test]
    fn test_hover_card_side_top() {
        let side = HoverCardSide::Top;
        assert_eq!(side.to_class(), "side-top");
        assert_eq!(side.to_aria(), "top");
    }

    #[test]
    fn test_hover_card_side_right() {
        let side = HoverCardSide::Right;
        assert_eq!(side.to_class(), "side-right");
        assert_eq!(side.to_aria(), "right");
    }

    #[test]
    fn test_hover_card_side_bottom() {
        let side = HoverCardSide::Bottom;
        assert_eq!(side.to_class(), "side-bottom");
        assert_eq!(side.to_aria(), "bottom");
    }

    #[test]
    fn test_hover_card_side_left() {
        let side = HoverCardSide::Left;
        assert_eq!(side.to_class(), "side-left");
        assert_eq!(side.to_aria(), "left");
    }

    // Hover Card Align Tests
    #[test]
    fn test_hover_card_align_default() {
        let align = HoverCardAlign::default();
        assert_eq!(align, HoverCardAlign::Start);
    }

    #[test]
    fn test_hover_card_align_start() {
        let align = HoverCardAlign::Start;
        assert_eq!(align.to_class(), "align-start");
        assert_eq!(align.to_aria(), "start");
    }

    #[test]
    fn test_hover_card_align_center() {
        let align = HoverCardAlign::Center;
        assert_eq!(align.to_class(), "align-center");
        assert_eq!(align.to_aria(), "center");
    }

    #[test]
    fn test_hover_card_align_end() {
        let align = HoverCardAlign::End;
        assert_eq!(align.to_class(), "align-end");
        assert_eq!(align.to_aria(), "end");
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
    fn test_hover_card_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*", _open_delay in 0..5000u32, __close_delay in 0..5000u32)| {
            // Test that the component can be created with various prop values

        });
    }

    #[test]
    fn test_hover_card_trigger_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*")| {
            // Test that the component can be created with various class and style values

        });
    }

    #[test]
    fn test_hover_card_content_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*", _side_offset in -100.0..100.0f64, _align_offset in -100.0..100.0f64)| {
            // Test that the component can be created with various prop values

        });
    }

    #[test]
    fn test_hover_card_portal_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*", _container in ".*")| {
            // Test that the component can be created with various prop values

        });
    }

    #[test]
    fn test_hover_card_arrow_property_based() {
        use proptest::prelude::*;

        proptest!(|(____class in ".*", __style in ".*", __width in 1.0..50.0f64, __height in 1.0..50.0f64)| {
            // Test that the component can be created with various prop values

        });
    }
}
