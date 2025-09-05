
/// Avatar size variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AvatarSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

/// Avatar shape variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AvatarShape {
    Circle,
    Square,
    Rounded,
}

/// Merge CSS classes with proper spacing
fn merge_classes(classes: &[&str]) -> String {
    classes.iter().filter(|&&c| !c.is_empty()).map(|&s| s).collect::<Vec<&str>>().join(" ")
}

/// Root Avatar component
#[component]
pub fn Avatar(
    /// Avatar size
    #[prop(optional, default = AvatarSize::Medium)]
    size: AvatarSize,
    /// Avatar shape
    #[prop(optional, default = AvatarShape::Circle)]
    shape: AvatarShape,
    /// Whether the avatar is interactive (clickable)
    #[prop(optional, default = false)]
    _interactive: bool,
    /// Whether the avatar is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let size_class = move || {
        match size {
            AvatarSize::Small => "radix-avatar--size-small",
            AvatarSize::Medium => "radix-avatar--size-medium",
            AvatarSize::Large => "radix-avatar--size-large",
            AvatarSize::ExtraLarge => "radix-avatar--size-extra-large",
        }
    };
    
    let shape_class = move || {
        match shape {
            AvatarShape::Circle => "radix-avatar--shape-circle",
            AvatarShape::Square => "radix-avatar--shape-square",
            AvatarShape::Rounded => "radix-avatar--shape-rounded",
        }
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
    
    let mut base_classes = [
        "radix-avatar",
        &size_class(),
        &shape_class(),
        &class_value,
    ];
    
    if interactive && !disabled {
        base_classes.push("radix-avatar--interactive");
    }
    
    view! {
        <span
            class=merge_classes(&base_classes)
            role="img"
            on:click=handle_click
        >
            {children_view}
        </span>
    }
}

/// Avatar image component
#[component]
pub fn AvatarImage(
    /// Image source URL
    src: String,
    /// Alt text for accessibility
    #[prop(optional)]
    alt: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();
    let alt_text = alt.unwrap_or_else(|| "Avatar image".to_string());
    
    view! {
        <img
            src=src
            alt=alt_text
            class=merge_classes(&["radix-avatar-image", &class_value])
        />
    }
}

/// Avatar fallback component (shown when image fails to load)
#[component]
pub fn AvatarFallback(
    /// Fallback content (usually initials or icon)
    children: Children,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();
    let children_view = children();
    
    view! {
        <span
            class=merge_classes(&["radix-avatar-fallback", &class_value])
            role="img"
        >
            {children_view}
        </span>
    }
}

/// Avatar group component for displaying multiple avatars
#[component]
pub fn AvatarGroup(
    /// Maximum number of avatars to show
    #[prop(optional, default = 5)]
    max_avatars: usize,
    /// Avatar size
    #[prop(optional, default = AvatarSize::Medium)]
    size: AvatarSize,
    /// Avatar shape
    #[prop(optional, default = AvatarShape::Circle)]
    shape: AvatarShape,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child avatars
    children: Children,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();
    let children_view = children();
    
    let size_class = move || {
        match size {
            AvatarSize::Small => "radix-avatar-group--size-small",
            AvatarSize::Medium => "radix-avatar-group--size-medium",
            AvatarSize::Large => "radix-avatar-group--size-large",
            AvatarSize::ExtraLarge => "radix-avatar-group--size-extra-large",
        }
    };
    
    let shape_class = move || {
        match shape {
            AvatarShape::Circle => "radix-avatar-group--shape-circle",
            AvatarShape::Square => "radix-avatar-group--shape-square",
            AvatarShape::Rounded => "radix-avatar-group--shape-rounded",
        }
    };
    
    view! {
        <div
            class=merge_classes(&[
                "radix-avatar-group",
                &size_class(),
                &shape_class(),
                &class_value,
            ])
            role="group"
        >
            {children_view}
        </div>
    }
}

/// Avatar with initials fallback
#[component]
pub fn AvatarWithInitials(
    /// User's name for generating initials
    name: String,
    /// Avatar size
    #[prop(optional, default = AvatarSize::Medium)]
    size: AvatarSize,
    /// Avatar shape
    #[prop(optional, default = AvatarShape::Circle)]
    shape: AvatarShape,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let initials = move || {
        let name_parts: Vec<&str> = name.split_whitespace().collect();
        if name_parts.len() >= 2 {
            format!("{}{}", 
                name_parts[0].chars().next().unwrap_or('?').to_uppercase(),
                name_parts[1].chars().next().unwrap_or('?').to_uppercase()
            )
        } else if name_parts.len() == 1 {
            name_parts[0].chars().next().unwrap_or('?').to_uppercase().to_string()
    };
    
    let class_value = class.unwrap_or_default();
    
    view! {
        <Avatar size=size shape=shape class=class_value>
            <AvatarFallback>
                <span class="radix-avatar-initials">
                    {initials()}
                </span>
            </AvatarFallback>
        </Avatar>
    }
}
