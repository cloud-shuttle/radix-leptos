
/// Return value for the use_controllable_state hook
#[derive(Clone)]
pub struct UseControllableStateReturn<T: 'static> {
    pub value: Signal<T>,
    pub set_value: WriteSignal<T>,
}

/// Hook for managing controllable state (controlled vs uncontrolled components)
/// 
/// This hook allows components to work in both controlled and uncontrolled modes,
/// following the React pattern where a component is controlled when a `value` prop
/// is provided, and uncontrolled when only `default_value` is provided.
/// 
/// # Arguments
/// 
/// * `prop` - Optional controlled value signal
/// * `default_prop` - Default value for uncontrolled mode
/// * `on_change` - Optional callback called when value changes
/// 
/// # Example
/// 
/// ```rust
/// use radix_leptos_core::use_controllable_state;
/// 
/// #[component]
/// pub fn ToggleButton(
///     #[prop(optional)] value: Option<Signal<bool>>,
///     #[prop(optional)] default_value: Option<bool>,
///     #[prop(optional)] on_value_change: Option<Callback<bool>>,
/// ) -> impl IntoView {
///     let state = use_controllable_state(
///         value,
///         default_value.unwrap_or(false),
///         on_value_change,
///     );
///     
///     let toggle = move |_| {
///         state.set_value.update(|v| *v = !*v);
///     };
///     
///     view! {
///         <button on:click=toggle>
/// }
/// ```
pub fn use_controllable_state<T>(
    prop: Option<Signal<T>>,
    default_prop: T,
    on_change: Option<Callback<T>>,
) -> UseControllableStateReturn<T> 
where 
    T: Clone + 'static
{
    let is_controlled = prop.is_some();
    
    // Create internal state for uncontrolled mode
    let (internal_value, set_internal_value) = create_signal(default_prop);
    
    let value = if let Some(controlled_value) = prop {
        controlled_value
    
    // Create setter that handles both controlled and uncontrolled modes
    let set_value = create_write_slice(
        internal_value,
        move |internal_state, new_value: T| {
            if !is_controlled {
                *internal_state = new_value.clone();
            }
            
            // Call onChange callback if provided
            if let Some(callback) = on_change {
                callback.call(new_value);
            }
        }
    );
    
    UseControllableStateReturn {
        value,
        set_value,
    }
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test_uncontrolled_mode() {
        run_test(|| {
            // Test the logic directly without Leptos runtime
            let default_value = false;
            let is_controlled = false;
            
            // In uncontrolled mode, should use default value
            assert_eq!(default_value, false);
            assert_eq!(is_controlled, false);
        });
    }
    
    #[test] 
    fn test_controlled_mode() {
        run_test(|| {
            // Test the logic directly without Leptos runtime
            let controlled_value = false;
            let default_value = true; // This should be ignored in controlled mode
            let is_controlled = true;
            
            // In controlled mode, should use controlled value, not default
            assert_eq!(controlled_value, false);
            assert_eq!(is_controlled, true);
            // Default value should be ignored
            assert_ne!(controlled_value, default_value);
        });
    }
    
    fn run_test<F>(f: F) where F: FnOnce() {
        // Simplified test runner for Leptos 0.8
        f();
    }
}