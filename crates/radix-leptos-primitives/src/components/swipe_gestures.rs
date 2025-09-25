
/// Swipeable card component that responds to mouse gestures
#[component]
pub fn SwipeableCard(
    /// Card content
    #[prop(into)]
    children: Children,
    /// CSS classes to apply
    #[prop(optional)]
    class: Option<String>,
    /// Callback when swiped left
    #[prop(optional)]
    on_swipe_left: Option<Callback<()>>,
    /// Callback when swiped right
    #[prop(optional)]
    on_swipe_right: Option<Callback<()>>,
    /// Minimum swipe distance to trigger action (in pixels)
    #[prop(optional, default = 50.0)]
    min_swipe_distance: f64,
) -> impl IntoView {
    let (start_x, set_start_x) = signal(0.0);
    let (current_x, setcurrent_x) = signal(0.0);
    let (is_swiping, set_is_swiping) = signal(false);
    let (translate_x, set_translate_x) = signal(0.0);

    // Handle mouse events for desktop compatibility
    let handle_mouse_down = move |event: web_sys::MouseEvent| {
        set_start_x.set(event.client_x() as f64);
use crate::utils::merge_optional_classes;
        setcurrent_x.set(event.client_x() as f64);
        set_is_swiping.set(true);
        set_translate_x.set(0.0);
    };

    let handle_mouse_move = move |event: web_sys::MouseEvent| {
        if is_swiping.get() {
            let new_x = event.client_x() as f64;
            setcurrent_x.set(new_x);
            
            let delta_x: f64 = new_x - start_x.get();
            set_translate_x.set(delta_x * 0.3);
        }
    };

    let handle_mouse_up = move |_| {
        if is_swiping.get() {
            let delta_x: f64 = current_x.get() - start_x.get();
            
            if delta_x.abs() >= min_swipe_distance {
                if delta_x > 0.0 {
                    // Swiped right
                    if let Some(callback) = on_swipe_right {
                        callback.run(());
                    }
                }
            }
            
            // Reset state
            set_is_swiping.set(false);
            set_translate_x.set(0.0);
        }
    };

    view! {
        <div
            class={format!("swipeable-card {}", class.unwrap_or_default())}
            style={format!(
                "transform: translateX({}px); transition: {};",
                translate_x.get(),
            on:mousedown=handle_mouse_down
            on:mousemove=handle_mouse_move
            on:mouseup=handle_mouse_up
        >
            {children()}
        </div>
    }
}

/// Swipeable list item component
#[component]
pub fn SwipeableListItem(
    /// Item content
    #[prop(into)]
    children: Children,
    /// CSS classes to apply
    #[prop(optional)]
    class: Option<String>,
    /// Callback when swiped left (usually to delete)
    #[prop(optional)]
    on_swipe_left: Option<Callback<()>>,
    /// Callback when swiped right (usually to edit)
    #[prop(optional)]
    on_swipe_right: Option<Callback<()>>,
    /// Minimum swipe distance to trigger action (in pixels)
    #[prop(optional, default = 80.0)]
    min_swipe_distance: f64,
) -> impl IntoView {
    let (start_x, set_start_x) = signal(0.0);
    let (current_x, setcurrent_x) = signal(0.0);
    let (is_swiping, set_is_swiping) = signal(false);
    let (translate_x, set_translate_x) = signal(0.0);

    // Handle mouse events for desktop compatibility
    let handle_mouse_down = move |event: web_sys::MouseEvent| {
        set_start_x.set(event.client_x() as f64);
use crate::utils::merge_optional_classes;
        setcurrent_x.set(event.client_x() as f64);
        set_is_swiping.set(true);
        set_translate_x.set(0.0);
    };

    let handle_mouse_move = move |event: web_sys::MouseEvent| {
        if is_swiping.get() {
            let new_x = event.client_x() as f64;
            setcurrent_x.set(new_x);
            
            let delta_x: f64 = new_x - start_x.get();
            set_translate_x.set(delta_x * 0.5);
        }
    };

    let handle_mouse_up = move |_| {
        if is_swiping.get() {
            let delta_x: f64 = current_x.get() - start_x.get();
            
            if delta_x.abs() >= min_swipe_distance {
                if delta_x > 0.0 {
                    // Swiped right
                    if let Some(callback) = on_swipe_right {
                        callback.run(());
                    }
                }
            }
            
            // Reset state
            set_is_swiping.set(false);
            set_translate_x.set(0.0);
        }
    };

    view! {
        <div
            class={format!("swipeable-list-item {}", class.unwrap_or_default())}
            style={format!(
                "transform: translateX({}px); transition: {};",
                translate_x.get(),
            on:mousedown=handle_mouse_down
            on:mousemove=handle_mouse_move
            on:mouseup=handle_mouse_up
        >
            {children()}
        </div>
    }
}
