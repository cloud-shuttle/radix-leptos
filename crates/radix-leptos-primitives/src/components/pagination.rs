use leptos::*;
use leptos::prelude::*;

/// Pagination page information
#[derive(Clone, Debug, PartialEq)]
pub struct PaginationPage {
    pub number: usize,
    pub label: Option<String>,
    pub disabled: bool,
    pub current: bool,
}

impl PaginationPage {
    pub fn new(number: usize) -> Self {
        Self {
            number,
            label: None,
            disabled: false,
            current: false,
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_current(mut self, current: bool) -> Self {
        self.current = current;
        self
    }
}

/// Pagination size
#[derive(Clone, Debug, PartialEq)]
pub enum PaginationSize {
    Small,
    Medium,
    Large,
}

impl PaginationSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            PaginationSize::Small => "small",
            PaginationSize::Medium => "medium",
            PaginationSize::Large => "large",
        }
    }
}

/// Pagination variant
#[derive(Clone, Debug, PartialEq)]
pub enum PaginationVariant {
    Default,
    Compact,
    Detailed,
}

impl PaginationVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            PaginationVariant::Default => "default",
            PaginationVariant::Compact => "compact",
            PaginationVariant::Detailed => "detailed",
        }
    }
}

/// Pagination context for state management
#[derive(Clone)]
pub struct PaginationContext {
    pub current_page: Signal<usize>,
    pub total_pages: usize,
    pub page_size: usize,
    pub total_items: usize,
    pub size: PaginationSize,
    pub variant: PaginationVariant,
    pub show_first_last: bool,
    pub show_prev_next: bool,
    pub show_page_numbers: bool,
    pub pagination_id: String,
    pub on_page_change: Option<Callback<usize>>,
}

/// Generate a simple unique ID for components
fn generate_id(prefix: &str) -> String {
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("{}-{}", prefix, id)
}

/// Merge CSS classes
fn merge_classes(existing: Option<&str>, additional: Option<&str>) -> Option<String> {
    match (existing, additional) {
        (Some(a), Some(b)) => Some(format!("{} {}", a, b)),
        (Some(a), None) => Some(a.to_string()),
        (None, Some(b)) => Some(b.to_string()),
        (None, None) => None,
    }
}

/// Calculate visible page range
fn calculate_page_range(current_page: usize, total_pages: usize, max_visible: usize) -> (usize, usize) {
    if total_pages <= max_visible {
        return (1, total_pages);
    }

    let half_visible = max_visible / 2;
    let mut start = current_page.saturating_sub(half_visible);
    let mut end = start + max_visible - 1;

    if end > total_pages {
        end = total_pages;
        start = end.saturating_sub(max_visible - 1);
    }

    (start, end)
}

/// Main Pagination component
#[component]
pub fn Pagination(
    /// Current page number (1-based)
    #[prop(optional, default = 1)]
    current_page: usize,
    /// Total number of pages
    #[prop(optional, default = 1)]
    total_pages: usize,
    /// Number of items per page
    #[prop(optional, default = 10)]
    page_size: usize,
    /// Total number of items
    #[prop(optional)]
    total_items: Option<usize>,
    /// Pagination size
    #[prop(optional, default = PaginationSize::Medium)]
    size: PaginationSize,
    /// Pagination variant
    #[prop(optional, default = PaginationVariant::Default)]
    variant: PaginationVariant,
    /// Whether to show first/last page buttons
    #[prop(optional, default = true)]
    show_first_last: bool,
    /// Whether to show previous/next buttons
    #[prop(optional, default = true)]
    show_prev_next: bool,
    /// Whether to show page numbers
    #[prop(optional, default = true)]
    show_page_numbers: bool,
    /// Page change event handler
    #[prop(optional)]
    on_page_change: Option<Callback<usize>>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child content (pagination items, etc.)
    children: Children,
) -> impl IntoView {
    let pagination_id = generate_id("pagination");
    
    // Reactive state
    let (current_page_signal, set_current_page_signal) = signal(current_page);
    let total_items_calculated = total_items.unwrap_or_else(|| total_pages * page_size);
    
    // Create context
    let context = PaginationContext {
        current_page: current_page_signal.into(),
        total_pages,
        page_size,
        total_items: total_items_calculated,
        size: size.clone(),
        variant: variant.clone(),
        show_first_last,
        show_prev_next,
        show_page_numbers,
        pagination_id: pagination_id.clone(),
        on_page_change,
    };
    
    // Build base classes
    let base_classes = "radix-pagination";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Provide the context
    provide_context(context);
    
    view! {
        <nav
            id=pagination_id
            class=combined_class
            data-current-page=current_page_signal.get()
            data-total-pages=total_pages
            data-page-size=page_size
            data-total-items=total_items_calculated
            data-size=size.as_str()
            data-variant=variant.as_str()
            data-show-first-last=show_first_last
            data-show-prev-next=show_prev_next
            data-show-page-numbers=show_page_numbers
            role="navigation"
            aria-label="Pagination"
        >
            {children()}
        </nav>
    }
}

/// PaginationList component for the pagination items container
#[component]
pub fn PaginationList(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (pagination items)
    children: Children,
) -> impl IntoView {
    let _context = use_context::<PaginationContext>().expect("PaginationList must be used within Pagination");
    let list_id = generate_id("pagination-list");
    
    // Build base classes
    let base_classes = "radix-pagination-list";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <ul
            id=list_id
            class=combined_class
            style=style.unwrap_or_default()
            role="list"
        >
            {children()}
        </ul>
    }
}

/// PaginationItem component for individual pagination items
#[component]
pub fn PaginationItem(
    /// The pagination item this component represents
    #[prop(optional)]
    page: Option<PaginationPage>,
    /// Whether this item is current
    #[prop(optional)]
    current: Option<bool>,
    /// Whether this item is disabled
    #[prop(optional)]
    disabled: Option<bool>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<PaginationContext>().expect("PaginationItem must be used within Pagination");
    let item_id = generate_id("pagination-item");
    
    let page_clone = page.clone();
    let handle_click = move |event: web_sys::MouseEvent| {
        event.prevent_default();
        
        if let Some(page) = page_clone.clone() {
            if !page.disabled {
                // Call the page change handler
                if let Some(callback) = context.on_page_change.clone() {
                    callback.run(page.number);
                }
            }
        }
    };
    
    let page_for_current = page.clone();
    let page_for_disabled = page.clone();
    
    // Determine if this item is current
    let is_current = Memo::new(move |_| {
        if let Some(current) = current {
            current
        } else if let Some(page) = page_for_current.as_ref() {
            page.current
        } else {
            false
        }
    });
    
    // Determine if this item is disabled
    let is_disabled = Memo::new(move |_| {
        if let Some(disabled) = disabled {
            disabled
        } else if let Some(page) = page_for_disabled.as_ref() {
            page.disabled
        } else {
            false
        }
    });
    
    // Build base classes
    let base_classes = "radix-pagination-item";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <li
            id=item_id
            class=combined_class
            style=style.unwrap_or_default()
            data-current=is_current.get()
            data-disabled=is_disabled.get()
            role="listitem"
        >
            <button
                class="radix-pagination-button"
                data-current=is_current.get()
                data-disabled=is_disabled.get()
                type="button"
                role="button"
                tabindex=if is_disabled.get() { "-1" } else { "0" }
                aria-disabled=is_disabled.get()
                aria-current=if is_current.get() { "page" } else { "false" }
                on:click=handle_click
            >
                {children()}
            </button>
        </li>
    }
}

/// PaginationFirst component for first page button
#[component]
pub fn PaginationFirst(
    /// Button text
    #[prop(optional)]
    text: Option<String>,
    /// Button icon
    #[prop(optional)]
    icon: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<PaginationContext>().expect("PaginationFirst must be used within Pagination");
    let first_id = generate_id("pagination-first");
    
    let handle_click = move |event: web_sys::MouseEvent| {
        event.prevent_default();
        
        if context.current_page.get() > 1 {
            // Call the page change handler
            if let Some(callback) = context.on_page_change.clone() {
                callback.run(1);
            }
        }
    };
    
    let is_disabled = Memo::new(move |_| context.current_page.get() <= 1);
    
    // Build base classes
    let base_classes = "radix-pagination-first";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <li
            id=first_id
            class=combined_class
            style=style.unwrap_or_default()
            data-disabled=is_disabled.get()
            role="listitem"
        >
            <button
                class="radix-pagination-button"
                data-disabled=is_disabled.get()
                type="button"
                role="button"
                tabindex=if is_disabled.get() { "-1" } else { "0" }
                aria-disabled=is_disabled.get()
                aria-label="Go to first page"
                on:click=handle_click
            >
                {icon.map(|icon_text| view! {
                    <span class="radix-pagination-icon">{icon_text}</span>
                })}
                {text.map(|button_text| view! {
                    <span class="radix-pagination-text">{button_text}</span>
                })}
                {children()}
            </button>
        </li>
    }
}

/// PaginationPrevious component for previous page button
#[component]
pub fn PaginationPrevious(
    /// Button text
    #[prop(optional)]
    text: Option<String>,
    /// Button icon
    #[prop(optional)]
    icon: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<PaginationContext>().expect("PaginationPrevious must be used within Pagination");
    let prev_id = generate_id("pagination-previous");
    
    let handle_click = move |event: web_sys::MouseEvent| {
        event.prevent_default();
        
        let current = context.current_page.get();
        if current > 1 {
            // Call the page change handler
            if let Some(callback) = context.on_page_change.clone() {
                callback.run(current - 1);
            }
        }
    };
    
    let is_disabled = Memo::new(move |_| context.current_page.get() <= 1);
    
    // Build base classes
    let base_classes = "radix-pagination-previous";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <li
            id=prev_id
            class=combined_class
            style=style.unwrap_or_default()
            data-disabled=is_disabled.get()
            role="listitem"
        >
            <button
                class="radix-pagination-button"
                data-disabled=is_disabled.get()
                type="button"
                role="button"
                tabindex=if is_disabled.get() { "-1" } else { "0" }
                aria-disabled=is_disabled.get()
                aria-label="Go to previous page"
                on:click=handle_click
            >
                {icon.map(|icon_text| view! {
                    <span class="radix-pagination-icon">{icon_text}</span>
                })}
                {text.map(|button_text| view! {
                    <span class="radix-pagination-text">{button_text}</span>
                })}
                {children()}
            </button>
        </li>
    }
}

/// PaginationNext component for next page button
#[component]
pub fn PaginationNext(
    /// Button text
    #[prop(optional)]
    text: Option<String>,
    /// Button icon
    #[prop(optional)]
    icon: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<PaginationContext>().expect("PaginationNext must be used within Pagination");
    let next_id = generate_id("pagination-next");
    
    let handle_click = move |event: web_sys::MouseEvent| {
        event.prevent_default();
        
        let current = context.current_page.get();
        if current < context.total_pages {
            // Call the page change handler
            if let Some(callback) = context.on_page_change.clone() {
                callback.run(current + 1);
            }
        }
    };
    
    let is_disabled = Memo::new(move |_| context.current_page.get() >= context.total_pages);
    
    // Build base classes
    let base_classes = "radix-pagination-next";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <li
            id=next_id
            class=combined_class
            style=style.unwrap_or_default()
            data-disabled=is_disabled.get()
            role="listitem"
        >
            <button
                class="radix-pagination-button"
                data-disabled=is_disabled.get()
                type="button"
                role="button"
                tabindex=if is_disabled.get() { "-1" } else { "0" }
                aria-disabled=is_disabled.get()
                aria-label="Go to next page"
                on:click=handle_click
            >
                {icon.map(|icon_text| view! {
                    <span class="radix-pagination-icon">{icon_text}</span>
                })}
                {text.map(|button_text| view! {
                    <span class="radix-pagination-text">{button_text}</span>
                })}
                {children()}
            </button>
        </li>
    }
}

/// PaginationLast component for last page button
#[component]
pub fn PaginationLast(
    /// Button text
    #[prop(optional)]
    text: Option<String>,
    /// Button icon
    #[prop(optional)]
    icon: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<PaginationContext>().expect("PaginationLast must be used within Pagination");
    let last_id = generate_id("pagination-last");
    
    let handle_click = move |event: web_sys::MouseEvent| {
        event.prevent_default();
        
        if context.current_page.get() < context.total_pages {
            // Call the page change handler
            if let Some(callback) = context.on_page_change.clone() {
                callback.run(context.total_pages);
            }
        }
    };
    
    let is_disabled = Memo::new(move |_| context.current_page.get() >= context.total_pages);
    
    // Build base classes
    let base_classes = "radix-pagination-last";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <li
            id=last_id
            class=combined_class
            style=style.unwrap_or_default()
            data-disabled=is_disabled.get()
            role="listitem"
        >
            <button
                class="radix-pagination-button"
                data-disabled=is_disabled.get()
                type="button"
                role="button"
                tabindex=if is_disabled.get() { "-1" } else { "0" }
                aria-disabled=is_disabled.get()
                aria-label="Go to last page"
                on:click=handle_click
            >
                {icon.map(|icon_text| view! {
                    <span class="radix-pagination-icon">{icon_text}</span>
                })}
                {text.map(|button_text| view! {
                    <span class="radix-pagination-text">{button_text}</span>
                })}
                {children()}
            </button>
        </li>
    }
}

/// PaginationEllipsis component for truncated page ranges
#[component]
pub fn PaginationEllipsis(
    /// Ellipsis text
    #[prop(optional)]
    text: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let ellipsis_id = generate_id("pagination-ellipsis");
    
    // Build base classes
    let base_classes = "radix-pagination-ellipsis";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <li
            id=ellipsis_id
            class=combined_class
            style=style.unwrap_or_default()
            role="separator"
            aria-hidden="true"
        >
            <span class="radix-pagination-ellipsis-text">
                {text.unwrap_or_else(|| "…".to_string())}
            </span>
            {children()}
        </li>
    }
}

/// PaginationInfo component for displaying pagination information
#[component]
pub fn PaginationInfo(
    /// Information format
    #[prop(optional)]
    format: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<PaginationContext>().expect("PaginationInfo must be used within Pagination");
    let info_id = generate_id("pagination-info");
    
    // Calculate pagination information
    let start_item = Memo::new(move |_| {
        let current = context.current_page.get();
        let page_size = context.page_size;
        ((current - 1) * page_size) + 1
    });
    
    let end_item = Memo::new(move |_| {
        let current = context.current_page.get();
        let page_size = context.page_size;
        let total_items = context.total_items;
        std::cmp::min(current * page_size, total_items)
    });
    
    let total_items = context.total_items;
    
    // Build base classes
    let base_classes = "radix-pagination-info";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=info_id
            class=combined_class
            style=style.unwrap_or_default()
            role="status"
            aria-live="polite"
        >
            {if let Some(format_str) = format {
                let start = start_item.get();
                let end = end_item.get();
                let total = total_items;
                let current = context.current_page.get();
                let total_pages = context.total_pages;
                
                let info_text = format_str
                    .replace("{start}", &start.to_string())
                    .replace("{end}", &end.to_string())
                    .replace("{total}", &total.to_string())
                    .replace("{current}", &current.to_string())
                    .replace("{total_pages}", &total_pages.to_string());
                
                view! {
                    <span class="radix-pagination-info-text">{info_text}</span>
                }
            } else {
                let start = start_item.get();
                let end = end_item.get();
                let total = total_items;
                view! {
                    <span class="radix-pagination-info-text">
                        {format!("Showing {} to {} of {} results", start, end, total)}
                    </span>
                }
            }}
            {children()}
        </div>
    }
}

/// PaginationContent component for wrapping pagination content
#[component]
pub fn PaginationContent(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let content_id = generate_id("pagination-content");
    
    // Build base classes
    let base_classes = "radix-pagination-content";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=content_id
            class=combined_class
            style=style.unwrap_or_default()
        >
            {children()}
        </div>
    }
}

/// Helper function to generate page numbers for pagination
pub fn generate_page_numbers(current_page: usize, total_pages: usize, max_visible: usize) -> Vec<PaginationPage> {
    if total_pages <= max_visible {
        return (1..=total_pages)
            .map(|page| {
                PaginationPage::new(page)
                    .with_current(page == current_page)
            })
            .collect();
    }
    
    let (start, end) = calculate_page_range(current_page, total_pages, max_visible);
    let mut pages = Vec::new();
    
    // Add first page if not in range
    if start > 1 {
        pages.push(PaginationPage::new(1));
        if start > 2 {
            pages.push(PaginationPage::new(0).with_disabled(true)); // Placeholder for ellipsis
        }
    }
    
    // Add visible pages
    for page in start..=end {
        pages.push(
            PaginationPage::new(page)
                .with_current(page == current_page)
        );
    }
    
    // Add last page if not in range
    if end < total_pages {
        if end < total_pages - 1 {
            pages.push(PaginationPage::new(0).with_disabled(true)); // Placeholder for ellipsis
        }
        pages.push(PaginationPage::new(total_pages));
    }
    
    pages
}

/// Helper function to generate page numbers for pagination
/// This function returns a vector of page numbers that should be displayed
/// It handles ellipsis for large page counts
pub fn get_visible_page_numbers(current_page: usize, total_pages: usize, max_visible: usize) -> Vec<usize> {
    if total_pages <= max_visible {
        return (1..=total_pages).collect();
    }
    
    let (start, end) = calculate_page_range(current_page, total_pages, max_visible);
    let mut pages = Vec::new();
    
    // Add first page if not in range
    if start > 1 {
        pages.push(1);
        if start > 2 {
            pages.push(0); // Placeholder for ellipsis
        }
    }
    
    // Add visible pages
    for page in start..=end {
        pages.push(page);
    }
    
    // Add last page if not in range
    if end < total_pages {
        if end < total_pages - 1 {
            pages.push(0); // Placeholder for ellipsis
        }
        pages.push(total_pages);
    }
    
    pages
}
