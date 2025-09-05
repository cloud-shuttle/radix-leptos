use leptos::*;
use leptos::prelude::*;

/// Image aspect ratio variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageAspectRatio {
    Square,
    Landscape,
    Portrait,
    Wide,
    UltraWide,
}

/// Image fit variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageFit {
    Cover,
    Contain,
    Fill,
    None,
    ScaleDown,
}

/// Image loading strategy
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageLoading {
    Lazy,
    Eager,
}

/// Merge CSS classes with proper spacing
fn merge_classes(classes: &[&str]) -> String {
    classes.iter().filter(|&&c| !c.is_empty()).map(|&s| s).collect::<Vec<&str>>().join(" ")
}

/// Root Image component
#[component]
pub fn Image(
    /// Image source URL
    src: String,
    /// Alt text for accessibility
    #[prop(optional)]
    alt: Option<String>,
    /// Image aspect ratio
    #[prop(optional, default = ImageAspectRatio::Landscape)]
    aspect_ratio: ImageAspectRatio,
    /// Image fit behavior
    #[prop(optional, default = ImageFit::Cover)]
    fit: ImageFit,
    /// Loading strategy
    #[prop(optional, default = ImageLoading::Lazy)]
    loading: ImageLoading,
    /// Whether the image is interactive (clickable)
    #[prop(optional, default = false)]
    _interactive: bool,
    /// Whether the image is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
) -> impl IntoView {
    let aspect_ratio_class = move || {
        match aspect_ratio {
            ImageAspectRatio::Square => "radix-image--aspect-square",
            ImageAspectRatio::Landscape => "radix-image--aspect-landscape",
            ImageAspectRatio::Portrait => "radix-image--aspect-portrait",
            ImageAspectRatio::Wide => "radix-image--aspect-wide",
            ImageAspectRatio::UltraWide => "radix-image--aspect-ultra-wide",
        }
    };
    
    let fit_class = move || {
        match fit {
            ImageFit::Cover => "radix-image--fit-cover",
            ImageFit::Contain => "radix-image--fit-contain",
            ImageFit::Fill => "radix-image--fit-fill",
            ImageFit::None => "radix-image--fit-none",
            ImageFit::ScaleDown => "radix-image--fit-scale-down",
        }
    };
    
    let loading_attr = move || {
        match loading {
            ImageLoading::Lazy => "lazy",
            ImageLoading::Eager => "eager",
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
    let alt_text = alt.unwrap_or_else(|| "Image".to_string());
    
    let mut base_classes = [
        "radix-image",
        &aspect_ratio_class(),
        &fit_class(),
        &class_value,
    ];
    
    if interactive && !disabled {
        base_classes.push("radix-image--interactive");
    }
    
    view! {
        <div
            class=merge_classes(&base_classes)
            on:click=handle_click
        >
            <img
                src=src
                alt=alt_text
                loading=loading_attr()
                class="radix-image-element"
            />
        </div>
    }
}

/// Responsive image component with multiple sources
#[component]
pub fn ResponsiveImage(
    /// Image sources for different screen sizes
    sources: Vec<(String, String)>, // (srcset, media_query)
    /// Default image source
    src: String,
    /// Alt text for accessibility
    #[prop(optional)]
    alt: Option<String>,
    /// Image aspect ratio
    #[prop(optional, default = ImageAspectRatio::Landscape)]
    aspect_ratio: ImageAspectRatio,
    /// Image fit behavior
    #[prop(optional, default = ImageFit::Cover)]
    fit: ImageFit,
    /// Loading strategy
    #[prop(optional, default = ImageLoading::Lazy)]
    loading: ImageLoading,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();
    let alt_text = alt.unwrap_or_else(|| "Responsive image".to_string());
    
    let aspect_ratio_class = move || {
        match aspect_ratio {
            ImageAspectRatio::Square => "radix-responsive-image--aspect-square",
            ImageAspectRatio::Landscape => "radix-responsive-image--aspect-landscape",
            ImageAspectRatio::Portrait => "radix-responsive-image--aspect-portrait",
            ImageAspectRatio::Wide => "radix-responsive-image--aspect-wide",
            ImageAspectRatio::UltraWide => "radix-responsive-image--aspect-ultra-wide",
        }
    };
    
    let fit_class = move || {
        match fit {
            ImageFit::Cover => "radix-responsive-image--fit-cover",
            ImageFit::Contain => "radix-responsive-image--fit-contain",
            ImageFit::Fill => "radix-responsive-image--fit-fill",
            ImageFit::None => "radix-responsive-image--fit-none",
            ImageFit::ScaleDown => "radix-responsive-image--fit-scale-down",
        }
    };
    
    let loading_attr = move || {
        match loading {
            ImageLoading::Lazy => "lazy",
            ImageLoading::Eager => "eager",
        }
    };
    
    view! {
        <div
            class=merge_classes(&[
                "radix-responsive-image",
                &aspect_ratio_class(),
                &fit_class(),
                &class_value,
            ])
        >
            <picture class="radix-responsive-image-picture">
                {move || {
                    sources.iter().map(|(srcset, media)| {
                        view! {
                            <source
                                srcset=srcset.clone()
                                media=media.clone()
                            />
                        }
                    }).collect::<Vec<_>>()
                }}
                <img
                    src=src
                    alt=alt_text
                    loading=loading_attr()
                    class="radix-responsive-image-element"
                />
            </picture>
        </div>
    }
}

/// Image with placeholder/skeleton loading
#[component]
pub fn ImageWithPlaceholder(
    /// Image source URL
    src: String,
    /// Alt text for accessibility
    #[prop(optional)]
    alt: Option<String>,
    /// Image aspect ratio
    #[prop(optional, default = ImageAspectRatio::Landscape)]
    aspect_ratio: ImageAspectRatio,
    /// Image fit behavior
    #[prop(optional, default = ImageFit::Cover)]
    fit: ImageFit,
    /// Loading strategy
    #[prop(optional, default = ImageLoading::Lazy)]
    loading: ImageLoading,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();
    let alt_text = alt.unwrap_or_else(|| "Image".to_string());
    
    let aspect_ratio_class = move || {
        match aspect_ratio {
            ImageAspectRatio::Square => "radix-image-placeholder--aspect-square",
            ImageAspectRatio::Landscape => "radix-image-placeholder--aspect-landscape",
            ImageAspectRatio::Portrait => "radix-image-placeholder--aspect-portrait",
            ImageAspectRatio::Wide => "radix-image-placeholder--aspect-wide",
            ImageAspectRatio::UltraWide => "radix-image-placeholder--aspect-ultra-wide",
        }
    };
    
    let fit_class = move || {
        match fit {
            ImageFit::Cover => "radix-image-placeholder--fit-cover",
            ImageFit::Contain => "radix-image-placeholder--fit-contain",
            ImageFit::Fill => "radix-image-placeholder--fit-fill",
            ImageFit::None => "radix-image-placeholder--fit-none",
            ImageFit::ScaleDown => "radix-image-placeholder--fit-scale-down",
        }
    };
    
    let loading_attr = move || {
        match loading {
            ImageLoading::Lazy => "lazy",
            ImageLoading::Eager => "eager",
        }
    };
    
    view! {
        <div
            class=merge_classes(&[
                "radix-image-placeholder",
                &aspect_ratio_class(),
                &fit_class(),
                &class_value,
            ])
        >
            <img
                src=src
                alt=alt_text
                loading=loading_attr()
                class="radix-image-placeholder-element"
            />
        </div>
    }
}
