use web_sys::{Event, EventTarget, MouseEvent, KeyboardEvent, FocusEvent};
use wasm_bindgen::{JsCast, JsValue};

/// Utility functions for event handling and composition

/// Compose multiple event handlers into a single handler
pub fn compose_event_handlers<E, F1, F2>(
    handler1: Option<F1>, 
    handler2: Option<F2>,
) -> impl Fn(E)
where
    E: Clone,
    F1: Fn(E),
    F2: Fn(E),
{
    move |event: E| {
        if let Some(ref h1) = handler1 {
            h1(event.clone());
        }
        if let Some(ref h2) = handler2 {
            h2(event);
        }
    }
}

/// Check if an event should be handled based on keyboard modifiers
pub fn should_handle_key_event(event: &KeyboardEvent, key: &str, modifiers: Option<KeyModifiers>) -> bool {
    if event.key() != key {
        return false;
    }
    
    if let Some(mods) = modifiers {
        return event.ctrl_key() == mods.ctrl &&
               event.shift_key() == mods.shift &&
               event.alt_key() == mods.alt &&
               event.meta_key() == mods.meta;
    }
    
    true
}

/// Keyboard modifier state
#[derive(Debug, Clone, PartialEq)]
pub struct KeyModifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}

impl KeyModifiers {
    pub fn none() -> Self {
        Self {
            ctrl: false,
            shift: false,
            alt: false,
            meta: false,
        }
    }
    
    pub fn ctrl() -> Self {
        Self {
            ctrl: true,
            shift: false,
            alt: false,
            meta: false,
        }
    }
    
    pub fn shift() -> Self {
        Self {
            ctrl: false,
            shift: true,
            alt: false,
            meta: false,
        }
    }
}

/// Prevent default and stop propagation on an event
pub fn prevent_default_and_stop_propagation(event: &Event) {
    event.prevent_default();
    event.stop_propagation();
}

/// Get the target element of an event, safely cast to Element
pub fn get_event_target_element(event: &Event) -> Option<web_sys::Element> {
    event
        .target()?
        .dyn_into()
        .ok()
}

/// Create a synthetic keyboard event for testing
#[cfg(test)]
pub fn create_keyboard_event(event_type: &str, key: &str, modifiers: Option<KeyModifiers>) -> KeyboardEvent {
    use web_sys::KeyboardEventInit;
    
    let mut init = KeyboardEventInit::new();
    init.key(key);
    
    if let Some(mods) = modifiers {
        init.ctrl_key(mods.ctrl);
        init.shift_key(mods.shift);
        init.alt_key(mods.alt);
        init.meta_key(mods.meta);
    }
    
    KeyboardEvent::new_with_keyboard_event_init_dict(event_type, &init)
        .expect("Failed to create keyboard event")
}

/// Create a synthetic mouse event for testing  
#[cfg(test)]
pub fn create_mouse_event(event_type: &str) -> MouseEvent {
    MouseEvent::new(event_type).expect("Failed to create mouse event")
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[test]
    fn test_key_modifiers() {
        let none = KeyModifiers::none();
        assert!(!none.ctrl && !none.shift && !none.alt && !none.meta);
        
        let ctrl = KeyModifiers::ctrl();
        assert!(ctrl.ctrl && !ctrl.shift && !ctrl.alt && !ctrl.meta);
        
        let shift = KeyModifiers::shift();
        assert!(!shift.ctrl && shift.shift && !shift.alt && !shift.meta);
    }
    
    #[wasm_bindgen_test]
    fn test_keyboard_event_creation() {
        let event = create_keyboard_event("keydown", "Enter", Some(KeyModifiers::ctrl()));
        
        assert_eq!(event.key(), "Enter");
        assert!(event.ctrl_key());
        assert!(!event.shift_key());
    }
    
    #[wasm_bindgen_test]
    fn test_should_handle_key_event() {
        let event = create_keyboard_event("keydown", "Escape", None);
        
        assert!(should_handle_key_event(&event, "Escape", None));
        assert!(!should_handle_key_event(&event, "Enter", None));
        
        let event_with_ctrl = create_keyboard_event("keydown", "s", Some(KeyModifiers::ctrl()));
        assert!(should_handle_key_event(&event_with_ctrl, "s", Some(KeyModifiers::ctrl())));
        assert!(!should_handle_key_event(&event_with_ctrl, "s", None));
    }
}