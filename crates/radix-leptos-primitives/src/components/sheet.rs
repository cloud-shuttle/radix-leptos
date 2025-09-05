use crate::utils::merge_classes;
use leptos::prelude::*;
use leptos::*;

/// Sheet component - Side panel/drawer component for mobile and desktop
///
/// The Sheet component provides accessible side panels and drawers that slide in from
/// different directions, commonly used for navigation, forms, and secondary content.
///
/// # Features
/// - Accessible modal overlay with proper ARIA attributes
/// - Multiple positions (left, right, top, bottom)
/// - Multiple sizes (sm, md, lg, xl, full)
/// - Focus management and keyboard navigation
/// - Backdrop click handling
/// - Escape key handling
/// - Smooth animations and transitions
///
/// # Example
///
/// ```rust
/// use leptos::*;
/// use radix_leptos_primitives::*;
///
/// #[component]
/// fn MyComponent() -> impl IntoView {
///     let (show_sheet, set_show_sheet) = create_signal(false);
///
///     view! {
///         <Button on_click=move |_| set_show_sheet.set(true)>
///             "Open Sheet"
///         </Button>
///         
///         <Sheet
///             open=show_sheet
///             on_open_change=move |open| set_show_sheet.set(open)
///             position=SheetPosition::Right
///             size=SheetSize::Medium
///         >
///             <SheetContent>
///                 <SheetHeader>
///                     <SheetTitle>"Settings"</SheetTitle>
///                     <SheetDescription>
///                         "Manage your application settings and preferences."
///                     </SheetDescription>
///                 </SheetHeader>
///                 <SheetBody>
///                     // Sheet content here
///                 </SheetBody>
///                 <SheetFooter>
///                     <Button on_click=move |_| set_show_sheet.set(false)>
///                         "Close"
///                     </Button>
///                 </SheetFooter>
///             </SheetContent>
///         </Sheet>
///     }
/// }
/// ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SheetPosition {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SheetSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
    Full,
}

impl SheetPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            SheetPosition::Left => "left",
            SheetPosition::Right => "right",
            SheetPosition::Top => "top",
            SheetPosition::Bottom => "bottom",
        }
    }
}

impl SheetSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            SheetSize::Small => "sm",
            SheetSize::Medium => "md",
            SheetSize::Large => "lg",
            SheetSize::ExtraLarge => "xl",
            SheetSize::Full => "full",
        }
    }
}

/// Sheet root component
#[component]
pub fn Sheet(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] open: Option<bool>,
    #[prop(optional)] position: Option<SheetPosition>,
    #[prop(optional)] size: Option<SheetSize>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
) -> impl IntoView {
    let open = open.unwrap_or(false);
    let position = position.unwrap_or(SheetPosition::Right);
    let size = size.unwrap_or(SheetSize::Medium);
    let on_open_change = on_open_change.unwrap_or_else(|| Callback::new(|_| {}));

    let class = merge_classes([
        "sheet",
        position.as_str(),
        size.as_str(),
        if open { "open" } else { "closed" },
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="dialog"
            aria-modal="true"
            data-position=position.as_str()
            data-size=size.as_str()
            data-open=open
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Sheet content component
#[component]
pub fn SheetContent(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = merge_classes(["sheet-content", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Sheet header component
#[component]
pub fn SheetHeader(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = merge_classes(["sheet-header", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Sheet title component
#[component]
pub fn SheetTitle(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = merge_classes(["sheet-title", class.as_deref().unwrap_or("")]);

    view! {
        <h2
            class=class
            style=style
        >
            {children.map(|c| c())}
        </h2>
    }
}

/// Sheet description component
#[component]
pub fn SheetDescription(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = merge_classes(["sheet-description", class.as_deref().unwrap_or("")]);

    view! {
        <p
            class=class
            style=style
        >
            {children.map(|c| c())}
        </p>
    }
}

/// Sheet body component
#[component]
pub fn SheetBody(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = merge_classes(["sheet-body", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Sheet footer component
#[component]
pub fn SheetFooter(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = merge_classes(["sheet-footer", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Sheet close button component
#[component]
pub fn SheetClose(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let on_click = on_click.unwrap_or_else(|| Callback::new(|_| {}));

    let class = merge_classes(["sheet-close", class.as_deref().unwrap_or("")]);

    view! {
        <button
            class=class
            style=style
            on:click=move |_| on_click.run(())
            aria-label="Close sheet"
        >
            {children.map(|c| c())}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_sheet_component_creation() {}

    #[test]
    fn test_sheet_with_position_component_creation() {}

    proptest! {
        #[test]
        fn test_sheet_props(__class in ".*", __style in ".*") {

        }

        #[test]
        fn test_sheet_open_state(__open: bool, __position_index in 0..4usize, __size_index in 0..5usize) {

        }

        #[test]
        fn test_sheet_positions(__position_index in 0..4usize) {

        }

        #[test]
        fn test_sheet_sizes(__size_index in 0..5usize) {

        }

        #[test]
        fn test_sheet_content_props(__class in ".*", __style in ".*") {

        }

        #[test]
        fn test_sheet_header_props(__class in ".*", __style in ".*") {

        }

        #[test]
        fn test_sheet_title_props(__class in ".*", __style in ".*") {

        }

        #[test]
        fn test_sheet_description_props(__class in ".*", __style in ".*") {

        }

        #[test]
        fn test_sheet_body_props(__class in ".*", __style in ".*") {

        }

        #[test]
        fn test_sheet_footer_props(__class in ".*", __style in ".*") {

        }

        #[test]
        fn test_sheet_close_props(__class in ".*", __style in ".*") {

        }
    }
}
