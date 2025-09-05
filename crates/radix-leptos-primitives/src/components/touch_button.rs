
/// Touch-optimized button component for mobile devices
#[component]
pub fn TouchButton(
    /// Button text content
    #[prop(into)]
    children: Children,
    /// Whether the button is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// Button type
    #[prop(optional, default = "button")]
    button_type: &'static str,
    /// CSS classes to apply
    #[prop(optional)]
    class: Option<String>,
    /// Click event handler
    #[prop(optional)]
    on_click: Option<Callback<()>>,
    /// Touch start event handler
    #[prop(optional)]
    on_touch_start: Option<Callback<()>>,
    /// Touch end event handler
    #[prop(optional)]
    on_touch_end: Option<Callback<()>>,
) -> impl IntoView {
    let (is_pressed, set_is_pressed) = signal(false);
    let (is_touching, set_is_touching) = signal(false);

    // Handle click events
    let handle_click = move |_| {
        if !disabled {
            if let Some(callback) = on_click {
                callback.run(());
            }
        }
    };

    // Handle touch start
    let handle_touch_start = move |_| {
        if !disabled {
            set_is_touching.set(true);
            set_is_pressed.set(true);
            if let Some(callback) = on_touch_start {
                callback.run(());
            }
        }
    };

    // Handle touch end
    let handle_touch_end = move |_| {
        if !disabled {
            set_is_touching.set(false);
            set_is_pressed.set(false);
            if let Some(callback) = on_touch_end {
                callback.run(());
            }
        }
    };

    // Handle mouse events for desktop compatibility
    let handle_mouse_down = move |_| {
        if !disabled {
            set_is_pressed.set(true);
        }
    };

    let handle_mouse_up = move |_| {
        if !disabled {
            set_is_pressed.set(false);
        }
    };

    let handle_mouse_leave = move |_| {
        if !disabled {
            set_is_pressed.set(false);
        }
    };

    view! {
        <button
            type=button_type
            class={format!(
                "touch-button {} {} {} {}",
            disabled=disabled
            on:click=handle_click
            on:touchstart=handle_touch_start
            on:touchend=handle_touch_end
            on:mousedown=handle_mouse_down
            on:mouseup=handle_mouse_up
            on:mouseleave=handle_mouse_leave
        >
            {children()}
        </button>
    }
}

/// Touch-optimized icon button for mobile devices
#[component]
pub fn TouchIconButton(
    /// Icon content (can be text, SVG, or any content)
    #[prop(into)]
    icon: Children,
    /// Whether the button is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// Button type
    #[prop(optional, default = "button")]
    button_type: &'static str,
    /// CSS classes to apply
    #[prop(optional)]
    class: Option<String>,
    /// Click event handler
    #[prop(optional)]
    on_click: Option<Callback<()>>,
    /// Touch start event handler
    #[prop(optional)]
    on_touch_start: Option<Callback<()>>,
    /// Touch end event handler
    #[prop(optional)]
    on_touch_end: Option<Callback<()>>,
) -> impl IntoView {
    let (is_pressed, set_is_pressed) = signal(false);
    let (is_touching, set_is_touching) = signal(false);

    // Handle click events
    let handle_click = move |_| {
        if !disabled {
            if let Some(callback) = on_click {
                callback.run(());
            }
        }
    };

    // Handle touch start
    let handle_touch_start = move |_| {
        if !disabled {
            set_is_touching.set(true);
            set_is_pressed.set(true);
            if let Some(callback) = on_touch_start {
                callback.run(());
            }
        }
    };

    // Handle touch end
    let handle_touch_end = move |_| {
        if !disabled {
            set_is_touching.set(false);
            set_is_pressed.set(false);
            if let Some(callback) = on_touch_end {
                callback.run(());
            }
        }
    };

    // Handle mouse events for desktop compatibility
    let handle_mouse_down = move |_| {
        if !disabled {
            set_is_pressed.set(true);
        }
    };

    let handle_mouse_up = move |_| {
        if !disabled {
            set_is_pressed.set(false);
        }
    };

    let handle_mouse_leave = move |_| {
        if !disabled {
            set_is_pressed.set(false);
        }
    };

    view! {
        <button
            type=button_type
            class={format!(
                "touch-icon-button {} {} {} {}",
            disabled=disabled
            on:click=handle_click
            on:touchstart=handle_touch_start
            on:touchend=handle_touch_end
            on:mousedown=handle_mouse_down
            on:mouseup=handle_mouse_up
            on:mouseleave=handle_mouse_leave
        >
            {icon()}
        </button>
    }
}

/// Floating Action Button optimized for mobile
#[component]
pub fn FloatingActionButton(
    /// Button content (usually an icon)
    #[prop(into)]
    children: Children,
    /// Whether the button is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// Button type
    #[prop(optional, default = "button")]
    button_type: &'static str,
    /// CSS classes to apply
    #[prop(optional)]
    class: Option<String>,
    /// Click event handler
    #[prop(optional)]
    on_click: Option<Callback<()>>,
    /// Touch start event handler
    #[prop(optional)]
    on_touch_start: Option<Callback<()>>,
    /// Touch end event handler
    #[prop(optional)]
    on_touch_end: Option<Callback<()>>,
) -> impl IntoView {
    let (is_pressed, set_is_pressed) = signal(false);
    let (is_touching, set_is_touching) = signal(false);

    // Handle click events
    let handle_click = move |_| {
        if !disabled {
            if let Some(callback) = on_click {
                callback.run(());
            }
        }
    };

    // Handle touch start
    let handle_touch_start = move |_| {
        if !disabled {
            set_is_touching.set(true);
            set_is_pressed.set(true);
            if let Some(callback) = on_touch_start {
                callback.run(());
            }
        }
    };

    // Handle touch end
    let handle_touch_end = move |_| {
        if !disabled {
            set_is_touching.set(false);
            set_is_pressed.set(false);
            if let Some(callback) = on_touch_end {
                callback.run(());
            }
        }
    };

    // Handle mouse events for desktop compatibility
    let handle_mouse_down = move |_| {
        if !disabled {
            set_is_pressed.set(true);
        }
    };

    let handle_mouse_up = move |_| {
        if !disabled {
            set_is_pressed.set(false);
        }
    };

    let handle_mouse_leave = move |_| {
        if !disabled {
            set_is_pressed.set(false);
        }
    };

    view! {
        <button
            type=button_type
            class={format!(
                "floating-action-button {} {} {} {}",
            disabled=disabled
            on:click=handle_click
            on:touchstart=handle_touch_start
            on:touchend=handle_touch_end
            on:mousedown=handle_mouse_down
            on:mouseup=handle_mouse_up
            on:mouseleave=handle_mouse_leave
        >
            {children()}
        </button>
    }
}
