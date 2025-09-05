use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

/// ARIA live region politeness levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AriaLive {
    Off,
    Polite,
    Assertive,
}

impl AriaLive {
    pub fn as_str(&self) -> &'static str {
        match self {
            AriaLive::Off => "off",
            AriaLive::Polite => "polite",
            AriaLive::Assertive => "assertive",
        }
    }
}

/// Announce a message to screen readers using a live region
pub fn announce_to_screen_reader(message: &str, politeness: AriaLive) {
    if let Some(live_region) = get_or_create_live_region(politeness) {
        live_region.set_text_content(Some(message));

        // Clear after a brief delay to allow re-announcements
        let live_region_clone = live_region.clone();
        wasm_bindgen_futures::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(1000).await;
            live_region_clone.set_text_content(Some(""));
        });
    }
}

/// Get or create a live region for screen reader announcements
pub fn get_or_create_live_region(politeness: AriaLive) -> Option<HtmlElement> {
    let document = web_sys::window()?.document()?;

    let id = format!("radix-live-region-{}", politeness.as_str());

    // Try to find existing live region
    if let Some(existing) = document.get_element_by_id(&id) {
        return existing.dyn_into().ok();
    }

    // Create new live region
    let live_region = document.create_element("div").ok()?;
    live_region.set_id(&id);
    live_region
        .set_attribute("aria-live", politeness.as_str())
        .ok()?;
    live_region.set_attribute("aria-atomic", "true").ok()?;
    live_region
        .set_attribute(
            "style",
            "position: absolute; left: -10000px; width: 1px; height: 1px; overflow: hidden;",
        )
        .ok()?;

    document.body()?.append_child(&live_region).ok()?;

    live_region.dyn_into().ok()
}

/// Apply ARIA attributes to an element based on component state
pub struct AriaAttributes {
    pub role: Option<String>,
    pub label: Option<String>,
    pub labelledby: Option<String>,
    pub describedby: Option<String>,
    pub expanded: Option<bool>,
    pub checked: Option<bool>,
    pub selected: Option<bool>,
    pub disabled: Option<bool>,
    pub hidden: Option<bool>,
    pub modal: Option<bool>,
    pub popup: Option<String>,
    pub controls: Option<String>,
    pub owns: Option<String>,
}

impl AriaAttributes {
    pub fn new() -> Self {
        Self {
            role: None,
            label: None,
            labelledby: None,
            describedby: None,
            expanded: None,
            checked: None,
            selected: None,
            disabled: None,
            hidden: None,
            modal: None,
            popup: None,
            controls: None,
            owns: None,
        }
    }

    pub fn role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn labelledby(mut self, labelledby: impl Into<String>) -> Self {
        self.labelledby = Some(labelledby.into());
        self
    }

    pub fn describedby(mut self, describedby: impl Into<String>) -> Self {
        self.describedby = Some(describedby.into());
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = Some(expanded);
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = Some(hidden);
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    pub fn popup(mut self, popup: impl Into<String>) -> Self {
        self.popup = Some(popup.into());
        self
    }

    pub fn controls(mut self, controls: impl Into<String>) -> Self {
        self.controls = Some(controls.into());
        self
    }

    pub fn owns(mut self, owns: impl Into<String>) -> Self {
        self.owns = Some(owns.into());
        self
    }

    /// Apply all attributes to an element
    pub fn apply_to(&self, element: &Element) -> Result<(), wasm_bindgen::JsValue> {
        if let Some(ref role) = self.role {
            element.set_attribute("role", role)?;
        }

        if let Some(ref label) = self.label {
            element.set_attribute("aria-label", label)?;
        }

        if let Some(ref labelledby) = self.labelledby {
            element.set_attribute("aria-labelledby", labelledby)?;
        }

        if let Some(ref describedby) = self.describedby {
            element.set_attribute("aria-describedby", describedby)?;
        }

        if let Some(expanded) = self.expanded {
            element.set_attribute("aria-expanded", &expanded.to_string())?;
        }

        if let Some(checked) = self.checked {
            element.set_attribute("aria-checked", &checked.to_string())?;
        }

        if let Some(selected) = self.selected {
            element.set_attribute("aria-selected", &selected.to_string())?;
        }

        if let Some(disabled) = self.disabled {
            element.set_attribute("aria-disabled", &disabled.to_string())?;
        }

        if let Some(hidden) = self.hidden {
            element.set_attribute("aria-hidden", &hidden.to_string())?;
        }

        if let Some(modal) = self.modal {
            element.set_attribute("aria-modal", &modal.to_string())?;
        }

        if let Some(ref popup) = self.popup {
            element.set_attribute("aria-haspopup", popup)?;
        }

        if let Some(ref controls) = self.controls {
            element.set_attribute("aria-controls", controls)?;
        }

        if let Some(ref owns) = self.owns {
            element.set_attribute("aria-owns", owns)?;
        }

        Ok(())
    }
}

impl Default for AriaAttributes {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn test_aria_attributes() {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.create_element("button").unwrap();

        let attrs = AriaAttributes::new()
            .role("button")
            .label("Close dialog")
            .expanded(true)
            .disabled(false);

        attrs.apply_to(&element).unwrap();

        assert_eq!(element.get_attribute("role").as_deref(), Some("button"));
        assert_eq!(
            element.get_attribute("aria-label").as_deref(),
            Some("Close dialog")
        );
        assert_eq!(
            element.get_attribute("aria-expanded").as_deref(),
            Some("true")
        );
        assert_eq!(
            element.get_attribute("aria-disabled").as_deref(),
            Some("false")
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn test_live_region_creation() {
        let live_region = get_or_create_live_region(AriaLive::Polite).unwrap();

        assert_eq!(
            live_region.get_attribute("aria-live").as_deref(),
            Some("polite")
        );
        assert_eq!(
            live_region.get_attribute("aria-atomic").as_deref(),
            Some("true")
        );
    }
}
