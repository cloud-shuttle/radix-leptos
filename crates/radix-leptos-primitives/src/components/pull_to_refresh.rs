use wasm_bindgen::JsCast;

/// Pull-to-refresh component for mobile devices
#[component]
pub fn PullToRefresh(
    /// Content to wrap with pull-to-refresh
    #[prop(into)]
    children: Children,
    /// CSS classes to apply
    #[prop(optional)]
    class: Option<String>,
    /// Callback when refresh is triggered
    #[prop(optional)]
    on_refresh: Option<Callback<()>>,
    /// Threshold distance to trigger refresh (in pixels)
    #[prop(optional, default = 80.0)]
    refresh_threshold: f64,
    /// Whether refresh is currently in progress
    #[prop(optional)]
    is_refreshing: Option<Signal<bool>>,
) -> impl IntoView {
    let (start_y, set_start_y) = signal(0.0);
    let (current_y, setcurrent_y) = signal(0.0);
    let (is_pulling, set_is_pulling) = signal(false);
    let (translate_y, set_translate_y) = signal(0.0);

    // Handle mouse events for desktop compatibility
    let handle_mouse_down = move |event: web_sys::MouseEvent| {
        // Only start pulling if we're at the top of the scrollable area
        if let Some(element) = event.target() {
            if let Ok(html_element) = element.dyn_into::<web_sys::Element>() {
                if html_element.scroll_top() == 0 {
                    set_start_y.set(event.client_y() as f64);
                    setcurrent_y.set(event.client_y() as f64);
                    set_is_pulling.set(true);
                    set_translate_y.set(0.0);
                }
            }
        }
    };

    let handle_mouse_move = move |event: web_sys::MouseEvent| {
        if is_pulling.get() {
            let new_y = event.client_y() as f64;
            setcurrent_y.set(new_y);
            
            let delta_y = new_y - start_y.get();
            if delta_y > 0.0 {
                // Only allow downward movement
                set_translate_y.set(delta_y * 0.5); // Add resistance
            }
        }
    };

    let handle_mouse_up = move |_| {
        if is_pulling.get() {
            let delta_y = current_y.get() - start_y.get();
            
            if delta_y >= refresh_threshold {
                // Trigger refresh
                if let Some(callback) = on_refresh {
                    callback.run(());
                }
            }
            
            // Reset state
            set_is_pulling.set(false);
            set_translate_y.set(0.0);
        }
    };

    // Handle wheel events for scroll detection
    let handle_wheel = move |event: web_sys::WheelEvent| {
        if let Some(element) = event.target() {
            if let Ok(html_element) = element.dyn_into::<web_sys::Element>() {
                if html_element.scroll_top() == 0 && event.delta_y() < 0.0 {
                    // At top and scrolling up, allow pull gesture
                    set_is_pulling.set(true);
                }
            }
        }
    };

    let is_refreshing_state = is_refreshing.as_ref().map(|r| r.get()).unwrap_or(false);

    view! {
        <div
            class={format!("pull-to-refresh {}", class.unwrap_or_default())}
            style={format!(
                "transform: translateY({}px); transition: {};",
                translate_y.get(),
            on:mousedown=handle_mouse_down
            on:mousemove=handle_mouse_move
            on:mouseup=handle_mouse_up
            on:wheel=handle_wheel
        >
            // Refresh indicator
            <div
                class="refresh-indicator"
                style={format!(
                    "opacity: {}; transform: scale({});",
                )}
            >
                {if is_refreshing_state {
                    "🔄 Refreshing..."
            </div>
            
            // Main content
            <div class="refresh-content">
                {children()}
            </div>
        </div>
    }
}

/// Simple refresh button component
#[component]
pub fn RefreshButton(
    /// Button content
    #[prop(into)]
    children: Children,
    /// CSS classes to apply
    #[prop(optional)]
    class: Option<String>,
    /// Click event handler
    #[prop(optional)]
    on_click: Option<Callback<()>>,
    /// Whether refresh is currently in progress
    #[prop(optional)]
    is_refreshing: Option<Signal<bool>>,
) -> impl IntoView {
    let (is_pressed, set_is_pressed) = signal(false);

    // Handle click events
    let handle_click = move |_| {
        if let Some(callback) = on_click {
            callback.run(());
        }
    };

    // Handle mouse events for visual feedback
    let handle_mouse_down = move |_| {
        set_is_pressed.set(true);
use crate::utils::{merge_optional_classes, generate_id};
    };

    let handle_mouse_up = move |_| {
        set_is_pressed.set(false);
    };

    let handle_mouse_leave = move |_| {
        set_is_pressed.set(false);
    };

    let is_refreshing_state = is_refreshing.as_ref().map(|r| r.get()).unwrap_or(false);
    
    view! {
        <button
            class={format!(
                "refresh-button {} {} {}",
                class.unwrap_or_default()
            )}
            disabled=is_refreshing_state
            on:click=handle_click
            on:mousedown=handle_mouse_down
            on:mouseup=handle_mouse_up
            on:mouseleave=handle_mouse_leave
        >
            {children()}
        </button>
    }
}
