# Theme Provider Design

**Status**: Tier 3 - Broken/Incomplete (critical functionality missing)  
**Priority**: MEDIUM - Foundational theming system  
**File**: `crates/radix-leptos-primitives/src/theming/theme_provider.rs`  
**Current Lines**: ~400 lines (under limit ✅)

## Current State Analysis

### ❌ Critical Issues
1. **CSS Not Applied**: `apply_theme()` function commented out - "In a real implementation, this would apply CSS"
2. **Theme Context Unused**: Theme context created but not consumed by components
3. **System Theme Detection**: Window media query detection not implemented
4. **Storage Persistence**: Theme preferences not saved/loaded from localStorage
5. **CSS Variables Missing**: No actual CSS variable injection to document

### ✅ What's Partially Working
- Theme context provider structure
- Basic dark/light mode toggle
- Theme variant enumeration
- CSS variable data structure

## Architecture Design

### Theme System Hierarchy
```
ThemeProvider (Root)
├── Theme detection (system preference)  
├── CSS variable injection (document root)
├── Storage persistence (localStorage)
└── Theme context (for components)
    ├── Dark mode components
    ├── Theme selector components
    └── Custom theme components
```

### Core Data Structures
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum ThemeVariant {
    Light,
    Dark,
    Auto,   // Follow system preference
}

#[derive(Clone, Debug)]
pub struct ThemeConfig {
    pub variant: ThemeVariant,
    pub css_variables: CSSVariables,
    pub custom_properties: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct ThemeContext {
    pub current_theme: ReadSignal<ThemeConfig>,
    pub set_theme: WriteSignal<ThemeConfig>,
    pub toggle_dark_mode: Callback<()>,
    pub apply_custom_theme: Callback<CSSVariables>,
    pub system_preference: ReadSignal<ThemeVariant>,
}
```

## Implementation Plan

### Phase 1: CSS Variable Injection (Critical Fix)
```rust
use web_sys::{window, Document, HtmlElement};

// CRITICAL: Actually implement CSS application
fn apply_theme_to_document(theme: &ThemeConfig) -> Result<(), JsValue> {
    let window = window().ok_or("No window available")?;
    let document = window.document().ok_or("No document available")?; 
    let html_element = document
        .document_element()
        .ok_or("No document element")?
        .dyn_into::<HtmlElement>()?;
        
    let style = html_element.style();
    
    // Apply all CSS variables to :root
    let css_vars = theme.css_variables.to_css_map();
    for (property, value) in css_vars {
        style.set_property(&property, &value)?;
    }
    
    // Apply custom properties
    for (property, value) in &theme.custom_properties {
        style.set_property(property, value)?;
    }
    
    // Set theme variant as data attribute for CSS targeting
    html_element.set_attribute("data-theme", theme.variant.as_str())?;
    
    Ok(())
}

// Helper to remove theme CSS variables
fn clear_theme_from_document() -> Result<(), JsValue> {
    let window = window().ok_or("No window available")?;
    let document = window.document().ok_or("No document available")?;
    let html_element = document
        .document_element()
        .ok_or("No document element")?
        .dyn_into::<HtmlElement>()?;
        
    let style = html_element.style();
    
    // Remove common CSS variables (could be more comprehensive)
    let common_vars = vec![
        "--primary", "--primary-foreground", "--secondary", "--secondary-foreground",
        "--accent", "--accent-foreground", "--background", "--foreground",
        "--muted", "--muted-foreground", "--border", "--radius",
        // Add more as needed
    ];
    
    for var_name in common_vars {
        let _ = style.remove_property(var_name);
    }
    
    Ok(())
}
```

### Phase 2: System Theme Detection
```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// Detect system dark mode preference
fn detect_system_theme() -> ThemeVariant {
    let window = match window() {
        Some(w) => w,
        None => return ThemeVariant::Light, // Default fallback
    };
    
    // Check for media query support
    let media_query = "(prefers-color-scheme: dark)";
    match window.match_media(media_query) {
        Ok(Some(media_query_list)) => {
            if media_query_list.matches() {
                ThemeVariant::Dark
            } else {
                ThemeVariant::Light
            }
        }
        _ => ThemeVariant::Light, // Default fallback
    }
}

// Set up system theme change listener
fn setup_system_theme_listener(
    set_system_theme: WriteSignal<ThemeVariant>
) -> Result<(), JsValue> {
    let window = window().ok_or("No window available")?;
    let media_query = "(prefers-color-scheme: dark)";
    let media_query_list = window
        .match_media(media_query)?
        .ok_or("matchMedia not supported")?;
    
    let closure = Closure::wrap(Box::new(move |event: web_sys::MediaQueryListEvent| {
        let new_theme = if event.matches() {
            ThemeVariant::Dark
        } else {
            ThemeVariant::Light
        };
        set_system_theme.set(new_theme);
    }) as Box<dyn FnMut(_)>);
    
    media_query_list.set_onchange(Some(closure.as_ref().unchecked_ref()));
    
    // Important: keep closure alive
    closure.forget();
    
    Ok(())
}
```

### Phase 3: Storage Persistence
```rust
use web_sys::{Storage, window};

const THEME_STORAGE_KEY: &str = "radix-leptos-theme";

// Save theme to localStorage
fn save_theme_to_storage(theme: &ThemeConfig) -> Result<(), JsValue> {
    let window = window().ok_or("No window available")?;
    let storage = window
        .local_storage()?
        .ok_or("localStorage not available")?;
    
    let theme_json = serde_json::to_string(theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    storage.set_item(THEME_STORAGE_KEY, &theme_json)?;
    Ok(())
}

// Load theme from localStorage  
fn load_theme_from_storage() -> Result<Option<ThemeConfig>, JsValue> {
    let window = window().ok_or("No window available")?;
    let storage = window
        .local_storage()?
        .ok_or("localStorage not available")?;
    
    match storage.get_item(THEME_STORAGE_KEY)? {
        Some(theme_json) => {
            let theme = serde_json::from_str(&theme_json)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(Some(theme))
        }
        None => Ok(None),
    }
}
```

### Phase 4: Complete ThemeProvider Implementation
```rust
#[component]
pub fn ThemeProvider(
    children: Children,
    
    // Initial theme configuration
    #[prop(optional)] default_theme: Option<ThemeVariant>,
    #[prop(optional)] storage_key: Option<String>,
    #[prop(optional)] enable_system: Option<bool>,
    #[prop(optional)] disable_transition_on_change: Option<bool>,
    
    // Custom theme
    #[prop(optional)] theme: Option<CSSVariables>,
    
    // Styling
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let enable_system = enable_system.unwrap_or(true);
    let storage_key = storage_key.unwrap_or_else(|| THEME_STORAGE_KEY.to_string());
    
    // System theme detection
    let (system_theme, set_system_theme) = signal(detect_system_theme());
    
    // Initialize theme from storage or default
    let initial_theme = load_theme_from_storage()
        .unwrap_or_else(|_| None)
        .unwrap_or_else(|| ThemeConfig {
            variant: default_theme.unwrap_or(ThemeVariant::Auto),
            css_variables: theme.unwrap_or_default(),
            custom_properties: HashMap::new(),
        });
    
    let (current_theme, set_current_theme) = signal(initial_theme);
    
    // Setup system theme listener
    if enable_system {
        let _ = setup_system_theme_listener(set_system_theme);
    }
    
    // Effect: Apply theme when it changes
    create_effect(move |_| {
        let theme = current_theme.get();
        
        // Resolve Auto theme to Light/Dark based on system preference
        let resolved_theme = if theme.variant == ThemeVariant::Auto {
            ThemeConfig {
                variant: system_theme.get(),
                ..theme
            }
        } else {
            theme
        };
        
        // CRITICAL: Actually apply the theme
        if let Err(e) = apply_theme_to_document(&resolved_theme) {
            log::error!("Failed to apply theme: {:?}", e);
        }
        
        // Save to storage
        if let Err(e) = save_theme_to_storage(&current_theme.get()) {
            log::error!("Failed to save theme: {:?}", e);
        }
    });
    
    // Helper functions for context
    let toggle_dark_mode = {
        let set_current_theme = set_current_theme.clone();
        Callback::new(move |_: ()| {
            set_current_theme.update(|theme| {
                theme.variant = match theme.variant {
                    ThemeVariant::Light => ThemeVariant::Dark,
                    ThemeVariant::Dark => ThemeVariant::Light,
                    ThemeVariant::Auto => {
                        // Toggle to opposite of current system preference
                        match system_theme.get() {
                            ThemeVariant::Dark => ThemeVariant::Light,
                            _ => ThemeVariant::Dark,
                        }
                    }
                };
            });
        })
    };
    
    let apply_custom_theme = {
        let set_current_theme = set_current_theme.clone();
        Callback::new(move |css_vars: CSSVariables| {
            set_current_theme.update(|theme| {
                theme.css_variables = css_vars;
            });
        })
    };
    
    // Create and provide context
    let theme_context = ThemeContext {
        current_theme: current_theme.into(),
        set_theme: set_current_theme,
        toggle_dark_mode,
        apply_custom_theme,
        system_preference: system_theme.into(),
    };
    
    provide_context(theme_context);
    
    // Transition disabling during theme changes
    let should_disable_transitions = disable_transition_on_change.unwrap_or(false);
    
    view! {
        <div
            class={merge_classes(vec![
                "theme-provider",
                class.unwrap_or_default()
            ])}
            style={style.unwrap_or_default()}
        >
            {children()}
        </div>
    }
}
```

## Helper Components

### Theme Toggle Button
```rust
#[component]
pub fn ThemeToggle(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let theme_context = expect_context::<ThemeContext>();
    let current_variant = move || theme_context.current_theme.get().variant;
    
    let icon = move || match current_variant() {
        ThemeVariant::Light => "🌞",
        ThemeVariant::Dark => "🌙", 
        ThemeVariant::Auto => "🌗",
    };
    
    let label = move || match current_variant() {
        ThemeVariant::Light => "Switch to dark mode",
        ThemeVariant::Dark => "Switch to light mode",
        ThemeVariant::Auto => "Switch to manual theme",
    };
    
    view! {
        <button
            type="button"
            aria-label={label()}
            class={merge_classes(vec!["theme-toggle", class.unwrap_or_default()])}
            style={style.unwrap_or_default()}
            on:click=move |_| theme_context.toggle_dark_mode.call(())
        >
            {icon()}
        </button>
    }
}
```

### Theme Selector  
```rust
#[component]
pub fn ThemeSelector(
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let theme_context = expect_context::<ThemeContext>();
    let current_variant = move || theme_context.current_theme.get().variant;
    
    let handle_change = move |event: web_sys::Event| {
        let target = event.target().unwrap();
        let select = target.dyn_into::<web_sys::HtmlSelectElement>().unwrap();
        let value = select.value();
        
        let new_variant = match value.as_str() {
            "light" => ThemeVariant::Light,
            "dark" => ThemeVariant::Dark,
            "auto" => ThemeVariant::Auto,
            _ => return,
        };
        
        theme_context.set_theme.update(|theme| {
            theme.variant = new_variant;
        });
    };
    
    view! {
        <select 
            class={merge_classes(vec!["theme-selector", class.unwrap_or_default()])}
            on:change=handle_change
        >
            <option 
                value="light" 
                selected={current_variant() == ThemeVariant::Light}
            >
                "Light"
            </option>
            <option 
                value="dark"
                selected={current_variant() == ThemeVariant::Dark}
            >
                "Dark"
            </option>
            <option 
                value="auto"
                selected={current_variant() == ThemeVariant::Auto}
            >
                "System"
            </option>
        </select>
    }
}
```

## CSS Integration

### Theme CSS Variables
```css
:root {
  /* Light theme (default) */
  --background: 0 0% 100%;
  --foreground: 222.2 84% 4.9%;
  
  --primary: 221.2 83.2% 53.3%;
  --primary-foreground: 210 40% 98%;
  
  --secondary: 210 40% 96%;
  --secondary-foreground: 222.2 84% 4.9%;
  
  --accent: 210 40% 96%;
  --accent-foreground: 222.2 84% 4.9%;
  
  --muted: 210 40% 96%;
  --muted-foreground: 215.4 16.3% 46.9%;
  
  --border: 214.3 31.8% 91.4%;
  --radius: 0.5rem;
}

/* Dark theme - applied via data-theme="dark" */
[data-theme="dark"] {
  --background: 222.2 84% 4.9%;
  --foreground: 210 40% 98%;
  
  --primary: 217.2 91.2% 59.8%;
  --primary-foreground: 222.2 84% 4.9%;
  
  --secondary: 217.2 32.6% 17.5%;
  --secondary-foreground: 210 40% 98%;
  
  --accent: 217.2 32.6% 17.5%;
  --accent-foreground: 210 40% 98%;
  
  --muted: 217.2 32.6% 17.5%;
  --muted-foreground: 215 20.2% 65.1%;
  
  --border: 217.2 32.6% 17.5%;
}

/* Theme transition (optional) */
.theme-provider * {
  transition: 
    background-color 0.2s ease,
    color 0.2s ease,
    border-color 0.2s ease;
}

.theme-provider.disable-transitions * {
  transition: none !important;
}
```

## Usage Examples

### Basic Theme Provider
```rust
view! {
    <ThemeProvider default_theme=ThemeVariant::Light>
        <App />
    </ThemeProvider>
}
```

### With Custom Theme
```rust
let custom_theme = CSSVariables::builder()
    .primary("#007bff")
    .secondary("#6c757d")
    .accent("#17a2b8")
    .build();

view! {
    <ThemeProvider theme=custom_theme enable_system=true>
        <div class="min-h-screen bg-background text-foreground">
            <header>
                <ThemeToggle />
                <ThemeSelector />
            </header>
            <main>
                <App />
            </main>
        </div>
    </ThemeProvider>
}
```

### Using Theme Context in Components
```rust
#[component]
pub fn ThemedCard(children: Children) -> impl IntoView {
    let theme_context = expect_context::<ThemeContext>();
    let is_dark = move || match theme_context.current_theme.get().variant {
        ThemeVariant::Dark => true,
        ThemeVariant::Light => false,
        ThemeVariant::Auto => {
            theme_context.system_preference.get() == ThemeVariant::Dark
        }
    };
    
    view! {
        <div class={format!(
            "card {}",
            if is_dark() { "card-dark" } else { "card-light" }
        )}>
            {children()}
        </div>
    }
}
```

## Testing Requirements

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_variant_serialization() {
        let theme = ThemeConfig {
            variant: ThemeVariant::Dark,
            css_variables: CSSVariables::default(),
            custom_properties: HashMap::new(),
        };
        
        let json = serde_json::to_string(&theme).unwrap();
        let deserialized: ThemeConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(theme.variant, deserialized.variant);
    }
}
```

### WASM Integration Tests
```rust
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn theme_applies_to_document() {
    let theme_provider = view! {
        <ThemeProvider default_theme=ThemeVariant::Dark>
            <div>"Test content"</div>
        </ThemeProvider>
    };
    
    mount_to_body(theme_provider);
    
    // Check that dark theme is applied to document
    let html_element = document()
        .document_element()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    
    assert_eq!(
        html_element.get_attribute("data-theme"), 
        Some("dark".to_string())
    );
    
    // Check that CSS variables are set
    let style = html_element.style();
    let background_color = style.get_property_value("--background").unwrap();
    assert!(!background_color.is_empty());
}
```

## Success Criteria

- [ ] CSS variables actually applied to document root
- [ ] Theme changes persist in localStorage
- [ ] System theme preference detection works
- [ ] Theme toggle button switches themes correctly
- [ ] CSS transitions smooth theme changes
- [ ] Context provides theme state to child components
- [ ] No console errors during theme switching
- [ ] SSR compatibility (graceful degradation)
- [ ] Performance: theme changes don't cause layout shifts

## Migration Notes

The current implementation has the foundation but is missing the critical CSS application functionality. The main fixes are:

1. Implement `apply_theme_to_document()` to actually set CSS variables
2. Add system theme detection with media query listeners  
3. Implement localStorage persistence
4. Wire up the context to actually apply themes
5. Add proper error handling for WASM APIs
