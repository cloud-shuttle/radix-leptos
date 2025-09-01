use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TabsOrientation {
    Horizontal,
    Vertical,
}

impl Default for TabsOrientation {
    fn default() -> Self {
        TabsOrientation::Horizontal
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TabsSize {
    Small,
    Medium,
    Large,
}

impl Default for TabsSize {
    fn default() -> Self {
        TabsSize::Medium
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TabsVariant {
    Default,
    Pills,
    Underlined,
}

impl Default for TabsVariant {
    fn default() -> Self {
        TabsVariant::Default
    }
}

fn merge_classes(base: &str, custom: Option<String>) -> String {
    let class_value = custom.unwrap_or_default();
    let final_class = format!("{} {}", base, class_value);
    final_class.trim().to_string()
}

#[component]
pub fn Tabs(
    #[prop(optional)] orientation: Option<TabsOrientation>,
    #[prop(optional)] size: Option<TabsSize>,
    #[prop(optional)] variant: Option<TabsVariant>,
    #[prop(optional)] _default_value: Option<String>,
    #[prop(optional)] _value: Option<ReadSignal<String>>,
    #[prop(optional)] _on_change: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let orientation = orientation.unwrap_or_default();
    let _size = size.unwrap_or_default();
    let variant = variant.unwrap_or_default();
    
    let base_classes = match (orientation, variant) {
        (TabsOrientation::Horizontal, TabsVariant::Default) => "flex flex-col",
        (TabsOrientation::Horizontal, TabsVariant::Pills) => "flex flex-col",
        (TabsOrientation::Horizontal, TabsVariant::Underlined) => "flex flex-col",
        (TabsOrientation::Vertical, TabsVariant::Default) => "flex flex-row",
        (TabsOrientation::Vertical, TabsVariant::Pills) => "flex flex-row",
        (TabsOrientation::Vertical, TabsVariant::Underlined) => "flex flex-row",
    };
    
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    view! {
        <div class={final_class} style={style_attr}>
            {children()}
        </div>
    }
}

#[component]
pub fn TabsList(
    #[prop(optional)] orientation: Option<TabsOrientation>,
    #[prop(optional)] size: Option<TabsSize>,
    #[prop(optional)] variant: Option<TabsVariant>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let orientation = orientation.unwrap_or_default();
    let _size = size.unwrap_or_default();
    let variant = variant.unwrap_or_default();
    
    let base_classes = match (orientation, variant, _size) {
        (TabsOrientation::Horizontal, TabsVariant::Default, TabsSize::Small) => "inline-flex h-8 items-center justify-center rounded-md bg-gray-100 p-1 text-gray-600",
        (TabsOrientation::Horizontal, TabsVariant::Default, TabsSize::Medium) => "inline-flex h-10 items-center justify-center rounded-md bg-gray-100 p-1 text-gray-600",
        (TabsOrientation::Horizontal, TabsVariant::Default, TabsSize::Large) => "inline-flex h-12 items-center justify-center rounded-md bg-gray-100 p-1 text-gray-600",
        (TabsOrientation::Horizontal, TabsVariant::Pills, TabsSize::Small) => "inline-flex h-8 items-center justify-center rounded-lg bg-gray-100 p-1 text-gray-600",
        (TabsOrientation::Horizontal, TabsVariant::Pills, TabsSize::Medium) => "inline-flex h-10 items-center justify-center rounded-lg bg-gray-100 p-1 text-gray-600",
        (TabsOrientation::Horizontal, TabsVariant::Pills, TabsSize::Large) => "inline-flex h-12 items-center justify-center rounded-lg bg-gray-100 p-1 text-gray-600",
        (TabsOrientation::Horizontal, TabsVariant::Underlined, TabsSize::Small) => "inline-flex h-8 items-center justify-center border-b border-gray-300 text-gray-600",
        (TabsOrientation::Horizontal, TabsVariant::Underlined, TabsSize::Medium) => "inline-flex h-10 items-center justify-center border-b border-gray-300 text-gray-600",
        (TabsOrientation::Horizontal, TabsVariant::Underlined, TabsSize::Large) => "inline-flex h-12 items-center justify-center border-b border-gray-300 text-gray-600",
        (TabsOrientation::Vertical, TabsVariant::Default, TabsSize::Small) => "inline-flex w-8 flex-col items-center justify-center rounded-md bg-gray-100 p-1 text-gray-600",
        (TabsOrientation::Vertical, TabsVariant::Default, TabsSize::Medium) => "inline-flex w-10 flex-col items-center justify-center rounded-md bg-gray-100 p-1 text-gray-600",
        (TabsOrientation::Vertical, TabsVariant::Default, TabsSize::Large) => "inline-flex w-12 flex-col items-center justify-center rounded-md bg-gray-100 p-1 text-gray-600",
        (TabsOrientation::Vertical, TabsVariant::Pills, TabsSize::Small) => "inline-flex w-8 flex-col items-center justify-center rounded-lg bg-gray-100 p-1 text-gray-600",
        (TabsOrientation::Vertical, TabsVariant::Pills, TabsSize::Medium) => "inline-flex w-10 flex-col items-center justify-center rounded-lg bg-gray-100 p-1 text-gray-600",
        (TabsOrientation::Vertical, TabsVariant::Pills, TabsSize::Large) => "inline-flex w-12 flex-col items-center justify-center rounded-lg bg-gray-100 p-1 text-gray-600",
        (TabsOrientation::Vertical, TabsVariant::Underlined, TabsSize::Small) => "inline-flex w-8 flex-col items-center justify-center border-r border-gray-300 text-gray-600",
        (TabsOrientation::Vertical, TabsVariant::Underlined, TabsSize::Medium) => "inline-flex w-10 flex-col items-center justify-center border-r border-gray-300 text-gray-600",
        (TabsOrientation::Vertical, TabsVariant::Underlined, TabsSize::Large) => "inline-flex w-12 flex-col items-center justify-center border-r border-gray-300 text-gray-600",
    };
    
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    view! {
        <div class={final_class} style={style_attr} role="tablist">
            {children()}
        </div>
    }
}

#[component]
pub fn TabsTrigger(
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] size: Option<TabsSize>,
    #[prop(optional)] variant: Option<TabsVariant>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] on_click: Option<Callback<String>>,
    children: Children,
) -> impl IntoView {
    let value = value.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);
    let _size = size.unwrap_or_default();
    let variant = variant.unwrap_or_default();
    
    let base_classes = match (variant, _size) {
        (TabsVariant::Default, TabsSize::Small) => "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1 text-xs font-medium transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-gray-200 data-[state=active]:bg-white data-[state=active]:text-gray-900 data-[state=active]:shadow-sm",
        (TabsVariant::Default, TabsSize::Medium) => "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-gray-200 data-[state=active]:bg-white data-[state=active]:text-gray-900 data-[state=active]:shadow-sm",
        (TabsVariant::Default, TabsSize::Large) => "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-4 py-2 text-base font-medium transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-gray-200 data-[state=active]:bg-white data-[state=active]:text-gray-900 data-[state=active]:shadow-sm",
        (TabsVariant::Pills, TabsSize::Small) => "inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1 text-xs font-medium transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-gray-200 data-[state=active]:bg-white data-[state=active]:text-gray-900 data-[state=active]:shadow-sm",
        (TabsVariant::Pills, TabsSize::Medium) => "inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1.5 text-sm font-medium transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-gray-200 data-[state=active]:bg-white data-[state=active]:text-gray-900 data-[state=active]:shadow-sm",
        (TabsVariant::Pills, TabsSize::Large) => "inline-flex items-center justify-center whitespace-nowrap rounded-md px-4 py-2 text-base font-medium transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-gray-200 data-[state=active]:bg-white data-[state=active]:text-gray-900 data-[state=active]:shadow-sm",
        (TabsVariant::Underlined, TabsSize::Small) => "inline-flex items-center justify-center whitespace-nowrap border-b-2 border-transparent px-3 py-1 text-xs font-medium transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:border-gray-300 data-[state=active]:border-blue-500 data-[state=active]:text-gray-900",
        (TabsVariant::Underlined, TabsSize::Medium) => "inline-flex items-center justify-center whitespace-nowrap border-b-2 border-transparent px-3 py-1.5 text-sm font-medium transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:border-gray-300 data-[state=active]:border-blue-500 data-[state=active]:text-gray-900",
        (TabsVariant::Underlined, TabsSize::Large) => "inline-flex items-center justify-center whitespace-nowrap border-b-2 border-transparent px-4 py-2 text-base font-medium transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:border-gray-300 data-[state=active]:border-blue-500 data-[state=active]:text-gray-900",
    };
    
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    let value_clone = value.clone();
    let handle_click = move |_| {
        if !disabled {
            if let Some(callback) = on_click {
                callback.run(value_clone.clone());
            }
        }
    };

    let value_clone = value.clone();
    let handle_keydown = move |event: web_sys::KeyboardEvent| {
        if !disabled && (event.key() == "Enter" || event.key() == " ") {
            event.prevent_default();
            if let Some(callback) = on_click {
                callback.run(value_clone.clone());
            }
        }
    };

    view! {
        <button
            class={final_class}
            style={style_attr}
            disabled={disabled}
            data-value={value}
            role="tab"
            on:click=handle_click
            on:keydown=handle_keydown
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabsContent(
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let value = value.unwrap_or_default();
    let base_classes = "mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2";
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    view! {
        <div
            class={final_class}
            style={style_attr}
            data-value={value}
            role="tabpanel"
            tabindex="0"
        >
            {children()}
        </div>
    }
}
