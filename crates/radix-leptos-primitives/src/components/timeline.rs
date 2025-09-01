use leptos::*;
use leptos::prelude::*;

// Timeline item structure
#[derive(Clone, Debug, PartialEq)]
pub struct TimelineItem<T>
where
    T: Send + Sync + Clone + 'static,
{
    pub id: String,
    pub data: T,
    pub title: String,
    pub description: Option<String>,
    pub date: Option<String>,
    pub icon: Option<String>,
    pub status: TimelineStatus,
    pub disabled: bool,
}

// Timeline status variants
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TimelineStatus {
    Default,
    Success,
    Warning,
    Error,
    Info,
}

// Timeline size variants
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TimelineSize {
    Small,
    Medium,
    Large,
}

// Timeline variant styles
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TimelineVariant {
    Default,
    Bordered,
    Compact,
    Vertical,
    Horizontal,
}

// Timeline context for sharing state
#[derive(Clone)]
pub struct TimelineContext<T>
where
    T: Send + Sync + Clone + 'static,
{
    pub items: Vec<TimelineItem<T>>,
    pub selected_items: Signal<Vec<String>>,
    pub focused_item: Signal<Option<String>>,
    pub size: TimelineSize,
    pub variant: TimelineVariant,
    pub timeline_id: String,
    pub on_item_click: Option<Callback<TimelineItem<T>>>,
    pub on_item_focus: Option<Callback<TimelineItem<T>>>,
}

// Utility functions
fn generate_id() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let mut hasher = DefaultHasher::new();
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    time.hash(&mut hasher);
    format!("timeline-{:x}", hasher.finish())
}

fn merge_classes(classes: Vec<String>) -> String {
    classes.into_iter().filter(|c| !c.is_empty()).collect::<Vec<_>>().join(" ")
}

// Helper function to create a timeline item
pub fn create_timeline_item<T>(
    id: &str,
    data: T,
    title: &str,
    description: Option<&str>,
    date: Option<&str>,
    icon: Option<&str>,
    status: TimelineStatus,
) -> TimelineItem<T>
where
    T: Send + Sync + Clone + 'static,
{
    TimelineItem {
        id: id.to_string(),
        data,
        title: title.to_string(),
        description: description.map(|s| s.to_string()),
        date: date.map(|s| s.to_string()),
        icon: icon.map(|s| s.to_string()),
        status,
        disabled: false,
    }
}

// Helper function to create a disabled timeline item
pub fn create_disabled_timeline_item<T>(
    id: &str,
    data: T,
    title: &str,
    description: Option<&str>,
    date: Option<&str>,
    icon: Option<&str>,
    status: TimelineStatus,
) -> TimelineItem<T>
where
    T: Send + Sync + Clone + 'static,
{
    TimelineItem {
        id: id.to_string(),
        data,
        title: title.to_string(),
        description: description.map(|s| s.to_string()),
        date: date.map(|s| s.to_string()),
        icon: icon.map(|s| s.to_string()),
        status,
        disabled: true,
    }
}

// Main Timeline component
#[component]
pub fn Timeline<T>(
    #[prop(into)] items: Vec<TimelineItem<T>>,
    #[prop(optional)] size: Option<TimelineSize>,
    #[prop(optional)] variant: Option<TimelineVariant>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] on_item_click: Option<Callback<TimelineItem<T>>>,
    #[prop(optional)] on_item_focus: Option<Callback<TimelineItem<T>>>,
    children: Children,
) -> impl IntoView
where
    T: Send + Sync + Clone + 'static,
{
    let timeline_id = generate_id();
    let size = size.unwrap_or(TimelineSize::Medium);
    let variant = variant.unwrap_or(TimelineVariant::Default);
    
    let (selected_items_signal, _set_selected_items) = signal(Vec::<String>::new());
    let (focused_item, _set_focused_item) = signal(None::<String>);
    
    let context = TimelineContext {
        items: items.clone(),
        selected_items: selected_items_signal.into(),
        focused_item: focused_item.into(),
        size: size.clone(),
        variant: variant.clone(),
        timeline_id: timeline_id.clone(),
        on_item_click,
        on_item_focus,
    };
    
    provide_context(context);
    
    let base_classes = vec![
        "radix-timeline".to_string(),
        format!("radix-timeline--size-{}", match size {
            TimelineSize::Small => "small",
            TimelineSize::Medium => "medium",
            TimelineSize::Large => "large",
        }),
        format!("radix-timeline--variant-{}", match variant {
            TimelineVariant::Default => "default",
            TimelineVariant::Bordered => "bordered",
            TimelineVariant::Compact => "compact",
            TimelineVariant::Vertical => "vertical",
            TimelineVariant::Horizontal => "horizontal",
        }),
        class.unwrap_or_default(),
    ];
    
    let classes = merge_classes(base_classes);
    
    view! {
        <div
            id=timeline_id
            class=classes
            role="list"
        >
            {children()}
        </div>
    }
}

// TimelineItem component for individual items
#[component]
pub fn TimelineItemComponent<T>(
    #[prop(into)] item: TimelineItem<T>,
    children: Children,
) -> impl IntoView
where
    T: Send + Sync + Clone + 'static,
{
    let context = use_context::<TimelineContext<T>>().expect("TimelineItem must be used within Timeline");
    let item_id = item.id.clone();
    let item_clone_1 = item.clone();
    let item_clone_2 = item.clone();
    let item_disabled = item.disabled;
    
    let is_focused = move || context.focused_item.get().as_ref() == Some(&item_id);
    
    let handle_click = move |event: web_sys::MouseEvent| {
        event.prevent_default();
        if item_disabled {
            return;
        }
        
        if let Some(on_click) = context.on_item_click.as_ref() {
            on_click.run(item_clone_1.clone());
        }
    };
    
    let handle_focus = move |_: web_sys::FocusEvent| {
        if let Some(on_focus) = context.on_item_focus.as_ref() {
            on_focus.run(item_clone_2.clone());
        }
    };
    
    let item_classes = vec![
        "radix-timeline-item".to_string(),
        format!("radix-timeline-item--status-{}", match item.status {
            TimelineStatus::Default => "default",
            TimelineStatus::Success => "success",
            TimelineStatus::Warning => "warning",
            TimelineStatus::Error => "error",
            TimelineStatus::Info => "info",
        }),
        if is_focused() { "radix-timeline-item--focused".to_string() } else { String::new() },
        if item_disabled { "radix-timeline-item--disabled".to_string() } else { String::new() },
    ];
    
    let item_classes = merge_classes(item_classes);
    
    let status_icon = match item.status {
        TimelineStatus::Success => "✓",
        TimelineStatus::Warning => "⚠",
        TimelineStatus::Error => "✗",
        TimelineStatus::Info => "ℹ",
        TimelineStatus::Default => "•",
    };
    
    view! {
        <div
            class=item_classes
            role="listitem"
            tabindex=if is_focused() { "0" } else { "-1" }
            on:click=handle_click
            on:focus=handle_focus
        >
            <div class="radix-timeline-item-content">
                <div class="radix-timeline-item-icon">
                    {if let Some(icon) = item.icon.as_ref() {
                        let icon_clone = icon.clone();
                        view! {
                            <span class="radix-timeline-item-custom-icon">{icon_clone}</span>
                        }
                    } else {
                        let status_icon_str = status_icon.to_string();
                        view! {
                            <span class="radix-timeline-item-status-icon">{status_icon_str}</span>
                        }
                    }}
                </div>
                
                <div class="radix-timeline-item-body">
                    <div class="radix-timeline-item-header">
                        <h3 class="radix-timeline-item-title">{item.title}</h3>
                        {if let Some(date) = item.date.as_ref() {
                            let date_clone = date.clone();
                            view! {
                                <span class="radix-timeline-item-date">{date_clone}</span>
                            }
                        } else {
                            view! {
                                <span class="radix-timeline-item-date">{String::new()}</span>
                            }
                        }}
                    </div>
                    
                    {if let Some(description) = item.description.as_ref() {
                        let description_clone = description.clone();
                        view! {
                            <p class="radix-timeline-item-description">{description_clone}</p>
                        }
                    } else {
                        view! {
                            <p class="radix-timeline-item-description">{String::new()}</p>
                        }
                    }}
                    
                    <div class="radix-timeline-item-content">
                        {children()}
                    </div>
                </div>
            </div>
        </div>
    }
}

// TimelineEmpty component for empty state
#[component]
pub fn TimelineEmpty(
    #[prop(optional)] message: Option<String>,
    children: Children,
) -> impl IntoView {
    let message = message.unwrap_or_else(|| "No timeline items available".to_string());
    
    view! {
        <div class="radix-timeline-empty">
            <div class="radix-timeline-empty-content">
                <span class="radix-timeline-empty-icon">"📅"</span>
                <p class="radix-timeline-empty-message">{message}</p>
                {children()}
            </div>
        </div>
    }
}

// TimelineLoading component for loading state
#[component]
pub fn TimelineLoading(
    #[prop(optional)] message: Option<String>,
    children: Children,
) -> impl IntoView {
    let message = message.unwrap_or_else(|| "Loading timeline...".to_string());
    
    view! {
        <div class="radix-timeline-loading">
            <div class="radix-timeline-loading-content">
                <div class="radix-timeline-loading-spinner">"⏳"</div>
                <p class="radix-timeline-loading-message">{message}</p>
                {children()}
            </div>
        </div>
    }
}
