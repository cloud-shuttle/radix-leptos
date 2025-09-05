
/// Separator component for visual separation between content
///
/// The Separator component provides accessible visual separation between
/// content sections with proper ARIA attributes and orientation support.
///
/// # Features
/// - Horizontal and vertical orientation
/// - Proper ARIA attributes for screen readers
/// - Flexible styling with data attributes
/// - Decorative and semantic variants
///
/// # Example
///
/// ```rust
/// use radix_leptos_primitives::*;
///
/// #[component]
/// fn MyComponent() -> impl IntoView {
///     view! {
///         <div>
///             <h2>"Section 1"</h2>
///             <p>"Content for section 1"</p>
///             <Separator orientation=SeparatorOrientation::Horizontal />
///             <h2>"Section 2"</h2>
///             <p>"Content for section 2"</p>
///         </div>
///     }
/// }
/// ```
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SeparatorOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SeparatorVariant {
    /// Decorative separator (no semantic meaning)
    Decorative,
    /// Semantic separator (indicates content separation)
    Semantic,
}

impl SeparatorOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            SeparatorOrientation::Horizontal => "horizontal",
            SeparatorOrientation::Vertical => "vertical",
        }
    }
}

impl SeparatorVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SeparatorVariant::Decorative => "decorative",
            SeparatorVariant::Semantic => "semantic",
        }
    }
}

/// Separator component for visual separation between content
#[component]
pub fn Separator(
    /// Orientation of the separator
    #[prop(optional, default = SeparatorOrientation::Horizontal)]
    orientation: SeparatorOrientation,
    /// Variant of the separator
    #[prop(optional, default = SeparatorVariant::Decorative)]
    variant: SeparatorVariant,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let separator_id = generate_id("separator");
    
    // Build base classes with data attributes for CSS targeting
    let base_classes = "radix-separator";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Data attributes for styling
    let data_orientation = orientation.as_str();
    let data_variant = variant.as_str();
    
    // Role and aria attributes for semantic separators
    let (role_attr, aria_orientation) = if variant == SeparatorVariant::Semantic {
        ("separator", Some(orientation.as_str()))
    
    view! {
        <div
            id=separator_id
            class=combined_class
            style=style.unwrap_or_default()
        </div>
    }
}

/// Horizontal separator component
#[component]
pub fn HorizontalSeparator(
    /// Variant of the separator
    #[prop(optional, default = SeparatorVariant::Decorative)]
    variant: SeparatorVariant,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    view! {
        <Separator
            orientation=SeparatorOrientation::Horizontal
            variant=variant
            class=class.unwrap_or_default()
            style=style.unwrap_or_default()
        >
            {children()}
        </Separator>
    }
}

/// Vertical separator component
#[component]
pub fn VerticalSeparator(
    /// Variant of the separator
    #[prop(optional, default = SeparatorVariant::Decorative)]
    variant: SeparatorVariant,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    view! {
        <Separator
            orientation=SeparatorOrientation::Vertical
            variant=variant
            class=class.unwrap_or_default()
            style=style.unwrap_or_default()
        >
            {children()}
        </Separator>
    }
}


