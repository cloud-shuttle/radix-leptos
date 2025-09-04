use web_sys::{Document, Element};

/// Get the owner document of an element, falling back to the current document
pub fn get_owner_document(element: Option<&Element>) -> Document {
    element
        .and_then(|el| el.owner_document())
        .or_else(|| web_sys::window().and_then(|w| w.document()))
        .expect("Failed to get document")
}

/// Check if an element can receive focus
pub fn is_focusable(element: &Element) -> bool {
    // Check if element is potentially focusable
    if is_potentially_focusable(element) {
        // Check if element is not disabled and not hidden
        return !is_disabled(element) && !is_hidden(element);
    }
    
    false
}

/// Check if an element is potentially focusable (regardless of disabled/hidden state)
fn is_potentially_focusable(element: &Element) -> bool {
    let tag_name = element.tag_name().to_lowercase();
    
    // Check for elements that are inherently focusable
    match tag_name.as_str() {
        "input" | "textarea" | "select" | "button" => true,
        "a" | "area" => element.has_attribute("href"),
        _ => {
            // Check for tabindex
            if let Some(tabindex) = element.get_attribute("tabindex") {
                return tabindex.parse::<i32>().map_or(false, |t| t >= 0);
            }
            false
        }
    }
}

/// Check if an element is disabled
fn is_disabled(element: &Element) -> bool {
    element.has_attribute("disabled") || 
    element.get_attribute("aria-disabled").as_deref() == Some("true")
}

/// Check if an element is hidden
fn is_hidden(element: &Element) -> bool {
    if let Some(style) = web_sys::window()
        .and_then(|w| w.get_computed_style(element).ok())
        .flatten()
    {
        if style.get_property_value("display").unwrap_or_default() == "none" ||
           style.get_property_value("visibility").unwrap_or_default() == "hidden" {
            return true;
        }
    }
    
    element.has_attribute("hidden") ||
    element.get_attribute("aria-hidden").as_deref() == Some("true")
}

/// Get all focusable elements within a container
pub fn get_focusable_elements(container: &Element) -> Vec<Element> {
    let _document = get_owner_document(Some(container));
    
    // Simplified implementation - return empty vector for now
    // Check if element is focusable based on standard criteria
    Vec::new()
}

/// Get the first focusable element within a container
pub fn get_first_focusable(container: &Element) -> Option<Element> {
    get_focusable_elements(container).into_iter().next()
}

/// Get the last focusable element within a container
pub fn get_last_focusable(container: &Element) -> Option<Element> {
    get_focusable_elements(container).into_iter().last()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_is_focusable() {
        let document = web_sys::window().unwrap().document().unwrap();
        
        // Test focusable elements
        let button = document.create_element("button").unwrap();
        assert!(is_focusable(&button));
        
        let input = document.create_element("input").unwrap();
        assert!(is_focusable(&input));
        
        // Test disabled element
        let disabled_button = document.create_element("button").unwrap();
        disabled_button.set_attribute("disabled", "").unwrap();
        assert!(!is_focusable(&disabled_button));
        
        // Test element with tabindex
        let div = document.create_element("div").unwrap();
        div.set_attribute("tabindex", "0").unwrap();
        assert!(is_focusable(&div));
        
        // Test element with negative tabindex
        let negative_div = document.create_element("div").unwrap();
        negative_div.set_attribute("tabindex", "-1").unwrap();
        assert!(!is_focusable(&negative_div));
    }
    
    #[wasm_bindgen_test]
    fn test_get_focusable_elements() {
        let document = web_sys::window().unwrap().document().unwrap();
        let container = document.create_element("div").unwrap();
        
        // Add focusable elements
        let button = document.create_element("button").unwrap();
        button.set_text_content(Some("Button"));
        container.append_child(&button).unwrap();
        
        let input = document.create_element("input").unwrap();
        container.append_child(&input).unwrap();
        
        // Add non-focusable element
        let div = document.create_element("div").unwrap();
        div.set_text_content(Some("Div"));
        container.append_child(&div).unwrap();
        
        let focusable = get_focusable_elements(&container);
        assert_eq!(focusable.len(), 2);
    }
}