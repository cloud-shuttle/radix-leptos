
/// Carousel size variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CarouselSize {
    Small,
    Medium,
    Large,
    FullWidth,
}

/// Carousel navigation variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CarouselNavigation {
    Dots,
    Arrows,
    Both,
    None,
}

/// Carousel autoplay variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CarouselAutoplay {
    None,
    Slow,
    Medium,
    Fast,
}

/// Merge CSS classes with proper spacing
fn merge_classes(classes: &[&str]) -> String {
    classes.iter().filter(|&&c| !c.is_empty()).map(|&s| s).collect::<Vec<&str>>().join(" ")
}

/// Root Carousel component
#[component]
pub fn Carousel(
    /// Carousel size
    #[prop(optional, default = CarouselSize::Medium)]
    size: CarouselSize,
    /// Carousel navigation
    #[prop(optional, default = CarouselNavigation::Both)]
    navigation: CarouselNavigation,
    /// Carousel autoplay
    #[prop(optional, default = CarouselAutoplay::None)]
    autoplay: CarouselAutoplay,
    /// Whether the carousel should loop
    #[prop(optional, default = true)]
    __loop_carousel: bool,
    /// Whether the carousel is interactive (clickable)
    #[prop(optional, default = false)]
    _interactive: bool,
    /// Whether the carousel is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Carousel content
    children: Children,
) -> impl IntoView {
    let (current_slide, setcurrent_slide) = signal(0);
    let (total_slides, set_total_slides) = signal(3); // Default to 3 slides
    let (is_playing, set_is_playing) = signal(false);

    let size_class = move || {
        match size {
            CarouselSize::Small => "radix-carousel--size-small",
            CarouselSize::Medium => "radix-carousel--size-medium",
            CarouselSize::Large => "radix-carousel--size-large",
            CarouselSize::FullWidth => "radix-carousel--size-full-width",
        }
    };

    let navigation_class = move || {
        match navigation {
            CarouselNavigation::Dots => "radix-carousel--navigation-dots",
            CarouselNavigation::Arrows => "radix-carousel--navigation-arrows",
            CarouselNavigation::Both => "radix-carousel--navigation-both",
            CarouselNavigation::None => "radix-carousel--navigation-none",
        }
    };

    let handle_click = move |e: web_sys::MouseEvent| {
        if !disabled && interactive {
            if let Some(callback) = on_click {
                callback.run(e);
            }
        }
    };

    let next_slide = move |_| {
        if !disabled {
            let current = current_slide.get();
            let total = total_slides.get();
            if current < total - 1 {
                setcurrent_slide.set(current + 1);
            } else if loop_carousel {
                setcurrent_slide.set(0);
            }
        }
    };

    let prev_slide = move |_| {
        if !disabled {
            let current = current_slide.get();
            let total = total_slides.get();
            if current > 0 {
                setcurrent_slide.set(current - 1);
            } else if loop_carousel {
                setcurrent_slide.set(total - 1);
            }
        }
    };

    let go_to_slide = move |index: usize| {
        if !disabled && index < total_slides.get() {
            setcurrent_slide.set(index);
        }
    };

    let toggle_autoplay = move |_| {
        if !disabled {
            set_is_playing.update(|playing| *playing = !*playing);
        }
    };

    let class_value = class.unwrap_or_default();

    let mut base_classes = [
        "radix-carousel",
        &size_class(),
        &navigation_class(),
        &class_value,
    ];

    if interactive && !disabled {
        base_classes.push("radix-carousel--interactive");
    }

    let children_vec = children();

    let show_arrows = move || navigation == CarouselNavigation::Arrows || navigation == CarouselNavigation::Both;
    let show_dots = move || navigation == CarouselNavigation::Dots || navigation == CarouselNavigation::Both;
    let show_autoplay = move || autoplay != CarouselAutoplay::None;

    view! {
        <div
            class=merge_classes(&base_classes)
            data-radix-carousel=""
            on:click=handle_click
        >
            <div class="carousel-container radix-carousel-container">
                <div 
                    class="radix-carousel-track"
                    style=move || format!("transform: translateX(-{}%)", current_slide.get() * 100)
                >
                    {children_vec}
                </div>
            </div>

                {move || {
                    (0..total_slides.get()).map(|index| {
                        let is_active = move || current_slide.get() == index;
                        let index_clone = index;
                        view! {
                            <button
                                class=move || {
                                    if is_active() {
                                        "carousel-indicator radix-carousel-dot radix-carousel-dot--active"
                                }
                                on:click=move |_| go_to_slide(index_clone)
                                disabled=disabled
                            >
                            </button>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>

                </button>
            </div>
        </div>
    }
}

/// Carousel slide component
#[component]
pub fn CarouselSlide(
    /// Slide content
    children: Children,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();

    view! {
        <div class=format!("carousel-slide radix-carousel-slide {}", class_value)>
            {children()}
        </div>
    }
}

/// Carousel with custom navigation component
#[component]
pub fn CarouselWithCustomNavigation(
    /// Carousel size
    #[prop(optional, default = CarouselSize::Medium)]
    size: CarouselSize,
    /// Whether the carousel should loop
    #[prop(optional, default = true)]
    __loop_carousel: bool,
    /// Whether the carousel is interactive (clickable)
    #[prop(optional, default = false)]
    _interactive: bool,
    /// Whether the carousel is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Carousel content
    children: Children,
) -> impl IntoView {
    let (current_slide, setcurrent_slide) = signal(0);
    let (total_slides, set_total_slides) = signal(3); // Default to 3 slides

    let size_class = move || {
        match size {
            CarouselSize::Small => "radix-carousel-custom--size-small",
            CarouselSize::Medium => "radix-carousel-custom--size-medium",
            CarouselSize::Large => "radix-carousel-custom--size-large",
            CarouselSize::FullWidth => "radix-carousel-custom--size-full-width",
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
        "radix-carousel-custom",
        &size_class(),
        &class_value,
    ];

    if interactive && !disabled {
        base_classes.push("radix-carousel-custom--interactive");
    }

    let children_vec = children();

    view! {
        <div
            class=merge_classes(&base_classes)
            on:click=handle_click
        >
            <div class="radix-carousel-custom-container">
                <div 
                    class="radix-carousel-custom-track"
                    style=move || format!("transform: translateX(-{}%)", current_slide.get() * 100)
                >
                    {children_vec}
                </div>
            </div>
        </div>
    }
}
