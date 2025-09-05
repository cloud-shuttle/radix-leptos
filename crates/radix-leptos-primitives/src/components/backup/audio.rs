
/// Audio size variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioSize {
    Small,
    Medium,
    Large,
    FullWidth,
}

/// Audio controls variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioControls {
    Minimal,
    Standard,
    Full,
    None,
}

/// Audio theme variant
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioTheme {
    Default,
    Dark,
    Light,
    Custom,
}

/// Merge CSS classes with proper spacing
fn merge_classes(classes: &[&str]) -> String {
    classes.iter().filter(|&&c| !c.is_empty()).map(|&s| s).collect::<Vec<&str>>().join(" ")
}

/// Root Audio component
#[component]
pub fn Audio(
    /// Audio source URL
    src: String,
    /// Audio title
    #[prop(optional)]
    title: Option<String>,
    /// Audio artist
    #[prop(optional)]
    artist: Option<String>,
    /// Audio size
    #[prop(optional, default = AudioSize::Medium)]
    size: AudioSize,
    /// Audio controls
    #[prop(optional, default = AudioControls::Standard)]
    controls: AudioControls,
    /// Audio theme
    #[prop(optional, default = AudioTheme::Default)]
    theme: AudioTheme,
    /// Whether the audio should autoplay
    #[prop(optional, default = false)]
    __autoplay: bool,
    /// Whether the audio should loop
    #[prop(optional, default = false)]
    __loop_audio: bool,
    /// Whether the audio should be muted
    #[prop(optional, default = false)]
    __muted: bool,
    /// Whether the audio should show controls
    #[prop(optional, default = true)]
    __show_controls: bool,
    /// Whether the audio is interactive (clickable)
    #[prop(optional, default = false)]
    _interactive: bool,
    /// Whether the audio is disabled
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
            AudioSize::Small => "radix-audio--size-small",
            AudioSize::Medium => "radix-audio--size-medium",
            AudioSize::Large => "radix-audio--size-large",
            AudioSize::FullWidth => "radix-audio--size-full-width",
        }
    };

    let controls_class = move || {
        match controls {
            AudioControls::Minimal => "radix-audio--controls-minimal",
            AudioControls::Standard => "radix-audio--controls-standard",
            AudioControls::Full => "radix-audio--controls-full",
            AudioControls::None => "radix-audio--controls-none",
        }
    };

    let theme_class = move || {
        match theme {
            AudioTheme::Default => "radix-audio--theme-default",
            AudioTheme::Dark => "radix-audio--theme-dark",
            AudioTheme::Light => "radix-audio--theme-light",
            AudioTheme::Custom => "radix-audio--theme-custom",
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
        "radix-audio",
        &size_class(),
        &controls_class(),
        &theme_class(),
        &class_value,
    ];

    if interactive && !disabled {
        base_classes.push("radix-audio--interactive");
    }

    view! {
        <div
            class=merge_classes(&base_classes)
            on:click=handle_click
        >
            <audio
                src=src
                autoplay=autoplay
                loop=loop_audio
                muted=muted
                controls=show_controls
                class="radix-audio-element"
            />
        </div>
    }
}

/// Audio with custom controls component
#[component]
pub fn AudioWithCustomControls(
    /// Audio source URL
    src: String,
    /// Audio title
    #[prop(optional)]
    title: Option<String>,
    /// Audio artist
    #[prop(optional)]
    artist: Option<String>,
    /// Audio size
    #[prop(optional, default = AudioSize::Medium)]
    size: AudioSize,
    /// Audio theme
    #[prop(optional, default = AudioTheme::Default)]
    theme: AudioTheme,
    /// Whether the audio should autoplay
    #[prop(optional, default = false)]
    __autoplay: bool,
    /// Whether the audio should loop
    #[prop(optional, default = false)]
    __loop_audio: bool,
    /// Whether the audio should be muted
    #[prop(optional, default = false)]
    __muted: bool,
    /// Whether the audio is interactive (clickable)
    #[prop(optional, default = false)]
    _interactive: bool,
    /// Whether the audio is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Custom controls content
    children: Children,
) -> impl IntoView {
    let size_class = move || {
        match size {
            AudioSize::Small => "radix-audio-custom--size-small",
            AudioSize::Medium => "radix-audio-custom--size-medium",
            AudioSize::Large => "radix-audio-custom--size-large",
            AudioSize::FullWidth => "radix-audio-custom--size-full-width",
        }
    };

    let theme_class = move || {
        match theme {
            AudioTheme::Default => "radix-audio-custom--theme-default",
            AudioTheme::Dark => "radix-audio-custom--theme-dark",
            AudioTheme::Light => "radix-audio-custom--theme-light",
            AudioTheme::Custom => "radix-audio-custom--theme-custom",
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
        "radix-audio-custom",
        &size_class(),
        &theme_class(),
        &class_value,
    ];

    if interactive && !disabled {
        base_classes.push("radix-audio-custom--interactive");
    }

    view! {
        <div
            class=merge_classes(&base_classes)
            on:click=handle_click
        >
            <audio
                src=src
                autoplay=autoplay
                loop=loop_audio
                muted=muted
                controls=false
                class="radix-audio-custom-element"
            />
            <div class="radix-audio-custom-controls">
                {children()}
            </div>
        </div>
    }
}

/// Audio player component with built-in controls and metadata
#[component]
pub fn AudioPlayer(
    /// Audio source URL
    src: String,
    /// Audio title
    #[prop(optional)]
    title: Option<String>,
    /// Audio artist
    #[prop(optional)]
    artist: Option<String>,
    /// Audio album
    #[prop(optional)]
    album: Option<String>,
    /// Audio duration in seconds
    #[prop(optional)]
    duration: Option<u32>,
    /// Audio size
    #[prop(optional, default = AudioSize::Medium)]
    size: AudioSize,
    /// Audio theme
    #[prop(optional, default = AudioTheme::Default)]
    theme: AudioTheme,
    /// Whether the audio should autoplay
    #[prop(optional, default = false)]
    __autoplay: bool,
    /// Whether the audio should loop
    #[prop(optional, default = false)]
    __loop_audio: bool,
    /// Whether the audio should be muted
    #[prop(optional, default = false)]
    __muted: bool,
    /// Whether the audio is interactive (clickable)
    #[prop(optional, default = false)]
    _interactive: bool,
    /// Whether the audio is disabled
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
            AudioSize::Small => "radix-audio-player--size-small",
            AudioSize::Medium => "radix-audio-player--size-medium",
            AudioSize::Large => "radix-audio-player--size-large",
            AudioSize::FullWidth => "radix-audio-player--size-full-width",
        }
    };

    let theme_class = move || {
        match theme {
            AudioTheme::Default => "radix-audio-player--theme-default",
            AudioTheme::Dark => "radix-audio-player--theme-dark",
            AudioTheme::Light => "radix-audio-player--theme-light",
            AudioTheme::Custom => "radix-audio-player--theme-custom",
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
        "radix-audio-player",
        &size_class(),
        &theme_class(),
        &class_value,
    ];

    if interactive && !disabled {
        base_classes.push("radix-audio-player--interactive");
    }

    let title_text = title.unwrap_or_else(|| "Unknown Track".to_string());
    let artist_text = artist.unwrap_or_else(|| "Unknown Artist".to_string());
    let album_text = album.unwrap_or_else(|| "Unknown Album".to_string());

    let format_duration = move |seconds: u32| {
        let minutes = seconds / 60;
        let remaining_seconds = seconds % 60;
        format!("{}:{:02}", minutes, remaining_seconds)
    };

    let duration_text = duration.map(|d| format_duration(d)).unwrap_or_else(|| "0:00".to_string());

    view! {
        <div
            class=merge_classes(&base_classes)
            on:click=handle_click
        >
            <audio
                src=src
                autoplay=autoplay
                loop=loop_audio
                muted=muted
                controls=true
                class="radix-audio-player-element"
            />
            <div class="radix-audio-player-info">
                <div class="radix-audio-player-header">
                    <h3 class="radix-audio-player-title">{title_text}</h3>
                    <p class="radix-audio-player-artist">{artist_text}</p>
                    <p class="radix-audio-player-album">{album_text}</p>
                </div>
                <div class="radix-audio-player-duration">
                    <span class="radix-audio-player-duration-text">{duration_text}</span>
                </div>
            </div>
        </div>
    }
}
