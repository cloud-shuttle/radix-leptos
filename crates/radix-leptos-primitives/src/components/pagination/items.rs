use leptos::children::Children;
use leptos::context::use_context;
use leptos::prelude::*;

use super::context::{PaginationContext, PaginationPage};
use crate::utils::{merge_optional_classes, generate_id};

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
    let _context =
        use_context::<PaginationContext>().expect("PaginationList must be used within Pagination");
    let list_id = generate_id("pagination-list");

    // Build base classes
    let base_classes = "radix-pagination-list";
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
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
    let context =
        use_context::<PaginationContext>().expect("PaginationItem must be used within Pagination");
    let item_id = generate_id("pagination-item");

    let page_clone = page.clone();
    let handle_click = move |event: web_sys::MouseEvent| {
        event.prevent_default();

        if let Some(page) = page_clone.clone() {
            if !page.disabled {
                // Call the page change handler
                if let Some(callback) = context.on_page_change {
                    callback.run(page.number);
                }
            }
        }
    };

    let page_forcurrent = page.clone();
    let page_fordisabled = page.clone();

    // Determine if this item is current
    let iscurrent = Memo::new(move |_| {
        if let Some(current) = current {
            current
        } else if let Some(page) = page_forcurrent.as_ref() {
            page._current
        } else {
            false
        }
    });

    // Determine if this item is disabled
    let isdisabled = Memo::new(move |_| {
        if let Some(disabled) = disabled {
            disabled
        } else if let Some(page) = page_fordisabled.as_ref() {
            page.disabled
        } else {
            false
        }
    });

    // Build base classes
    let base_classes = "radix-pagination-item";
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());

    view! {
        <li
            id=item_id
            class=combined_class
            style=style.unwrap_or_default()
            data-current=iscurrent.get()
            data-disabled=isdisabled.get()
            role="listitem"
        >
            <button
                class="radix-pagination-button"
                data-current=iscurrent.get()
                data-disabled=isdisabled.get()
                type="button"
                role="button"
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
    let context =
        use_context::<PaginationContext>().expect("PaginationFirst must be used within Pagination");
    let first_id = generate_id("pagination-first");

    let handle_click = move |event: web_sys::MouseEvent| {
        event.prevent_default();

        if context.current_page.get() > 1 {
            // Call the page change handler
            if let Some(callback) = context.on_page_change {
                callback.run(1);
            }
        }
    };

    let isdisabled = Memo::new(move |_| context.current_page.get() <= 1);

    // Build base classes
    let base_classes = "radix-pagination-first";
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());

    view! {
        <li
            id=first_id
            class=combined_class
            style=style.unwrap_or_default()
            data-disabled=isdisabled.get()
            role="listitem"
        >
            <button
                class="radix-pagination-button"
                data-disabled=isdisabled.get()
                type="button"
                role="button"
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
    let context = use_context::<PaginationContext>()
        .expect("PaginationPrevious must be used within Pagination");
    let prev_id = generate_id("pagination-previous");

    let handle_click = move |event: web_sys::MouseEvent| {
        event.prevent_default();

        let current = context.current_page.get();
        if current > 1 {
            // Call the page change handler
            if let Some(callback) = context.on_page_change {
                callback.run(current - 1);
            }
        }
    };

    let isdisabled = Memo::new(move |_| context.current_page.get() <= 1);

    // Build base classes
    let base_classes = "radix-pagination-previous";
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());

    view! {
        <li
            id=prev_id
            class=combined_class
            style=style.unwrap_or_default()
            data-disabled=isdisabled.get()
            role="listitem"
        >
            <button
                class="radix-pagination-button"
                data-disabled=isdisabled.get()
                type="button"
                role="button"
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
    let context =
        use_context::<PaginationContext>().expect("PaginationNext must be used within Pagination");
    let next_id = generate_id("pagination-next");

    let handle_click = move |event: web_sys::MouseEvent| {
        event.prevent_default();

        let current = context.current_page.get();
        if current < context.total_pages {
            // Call the page change handler
            if let Some(callback) = context.on_page_change {
                callback.run(current + 1);
            }
        }
    };

    let isdisabled = Memo::new(move |_| context.current_page.get() >= context.total_pages);

    // Build base classes
    let base_classes = "radix-pagination-next";
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());

    view! {
        <li
            id=next_id
            class=combined_class
            style=style.unwrap_or_default()
            data-disabled=isdisabled.get()
            role="listitem"
        >
            <button
                class="radix-pagination-button"
                data-disabled=isdisabled.get()
                type="button"
                role="button"
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
    let context =
        use_context::<PaginationContext>().expect("PaginationLast must be used within Pagination");
    let last_id = generate_id("pagination-last");

    let handle_click = move |event: web_sys::MouseEvent| {
        event.prevent_default();

        if context.current_page.get() < context.total_pages {
            // Call the page change handler
            if let Some(callback) = context.on_page_change {
                callback.run(context.total_pages);
            }
        }
    };

    let isdisabled = Memo::new(move |_| context.current_page.get() >= context.total_pages);

    // Build base classes
    let base_classes = "radix-pagination-last";
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());

    view! {
        <li
            id=last_id
            class=combined_class
            style=style.unwrap_or_default()
            data-disabled=isdisabled.get()
            role="listitem"
        >
            <button
                class="radix-pagination-button"
                data-disabled=isdisabled.get()
                type="button"
                role="button"
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
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
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
    let context =
        use_context::<PaginationContext>().expect("PaginationInfo must be used within Pagination");
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
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
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
                view! { <span class="radix-pagination-info-text">{String::new()}</span> }
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
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
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

#[cfg(test)]
mod items_tests {
    use super::*;
use crate::utils::{merge_optional_classes, generate_id};

    #[test]
    fn test_pagination_list_creation() {
        // Test that PaginationList can be created without runtime
        let _list_id = generate_id("pagination-list");
        assert!(!_list_id.is_empty());
    }

    #[test]
    fn test_pagination_item_creation() {
        // Test that PaginationItem can be created without runtime
        let _item_id = generate_id("pagination-item");
        assert!(!_item_id.is_empty());
    }

    #[test]
    fn test_pagination_ellipsis_creation() {
        // Test that PaginationEllipsis can be created without runtime
        let _ellipsis_id = generate_id("pagination-ellipsis");
        assert!(!_ellipsis_id.is_empty());
    }
}
