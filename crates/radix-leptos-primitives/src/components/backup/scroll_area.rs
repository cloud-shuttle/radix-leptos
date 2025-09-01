use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ScrollAreaOrientation {
    Horizontal,
    Vertical,
    Both,
}

impl Default for ScrollAreaOrientation {
    fn default() -> Self {
        ScrollAreaOrientation::Both
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScrollAreaSize {
    Small,
    Medium,
    Large,
}

impl Default for ScrollAreaSize {
    fn default() -> Self {
        ScrollAreaSize::Medium
    }
}

fn merge_classes(base: &str, custom: Option<String>) -> String {
    let class_value = custom.unwrap_or_default();
    let final_class = format!("{} {}", base, class_value);
    final_class.trim().to_string()
}

#[component]
pub fn ScrollArea(
    #[prop(optional)] orientation: Option<ScrollAreaOrientation>,
    #[prop(optional)] size: Option<ScrollAreaSize>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let orientation = orientation.unwrap_or_default();
    let size = size.unwrap_or_default();
    
    let base_classes = match (orientation, size) {
        (ScrollAreaOrientation::Horizontal, ScrollAreaSize::Small) => "relative overflow-auto scrollbar-thin scrollbar-track-gray-100 scrollbar-thumb-gray-300",
        (ScrollAreaOrientation::Horizontal, ScrollAreaSize::Medium) => "relative overflow-auto scrollbar scrollbar-track-gray-100 scrollbar-thumb-gray-300",
        (ScrollAreaOrientation::Horizontal, ScrollAreaSize::Large) => "relative overflow-auto scrollbar-thick scrollbar-track-gray-100 scrollbar-thumb-gray-300",
        (ScrollAreaOrientation::Vertical, ScrollAreaSize::Small) => "relative overflow-auto scrollbar-thin scrollbar-track-gray-100 scrollbar-thumb-gray-300",
        (ScrollAreaOrientation::Vertical, ScrollAreaSize::Medium) => "relative overflow-auto scrollbar scrollbar-track-gray-100 scrollbar-thumb-gray-300",
        (ScrollAreaOrientation::Vertical, ScrollAreaSize::Large) => "relative overflow-auto scrollbar-thick scrollbar-track-gray-100 scrollbar-thumb-gray-300",
        (ScrollAreaOrientation::Both, ScrollAreaSize::Small) => "relative overflow-auto scrollbar-thin scrollbar-track-gray-100 scrollbar-thumb-gray-300",
        (ScrollAreaOrientation::Both, ScrollAreaSize::Medium) => "relative overflow-auto scrollbar scrollbar-track-gray-100 scrollbar-thumb-gray-300",
        (ScrollAreaOrientation::Both, ScrollAreaSize::Large) => "relative overflow-auto scrollbar-thick scrollbar-track-gray-100 scrollbar-thumb-gray-300",
    };
    
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    view! {
        <div class={final_class} style={style_attr} data-radix-scroll-area="">
            {children()}
        </div>
    }
}

#[component]
pub fn ScrollAreaViewport(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let base_classes = "h-full w-full";
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    view! {
        <div class={final_class} style={style_attr} data-radix-scroll-area-viewport="">
            {children()}
        </div>
    }
}

#[component]
pub fn ScrollAreaScrollbar(
    #[prop(optional)] orientation: Option<ScrollAreaOrientation>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let orientation = orientation.unwrap_or_default();
    
    let base_classes = match orientation {
        ScrollAreaOrientation::Horizontal => "flex h-2.5 touch-none select-none transition-colors duration-150 ease-out hover:bg-gray-100 data-[orientation=horizontal]:h-2.5 data-[orientation=horizontal]:flex-col",
        ScrollAreaOrientation::Vertical => "flex w-2.5 touch-none select-none transition-colors duration-150 ease-out hover:bg-gray-100 data-[orientation=vertical]:w-2.5 data-[orientation=vertical]:flex-col",
        ScrollAreaOrientation::Both => "flex w-2.5 touch-none select-none transition-colors duration-150 ease-out hover:bg-gray-100 data-[orientation=vertical]:w-2.5 data-[orientation=vertical]:flex-col",
    };
    
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    view! {
        <div class={final_class} style={style_attr} data-radix-scroll-area-scrollbar="">
            {children()}
        </div>
    }
}

#[component]
pub fn ScrollAreaThumb(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let base_classes = "flex-1 bg-gray-300 rounded-[10px] relative before:content-[''] before:absolute before:top-1/2 before:left-1/2 before:-translate-x-1/2 before:-translate-y-1/2 before:w-full before:h-full before:min-w-[44px] before:min-h-[44px]";
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    view! {
        <div class={final_class} style={style_attr} data-radix-scroll-area-thumb=""></div>
    }
}

#[component]
pub fn ScrollAreaCorner(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let base_classes = "bg-gray-100";
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    view! {
        <div class={final_class} style={style_attr}></div>
    }
}
