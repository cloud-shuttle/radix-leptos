use leptos::*;
use leptos::prelude::*;

/// Slider orientation enum
#[derive(Clone, Debug, PartialEq)]
pub enum SliderOrientation {
    Horizontal,
    Vertical,
}

impl SliderOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            SliderOrientation::Horizontal => "horizontal",
            SliderOrientation::Vertical => "vertical",
        }
    }
}

/// Slider mark structure
#[derive(Clone, Debug, PartialEq)]
pub struct SliderMark {
    pub value: f64,
    pub label: Option<String>,
}

impl SliderMark {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            label: None,
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }
}

/// Slider context for state management
#[derive(Clone)]
pub struct SliderContext {
    pub current_value: Signal<f64>,
    pub set_current_value: WriteSignal<f64>,
    pub is_dragging: Signal<bool>,
    pub set_is_dragging: WriteSignal<bool>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub orientation: SliderOrientation,
    pub _disabled: bool,
    pub on_change: Option<Callback<f64>>,
    pub on_change_commit: Option<Callback<f64>>,
    pub slider_id: String,
}

/// RangeSlider context for state management
#[derive(Clone)]
pub struct RangeSliderContext {
    pub current_values: Signal<(f64, f64)>,
    pub set_current_values: WriteSignal<(f64, f64)>,
    pub is_dragging: Signal<bool>,
    pub set_is_dragging: WriteSignal<bool>,
    pub active_thumb: Signal<usize>,
    pub set_active_thumb: WriteSignal<usize>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub orientation: SliderOrientation,
    pub _disabled: bool,
    pub on_change: Option<Callback<(f64, f64)>>,
    pub on_change_commit: Option<Callback<(f64, f64)>>,
    pub range_slider_id: String,
}

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

/// Clamp a value between min and max
fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

/// Round a value to the nearest step
fn round_to_step(value: f64, step: f64) -> f64 {
    (value / step).round() * step
}

/// Main Slider component with single value
#[component]
pub fn Slider(
    /// Current slider value
    #[prop(optional)]
    value: Option<f64>,
    /// Minimum value
    #[prop(optional, default = 0.0)]
    min: f64,
    /// Maximum value
    #[prop(optional, default = 100.0)]
    max: f64,
    /// Step increment
    #[prop(optional, default = 1.0)]
    step: f64,
    /// Slider orientation
    #[prop(optional, default = SliderOrientation::Horizontal)]
    orientation: SliderOrientation,
    /// Marks to display on the slider
    #[prop(optional)]
    _marks: Option<Vec<SliderMark>>,
    /// Whether the slider is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// Change event handler
    #[prop(optional)]
    on_change: Option<Callback<f64>>,
    /// Change commit event handler (when dragging ends)
    #[prop(optional)]
    on_change_commit: Option<Callback<f64>>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child content (track, thumb, etc.)
    children: Children,
) -> impl IntoView {
    let _slider_id = generate_id("slider");
    
    // Reactive state
    let (current_value, set_current_value) = signal(value.unwrap_or(min));
    let (is_dragging, set_is_dragging) = signal(false);
    
    // Create context
    let context = SliderContext {
        current_value: current_value.into(),
        set_current_value,
        is_dragging: is_dragging.into(),
        set_is_dragging,
        min,
        max,
        step,
        orientation: orientation.clone(),
        disabled,
        on_change,
        on_change_commit,
        slider_id: slider_id.clone(),
    };
    
    // Build base classes
    let base_classes = "radix-slider";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Provide the context
    provide_context(context);
    
    view! {
        <div
            id=slider_id
            class=combined_class
            data-value=current_value.get()
            data-min=min
            data-max=max
            data-step=step
            data-orientation=orientation.as_str()
            data-disabled=disabled
            data-dragging=is_dragging.get()
            role="slider"
            aria-valuemin=min
            aria-valuemax=max
            aria-valuenow=current_value.get()
            aria-orientation=orientation.as_str()
            aria-disabled=disabled
            tabindex="0"
        >
            {children()}
        </div>
    }
}

/// RangeSlider component with two values
#[component]
pub fn RangeSlider(
    /// Current slider values (min, max)
    #[prop(optional)]
    value: Option<(f64, f64)>,
    /// Minimum value
    #[prop(optional, default = 0.0)]
    min: f64,
    /// Maximum value
    #[prop(optional, default = 100.0)]
    max: f64,
    /// Step increment
    #[prop(optional, default = 1.0)]
    step: f64,
    /// Slider orientation
    #[prop(optional, default = SliderOrientation::Horizontal)]
    orientation: SliderOrientation,
    /// Marks to display on the slider
    #[prop(optional)]
    _marks: Option<Vec<SliderMark>>,
    /// Whether the slider is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// Change event handler
    #[prop(optional)]
    on_change: Option<Callback<(f64, f64)>>,
    /// Change commit event handler (when dragging ends)
    #[prop(optional)]
    on_change_commit: Option<Callback<(f64, f64)>>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child content (track, thumbs, etc.)
    children: Children,
) -> impl IntoView {
    let _range_slider_id = generate_id("range-slider");
    
    // Reactive state
    let (current_values, set_current_values) = signal(value.unwrap_or((min, max)));
    let (is_dragging, set_is_dragging) = signal(false);
    let (active_thumb, set_active_thumb) = signal(0); // 0 for min, 1 for max
    
    // Create context
    let context = RangeSliderContext {
        current_values: current_values.into(),
        set_current_values,
        is_dragging: is_dragging.into(),
        set_is_dragging,
        active_thumb: active_thumb.into(),
        set_active_thumb,
        min,
        max,
        step,
        orientation: orientation.clone(),
        disabled,
        on_change,
        on_change_commit,
        range_slider_id: range_slider_id.clone(),
    };
    
    // Build base classes
    let base_classes = "radix-range-slider";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Provide the context
    provide_context(context);
    
    view! {
        <div
            id=range_slider_id
            class=combined_class
            data-min-value=current_values.get().0
            data-max-value=current_values.get().1
            data-min=min
            data-max=max
            data-step=step
            data-orientation=orientation.as_str()
            data-disabled=disabled
            data-dragging=is_dragging.get()
            data-active-thumb=active_thumb.get()
            role="group"
            aria-label="Range slider"
            aria-disabled=disabled
        >
            {children()}
        </div>
    }
}

/// SliderTrack component for the slider track
#[component]
pub fn SliderTrack(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (range, thumb, etc.)
    children: Children,
) -> impl IntoView {
    let _track_id = generate_id("slider-track");
    
    // Build base classes
    let base_classes = "radix-slider-track";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=track_id
            class=combined_class
            style=style.unwrap_or_default()
            role="presentation"
        >
            {children()}
        </div>
    }
}

/// SliderRange component for the filled portion of the track
#[component]
pub fn SliderRange(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let _range_id = generate_id("slider-range");
    
    // Build base classes
    let base_classes = "radix-slider-range";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=range_id
            class=combined_class
            style=style.unwrap_or_default()
            role="presentation"
        >
            {children()}
        </div>
    }
}

/// SliderThumb component for the draggable thumb
#[component]
pub fn SliderThumb(
    /// Whether this thumb is active (being dragged)
    #[prop(optional, default = false)]
    __active: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<SliderContext>().expect("SliderThumb must be used within Slider");
    let _thumb_id = generate_id("slider-thumb");
    
    let handle_click = move |_event: web_sys::MouseEvent| {
        if !context.disabled {
            // For now, just increment by step on click
            let current_value = context.current_value.get();
            let new_value = clamp(current_value + context.step, context.min, context.max);
            context.set_current_value.set(new_value);
            
            if let Some(callback) = context.on_change.clone() {
                callback.run(new_value);
            }
        }
    };
    
    let handle_key_down = move |event: web_sys::KeyboardEvent| {
        if !context.disabled {
            let current_value = context.current_value.get();
            let step = context.step;
            let min = context.min;
            let max = context.max;
            
            match event.key().as_str() {
                "ArrowLeft" | "ArrowDown" => {
                    event.prevent_default();
                    let new_value = clamp(current_value - step, min, max);
                    context.set_current_value.set(new_value);
                    
                    if let Some(callback) = context.on_change.clone() {
                        callback.run(new_value);
                    }
                }
                "ArrowRight" | "ArrowUp" => {
                    event.prevent_default();
                    let new_value = clamp(current_value + step, min, max);
                    context.set_current_value.set(new_value);
                    
                    if let Some(callback) = context.on_change.clone() {
                        callback.run(new_value);
                    }
                }
                "Home" => {
                    event.prevent_default();
                    context.set_current_value.set(min);
                    
                    if let Some(callback) = context.on_change.clone() {
                        callback.run(min);
                    }
                }
                "End" => {
                    event.prevent_default();
                    context.set_current_value.set(max);
                    
                    if let Some(callback) = context.on_change.clone() {
                        callback.run(max);
                    }
                }
                "PageDown" => {
                    event.prevent_default();
                    let new_value = clamp(current_value - step * 10.0, min, max);
                    context.set_current_value.set(new_value);
                    
                    if let Some(callback) = context.on_change.clone() {
                        callback.run(new_value);
                    }
                }
                "PageUp" => {
                    event.prevent_default();
                    let new_value = clamp(current_value + step * 10.0, min, max);
                    context.set_current_value.set(new_value);
                    
                    if let Some(callback) = context.on_change.clone() {
                        callback.run(new_value);
                    }
                }
                _ => {}
            }
        }
    };
    
    // Build base classes
    let base_classes = "radix-slider-thumb";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=thumb_id
            class=combined_class
            style=style.unwrap_or_default()
            data-active=context.is_dragging.get()
            role="slider"
            tabindex="0"
            on:click=handle_click
            on:keydown=handle_key_down
        >
            {children()}
        </div>
    }
}

/// RangeSliderThumb component for range slider thumbs
#[component]
pub fn RangeSliderThumb(
    /// Which thumb this is (0 for min, 1 for max)
    #[prop(optional, default = 0)]
    thumb_index: usize,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<RangeSliderContext>().expect("RangeSliderThumb must be used within RangeSlider");
    let _thumb_id = generate_id("range-slider-thumb");
    
    let handle_click = move |_event: web_sys::MouseEvent| {
        if !context.disabled {
            let current_values = context.current_values.get();
            let step = context.step;
            let min = context.min;
            let max = context.max;
            
            let (current_value, other_value) = if thumb_index == 0 {
                (current_values.0, current_values.1)
            } else {
                (current_values.1, current_values.0)
            };
            
            let new_value = if thumb_index == 0 {
                clamp(current_value + step, min, other_value)
            } else {
                clamp(current_value + step, other_value, max)
            };
            
            let new_values = if thumb_index == 0 {
                (new_value, other_value)
            } else {
                (other_value, new_value)
            };
            
            context.set_current_values.set(new_values);
            
            if let Some(callback) = context.on_change.clone() {
                callback.run(new_values);
            }
        }
    };
    
    let handle_key_down = move |event: web_sys::KeyboardEvent| {
        if !context.disabled {
            let current_values = context.current_values.get();
            let step = context.step;
            let min = context.min;
            let max = context.max;
            
            let (current_value, other_value) = if thumb_index == 0 {
                (current_values.0, current_values.1)
            } else {
                (current_values.1, current_values.0)
            };
            
            match event.key().as_str() {
                "ArrowLeft" | "ArrowDown" => {
                    event.prevent_default();
                    let new_value = if thumb_index == 0 {
                        clamp(current_value - step, min, other_value)
                    } else {
                        clamp(current_value - step, other_value, max)
                    };
                    
                    let new_values = if thumb_index == 0 {
                        (new_value, other_value)
                    } else {
                        (other_value, new_value)
                    };
                    
                    context.set_current_values.set(new_values);
                    
                    if let Some(callback) = context.on_change.clone() {
                        callback.run(new_values);
                    }
                }
                "ArrowRight" | "ArrowUp" => {
                    event.prevent_default();
                    let new_value = if thumb_index == 0 {
                        clamp(current_value + step, min, other_value)
                    } else {
                        clamp(current_value + step, other_value, max)
                    };
                    
                    let new_values = if thumb_index == 0 {
                        (new_value, other_value)
                    } else {
                        (other_value, new_value)
                    };
                    
                    context.set_current_values.set(new_values);
                    
                    if let Some(callback) = context.on_change.clone() {
                        callback.run(new_values);
                    }
                }
                _ => {}
            }
        }
    };
    
    // Build base classes
    let base_classes = "radix-range-slider-thumb";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=thumb_id
            class=combined_class
            style=style.unwrap_or_default()
            data-active=context.is_dragging.get() && context.active_thumb.get() == thumb_index
            data-thumb-index=thumb_index
            role="slider"
            tabindex="0"
            on:click=handle_click
            on:keydown=handle_key_down
        >
            {children()}
        </div>
    }
}

/// SliderMark component for individual marks
#[component]
pub fn SliderMark(
    /// The value this mark represents
    #[prop(optional)]
    value: Option<f64>,
    /// The label for this mark
    #[prop(optional)]
    _label: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let _mark_id = generate_id("slider-mark");
    
    // Build base classes
    let base_classes = "radix-slider-mark";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=mark_id
            class=combined_class
            style=style.unwrap_or_default()
            data-value=value.unwrap_or_default()
            role="presentation"
        >
            {children()}
        </div>
    }
}
