use leptos::prelude::*;

/// Badge variant for different status types
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum BadgeVariant {
    Default,
    Primary,
    Secondary,
    Success,
    Error,
    Warning,
    Info,
    Outline,
}

/// Badge size variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum BadgeSize {
    Small,
    Medium,
    Large,
}

/// Merge CSS classes with proper spacing
fn merge_classes(classes: &[&str]) -> String {
    classes
        .iter()
        .filter(|&&c| !c.is_empty())
        .map(|&s| s)
        .collect::<Vec<&str>>()
        .join(" ")
}

/// Root Badge component
#[component]
pub fn Badge(
    /// Badge variant
    #[prop(optional, default = BadgeVariant::Default)]
    variant: BadgeVariant,
    /// Badge size
    #[prop(optional, default = BadgeSize::Medium)]
    size: BadgeSize,
    /// Whether the badge is interactive (clickable)
    #[prop(optional, default = false)]
    interactive: bool,
    /// Whether the badge is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let variant_class = move || match variant {
        BadgeVariant::Default => "radix-badge--variant-default",
        BadgeVariant::Primary => "radix-badge--variant-primary",
        BadgeVariant::Secondary => "radix-badge--variant-secondary",
        BadgeVariant::Success => "radix-badge--variant-success",
        BadgeVariant::Error => "radix-badge--variant-error",
        BadgeVariant::Warning => "radix-badge--variant-warning",
        BadgeVariant::Info => "radix-badge--variant-info",
        BadgeVariant::Outline => "radix-badge--variant-outline",
    };

    let size_class = move || match size {
        BadgeSize::Small => "radix-badge--size-small",
        BadgeSize::Medium => "radix-badge--size-medium",
        BadgeSize::Large => "radix-badge--size-large",
    };

    let handle_click = move |e: web_sys::MouseEvent| {
        if !disabled && interactive {
            if let Some(callback) = on_click {
                callback.run(e);
            }
        }
    };

    let class_value = class.unwrap_or_default();
    let children_view = children();

    let mut base_classes = vec!["radix-badge", &variant_class(), &size_class(), &class_value];

    if interactive && !disabled {
        base_classes.push("radix-badge--interactive");
    }

    let final_classes = base_classes;

    view! {
        <span
            class=merge_classes(&final_classes)
            role="status"
            on:click=handle_click
        >
            {children_view}
        </span>
    }
}

/// Badge with count/number
#[component]
pub fn BadgeCount(
    /// The count to display
    count: u32,
    /// Maximum count to display (shows as "99+" if exceeded)
    #[prop(optional, default = 99)]
    max_count: u32,
    /// Badge variant
    #[prop(optional, default = BadgeVariant::Error)]
    variant: BadgeVariant,
    /// Badge size
    #[prop(optional, default = BadgeSize::Small)]
    size: BadgeSize,
    /// Whether to show the badge when count is 0
    #[prop(optional, default = false)]
    show_zero: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let display_count = move || {
        if count > max_count {
            format!("{}+", max_count)
        } else {
            count.to_string()
        }
    };

    let should_show = move || show_zero || count > 0;

    let class_value = class.unwrap_or_default();

    view! {
        {move || {
            if should_show() {
                view! {
                    <Badge
                        variant=variant
                        size=size
                        class=class_value.clone()
                    >
                        <span class="radix-badge-count-text">
                            {display_count()}
                        </span>
                    </Badge>
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }
        }}
    }
}

/// Badge with dot indicator
#[component]
pub fn BadgeDot(
    /// Badge variant
    #[prop(optional, default = BadgeVariant::Success)]
    variant: BadgeVariant,
    /// Badge size
    #[prop(optional, default = BadgeSize::Small)]
    size: BadgeSize,
    /// Whether the dot is pulsing
    #[prop(optional, default = false)]
    pulsing: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();
    let variant_class = move || match variant {
        BadgeVariant::Default => "radix-badge-dot--variant-default",
        BadgeVariant::Primary => "radix-badge-dot--variant-primary",
        BadgeVariant::Secondary => "radix-badge-dot--variant-secondary",
        BadgeVariant::Success => "radix-badge-dot--variant-success",
        BadgeVariant::Error => "radix-badge-dot--variant-error",
        BadgeVariant::Warning => "radix-badge-dot--variant-warning",
        BadgeVariant::Info => "radix-badge-dot--variant-info",
        BadgeVariant::Outline => "radix-badge-dot--variant-outline",
    };

    let size_class = move || match size {
        BadgeSize::Small => "radix-badge-dot--size-small",
        BadgeSize::Medium => "radix-badge-dot--size-medium",
        BadgeSize::Large => "radix-badge-dot--size-large",
    };

    let pulsing_class = move || {
        if pulsing {
            "radix-badge-dot--pulsing"
        } else {
            ""
        }
    };

    view! {
        <span
            class=merge_classes(&[
                "radix-badge-dot",
                &variant_class(),
                &size_class(),
                &pulsing_class(),
                &class_value,
            ])
            role="status"
        >
            <span class="radix-badge-dot-inner"></span>
        </span>
    }
}
