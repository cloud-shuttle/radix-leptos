use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent};

/// Utility functions for event handling and composition
/// Compose multiple event handlers into a single handler
pub fn compose_event_handlers<E, F1, F2>(handler1: Option<F1>, handler2: Option<F2>) -> impl Fn(E)
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
pub fn should_handle_key_event(
    event: &KeyboardEvent,
    key: &str,
    modifiers: Option<KeyModifiers>,
) -> bool {
    if event.key() != key {
        return false;
    }

    if let Some(mods) = modifiers {
        return event.ctrl_key() == mods._ctrl
            && event.shift_key() == mods._shift
            && event.alt_key() == mods._alt
            && event.meta_key() == mods._meta;
    }

    true
}

/// Keyboard modifier state
#[derive(Debug, Clone, PartialEq)]
pub struct KeyModifiers {
    pub _ctrl: bool,
    pub _shift: bool,
    pub _alt: bool,
    pub _meta: bool,
}

impl KeyModifiers {
    pub fn none() -> Self {
        Self {
            _ctrl: false,
            _shift: false,
            _alt: false,
            _meta: false,
        }
    }

    pub fn ctrl() -> Self {
        Self {
            _ctrl: true,
            _shift: false,
            _alt: false,
            _meta: false,
        }
    }

    pub fn shift() -> Self {
        Self {
            _ctrl: false,
            _shift: true,
            _alt: false,
            _meta: false,
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
    event.target()?.dyn_into().ok()
}

/// Create a synthetic keyboard event for testing
#[cfg(test)]
pub fn create_keyboard_event(
    event_type: &str,
    _key: &str,
    _modifiers: Option<KeyModifiers>,
) -> KeyboardEvent {
    use web_sys::KeyboardEvent;

    // Simplified keyboard event creation for testing
    KeyboardEvent::new(event_type).expect("Failed to create keyboard event")
}

/// Create a synthetic mouse event for testing  
#[cfg(test)]
pub fn create_mouse_event(event_type: &str) -> web_sys::MouseEvent {
    web_sys::MouseEvent::new(event_type).expect("Failed to create mouse event")
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_key_modifiers() {
        let none = KeyModifiers::none();
        assert!(!none._ctrl && !none._shift && !none._alt && !none._meta);

        let ctrl = KeyModifiers::ctrl();
        assert!(ctrl._ctrl && !ctrl._shift && !ctrl._alt && !ctrl._meta);

        let shift = KeyModifiers::shift();
        assert!(!shift._ctrl && shift._shift && !shift._alt && !shift._meta);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn test_keyboard_event_creation() {
        let event = create_keyboard_event("keydown", "Enter", Some(KeyModifiers::ctrl()));

        assert_eq!(event.key(), "Enter");
        assert!(event.ctrl_key());
        assert!(!event.shift_key());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn test_should_handle_key_event() {
        let event = create_keyboard_event("keydown", "Escape", None);

        assert!(should_handle_key_event(&event, "Escape", None));
        assert!(!should_handle_key_event(&event, "Enter", None));

        let event_with_ctrl = create_keyboard_event("keydown", "s", Some(KeyModifiers::ctrl()));
        assert!(should_handle_key_event(
            &event_with_ctrl,
            "s",
            Some(KeyModifiers::ctrl())
        ));
        assert!(!should_handle_key_event(&event_with_ctrl, "s", None));
    }
}
