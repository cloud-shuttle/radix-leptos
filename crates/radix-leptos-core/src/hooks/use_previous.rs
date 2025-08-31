use leptos::*;

/// Hook for tracking the previous value of a signal
/// 
/// This hook is useful for detecting changes and implementing
/// transition effects based on value changes.
/// 
/// # Example
/// 
/// ```rust
/// use leptos::*;
/// use radix_leptos_core::use_previous;
/// 
/// #[component]
/// pub fn Counter() -> impl IntoView {
///     let (count, set_count) = create_signal(0);
///     let previous_count = use_previous(count.into());
///     
///     let direction = move || {
///         match previous_count.get() {
///             Some(prev) if prev < count.get() => "up",
///             Some(prev) if prev > count.get() => "down", 
///             _ => "unchanged"
///         }
///     };
///     
///     view! {
///         <div>
///             <button on:click=move |_| set_count.update(|n| *n -= 1)>"-"</button>
///             <span>"Count: " {count} " (" {direction} ")"</span>
///             <button on:click=move |_| set_count.update(|n| *n += 1)>"+"</button>
///         </div>
///     }
/// }
/// ```
pub fn use_previous<T>(signal: Signal<T>) -> Signal<Option<T>>
where
    T: Clone + PartialEq + 'static,
{
    let (previous, set_previous) = create_signal::<Option<T>>(None);
    
    create_effect(move |prev_value: Option<Option<T>>| {
        let current_value = signal.get();
        
        // Update previous with the last value (from the effect's previous run)
        if let Some(Some(last_value)) = prev_value {
            set_previous.set(Some(last_value));
        }
        
        // Return current value to be used as prev_value in next run
        Some(current_value)
    });
    
    previous.into()
}

/// Hook for tracking previous value with a custom comparison function
pub fn use_previous_with_eq<T, F>(
    signal: Signal<T>,
    eq_fn: F,
) -> Signal<Option<T>>
where
    T: Clone + 'static,
    F: Fn(&T, &T) -> bool + 'static,
{
    let (previous, set_previous) = create_signal::<Option<T>>(None);
    
    create_effect(move |prev_value: Option<Option<T>>| {
        let current_value = signal.get();
        
        // Only update if value actually changed according to custom eq function
        let should_update = match (prev_value.as_ref().and_then(|v| v.as_ref()), previous.get_untracked().as_ref()) {
            (Some(last), Some(stored)) => !eq_fn(last, stored),
            (Some(_), None) => true,
            _ => false,
        };
        
        if should_update {
            if let Some(Some(last_value)) = prev_value {
                set_previous.set(Some(last_value));
            }
        }
        
        Some(current_value)
    });
    
    previous.into()
}

/// Hook for detecting when a value has changed
pub fn use_changed<T>(signal: Signal<T>) -> Signal<bool>
where
    T: Clone + PartialEq + 'static,
{
    let previous = use_previous(signal);
    
    create_memo(move |_| {
        if let Some(prev) = previous.get() {
            prev != signal.get()
        } else {
            false // No previous value means no change yet
        }
    }).into()
}

/// Hook for getting both current and previous values
pub fn use_current_and_previous<T>(signal: Signal<T>) -> Signal<(T, Option<T>)>
where
    T: Clone + PartialEq + 'static,
{
    let previous = use_previous(signal);
    
    create_memo(move |_| {
        (signal.get(), previous.get())
    }).into()
}

/// Hook for tracking multiple previous values (history)
pub fn use_history<T>(signal: Signal<T>, max_history: usize) -> Signal<Vec<T>>
where
    T: Clone + PartialEq + 'static,
{
    let (history, set_history) = create_signal::<Vec<T>>(Vec::new());
    
    create_effect(move |_| {
        let current_value = signal.get();
        
        set_history.update(|hist| {
            hist.push(current_value);
            
            // Keep only the max number of history items
            if hist.len() > max_history {
                hist.remove(0);
            }
        });
    });
    
    history.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_use_previous() {
        run_test(|cx| {
            let (value, set_value) = create_signal(cx, 1);
            let previous = use_previous(value.into());
            
            // Initially no previous value
            assert_eq!(previous.get(), None);
            
            // Update value
            set_value.set(2);
            
            // Previous should now be the old value
            assert_eq!(previous.get(), Some(1));
            
            // Update again
            set_value.set(3);
            
            // Previous should be the previous update
            assert_eq!(previous.get(), Some(2));
        });
    }
    
    #[test]
    fn test_use_changed() {
        run_test(|cx| {
            let (value, set_value) = create_signal(cx, 1);
            let changed = use_changed(value.into());
            
            // Initially not changed
            assert_eq!(changed.get(), false);
            
            // Update value
            set_value.set(2);
            
            // Should detect change
            assert_eq!(changed.get(), true);
        });
    }
    
    #[test]
    fn test_use_history() {
        run_test(|cx| {
            let (value, set_value) = create_signal(cx, 1);
            let history = use_history(value.into(), 3);
            
            // Should start with just the initial value
            assert_eq!(history.get(), vec![1]);
            
            set_value.set(2);
            assert_eq!(history.get(), vec![1, 2]);
            
            set_value.set(3);
            assert_eq!(history.get(), vec![1, 2, 3]);
            
            set_value.set(4);
            // Should maintain max history size
            assert_eq!(history.get(), vec![2, 3, 4]);
        });
    }
    
    fn run_test<F>(f: F) where F: FnOnce(Scope) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let _ = create_runtime();
            run_scope(create_runtime(), f);
        });
    }
}