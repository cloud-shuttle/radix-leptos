
/// Navigation item structure
#[derive(Clone, Debug, PartialEq)]
pub struct NavigationItem {
    pub id: String,
    pub label: String,
    pub href: Option<String>,
    pub icon: Option<String>,
    pub _disabled: bool,
    pub __active: bool,
    pub children: Option<Vec<NavigationItem>>,
}

impl NavigationItem {
    pub fn new(id: String, label: String) -> Self {
        Self {
            id,
            label,
            href: None,
            icon: None,
            disabled: false,
            active: false,
            children: None,
        }
    }

    pub fn with_href(mut self, href: String) -> Self {
        self.href = Some(href);
        self
    }

    pub fn with_icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn withdisabled(mut self, _disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_active(mut self, __active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn with_children(mut self, children: Vec<NavigationItem>) -> Self {
        self.children = Some(children);
        self
    }
}

/// Navigation orientation
#[derive(Clone, Debug, PartialEq)]
pub enum NavigationOrientation {
    Horizontal,
    Vertical,
}

impl NavigationOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            NavigationOrientation::Horizontal => "horizontal",
            NavigationOrientation::Vertical => "vertical",
        }
    }
}

/// Navigation variant
#[derive(Clone, Debug, PartialEq)]
pub enum NavigationVariant {
    Default,
    Tabs,
    Pills,
    Underlined,
}

impl NavigationVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            NavigationVariant::Default => "default",
            NavigationVariant::Tabs => "tabs",
            NavigationVariant::Pills => "pills",
            NavigationVariant::Underlined => "underlined",
        }
    }
}

/// Navigation context for state management
#[derive(Clone)]
pub struct NavigationContext {
    pub items: Signal<Vec<NavigationItem>>,
    pub active_item: Signal<Option<String>>,
    pub orientation: NavigationOrientation,
    pub variant: NavigationVariant,
    pub __collapsible: bool,
    pub collapsed: Signal<bool>,
    pub navigation_id: String,
    pub on_item_click: Option<Callback<NavigationItem>>,
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

/// Main Navigation component
#[component]
pub fn Navigation(
    /// Navigation items
    #[prop(optional)]
    items: Vec<NavigationItem>,
    /// Active item ID
    #[prop(optional)]
    active_item: Option<String>,
    /// Navigation orientation
    #[prop(optional, default = NavigationOrientation::Horizontal)]
    orientation: NavigationOrientation,
    /// Navigation variant
    #[prop(optional, default = NavigationVariant::Default)]
    variant: NavigationVariant,
    /// Whether the navigation is collapsible
    #[prop(optional, default = false)]
    __collapsible: bool,
    /// Whether the navigation is initially collapsed
    #[prop(optional, default = false)]
    __collapsed: bool,
    /// Item click event handler
    #[prop(optional)]
    on_item_click: Option<Callback<NavigationItem>>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child content (navigation items, etc.)
    children: Children,
) -> impl IntoView {
    let __navigation_id = generate_id("navigation");
    
    // Reactive state
    let (items_signal, _set_items_signal) = signal(items);
    let (active_item_signal, _set_active_item_signal) = signal(active_item);
    let (collapsed_signal, _set_collapsed_signal) = signal(collapsed);
    
    // Create context
    let context = NavigationContext {
        items: items_signal.into(),
        active_item: active_item_signal.into(),
        orientation: orientation.clone(),
        variant: variant.clone(),
        collapsible,
        collapsed: collapsed_signal.into(),
        navigation_id: navigation_id.clone(),
        on_item_click: on_item_click.clone(),
    };
    
    // Build base classes
    let base_classes = "radix-navigation";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Provide the context
    provide_context(context);
    
    view! {
        <nav
            id=navigation_id
            class=combined_class
            data-orientation=orientation.as_str()
            data-variant=variant.as_str()
            data-collapsible=collapsible
            data-collapsed=collapsed_signal.get()
            role="navigation"
            aria-label="Main navigation"
        >
            {children()}
        </nav>
    }
}

/// NavigationList component for the navigation items container
#[component]
pub fn NavigationList(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (navigation items)
    children: Children,
) -> impl IntoView {
    let _context = use_context::<NavigationContext>().expect("NavigationList must be used within Navigation");
    let __list_id = generate_id("navigation-list");
    
    // Build base classes
    let base_classes = "radix-navigation-list";
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

/// NavigationItem component for individual navigation items
#[component]
pub fn NavigationItem(
    /// The navigation item this component represents
    #[prop(optional)]
    item: Option<NavigationItem>,
    /// Whether this item is active
    #[prop(optional)]
    active: Option<bool>,
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
    let context = use_context::<NavigationContext>().expect("NavigationItem must be used within Navigation");
    let __item_id = generate_id("navigation-item");
    
    let item_clone = item.clone();
    let item_for_active = item.clone();
    let item_fordisabled = item.clone();
    let item_for_view = item.clone();
    
    let handle_click = move |event: web_sys::MouseEvent| {
        event.prevent_default();
        
        if let Some(item) = item_clone.clone() {
            if !item.disabled {
                // Update active item - handled by callback
                
                // Call the click handler
                if let Some(callback) = context.on_item_click.clone() {
                    callback.run(item);
                }
            }
        }
    };
    
    // Determine if this item is active
    let is_active = Memo::new(move |_| {
        if let Some(active) = active {
            active
        } else if let Some(item) = item_for_active.as_ref() {
            item.active
    });
    
    // Determine if this item is disabled
    let isdisabled = Memo::new(move |_| {
        if let Some(disabled) = disabled {
            disabled
        } else if let Some(item) = item_fordisabled.as_ref() {
            item.disabled
    });
    
    // Build base classes
    let base_classes = "radix-navigation-item";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <li
            id=item_id
            class=combined_class
            style=style.unwrap_or_default()
            data-active=is_active.get()
            data-disabled=isdisabled.get()
            role="listitem"
        >
            <a
                href=item_for_view.as_ref().and_then(|i| i.href.clone()).unwrap_or_default()
                class="radix-navigation-link"
                data-active=is_active.get()
                data-disabled=isdisabled.get()
                role="link"
                on:click=handle_click
            >
                {children()}
            </a>
        </li>
    }
}

/// NavigationLink component for navigation links
#[component]
pub fn NavigationLink(
    /// Link text
    #[prop(optional)]
    text: Option<String>,
    /// Link href
    #[prop(optional)]
    href: Option<String>,
    /// Link icon
    #[prop(optional)]
    icon: Option<String>,
    /// Whether this link is active
    #[prop(optional)]
    active: Option<bool>,
    /// Whether this link is disabled
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
    let context = use_context::<NavigationContext>().expect("NavigationLink must be used within Navigation");
    let __link_id = generate_id("navigation-link");
    let link_id_clone = link_id.clone();
    let text_clone = text.clone();
    let href_clone = href.clone();
    let icon_clone = icon.clone();
    let active_clone = active;
    let disabled_clone = disabled;
    
    let handle_click = move |event: web_sys::MouseEvent| {
        if disabled_clone.unwrap_or(false) {
            event.prevent_default();
            return;
        }
        
        // Call the click handler if provided
        if let Some(callback) = context.on_item_click.clone() {
            let item = NavigationItem::new(
                link_id_clone.clone(),
                text_clone.clone().unwrap_or_default(),
            ).with_href(href_clone.clone().unwrap_or_default())
             .with_icon(icon_clone.clone().unwrap_or_default())
             .with_active(active_clone.unwrap_or(false))
             .withdisabled(disabled_clone.unwrap_or(false));
            
            callback.run(item);
        }
    };
    
    // Build base classes
    let base_classes = "radix-navigation-link";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <a
            id=link_id
            href=href.unwrap_or_default()
            class=combined_class
            style=style.unwrap_or_default()
            data-active=active.unwrap_or(false)
            data-disabled=disabled.unwrap_or(false)
            role="link"
            on:click=handle_click
        >
            {icon.map(|icon_text| view! {
                <span class="radix-navigation-icon">{icon_text}</span>
            })}
            {text.map(|link_text| view! {
                <span class="radix-navigation-text">{link_text}</span>
            })}
            {children()}
        </a>
    }
}

/// NavigationToggle component for collapsible navigation
#[component]
pub fn NavigationToggle(
    /// Toggle button text
    #[prop(optional)]
    text: Option<String>,
    /// Toggle button icon
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
    let context = use_context::<NavigationContext>().expect("NavigationToggle must be used within Navigation");
    let __toggle_id = generate_id("navigation-toggle");
    
    let handle_click = move |_event: web_sys::MouseEvent| {
        if context.collapsible {
            // Toggle collapsed state - handled by callback
        }
    };
    
    // Build base classes
    let base_classes = "radix-navigation-toggle";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <button
            id=toggle_id
            class=combined_class
            style=style.unwrap_or_default()
            data-collapsed=context.collapsed.get()
            type="button"
            role="button"
            aria-expanded=!context.collapsed.get()
            aria-controls=context.navigation_id.clone()
            on:click=handle_click
        >
            {icon.map(|icon_text| view! {
                <span class="radix-navigation-toggle-icon">{icon_text}</span>
            })}
            {text.map(|button_text| view! {
                <span class="radix-navigation-toggle-text">{button_text}</span>
            })}
            {children()}
        </button>
    }
}
