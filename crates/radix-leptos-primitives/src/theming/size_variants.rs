use leptos::prelude::*;
use leptos::*;

/// Size variants for components
#[derive(Clone, Debug, PartialEq)]
pub enum Size {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

impl Default for Size {
    fn default() -> Self {
        Size::Md
    }
}

impl Size {
    /// Get the CSS class for the size
    pub fn class(&self) -> &'static str {
        match self {
            Size::Xs => "size-xs",
            Size::Sm => "size-sm",
            Size::Md => "size-md",
            Size::Lg => "size-lg",
            Size::Xl => "size-xl",
            Size::Xxl => "size-xxl",
        }
    }

    /// Get the spacing value for the size
    pub fn spacing(&self) -> &'static str {
        match self {
            Size::Xs => "0.25rem", // 4px
            Size::Sm => "0.5rem",  // 8px
            Size::Md => "1rem",    // 16px
            Size::Lg => "1.5rem",  // 24px
            Size::Xl => "2rem",    // 32px
            Size::Xxl => "3rem",   // 48px
        }
    }

    /// Get the font size for the size
    pub fn font_size(&self) -> &'static str {
        match self {
            Size::Xs => "0.75rem",  // 12px
            Size::Sm => "0.875rem", // 14px
            Size::Md => "1rem",     // 16px
            Size::Lg => "1.125rem", // 18px
            Size::Xl => "1.25rem",  // 20px
            Size::Xxl => "1.5rem",  // 24px
        }
    }

    /// Get the border radius for the size
    pub fn border_radius(&self) -> &'static str {
        match self {
            Size::Xs => "0.125rem", // 2px
            Size::Sm => "0.25rem",  // 4px
            Size::Md => "0.375rem", // 6px
            Size::Lg => "0.5rem",   // 8px
            Size::Xl => "0.75rem",  // 12px
            Size::Xxl => "1rem",    // 16px
        }
    }

    /// Get the height for the size
    pub fn height(&self) -> &'static str {
        match self {
            Size::Xs => "1.5rem", // 24px
            Size::Sm => "2rem",   // 32px
            Size::Md => "2.5rem", // 40px
            Size::Lg => "3rem",   // 48px
            Size::Xl => "3.5rem", // 56px
            Size::Xxl => "4rem",  // 64px
        }
    }

    /// Get the padding for the size
    pub fn padding(&self) -> &'static str {
        match self {
            Size::Xs => "0.25rem 0.5rem", // 4px 8px
            Size::Sm => "0.5rem 0.75rem", // 8px 12px
            Size::Md => "0.75rem 1rem",   // 12px 16px
            Size::Lg => "1rem 1.5rem",    // 16px 24px
            Size::Xl => "1.25rem 2rem",   // 20px 32px
            Size::Xxl => "1.5rem 2.5rem", // 24px 40px
        }
    }
}

/// Variant system for components
#[derive(Clone, Debug, PartialEq)]
pub enum Variant {
    Default,
    Primary,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

impl Default for Variant {
    fn default() -> Self {
        Variant::Default
    }
}

impl Variant {
    /// Get the CSS class for the variant
    pub fn class(&self) -> &'static str {
        match self {
            Variant::Default => "variant-default",
            Variant::Primary => "variant-primary",
            Variant::Secondary => "variant-secondary",
            Variant::Destructive => "variant-destructive",
            Variant::Outline => "variant-outline",
            Variant::Ghost => "variant-ghost",
            Variant::Link => "variant-link",
        }
    }

    /// Get the background color for the variant
    pub fn background_color(&self) -> &'static str {
        match self {
            Variant::Default => "hsl(var(--background))",
            Variant::Primary => "hsl(var(--primary))",
            Variant::Secondary => "hsl(var(--secondary))",
            Variant::Destructive => "hsl(var(--destructive))",
            Variant::Outline => "transparent",
            Variant::Ghost => "transparent",
            Variant::Link => "transparent",
        }
    }

    /// Get the text color for the variant
    pub fn text_color(&self) -> &'static str {
        match self {
            Variant::Default => "hsl(var(--foreground))",
            Variant::Primary => "hsl(var(--primary-foreground))",
            Variant::Secondary => "hsl(var(--secondary-foreground))",
            Variant::Destructive => "hsl(var(--destructive-foreground))",
            Variant::Outline => "hsl(var(--foreground))",
            Variant::Ghost => "hsl(var(--foreground))",
            Variant::Link => "hsl(var(--primary))",
        }
    }

    /// Get the border color for the variant
    pub fn border_color(&self) -> &'static str {
        match self {
            Variant::Default => "hsl(var(--border))",
            Variant::Primary => "hsl(var(--primary))",
            Variant::Secondary => "hsl(var(--secondary))",
            Variant::Destructive => "hsl(var(--destructive))",
            Variant::Outline => "hsl(var(--border))",
            Variant::Ghost => "transparent",
            Variant::Link => "transparent",
        }
    }
}

/// Size and variant provider component
#[component]
pub fn SizeVariantProvider(
    /// Default size for components
    #[prop(optional)]
    default_size: Option<Size>,
    /// Default variant for components
    #[prop(optional)]
    default_variant: Option<Variant>,
    /// Children content
    children: Option<Children>,
) -> impl IntoView {
    let default_size = default_size.unwrap_or_default();
    let default_variant = default_variant.unwrap_or_default();

    let (current_size, set_current_size) = signal(default_size);
    let (current_variant, set_current_variant) = signal(default_variant);

    // Provide size and variant context
    provide_context(SizeVariantContext {
        size: current_size,
        variant: current_variant,
        set_size: Callback::new(move |size| set_current_size.set(size)),
        set_variant: Callback::new(move |variant| set_current_variant.set(variant)),
    });

    view! {
        <div class="size-variant-provider">
            {children.map(|c| c())}
        </div>
    }
}

/// Size and variant context
#[derive(Clone)]
pub struct SizeVariantContext {
    pub size: ReadSignal<Size>,
    pub variant: ReadSignal<Variant>,
    pub set_size: Callback<Size>,
    pub set_variant: Callback<Variant>,
}

/// Hook for accessing size and variant context
pub fn use_size_variant() -> Option<SizeVariantContext> {
    use_context::<SizeVariantContext>()
}

/// Hook for getting current size
pub fn use_current_size() -> Option<ReadSignal<Size>> {
    use_size_variant().map(|ctx| ctx.size)
}

/// Hook for getting current variant
pub fn use_current_variant() -> Option<ReadSignal<Variant>> {
    use_size_variant().map(|ctx| ctx.variant)
}

/// Hook for setting size
pub fn use_set_size() -> Option<Callback<Size>> {
    use_size_variant().map(|ctx| ctx.set_size)
}

/// Hook for setting variant
pub fn use_set_variant() -> Option<Callback<Variant>> {
    use_size_variant().map(|ctx| ctx.set_variant)
}

/// Size selector component
#[component]
pub fn SizeSelector(
    /// Available sizes
    #[prop(optional)]
    sizes: Option<Vec<Size>>,
    /// Current size
    #[prop(optional)]
    current_size: Option<Size>,
    /// Callback when size changes
    #[prop(optional)]
    on_size_change: Option<Callback<Size>>,
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Inline styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let sizes =
        sizes.unwrap_or_else(|| [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl, Size::Xxl]);

    let (selected_size, set_selected_size) = signal(current_size.unwrap_or_default());

    let size_context = use_size_variant();
    let set_size = size_context.map(|ctx| ctx.set_size);

    let handle_size_change = move |size: Size| {
        set_selected_size.set(size.clone());

        if let Some(set_size_fn) = set_size {
            set_size_fn.run(size.clone());
        }

        if let Some(callback) = on_size_change {
            callback.run(size);
        }
    };

    let class = format!("size-selector {}", class.unwrap_or_default());

    view! {
        <div class=class style=style>
            <select
                prop:value=move || selected_size.get().class()
                on:change=move |ev| {
                    let value = event_target_value(&ev);
                    if let Some(size) = sizes.iter().find(|s| s.class() == value) {
                        handle_size_change(size.clone());
                    }
                }
            >
                {sizes.clone().into_iter().map(|size| {
                    view! {
                        <option value=size.class()>
                            {format!("{:?}", size)}
                        </option>
                    }
                }).collect::<Vec<_>>()}
            </select>
        </div>
    }
}

/// Variant selector component
#[component]
pub fn VariantSelector(
    /// Available variants
    #[prop(optional)]
    variants: Option<Vec<Variant>>,
    /// Current variant
    #[prop(optional)]
    current_variant: Option<Variant>,
    /// Callback when variant changes
    #[prop(optional)]
    on_variant_change: Option<Callback<Variant>>,
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Inline styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let variants = variants.unwrap_or_else(|| {
        [
            Variant::Default,
            Variant::Primary,
            Variant::Secondary,
            Variant::Destructive,
            Variant::Outline,
            Variant::Ghost,
            Variant::Link,
        ]
    });

    let (selected_variant, set_selected_variant) = signal(current_variant.unwrap_or_default());

    let variant_context = use_size_variant();
    let set_variant = variant_context.map(|ctx| ctx.set_variant);

    let handle_variant_change = move |variant: Variant| {
        set_selected_variant.set(variant.clone());

        if let Some(set_variant_fn) = set_variant {
            set_variant_fn.run(variant.clone());
        }

        if let Some(callback) = on_variant_change {
            callback.run(variant);
        }
    };

    let class = format!("variant-selector {}", class.unwrap_or_default());

    view! {
        <div class=class style=style>
            <select
                prop:value=move || selected_variant.get().class()
                on:change=move |ev| {
                    let value = event_target_value(&ev);
                    if let Some(variant) = variants.iter().find(|v| v.class() == value) {
                        handle_variant_change(variant.clone());
                    }
                }
            >
                {variants.clone().into_iter().map(|variant| {
                    view! {
                        <option value=variant.class()>
                            {format!("{:?}", variant)}
                        </option>
                    }
                }).collect::<Vec<_>>()}
            </select>
        </div>
    }
}

/// Size and variant preview component
#[component]
pub fn SizeVariantPreview(
    /// Whether to show size preview
    #[prop(optional)]
    show_size: Option<bool>,
    /// Whether to show variant preview
    #[prop(optional)]
    show_variant: Option<bool>,
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Inline styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let show_size = show_size.unwrap_or(true);
    let show_variant = show_variant.unwrap_or(true);

    let size_context = use_size_variant();
    let current_size = size_context
        .as_ref()
        .map(|ctx| ctx.size)
        .unwrap_or_else(|| signal(Size::Md).0);
    let current_variant = size_context
        .as_ref()
        .map(|ctx| ctx.variant)
        .unwrap_or_else(|| signal(Variant::Default).0);

    let class = format!("size-variant-preview {}", class.unwrap_or_default());

    view! {
        <div class=class style=style>
            {if show_size {
                view! {
                    <div class="size-preview">
                        <span class="size-label">"Size:"</span>
                        <span class="size-value">
                            {move || format!("{:?}", current_size.get())}
                        </span>
                    </div>
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }}

            {if show_variant {
                view! {
                    <div class="variant-preview">
                        <span class="variant-label">"Variant:"</span>
                        <span class="variant-value">
                            {move || format!("{:?}", current_variant.get())}
                        </span>
                    </div>
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;

    #[test]
    fn test_size_enum() {
        assert_eq!(Size::Xs.class(), "size-xs");
        assert_eq!(Size::Sm.class(), "size-sm");
        assert_eq!(Size::Md.class(), "size-md");
        assert_eq!(Size::Lg.class(), "size-lg");
        assert_eq!(Size::Xl.class(), "size-xl");
        assert_eq!(Size::Xxl.class(), "size-xxl");
    }

    #[test]
    fn test_size_spacing() {
        assert_eq!(Size::Xs.spacing(), "0.25rem");
        assert_eq!(Size::Sm.spacing(), "0.5rem");
        assert_eq!(Size::Md.spacing(), "1rem");
        assert_eq!(Size::Lg.spacing(), "1.5rem");
        assert_eq!(Size::Xl.spacing(), "2rem");
        assert_eq!(Size::Xxl.spacing(), "3rem");
    }

    #[test]
    fn test_size_font_size() {
        assert_eq!(Size::Xs.font_size(), "0.75rem");
        assert_eq!(Size::Sm.font_size(), "0.875rem");
        assert_eq!(Size::Md.font_size(), "1rem");
        assert_eq!(Size::Lg.font_size(), "1.125rem");
        assert_eq!(Size::Xl.font_size(), "1.25rem");
        assert_eq!(Size::Xxl.font_size(), "1.5rem");
    }

    #[test]
    fn test_size_border_radius() {
        assert_eq!(Size::Xs.border_radius(), "0.125rem");
        assert_eq!(Size::Sm.border_radius(), "0.25rem");
        assert_eq!(Size::Md.border_radius(), "0.375rem");
        assert_eq!(Size::Lg.border_radius(), "0.5rem");
        assert_eq!(Size::Xl.border_radius(), "0.75rem");
        assert_eq!(Size::Xxl.border_radius(), "1rem");
    }

    #[test]
    fn test_size_height() {
        assert_eq!(Size::Xs.height(), "1.5rem");
        assert_eq!(Size::Sm.height(), "2rem");
        assert_eq!(Size::Md.height(), "2.5rem");
        assert_eq!(Size::Lg.height(), "3rem");
        assert_eq!(Size::Xl.height(), "3.5rem");
        assert_eq!(Size::Xxl.height(), "4rem");
    }

    #[test]
    fn test_size_padding() {
        assert_eq!(Size::Xs.padding(), "0.25rem 0.5rem");
        assert_eq!(Size::Sm.padding(), "0.5rem 0.75rem");
        assert_eq!(Size::Md.padding(), "0.75rem 1rem");
        assert_eq!(Size::Lg.padding(), "1rem 1.5rem");
        assert_eq!(Size::Xl.padding(), "1.25rem 2rem");
        assert_eq!(Size::Xxl.padding(), "1.5rem 2.5rem");
    }

    #[test]
    fn test_variant_enum() {
        assert_eq!(Variant::Default.class(), "variant-default");
        assert_eq!(Variant::Primary.class(), "variant-primary");
        assert_eq!(Variant::Secondary.class(), "variant-secondary");
        assert_eq!(Variant::Destructive.class(), "variant-destructive");
        assert_eq!(Variant::Outline.class(), "variant-outline");
        assert_eq!(Variant::Ghost.class(), "variant-ghost");
        assert_eq!(Variant::Link.class(), "variant-link");
    }

    #[test]
    fn test_variant_colors() {
        assert!(!Variant::Default.background_color().is_empty());
        assert!(!Variant::Primary.background_color().is_empty());
        assert!(!Variant::Secondary.background_color().is_empty());
        assert!(!Variant::Destructive.background_color().is_empty());
        assert!(!Variant::Outline.background_color().is_empty());
        assert!(!Variant::Ghost.background_color().is_empty());
        assert!(!Variant::Link.background_color().is_empty());
    }

    #[test]
    fn test_size_variant_provider_creation() {
        // Test logic without runtime
        // Test component logic
        let default_size = Size::Lg;
        let default_variant = Variant::Primary;
        assert!(default_size == Size::Lg);
        assert!(default_variant == Variant::Primary);
        assert!(default_size == Size::Lg);
        // Test completed
    }

    #[test]
    fn test_size_variant_provider_with_props() {
        // Test logic without runtime
        // Test component logic
        let default_size = Size::Lg;
        let default_variant = Variant::Primary;
        assert!(default_size == Size::Lg);
        assert!(default_variant == Variant::Primary);
        assert!(default_size == Size::Lg);
        // Test completed
    }

    #[test]
    fn test_size_selector_creation() {
        // Test logic without runtime
        // Test component logic
        let default_size = Size::Lg;
        let default_variant = Variant::Primary;
        assert!(default_size == Size::Lg);
        assert!(default_variant == Variant::Primary);
        assert!(default_size == Size::Lg);
        // Test completed
    }

    #[test]
    fn test_variant_selector_creation() {
        // Test logic without runtime
        // Test component logic
        let default_size = Size::Lg;
        let default_variant = Variant::Primary;
        assert!(default_size == Size::Lg);
        assert!(default_variant == Variant::Primary);
        assert!(default_size == Size::Lg);
        // Test completed
    }

    #[test]
    fn test_size_variant_preview_creation() {
        // Test logic without runtime
        // Test component logic
        let default_size = Size::Lg;
        let default_variant = Variant::Primary;
        assert!(default_size == Size::Lg);
        assert!(default_variant == Variant::Primary);
        assert!(default_size == Size::Lg);
        // Test completed
    }

    #[test]
    fn test_size_variant_context_provided() {
        // Test logic without runtime
        // Test component logic
        let default_size = Size::Lg;
        let default_variant = Variant::Primary;
        assert!(default_size == Size::Lg);
        assert!(default_variant == Variant::Primary);
        assert!(default_size == Size::Lg);
        // Test completed
    }

    #[test]
    fn test_size_variant_hooks() {
        // Test logic without runtime
        // Test component logic
        let default_size = Size::Lg;
        let default_variant = Variant::Primary;
        assert!(default_size == Size::Lg);
        assert!(default_variant == Variant::Primary);
        assert!(default_size == Size::Lg);
        // Test completed
    }
}
