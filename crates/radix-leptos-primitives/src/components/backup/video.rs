use leptos::*;
use leptos::prelude::*;

/// Video size variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum VideoSize {
    Small,
    Medium,
    Large,
    FullWidth,
}

/// Video controls variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum VideoControls {
    Minimal,
    Standard,
    Full,
    None,
}

/// Video aspect ratio variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum VideoAspectRatio {
    Square,
    Landscape,
    Portrait,
    Wide,
    UltraWide,
}

/// Merge CSS classes with proper spacing
fn merge_classes(classes: &[&str]) -> String {
    classes.iter().filter(|&&c| !c.is_empty()).map(|&s| s).collect::<Vec<&str>>().join(" ")
}

/// Root Video component
#[component]
pub fn Video(
    /// Video source URL
    src: String,
    /// Video poster image URL
    #[prop(optional)]
    poster: Option<String>,
    /// Video size
    #[prop(optional, default = VideoSize::Medium)]
    size: VideoSize,
    /// Video controls
    #[prop(optional, default = VideoControls::Standard)]
    controls: VideoControls,
    /// Video aspect ratio
    #[prop(optional, default = VideoAspectRatio::Landscape)]
    aspect_ratio: VideoAspectRatio,
    /// Whether the video should autoplay
    #[prop(optional, default = false)]
    _autoplay: bool,
    /// Whether the video should loop
    #[prop(optional, default = false)]
    _loop_video: bool,
    /// Whether the video should be muted
    #[prop(optional, default = false)]
    _muted: bool,
    /// Whether the video should show controls
    #[prop(optional, default = true)]
    _show_controls: bool,
    /// Whether the video is interactive (clickable)
    #[prop(optional, default = false)]
    _interactive: bool,
    /// Whether the video is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
) -> impl IntoView {
    let size_class = move || {
        match size {
            VideoSize::Small => "radix-video--size-small",
            VideoSize::Medium => "radix-video--size-medium",
            VideoSize::Large => "radix-video--size-large",
            VideoSize::FullWidth => "radix-video--size-full-width",
        }
    };
    
    let controls_class = move || {
        match controls {
            VideoControls::Minimal => "radix-video--controls-minimal",
            VideoControls::Standard => "radix-video--controls-standard",
            VideoControls::Full => "radix-video--controls-full",
            VideoControls::None => "radix-video--controls-none",
        }
    };
    
    let aspect_ratio_class = move || {
        match aspect_ratio {
            VideoAspectRatio::Square => "radix-video--aspect-square",
            VideoAspectRatio::Landscape => "radix-video--aspect-landscape",
            VideoAspectRatio::Portrait => "radix-video--aspect-portrait",
            VideoAspectRatio::Wide => "radix-video--aspect-wide",
            VideoAspectRatio::UltraWide => "radix-video--aspect-ultra-wide",
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
    
    let mut base_classes = [
        "radix-video",
        &size_class(),
        &controls_class(),
        &aspect_ratio_class(),
        &class_value,
    ];
    
    if interactive && !disabled {
        base_classes.push("radix-video--interactive");
    }
    
    view! {
        <div
            class=merge_classes(&base_classes)
            on:click=handle_click
        >
            <video
                src=src
                poster=poster
                autoplay=autoplay
                loop=loop_video
                muted=muted
                controls=show_controls
                class="radix-video-element"
            />
        </div>
    }
}

/// Video with custom controls
#[component]
pub fn VideoWithCustomControls(
    /// Video source URL
    src: String,
    /// Video poster image URL
    #[prop(optional)]
    poster: Option<String>,
    /// Video size
    #[prop(optional, default = VideoSize::Medium)]
    size: VideoSize,
    /// Video aspect ratio
    #[prop(optional, default = VideoAspectRatio::Landscape)]
    aspect_ratio: VideoAspectRatio,
    /// Whether the video should autoplay
    #[prop(optional, default = false)]
    _autoplay: bool,
    /// Whether the video should loop
    #[prop(optional, default = false)]
    _loop_video: bool,
    /// Whether the video should be muted
    #[prop(optional, default = false)]
    _muted: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child content (custom controls)
    children: Children,
) -> impl IntoView {
    let size_class = move || {
        match size {
            VideoSize::Small => "radix-video-custom--size-small",
            VideoSize::Medium => "radix-video-custom--size-medium",
            VideoSize::Large => "radix-video-custom--size-large",
            VideoSize::FullWidth => "radix-video-custom--size-full-width",
        }
    };
    
    let aspect_ratio_class = move || {
        match aspect_ratio {
            VideoAspectRatio::Square => "radix-video-custom--aspect-square",
            VideoAspectRatio::Landscape => "radix-video-custom--aspect-landscape",
            VideoAspectRatio::Portrait => "radix-video-custom--aspect-portrait",
            VideoAspectRatio::Wide => "radix-video-custom--aspect-wide",
            VideoAspectRatio::UltraWide => "radix-video-custom--aspect-ultra-wide",
        }
    };
    
    let class_value = class.unwrap_or_default();
    let children_view = children();
    
    view! {
        <div
            class=merge_classes(&[
                "radix-video-custom",
                &size_class(),
                &aspect_ratio_class(),
                &class_value,
            ])
        >
            <video
                src=src
                poster=poster
                autoplay=autoplay
                loop=loop_video
                muted=muted
                class="radix-video-custom-element"
            />
            <div class="radix-video-custom-controls">
                {children_view}
            </div>
        </div>
    }
}

/// Video player with built-in controls
#[component]
pub fn VideoPlayer(
    /// Video source URL
    src: String,
    /// Video poster image URL
    #[prop(optional)]
    poster: Option<String>,
    /// Video title
    #[prop(optional)]
    title: Option<String>,
    /// Video description
    #[prop(optional)]
    description: Option<String>,
    /// Video size
    #[prop(optional, default = VideoSize::Medium)]
    size: VideoSize,
    /// Video aspect ratio
    #[prop(optional, default = VideoAspectRatio::Landscape)]
    aspect_ratio: VideoAspectRatio,
    /// Whether the video should autoplay
    #[prop(optional, default = false)]
    _autoplay: bool,
    /// Whether the video should loop
    #[prop(optional, default = false)]
    _loop_video: bool,
    /// Whether the video should be muted
    #[prop(optional, default = false)]
    _muted: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let size_class = move || {
        match size {
            VideoSize::Small => "radix-video-player--size-small",
            VideoSize::Medium => "radix-video-player--size-medium",
            VideoSize::Large => "radix-video-player--size-large",
            VideoSize::FullWidth => "radix-video-player--size-full-width",
        }
    };
    
    let aspect_ratio_class = move || {
        match aspect_ratio {
            VideoAspectRatio::Square => "radix-video-player--aspect-square",
            VideoAspectRatio::Landscape => "radix-video-player--aspect-landscape",
            VideoAspectRatio::Portrait => "radix-video-player--aspect-portrait",
            VideoAspectRatio::Wide => "radix-video-player--aspect-wide",
            VideoAspectRatio::UltraWide => "radix-video-player--aspect-ultra-wide",
        }
    };
    
    let class_value = class.unwrap_or_default();
    let title_text = title.unwrap_or_default();
    let description_text = description.unwrap_or_default();
    
    view! {
        <div
            class=merge_classes(&[
                "radix-video-player",
                &size_class(),
                &aspect_ratio_class(),
                &class_value,
            ])
        >
            <video
                src=src
                poster=poster
                autoplay=autoplay
                loop=loop_video
                muted=muted
                controls=true
                class="radix-video-player-element"
            />
            
            <div class="radix-video-player-overlay">
                <div class="radix-video-player-header">
                    <h3 class="radix-video-player-title">{title_text}</h3>
                    <p class="radix-video-player-description">{description_text}</p>
                </div>
            </div>
        </div>
    }
}
