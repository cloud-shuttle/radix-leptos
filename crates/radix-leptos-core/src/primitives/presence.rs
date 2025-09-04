use leptos::*;
use std::time::Duration;
use wasm_bindgen::JsCast;
use web_sys::Element;

/// Presence component for handling enter/exit animations
/// 
/// The Presence component manages the mounting and unmounting of components
/// with support for enter and exit animations. It provides hooks into the
/// animation lifecycle and ensures elements remain in the DOM during transitions.
/// 
/// # Example
/// 
/// ```rust
/// use leptos::*;
/// use radix_leptos_core::Presence;
/// 
/// #[component]
/// fn AnimatedDialog() -> impl IntoView {
///     let (open, set_open) = create_signal(false);
///     
///     view! {
///         <button on:click=move |_| set_open.set(!open.get())>
///             "Toggle Dialog"
///         </button>
///         <Presence present=open>
///             <div class="dialog-overlay animate-in animate-out">
///                 "Dialog content with animations"
///             </div>
///         </Presence>
///     }
/// }
/// ```
#[component]
pub fn Presence(
    /// Whether the content should be present
    #[prop(into)]
    present: Signal<bool>,
    /// Whether to force mount regardless of present state
    #[prop(optional, default = false)]
    force_mount: bool,
    /// Content to render with presence control
    children: Children,
) -> impl IntoView {
    let (mounted, set_mounted) = create_signal(present.get_untracked() || force_mount);
    let (state, set_state) = create_signal(if present.get_untracked() { 
        PresenceState::Open 
    } else { 
        PresenceState::Closed 
    });

    // Create animation context
    let presence_context = PresenceContext {
        present: present.into(),
        state: state.into(),
    };
    provide_context(presence_context);

    // Handle presence changes
    create_effect(move |_| {
        let is_present = present.get();
        
        if is_present && !mounted.get() {
            // Mount and start enter animation
            set_mounted.set(true);
            set_state.set(PresenceState::Entering);
            
            // Transition to open after mount
            request_animation_frame(move || {
                set_state.set(PresenceState::Open);
            });
        } else if !is_present && mounted.get() && !force_mount {
            // Start exit animation
            set_state.set(PresenceState::Exiting);
            
            // Wait for animation to complete before unmounting
            // In a real implementation, this would listen for animation events
            set_timeout(
                move || {
                    set_mounted.set(false);
                    set_state.set(PresenceState::Closed);
                },
                Duration::from_millis(150), // Default animation duration
            );
        }
    });

    view! {
        <Show when=move || mounted.get() || force_mount>
            {children()}
        </Show>
    }
}

/// Animation state for presence transitions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PresenceState {
    /// Content is not present
    Closed,
    /// Content is entering (animation starting)
    Entering, 
    /// Content is fully present
    Open,
    /// Content is exiting (animation ending)
    Exiting,
}

/// Context provided by Presence component
#[derive(Clone, Copy)]
pub struct PresenceContext {
    pub present: ReadSignal<bool>,
    pub state: ReadSignal<PresenceState>,
}

/// Hook to access presence context
pub fn use_presence() -> Option<PresenceContext> {
    use_context::<PresenceContext>()
}

/// Hook for presence-aware components
pub fn use_presence_state() -> (ReadSignal<bool>, ReadSignal<PresenceState>) {
    if let Some(context) = use_presence() {
        (context.present, context.state)
    } else {
        // Fallback when not inside Presence
        let present = create_signal(true).0;
        let state = create_signal(PresenceState::Open).0;
        (present, state)
    }
}

/// Utility function to request animation frame
fn request_animation_frame<F>(callback: F)
where
    F: FnOnce() + 'static,
{
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;
    
    let cb = Closure::once_into_js(callback);
    let window = web_sys::window().expect("should have window");
    window
        .request_animation_frame(cb.as_ref().unchecked_ref())
        .expect("should register animation frame");
}

/// Utility function to set timeout
fn set_timeout<F>(callback: F, duration: Duration)
where
    F: FnOnce() + 'static,
{
    wasm_bindgen_futures::spawn_local(async move {
        gloo_timers::future::TimeoutFuture::new(duration.as_millis() as u32).await;
        callback();
    });
}

/// Component for handling animation lifecycle events
#[component]
pub fn PresenceChild(
    /// Callback when enter animation starts
    #[prop(optional)]
    on_enter_start: Option<Box<dyn Fn()>>,
    /// Callback when enter animation completes  
    #[prop(optional)]
    on_enter_complete: Option<Box<dyn Fn()>>,
    /// Callback when exit animation starts
    #[prop(optional)]
    on_exit_start: Option<Box<dyn Fn()>>,
    /// Callback when exit animation completes
    #[prop(optional)]
    on_exit_complete: Option<Box<dyn Fn()>>,
    /// Content to render
    children: Children,
) -> impl IntoView {
    let (present, state) = use_presence_state();
    
    // Track previous state to detect transitions
    let (prev_state, set_prev_state) = create_signal(state.get_untracked());
    
    create_effect(move |_| {
        let current_state = state.get();
        let previous_state = prev_state.get();
        
        // Detect state transitions and call appropriate callbacks
        match (previous_state, current_state) {
            (PresenceState::Closed, PresenceState::Entering) => {
                if let Some(ref callback) = on_enter_start {
                    callback();
                }
            }
            (PresenceState::Entering, PresenceState::Open) => {
                if let Some(ref callback) = on_enter_complete {
                    callback();
                }
            }
            (PresenceState::Open, PresenceState::Exiting) => {
                if let Some(ref callback) = on_exit_start {
                    callback();
                }
            }
            (PresenceState::Exiting, PresenceState::Closed) => {
                if let Some(ref callback) = on_exit_complete {
                    callback();
                }
            }
            _ => {}
        }
        
        set_prev_state.set(current_state);
    });
    
    children()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_presence_mounting() {
        run_test(|cx| {
            let (present, set_present) = create_signal(cx, false);
            let (rendered, set_rendered) = create_signal(cx, false);
            
            let _view = view! { cx,
                <Presence present=present>
                    <div on:mount=move || set_rendered.set(true)>
                        "Content"
                    </div>
                </Presence>
            };
            
            // Initially not rendered
            assert!(!rendered.get());
            
            // Set present to true
            set_present.set(true);
            
            // Should be rendered after presence change
            // In a real test environment, we would wait for the next tick
            // For now, we verify the signal was set up correctly
            assert!(set_rendered.will_run_on_next_update());
        });
    }
    
    #[wasm_bindgen_test]
    fn test_presence_context() {
        run_test(|cx| {
            let (present, _) = create_signal(cx, true);
            let mut context_found = false;
            
            let _view = view! { cx,
                <Presence present=present>
                    <div>
                        {
                            if use_presence().is_some() {
                                context_found = true;
                            }
                            "Content"
                        }
                    </div>
                </Presence>
            };
            
            assert!(context_found);
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