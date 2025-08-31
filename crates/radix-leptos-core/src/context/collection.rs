use leptos::*;
use web_sys::Element;
use std::collections::HashMap;

/// Collection context for managing lists of items with roving focus
/// 
/// This context is used by components like RadioGroup, Menu, and Listbox
/// to manage collections of focusable items with proper keyboard navigation.
/// 
/// # Example
/// 
/// ```rust
/// use leptos::*;
/// use radix_leptos_core::{CollectionProvider, use_collection_context, CollectionItem};
/// 
/// #[component]
/// pub fn RadioGroup(children: Children) -> impl IntoView {
///     view! {
///         <CollectionProvider>
///             <div role="radiogroup">
///                 {children()}
///             </div>
///         </CollectionProvider>
///     }
/// }
/// 
/// #[component] 
/// pub fn RadioItem(value: String) -> impl IntoView {
///     let item_ref = create_node_ref::<Element>();
///     let collection = use_collection_context().expect("RadioItem must be inside RadioGroup");
///     
///     // Register this item with the collection
///     create_effect(move |_| {
///         if let Some(element) = item_ref.get() {
///             collection.register_item(CollectionItem {
///                 value: value.clone(),
///                 element,
///                 disabled: false,
///             });
///         }
///     });
///     
///     view! {
///         <div ref=item_ref role="radio">
///             {value}
///         </div>
///     }
/// }
/// ```
pub struct CollectionItem {
    pub value: String,
    pub element: Element,
    pub disabled: bool,
}

impl Clone for CollectionItem {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            element: self.element.clone(),
            disabled: self.disabled,
        }
    }
}

#[derive(Clone)]
pub struct CollectionContext {
    items: RwSignal<Vec<CollectionItem>>,
    active_item: RwSignal<Option<String>>,
}

impl CollectionContext {
    pub fn new() -> Self {
        Self {
            items: create_rw_signal(Vec::new()),
            active_item: create_rw_signal(None),
        }
    }
    
    /// Register a new item in the collection
    pub fn register_item(&self, item: CollectionItem) {
        self.items.update(|items| {
            // Remove existing item with same value
            items.retain(|existing| existing.value != item.value);
            items.push(item);
        });
    }
    
    /// Unregister an item from the collection
    pub fn unregister_item(&self, value: &str) {
        self.items.update(|items| {
            items.retain(|item| item.value != value);
        });
    }
    
    /// Get all items in the collection
    pub fn get_items(&self) -> Vec<CollectionItem> {
        self.items.get()
    }
    
    /// Get all enabled items in the collection
    pub fn get_enabled_items(&self) -> Vec<CollectionItem> {
        self.items.get()
            .into_iter()
            .filter(|item| !item.disabled)
            .collect()
    }
    
    /// Set the active item
    pub fn set_active_item(&self, value: Option<String>) {
        self.active_item.set(value);
    }
    
    /// Get the active item value
    pub fn get_active_item(&self) -> Option<String> {
        self.active_item.get()
    }
    
    /// Get the next enabled item in the collection
    pub fn get_next_item(&self, current_value: Option<&str>) -> Option<CollectionItem> {
        let items = self.get_enabled_items();
        if items.is_empty() {
            return None;
        }
        
        if let Some(current) = current_value {
            if let Some(current_index) = items.iter().position(|item| item.value == current) {
                let next_index = (current_index + 1) % items.len();
                return items.get(next_index).cloned();
            }
        }
        
        items.first().cloned()
    }
    
    /// Get the previous enabled item in the collection
    pub fn get_previous_item(&self, current_value: Option<&str>) -> Option<CollectionItem> {
        let items = self.get_enabled_items();
        if items.is_empty() {
            return None;
        }
        
        if let Some(current) = current_value {
            if let Some(current_index) = items.iter().position(|item| item.value == current) {
                let prev_index = if current_index == 0 {
                    items.len() - 1
                } else {
                    current_index - 1
                };
                return items.get(prev_index).cloned();
            }
        }
        
        items.last().cloned()
    }
    
    /// Get the first enabled item
    pub fn get_first_item(&self) -> Option<CollectionItem> {
        self.get_enabled_items().first().cloned()
    }
    
    /// Get the last enabled item
    pub fn get_last_item(&self) -> Option<CollectionItem> {
        self.get_enabled_items().last().cloned()
    }
    
    /// Find item by value
    pub fn find_item(&self, value: &str) -> Option<CollectionItem> {
        self.get_items()
            .into_iter()
            .find(|item| item.value == value)
    }
    
    /// Find items by predicate
    pub fn find_items<F>(&self, predicate: F) -> Vec<CollectionItem>
    where
        F: Fn(&CollectionItem) -> bool,
    {
        self.get_items()
            .into_iter()
            .filter(predicate)
            .collect()
    }
}

/// Provider component for collection context
#[component]
pub fn CollectionProvider(children: Children) -> impl IntoView {
    let collection = CollectionContext::new();
    provide_context(collection);
    
    children()
}

/// Hook to access collection context
pub fn use_collection_context() -> Option<CollectionContext> {
    use_context::<CollectionContext>()
}

/// Hook for roving tabindex management
pub fn use_roving_tabindex(
    collection: CollectionContext,
    current_value: Signal<Option<String>>,
) -> impl Fn(&str) -> i32 {
    move |item_value: &str| {
        match current_value.get() {
            Some(ref current) if current == item_value => 0, // Current item is focusable
            Some(_) => -1, // Other items are not in tab order
            None => {
                // No current item, make first item focusable
                if let Some(first) = collection.get_first_item() {
                    if first.value == item_value {
                        0
                    } else {
                        -1
                    }
                } else {
                    -1
                }
            }
        }
    }
}

/// Hook for keyboard navigation within a collection
pub fn use_collection_keyboard_navigation(
    collection: CollectionContext,
    current_value: RwSignal<Option<String>>,
) -> impl Fn(web_sys::KeyboardEvent) -> bool {
    move |event: web_sys::KeyboardEvent| {
        let current = current_value.get();
        
        match event.key().as_str() {
            "ArrowDown" | "ArrowRight" => {
                event.prevent_default();
                if let Some(next) = collection.get_next_item(current.as_deref()) {
                    current_value.set(Some(next.value.clone()));
                    focus_element(&next.element);
                }
                true
            }
            "ArrowUp" | "ArrowLeft" => {
                event.prevent_default();
                if let Some(prev) = collection.get_previous_item(current.as_deref()) {
                    current_value.set(Some(prev.value.clone()));
                    focus_element(&prev.element);
                }
                true
            }
            "Home" => {
                event.prevent_default();
                if let Some(first) = collection.get_first_item() {
                    current_value.set(Some(first.value.clone()));
                    focus_element(&first.element);
                }
                true
            }
            "End" => {
                event.prevent_default();
                if let Some(last) = collection.get_last_item() {
                    current_value.set(Some(last.value.clone()));
                    focus_element(&last.element);
                }
                true
            }
            _ => false,
        }
    }
}

/// Focus an element safely
fn focus_element(element: &Element) {
    use wasm_bindgen::JsCast;
    if let Ok(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
        let _ = html_element.focus();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[test]
    fn test_collection_context() {
        run_test(|cx| {
            let collection = CollectionContext::new();
            
            // Create mock elements
            let document = web_sys::window().unwrap().document().unwrap();
            let element1 = document.create_element("div").unwrap();
            let element2 = document.create_element("div").unwrap();
            
            // Register items
            collection.register_item(CollectionItem {
                value: "item1".to_string(),
                element: element1,
                disabled: false,
            });
            
            collection.register_item(CollectionItem {
                value: "item2".to_string(),
                element: element2,
                disabled: false,
            });
            
            // Test navigation
            assert_eq!(collection.get_items().len(), 2);
            
            let next = collection.get_next_item(Some("item1"));
            assert!(next.is_some());
            assert_eq!(next.unwrap().value, "item2");
            
            let prev = collection.get_previous_item(Some("item2"));
            assert!(prev.is_some());
            assert_eq!(prev.unwrap().value, "item1");
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