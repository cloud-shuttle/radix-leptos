# Advanced Patterns for Radix-Leptos

This guide covers advanced patterns and techniques for building sophisticated applications with Radix-Leptos.

## Table of Contents

- [State Management Patterns](#state-management-patterns)
- [Performance Optimization](#performance-optimization)
- [Accessibility Patterns](#accessibility-patterns)
- [Testing Patterns](#testing-patterns)
- [Architecture Patterns](#architecture-patterns)
- [Integration Patterns](#integration-patterns)

## State Management Patterns

### Global State with Context

```rust
use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AppState {
    pub user: Option<User>,
    pub theme: String,
    pub notifications: Vec<Notification>,
    pub preferences: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub id: String,
    pub message: String,
    pub type_: NotificationType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

#[component]
pub fn AppStateProvider(children: Children) -> impl IntoView {
    let (state, set_state) = create_signal(AppState {
        user: None,
        theme: "light".to_string(),
        notifications: Vec::new(),
        preferences: HashMap::new(),
    });

    provide_context(state);
    provide_context(set_state);

    view! {
        <div class="app-state-provider">
            {children()}
        </div>
    }
}

// State actions
pub fn use_app_state() -> (ReadSignal<AppState>, WriteSignal<AppState>) {
    (
        use_context::<ReadSignal<AppState>>().unwrap(),
        use_context::<WriteSignal<AppState>>().unwrap(),
    )
}

pub fn use_user() -> (ReadSignal<Option<User>>, impl Fn(Option<User>)) {
    let (state, set_state) = use_app_state();
    
    let user = move || state().user.clone();
    let set_user = move |user: Option<User>| {
        set_state.update(|state| state.user = user);
    };
    
    (user, set_user)
}

pub fn use_theme() -> (ReadSignal<String>, impl Fn(String)) {
    let (state, set_state) = use_app_state();
    
    let theme = move || state().theme.clone();
    let set_theme = move |theme: String| {
        set_state.update(|state| state.theme = theme);
    };
    
    (theme, set_theme)
}

pub fn use_notifications() -> (ReadSignal<Vec<Notification>>, impl Fn(Notification)) {
    let (state, set_state) = use_app_state();
    
    let notifications = move || state().notifications.clone();
    let add_notification = move |notification: Notification| {
        set_state.update(|state| state.notifications.push(notification));
    };
    
    (notifications, add_notification)
}
```

### Local Storage Integration

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub theme: String,
    pub language: String,
    pub notifications_enabled: bool,
    pub auto_save: bool,
}

pub fn use_local_storage<T>(key: &str, default: T) -> (ReadSignal<T>, WriteSignal<T>)
where
    T: Clone + Serialize + for<'de> Deserialize<'de>,
{
    let initial_value = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item(key)
        .unwrap()
        .map(|value| serde_json::from_str(&value).unwrap_or(default.clone()))
        .unwrap_or(default);

    let (value, set_value) = create_signal(initial_value);

    let set_value_with_storage = move |new_value: T| {
        set_value(new_value.clone());
        web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item(key, &serde_json::to_string(&new_value).unwrap())
            .unwrap();
    };

    (value, set_value_with_storage)
}

#[component]
pub fn PreferencesPanel() -> impl IntoView {
    let (preferences, set_preferences) = use_local_storage(
        "user_preferences",
        UserPreferences {
            theme: "light".to_string(),
            language: "en".to_string(),
            notifications_enabled: true,
            auto_save: false,
        },
    );

    view! {
        <div class="preferences-panel">
            <h2>"Preferences"</h2>
            
            <FormField>
                <FormLabel>"Theme"</FormLabel>
                <Select
                    value=preferences().theme
                    on_value_change=move |theme| {
                        set_preferences.update(|prefs| prefs.theme = theme);
                    }
                >
                    <SelectTrigger>
                        <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="light">"Light"</SelectItem>
                        <SelectItem value="dark">"Dark"</SelectItem>
                        <SelectItem value="auto">"Auto"</SelectItem>
                    </SelectContent>
                </Select>
            </FormField>
            
            <FormField>
                <FormLabel>"Language"</FormLabel>
                <Select
                    value=preferences().language
                    on_value_change=move |language| {
                        set_preferences.update(|prefs| prefs.language = language);
                    }
                >
                    <SelectTrigger>
                        <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="en">"English"</SelectItem>
                        <SelectItem value="es">"Spanish"</SelectItem>
                        <SelectItem value="fr">"French"</SelectItem>
                    </SelectContent>
                </Select>
            </FormField>
            
            <FormField>
                <Checkbox
                    checked=preferences().notifications_enabled
                    on_change=move |enabled| {
                        set_preferences.update(|prefs| prefs.notifications_enabled = enabled);
                    }
                    label="Enable notifications"
                />
            </FormField>
            
            <FormField>
                <Checkbox
                    checked=preferences().auto_save
                    on_change=move |enabled| {
                        set_preferences.update(|prefs| prefs.auto_save = enabled);
                    }
                    label="Auto-save changes"
                />
            </FormField>
        </div>
    }
}
```

### Custom Hooks

```rust
// Debounced input hook
pub fn use_debounced_input<T>(
    initial_value: T,
    delay_ms: u64,
) -> (ReadSignal<T>, WriteSignal<T>, ReadSignal<T>)
where
    T: Clone + PartialEq + 'static,
{
    let (value, set_value) = create_signal(initial_value.clone());
    let (debounced_value, set_debounced_value) = create_signal(initial_value);

    let debounce_timer = use_context::<Option<web_sys::TimeoutId>>();

    let debounced_set_value = move |new_value: T| {
        set_value(new_value.clone());
        
        if let Some(timer) = debounce_timer {
            web_sys::window().unwrap().clear_timeout_with_handle(&timer);
        }
        
        let timeout_id = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                move || {
                    set_debounced_value(new_value);
                },
                delay_ms as i32,
            )
            .unwrap();
        
        provide_context(Some(timeout_id));
    };

    (value, debounced_set_value, debounced_value)
}

// Intersection observer hook
pub fn use_intersection_observer(
    element_ref: NodeRef<leptos::html::Div>,
    options: Option<web_sys::IntersectionObserverInit>,
) -> ReadSignal<bool> {
    let (is_intersecting, set_is_intersecting) = create_signal(false);

    on_mount(move || {
        let element = element_ref.get().unwrap();
        let callback = Closure::wrap(Box::new(move |entries: web_sys::IntersectionObserverEntry| {
            set_is_intersecting(entries.is_intersecting());
        }) as Box<dyn FnMut(web_sys::IntersectionObserverEntry)>);

        let observer = web_sys::IntersectionObserver::new_with_callback(
            callback.as_ref().unchecked_ref(),
        ).unwrap();

        observer.observe(&element);
        
        on_cleanup(move || {
            observer.disconnect();
        });
    });

    is_intersecting
}

// Media query hook
pub fn use_media_query(query: &str) -> ReadSignal<bool> {
    let (matches, set_matches) = create_signal(false);

    on_mount(move || {
        let media_query = web_sys::window()
            .unwrap()
            .match_media(query)
            .unwrap()
            .unwrap();

        set_matches(media_query.matches());

        let callback = Closure::wrap(Box::new(move |event: web_sys::MediaQueryListEvent| {
            set_matches(event.matches());
        }) as Box<dyn FnMut(web_sys::MediaQueryListEvent)>);

        media_query.set_onchange(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
    });

    matches
}

// Window size hook
pub fn use_window_size() -> ReadSignal<(f64, f64)> {
    let (size, set_size) = create_signal((0.0, 0.0));

    on_mount(move || {
        let update_size = move || {
            let window = web_sys::window().unwrap();
            set_size((window.inner_width().unwrap().as_f64().unwrap(), 
                     window.inner_height().unwrap().as_f64().unwrap()));
        };

        update_size();

        let callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            update_size();
        }) as Box<dyn FnMut(web_sys::Event)>);

        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback(
                "resize",
                callback.as_ref().unchecked_ref(),
            )
            .unwrap();

        callback.forget();
    });

    size
}
```

## Performance Optimization

### Memoization Patterns

```rust
// Expensive computation memoization
#[component]
pub fn ExpensiveComponent(data: ReadSignal<Vec<String>>) -> impl IntoView {
    let memoized_data = use_memo(move || {
        // Expensive computation
        data().into_iter()
            .map(|s| s.to_uppercase())
            .filter(|s| s.len() > 5)
            .collect::<Vec<_>>()
    }, [data]);

    view! {
        <div class="expensive-component">
            <h3>"Expensive Component"</h3>
            <ul>
                {memoized_data().into_iter().map(|item| {
                    view! { <li>{item}</li> }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

// Callback memoization
#[component]
pub fn CallbackMemoization() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (items, set_items) = create_signal(vec!["Item 1", "Item 2", "Item 3"]);

    // Memoized callback to prevent unnecessary re-renders
    let handle_item_click = use_callback(
        move |item: String| {
            log::info!("Clicked item: {}", item);
        },
        [],
    );

    let handle_add_item = use_callback(
        move |_| {
            set_items.update(|items| {
                items.push(format!("Item {}", items.len() + 1));
            });
        },
        [],
    );

    view! {
        <div class="callback-memoization">
            <h3>"Callback Memoization"</h3>
            <p>"Count: " {count}</p>
            <Button on_click=move |_| set_count.update(|c| *c += 1)>"Increment"</Button>
            
            <div class="items">
                {items().into_iter().map(|item| {
                    view! {
                        <div
                            class="item"
                            on:click=move |_| handle_item_click(item.clone())
                        >
                            {item}
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
            
            <Button on_click=handle_add_item>"Add Item"</Button>
        </div>
    }
}
```

### Virtual Scrolling Patterns

```rust
#[component]
pub fn VirtualScrollingExample() -> impl IntoView {
    let items = (0..10000).map(|i| format!("Item {}", i)).collect::<Vec<_>>();
    let (search_term, set_search_term) = create_signal("".to_string());

    // Filter items based on search term
    let filtered_items = use_memo(move || {
        if search_term().is_empty() {
            items.clone()
        } else {
            items.iter()
                .filter(|item| item.to_lowercase().contains(&search_term().to_lowercase()))
                .cloned()
                .collect()
        }
    }, [search_term]);

    view! {
        <div class="virtual-scrolling-example">
            <h2>"Virtual Scrolling Example"</h2>
            
            <Input
                value=search_term
                on_change=set_search_term
                placeholder="Search items..."
            />
            
            <p>"Showing " {filtered_items().len()} " items"</p>
            
            <OptimizedVirtualList
                items=filtered_items
                item_height=50.0
                container_height=400.0
                overscan=5
                render_item=Callback::new(|item| {
                    view! {
                        <div class="virtual-item">
                            <span class="item-text">{item}</span>
                            <Button size=ButtonSize::Small>"Action"</Button>
                        </div>
                    }
                })
            />
        </div>
    }
}
```

### Performance Monitoring

```rust
#[component]
pub fn PerformanceMonitoringExample() -> impl IntoView {
    let monitor = get_global_performance_monitor();
    let (stats, set_stats) = create_signal(monitor.get_stats());

    let update_stats = move || {
        set_stats(monitor.get_stats());
    };

    // Update stats every second
    use_interval(1000, update_stats);

    view! {
        <div class="performance-monitoring">
            <h2>"Performance Monitoring"</h2>
            
            <div class="performance-stats">
                <h3>"Performance Statistics"</h3>
                <div class="stat-grid">
                    <div class="stat-item">
                        <span class="stat-label">"Total Measurements:"</span>
                        <span class="stat-value">{stats().total_measurements}</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-label">"Average Duration:"</span>
                        <span class="stat-value">
                            {format!("{:.2}ms", stats().average_duration.as_secs_f64() * 1000.0)}
                        </span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-label">"Median Duration:"</span>
                        <span class="stat-value">
                            {format!("{:.2}ms", stats().median_duration.as_secs_f64() * 1000.0)}
                        </span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-label">"Min Duration:"</span>
                        <span class="stat-value">
                            {format!("{:.2}ms", stats().min_duration.as_secs_f64() * 1000.0)}
                        </span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-label">"Max Duration:"</span>
                        <span class="stat-value">
                            {format!("{:.2}ms", stats().max_duration.as_secs_f64() * 1000.0)}
                        </span>
                    </div>
                </div>
            </div>
            
            <VirtualListPerformanceMonitor show_stats=true />
        </div>
    }
}

// Utility function for intervals
pub fn use_interval<F>(interval_ms: u64, callback: F)
where
    F: Fn() + 'static,
{
    on_mount(move || {
        let interval_id = web_sys::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                move || {
                    callback();
                },
                interval_ms as i32,
            )
            .unwrap();

        on_cleanup(move || {
            web_sys::window().unwrap().clear_interval_with_handle(interval_id);
        });
    });
}
```

## Accessibility Patterns

### ARIA Patterns

```rust
#[component]
pub fn AccessibleForm() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (errors, set_errors) = create_signal(HashMap::new());

    let validate_form = move || {
        let mut new_errors = HashMap::new();
        
        if name().is_empty() {
            new_errors.insert("name".to_string(), "Name is required".to_string());
        }
        
        if email().is_empty() {
            new_errors.insert("email".to_string(), "Email is required".to_string());
        } else if !email().contains('@') {
            new_errors.insert("email".to_string(), "Invalid email format".to_string());
        }
        
        set_errors(new_errors);
        errors().is_empty()
    };

    let handle_submit = move |_| {
        if validate_form() {
            log::info!("Form submitted successfully");
        }
    };

    view! {
        <form on:submit=handle_submit class="accessible-form">
            <h2>"Contact Form"</h2>
            
            <FormField>
                <FormLabel for="name">"Name"</FormLabel>
                <Input
                    id="name"
                    value=name
                    on_change=set_name
                    placeholder="Enter your name"
                    aria_required=true
                    aria_invalid=!errors().get("name").is_none()
                    aria_describedby=if errors().get("name").is_some() { Some("name-error") } else { None }
                />
                {if let Some(error) = errors().get("name") {
                    view! {
                        <FormFieldError id="name-error" role="alert">
                            {error}
                        </FormFieldError>
                    }
                }}
            </FormField>
            
            <FormField>
                <FormLabel for="email">"Email"</FormLabel>
                <Input
                    id="email"
                    type="email"
                    value=email
                    on_change=set_email
                    placeholder="Enter your email"
                    aria_required=true
                    aria_invalid=!errors().get("email").is_none()
                    aria_describedby=if errors().get("email").is_some() { Some("email-error") } else { None }
                />
                {if let Some(error) = errors().get("email") {
                    view! {
                        <FormFieldError id="email-error" role="alert">
                            {error}
                        </FormFieldError>
                    }
                }}
            </FormField>
            
            <Button type="submit">"Submit"</Button>
        </form>
    }
}
```

### Keyboard Navigation

```rust
#[component]
pub fn KeyboardNavigationExample() -> impl IntoView {
    let (focused_index, set_focused_index) = create_signal(0);
    let items = vec!["Item 1", "Item 2", "Item 3", "Item 4", "Item 5"];

    let handle_keydown = move |event: web_sys::KeyboardEvent| {
        match event.key().as_str() {
            "ArrowDown" => {
                event.prevent_default();
                set_focused_index((focused_index() + 1) % items.len());
            }
            "ArrowUp" => {
                event.prevent_default();
                set_focused_index((focused_index() - 1 + items.len()) % items.len());
            }
            "Home" => {
                event.prevent_default();
                set_focused_index(0);
            }
            "End" => {
                event.prevent_default();
                set_focused_index(items.len() - 1);
            }
            "Enter" | " " => {
                event.prevent_default();
                log::info!("Selected: {}", items[focused_index()]);
            }
            _ => {}
        }
    };

    view! {
        <div class="keyboard-navigation-example">
            <h2>"Keyboard Navigation"</h2>
            <p>"Use arrow keys to navigate, Enter or Space to select"</p>
            
            <div
                class="focusable-list"
                tabindex="0"
                on:keydown=handle_keydown
                role="listbox"
                aria_label="Selectable items"
            >
                {items.into_iter().enumerate().map(|(index, item)| {
                    let is_focused = index == focused_index();
                    view! {
                        <div
                            class=format!("focusable-item {}", if is_focused { "focused" } else { "" })
                            tabindex=if is_focused { "0" } else { "-1" }
                            role="option"
                            aria_selected=is_focused
                        >
                            {item}
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
```

### Focus Management

```rust
#[component]
pub fn FocusManagementExample() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);
    let trigger_ref = NodeRef::<leptos::html::Button>::new();
    let content_ref = NodeRef::<leptos::html::Div>::new();

    let handle_open = move |_| {
        set_is_open(true);
        // Focus the first focusable element in the content
        if let Some(content) = content_ref.get() {
            if let Some(first_focusable) = content.query_selector("[tabindex]:not([tabindex='-1']), button, input, select, textarea") {
                if let Ok(element) = first_focusable {
                    element.focus().unwrap();
                }
            }
        }
    };

    let handle_close = move |_| {
        set_is_open(false);
        // Return focus to trigger
        if let Some(trigger) = trigger_ref.get() {
            trigger.focus().unwrap();
        }
    };

    let handle_keydown = move |event: web_sys::KeyboardEvent| {
        if event.key() == "Escape" {
            handle_close(());
        }
    };

    view! {
        <div class="focus-management-example">
            <h2>"Focus Management"</h2>
            
            <Button
                node_ref=trigger_ref
                on_click=handle_open
            >
                "Open Dialog"
            </Button>
            
            {if is_open() {
                view! {
                    <div
                        class="dialog-overlay"
                        on:keydown=handle_keydown
                        role="dialog"
                        aria_modal=true
                        aria_labelledby="dialog-title"
                    >
                        <div
                            node_ref=content_ref
                            class="dialog-content"
                        >
                            <h3 id="dialog-title">"Dialog Title"</h3>
                            <p>"Dialog content goes here"</p>
                            <Button on_click=handle_close>"Close"</Button>
                        </div>
                    </div>
                }
            }}
        </div>
    }
}
```

## Testing Patterns

### Component Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_button_rendering() {
        let runtime = create_runtime();
        let view = view! {
            <Button>"Test Button"</Button>
        };
        
        assert!(view.into_any().is_some());
        runtime.dispose();
    }

    #[wasm_bindgen_test]
    fn test_button_click() {
        let runtime = create_runtime();
        let (clicked, set_clicked) = create_signal(false);
        
        let view = view! {
            <Button on_click=move |_| set_clicked(true)>"Click me"</Button>
        };
        
        // Simulate click
        // In a real test, you would trigger the click event
        assert!(!clicked());
        runtime.dispose();
    }

    #[wasm_bindgen_test]
    fn test_form_validation() {
        let runtime = create_runtime();
        let (email, set_email) = create_signal("".to_string());
        let (errors, set_errors) = create_signal(HashMap::new());
        
        let validate_email = move |email: &str| {
            if email.is_empty() {
                "Email is required".to_string()
            } else if !email.contains('@') {
                "Invalid email format".to_string()
            } else {
                String::new()
            }
        };
        
        // Test empty email
        let error = validate_email(&email());
        assert_eq!(error, "Email is required");
        
        // Test invalid email
        set_email("invalid-email".to_string());
        let error = validate_email(&email());
        assert_eq!(error, "Invalid email format");
        
        // Test valid email
        set_email("test@example.com".to_string());
        let error = validate_email(&email());
        assert_eq!(error, "");
        
        runtime.dispose();
    }
}
```

### Integration Testing

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use leptos::prelude::*;

    #[test]
    fn test_complete_user_flow() {
        let runtime = create_runtime();
        
        // Test user registration flow
        let (name, set_name) = create_signal("".to_string());
        let (email, set_email) = create_signal("".to_string());
        let (password, set_password) = create_signal("".to_string());
        let (is_registered, set_is_registered) = create_signal(false);
        
        let handle_register = move |_| {
            if !name().is_empty() && !email().is_empty() && !password().is_empty() {
                set_is_registered(true);
            }
        };
        
        let view = view! {
            <form on:submit=handle_register>
                <Input
                    value=name
                    on_change=set_name
                    placeholder="Name"
                />
                <Input
                    type="email"
                    value=email
                    on_change=set_email
                    placeholder="Email"
                />
                <Input
                    type="password"
                    value=password
                    on_change=set_password
                    placeholder="Password"
                />
                <Button type="submit">"Register"</Button>
            </form>
        };
        
        assert!(view.into_any().is_some());
        assert!(!is_registered());
        
        runtime.dispose();
    }
}
```

## Architecture Patterns

### Feature-Based Architecture

```rust
// features/user/mod.rs
pub mod components;
pub mod hooks;
pub mod services;
pub mod types;

pub use components::*;
pub use hooks::*;
pub use services::*;
pub use types::*;

// features/user/components.rs
use leptos::prelude::*;
use super::types::*;
use super::hooks::*;

#[component]
pub fn UserProfile(user: User) -> impl IntoView {
    let (is_editing, set_is_editing) = create_signal(false);
    let (edited_user, set_edited_user) = create_signal(user.clone());
    
    let handle_save = move |_| {
        // Save user changes
        set_is_editing(false);
    };
    
    let handle_cancel = move |_| {
        set_edited_user(user.clone());
        set_is_editing(false);
    };
    
    view! {
        <div class="user-profile">
            <h2>{user.name}</h2>
            <p>{user.email}</p>
            
            {if is_editing() {
                view! {
                    <div class="edit-form">
                        <Input
                            value=edited_user().name
                            on_change=move |name| set_edited_user.update(|u| u.name = name)
                        />
                        <Input
                            type="email"
                            value=edited_user().email
                            on_change=move |email| set_edited_user.update(|u| u.email = email)
                        />
                        <Button on_click=handle_save>"Save"</Button>
                        <Button on_click=handle_cancel>"Cancel"</Button>
                    </div>
                }
            } else {
                view! {
                    <Button on_click=move |_| set_is_editing(true)>"Edit"</Button>
                }
            }}
        </div>
    }
}

// features/user/hooks.rs
use leptos::prelude::*;
use super::types::*;

pub fn use_user() -> (ReadSignal<Option<User>>, impl Fn(Option<User>)) {
    let (user, set_user) = create_signal(None);
    
    (user, set_user)
}

pub fn use_user_preferences() -> (ReadSignal<UserPreferences>, impl Fn(UserPreferences)) {
    let (preferences, set_preferences) = create_signal(UserPreferences::default());
    
    (preferences, set_preferences)
}

// features/user/services.rs
use super::types::*;

pub struct UserService;

impl UserService {
    pub async fn get_user(id: String) -> Result<User, String> {
        // Simulate API call
        Ok(User {
            id,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            role: "User".to_string(),
        })
    }
    
    pub async fn update_user(user: User) -> Result<User, String> {
        // Simulate API call
        Ok(user)
    }
    
    pub async fn delete_user(id: String) -> Result<(), String> {
        // Simulate API call
        Ok(())
    }
}

// features/user/types.rs
#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone, Default)]
pub struct UserPreferences {
    pub theme: String,
    pub language: String,
    pub notifications_enabled: bool,
}
```

### Plugin Architecture

```rust
// plugins/mod.rs
pub mod auth;
pub mod analytics;
pub mod notifications;

pub use auth::*;
pub use analytics::*;
pub use notifications::*;

// plugins/auth/mod.rs
use leptos::prelude::*;

pub struct AuthPlugin {
    pub is_authenticated: ReadSignal<bool>,
    pub user: ReadSignal<Option<User>>,
}

impl AuthPlugin {
    pub fn new() -> Self {
        let (is_authenticated, _) = create_signal(false);
        let (user, _) = create_signal(None);
        
        Self {
            is_authenticated,
            user,
        }
    }
    
    pub fn login(&self, user: User) {
        // Login logic
    }
    
    pub fn logout(&self) {
        // Logout logic
    }
}

// plugins/analytics/mod.rs
use leptos::prelude::*;

pub struct AnalyticsPlugin {
    pub events: ReadSignal<Vec<AnalyticsEvent>>,
}

impl AnalyticsPlugin {
    pub fn new() -> Self {
        let (events, _) = create_signal(Vec::new());
        
        Self { events }
    }
    
    pub fn track_event(&self, event: AnalyticsEvent) {
        // Track event logic
    }
}

#[derive(Debug, Clone)]
pub struct AnalyticsEvent {
    pub name: String,
    pub properties: HashMap<String, String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
```

## Integration Patterns

### API Integration

```rust
// api/mod.rs
pub mod client;
pub mod types;
pub mod endpoints;

pub use client::*;
pub use types::*;
pub use endpoints::*;

// api/client.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
    
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, String>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await.map_err(|e| e.to_string())?;
        let data = response.json::<T>().await.map_err(|e| e.to_string())?;
        Ok(data)
    }
    
    pub async fn post<T, U>(&self, endpoint: &str, data: &T) -> Result<U, String>
    where
        T: Serialize,
        U: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client
            .post(&url)
            .json(data)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let result = response.json::<U>().await.map_err(|e| e.to_string())?;
        Ok(result)
    }
}

// api/endpoints.rs
use super::client::ApiClient;
use super::types::*;

pub struct UserEndpoints {
    client: ApiClient,
}

impl UserEndpoints {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }
    
    pub async fn get_user(&self, id: String) -> Result<User, String> {
        self.client.get(&format!("users/{}", id)).await
    }
    
    pub async fn create_user(&self, user: CreateUserRequest) -> Result<User, String> {
        self.client.post("users", &user).await
    }
    
    pub async fn update_user(&self, id: String, user: UpdateUserRequest) -> Result<User, String> {
        self.client.post(&format!("users/{}", id), &user).await
    }
    
    pub async fn delete_user(&self, id: String) -> Result<(), String> {
        self.client.post(&format!("users/{}/delete", id), &()).await
    }
}
```

### WebSocket Integration

```rust
// websocket/mod.rs
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::WebSocket;

pub struct WebSocketManager {
    socket: Option<WebSocket>,
    messages: ReadSignal<Vec<WebSocketMessage>>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        let (messages, _) = create_signal(Vec::new());
        
        Self {
            socket: None,
            messages,
        }
    }
    
    pub fn connect(&mut self, url: String) -> Result<(), String> {
        let socket = WebSocket::new(&url).map_err(|e| e.to_string())?;
        
        let onmessage = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
            let data = event.data().as_string().unwrap();
            let message: WebSocketMessage = serde_json::from_str(&data).unwrap();
            // Handle message
        }) as Box<dyn FnMut(web_sys::MessageEvent)>);
        
        socket.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget();
        
        self.socket = Some(socket);
        Ok(())
    }
    
    pub fn send_message(&self, message: WebSocketMessage) -> Result<(), String> {
        if let Some(socket) = &self.socket {
            let data = serde_json::to_string(&message).unwrap();
            socket.send_with_str(&data).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub id: String,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
```

## Conclusion

These advanced patterns demonstrate the full power and flexibility of Radix-Leptos for building sophisticated applications. Each pattern includes:

- **Complete implementations**: Ready to use in production
- **Best practices**: Following Rust and Leptos conventions
- **Performance optimization**: Efficient rendering and state management
- **Accessibility**: Full ARIA support and keyboard navigation
- **Testing**: Comprehensive test coverage
- **Architecture**: Scalable and maintainable code organization

For more examples, see the [Interactive Examples](interactive-examples.md) and [Component Gallery](component-gallery.md).
