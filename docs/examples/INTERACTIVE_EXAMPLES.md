# Interactive Examples for Radix-Leptos

This document provides comprehensive interactive examples for all Radix-Leptos components, demonstrating best practices and advanced usage patterns.

## Table of Contents

- [Getting Started](#getting-started)
- [Component Examples](#component-examples)
- [Advanced Patterns](#advanced-patterns)
- [Performance Examples](#performance-examples)
- [Accessibility Examples](#accessibility-examples)

## Getting Started

### Basic Setup

```rust
use leptos::prelude::*;
use radix_leptos_primitives::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app">
            <h1>"Welcome to Radix-Leptos"</h1>
            <Button>"Click me!"</Button>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
```

### Cargo.toml Configuration

```toml
[dependencies]
leptos = "0.6"
radix-leptos-primitives = "0.9.0"
radix-leptos-core = "0.9.0"
```

## Component Examples

### Button Components

#### Basic Button

```rust
use radix_leptos_primitives::components::Button;

#[component]
pub fn BasicButtonExample() -> impl IntoView {
    view! {
        <div class="button-examples">
            <h2>"Basic Buttons"</h2>
            
            <Button>"Default Button"</Button>
            <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
            <Button variant=ButtonVariant::Outline>"Outline"</Button>
            <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
            <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
            <Button variant=ButtonVariant::Link>"Link"</Button>
        </div>
    }
}
```

#### Button with Icons

```rust
#[component]
pub fn ButtonWithIconsExample() -> impl IntoView {
    view! {
        <div class="button-icon-examples">
            <h2>"Buttons with Icons"</h2>
            
            <Button>
                <svg class="icon" viewBox="0 0 24 24">
                    <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
                </svg>
                "Star"
            </Button>
            
            <Button>
                "Download"
                <svg class="icon" viewBox="0 0 24 24">
                    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                    <polyline points="7,10 12,15 17,10"/>
                    <line x1="12" y1="15" x2="12" y2="3"/>
                </svg>
            </Button>
        </div>
    }
}
```

#### Button Sizes

```rust
#[component]
pub fn ButtonSizesExample() -> impl IntoView {
    view! {
        <div class="button-size-examples">
            <h2>"Button Sizes"</h2>
            
            <Button size=ButtonSize::Small>"Small"</Button>
            <Button size=ButtonSize::Default>"Default"</Button>
            <Button size=ButtonSize::Large>"Large"</Button>
            <Button size=ButtonSize::Icon>
                <svg class="icon" viewBox="0 0 24 24">
                    <circle cx="12" cy="12" r="3"/>
                </svg>
            </Button>
        </div>
    }
}
```

### Form Components

#### Basic Form

```rust
use radix_leptos_primitives::components::*;

#[component]
pub fn BasicFormExample() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (message, set_message) = create_signal("".to_string());

    let handle_submit = move |_| {
        log::info!("Form submitted: {} - {} - {}", name(), email(), message());
    };

    view! {
        <form on:submit=handle_submit class="form-example">
            <h2>"Contact Form"</h2>
            
            <FormField>
                <FormLabel>"Name"</FormLabel>
                <Input
                    value=name
                    on_change=move |value| set_name(value)
                    placeholder="Enter your name"
                />
            </FormField>
            
            <FormField>
                <FormLabel>"Email"</FormLabel>
                <Input
                    type="email"
                    value=email
                    on_change=move |value| set_email(value)
                    placeholder="Enter your email"
                />
            </FormField>
            
            <FormField>
                <FormLabel>"Message"</FormLabel>
                <Textarea
                    value=message
                    on_change=move |value| set_message(value)
                    placeholder="Enter your message"
                    rows=4
                />
            </FormField>
            
            <Button type="submit">"Submit"</Button>
        </form>
    }
}
```

#### Form with Validation

```rust
#[component]
pub fn FormValidationExample() -> impl IntoView {
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
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

    let validate_password = move |password: &str| {
        if password.is_empty() {
            "Password is required".to_string()
        } else if password.len() < 8 {
            "Password must be at least 8 characters".to_string()
        } else {
            String::new()
        }
    };

    let handle_submit = move |_| {
        let mut new_errors = HashMap::new();
        
        let email_error = validate_email(&email());
        if !email_error.is_empty() {
            new_errors.insert("email".to_string(), email_error);
        }
        
        let password_error = validate_password(&password());
        if !password_error.is_empty() {
            new_errors.insert("password".to_string(), password_error);
        }
        
        set_errors(new_errors);
        
        if errors().is_empty() {
            log::info!("Form is valid!");
        }
    };

    view! {
        <form on:submit=handle_submit class="form-validation-example">
            <h2>"Form with Validation"</h2>
            
            <FormField>
                <FormLabel>"Email"</FormLabel>
                <Input
                    type="email"
                    value=email
                    on_change=move |value| set_email(value)
                    placeholder="Enter your email"
                />
                {if let Some(error) = errors().get("email") {
                    view! { <FormFieldError>{error}</FormFieldError> }
                }}
            </FormField>
            
            <FormField>
                <FormLabel>"Password"</FormLabel>
                <Input
                    type="password"
                    value=password
                    on_change=move |value| set_password(value)
                    placeholder="Enter your password"
                />
                {if let Some(error) = errors().get("password") {
                    view! { <FormFieldError>{error}</FormFieldError> }
                }}
            </FormField>
            
            <Button type="submit">"Submit"</Button>
        </form>
    }
}
```

### Navigation Components

#### Tabs

```rust
#[component]
pub fn TabsExample() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal("tab1".to_string());

    view! {
        <div class="tabs-example">
            <h2>"Tabs Example"</h2>
            
            <Tabs value=active_tab on_value_change=set_active_tab>
                <TabsList>
                    <TabsTrigger value="tab1">"Overview"</TabsTrigger>
                    <TabsTrigger value="tab2">"Details"</TabsTrigger>
                    <TabsTrigger value="tab3">"Settings"</TabsTrigger>
                </TabsList>
                
                <TabsContent value="tab1">
                    <div class="tab-content">
                        <h3>"Overview"</h3>
                        <p>"This is the overview content."</p>
                    </div>
                </TabsContent>
                
                <TabsContent value="tab2">
                    <div class="tab-content">
                        <h3>"Details"</h3>
                        <p>"This is the details content."</p>
                    </div>
                </TabsContent>
                
                <TabsContent value="tab3">
                    <div class="tab-content">
                        <h3>"Settings"</h3>
                        <p>"This is the settings content."</p>
                    </div>
                </TabsContent>
            </Tabs>
        </div>
    }
}
```

#### Navigation Menu

```rust
#[component]
pub fn NavigationMenuExample() -> impl IntoView {
    view! {
        <div class="navigation-example">
            <h2>"Navigation Menu"</h2>
            
            <NavigationMenu>
                <NavigationMenuList>
                    <NavigationMenuItem>
                        <NavigationMenuTrigger>"Products"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <div class="navigation-content">
                                <h3>"Product Categories"</h3>
                                <ul>
                                    <li><a href="/electronics">"Electronics"</a></li>
                                    <li><a href="/clothing">"Clothing"</a></li>
                                    <li><a href="/books">"Books"</a></li>
                                </ul>
                            </div>
                        </NavigationMenuContent>
                    </NavigationMenuItem>
                    
                    <NavigationMenuItem>
                        <NavigationMenuTrigger>"Services"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <div class="navigation-content">
                                <h3>"Our Services"</h3>
                                <ul>
                                    <li><a href="/consulting">"Consulting"</a></li>
                                    <li><a href="/support">"Support"</a></li>
                                    <li><a href="/training">"Training"</a></li>
                                </ul>
                            </div>
                        </NavigationMenuContent>
                    </NavigationMenuItem>
                    
                    <NavigationMenuItem>
                        <NavigationMenuLink href="/about">"About"</NavigationMenuLink>
                    </NavigationMenuItem>
                </NavigationMenuList>
            </NavigationMenu>
        </div>
    }
}
```

### Data Display Components

#### Tables

```rust
#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
}

#[component]
pub fn TableExample() -> impl IntoView {
    let users = vec![
        User {
            id: "1".to_string(),
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            role: "Admin".to_string(),
        },
        User {
            id: "2".to_string(),
            name: "Jane Smith".to_string(),
            email: "jane@example.com".to_string(),
            role: "User".to_string(),
        },
        User {
            id: "3".to_string(),
            name: "Bob Johnson".to_string(),
            email: "bob@example.com".to_string(),
            role: "User".to_string(),
        },
    ];

    view! {
        <div class="table-example">
            <h2>"User Table"</h2>
            
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead>"ID"</TableHead>
                        <TableHead>"Name"</TableHead>
                        <TableHead>"Email"</TableHead>
                        <TableHead>"Role"</TableHead>
                        <TableHead>"Actions"</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {users.into_iter().map(|user| {
                        view! {
                            <TableRow>
                                <TableCell>{user.id}</TableCell>
                                <TableCell>{user.name}</TableCell>
                                <TableCell>{user.email}</TableCell>
                                <TableCell>{user.role}</TableCell>
                                <TableCell>
                                    <Button size=ButtonSize::Small>"Edit"</Button>
                                    <Button size=ButtonSize::Small variant=ButtonVariant::Destructive>"Delete"</Button>
                                </TableCell>
                            </TableRow>
                        }
                    }).collect::<Vec<_>>()}
                </TableBody>
            </Table>
        </div>
    }
}
```

#### Cards

```rust
#[component]
pub fn CardExample() -> impl IntoView {
    view! {
        <div class="card-examples">
            <h2>"Card Examples"</h2>
            
            <div class="card-grid">
                <Card>
                    <CardHeader>
                        <CardTitle>"Basic Card"</CardTitle>
                        <CardDescription>"This is a basic card with header and content."</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <p>"Card content goes here."</p>
                    </CardContent>
                </Card>
                
                <Card>
                    <CardHeader>
                        <CardTitle>"Card with Actions"</CardTitle>
                        <CardDescription>"This card includes action buttons."</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <p>"Card content with actions."</p>
                    </CardContent>
                    <CardFooter>
                        <Button>"Action 1"</Button>
                        <Button variant=ButtonVariant::Outline>"Action 2"</Button>
                    </CardFooter>
                </Card>
            </div>
        </div>
    }
}
```

## Advanced Patterns

### State Management

#### Global State with Context

```rust
#[derive(Debug, Clone)]
pub struct AppState {
    pub user: Option<User>,
    pub theme: String,
    pub notifications: Vec<Notification>,
}

#[component]
pub fn AppStateProvider(children: Children) -> impl IntoView {
    let (state, set_state) = create_signal(AppState {
        user: None,
        theme: "light".to_string(),
        notifications: Vec::new(),
    });

    provide_context(state);
    provide_context(set_state);

    view! {
        <div class="app-state-provider">
            {children()}
        </div>
    }
}

#[component]
pub fn UserProfile() -> impl IntoView {
    let state = use_context::<ReadSignal<AppState>>().unwrap();

    view! {
        <div class="user-profile">
            {if let Some(user) = state().user {
                view! {
                    <h3>{user.name}</h3>
                    <p>{user.email}</p>
                }
            } else {
                view! {
                    <p>"No user logged in"</p>
                }
            }}
        </div>
    }
}
```

#### Custom Hooks

```rust
pub fn use_local_storage<T>(key: &str, default: T) -> (ReadSignal<T>, WriteSignal<T>)
where
    T: Clone + serde::Serialize + serde::de::DeserializeOwned,
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
pub fn LocalStorageExample() -> impl IntoView {
    let (name, set_name) = use_local_storage("user_name", "".to_string());

    view! {
        <div class="local-storage-example">
            <h2>"Local Storage Example"</h2>
            <Input
                value=name
                on_change=move |value| set_name(value)
                placeholder="Enter your name"
            />
            <p>"Your name: " {name}</p>
        </div>
    }
}
```

### Performance Optimization

#### Virtual Scrolling

```rust
#[component]
pub fn VirtualScrollingExample() -> impl IntoView {
    let items = (0..10000).map(|i| format!("Item {}", i)).collect::<Vec<_>>();

    view! {
        <div class="virtual-scrolling-example">
            <h2>"Virtual Scrolling Example"</h2>
            <p>"Rendering 10,000 items efficiently"</p>
            
            <OptimizedVirtualList
                items=items
                item_height=50.0
                container_height=400.0
                overscan=5
                render_item=Callback::new(|item| {
                    view! {
                        <div class="virtual-item">
                            {item}
                        </div>
                    }
                })
            />
        </div>
    }
}
```

#### Memoized Components

```rust
#[component]
pub fn ExpensiveComponent(data: ReadSignal<Vec<String>>) -> impl IntoView {
    let memoized_data = use_memo(move || {
        // Expensive computation
        data().into_iter().map(|s| s.to_uppercase()).collect::<Vec<_>>()
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
```

## Performance Examples

### Performance Monitoring

```rust
#[component]
pub fn PerformanceMonitoringExample() -> impl IntoView {
    let monitor = get_global_performance_monitor();
    let stats = monitor.get_stats();

    view! {
        <div class="performance-monitoring">
            <h2>"Performance Monitoring"</h2>
            
            <div class="performance-stats">
                <h3>"Performance Statistics"</h3>
                <p>"Total Measurements: " {stats.total_measurements}</p>
                <p>"Average Duration: " {format!("{:.2}ms", stats.average_duration.as_secs_f64() * 1000.0)}</p>
                <p>"Median Duration: " {format!("{:.2}ms", stats.median_duration.as_secs_f64() * 1000.0)}</p>
                <p>"Min Duration: " {format!("{:.2}ms", stats.min_duration.as_secs_f64() * 1000.0)}</p>
                <p>"Max Duration: " {format!("{:.2}ms", stats.max_duration.as_secs_f64() * 1000.0)}</p>
            </div>
            
            <VirtualListPerformanceMonitor show_stats=true />
        </div>
    }
}
```

### Benchmarking

```rust
#[component]
pub fn BenchmarkingExample() -> impl IntoView {
    let (benchmark_results, set_benchmark_results) = create_signal(None::<String>);

    let run_benchmark = move |_| {
        let config = BenchmarkConfig {
            iterations: 100,
            warmup_iterations: 10,
            timeout: Duration::from_secs(10),
            memory_tracking: true,
            detailed_reporting: false,
        };

        let mut benchmark = PerformanceBenchmark::new(config);
        let result = benchmark.benchmark_component("test-component", || {
            view! { <div>"Test component"</div> }
        });

        set_benchmark_results(Some(benchmark.generate_report()));
    };

    view! {
        <div class="benchmarking-example">
            <h2>"Performance Benchmarking"</h2>
            
            <Button on_click=run_benchmark>"Run Benchmark"</Button>
            
            {if let Some(results) = benchmark_results() {
                view! {
                    <div class="benchmark-results">
                        <h3>"Benchmark Results"</h3>
                        <pre>{results}</pre>
                    </div>
                }
            }}
        </div>
    }
}
```

## Accessibility Examples

### ARIA Labels and Roles

```rust
#[component]
pub fn AccessibilityExample() -> impl IntoView {
    view! {
        <div class="accessibility-example">
            <h2>"Accessibility Examples"</h2>
            
            <Button
                aria_label="Close dialog"
                aria_describedby="close-description"
            >
                "×"
            </Button>
            <div id="close-description" class="sr-only">
                "Click to close the dialog"
            </div>
            
            <Input
                aria_label="Search input"
                aria_required=true
                placeholder="Search..."
            />
            
            <div role="alert" aria_live="polite">
                "This is an important message"
            </div>
        </div>
    }
}
```

### Keyboard Navigation

```rust
#[component]
pub fn KeyboardNavigationExample() -> impl IntoView {
    let (focused_item, set_focused_item) = create_signal(0);
    let items = vec!["Item 1", "Item 2", "Item 3", "Item 4"];

    let handle_keydown = move |event: web_sys::KeyboardEvent| {
        match event.key().as_str() {
            "ArrowDown" => {
                event.prevent_default();
                set_focused_item((focused_item() + 1) % items.len());
            }
            "ArrowUp" => {
                event.prevent_default();
                set_focused_item((focused_item() - 1 + items.len()) % items.len());
            }
            "Enter" => {
                event.prevent_default();
                log::info!("Selected: {}", items[focused_item()]);
            }
            _ => {}
        }
    };

    view! {
        <div class="keyboard-navigation-example">
            <h2>"Keyboard Navigation"</h2>
            <p>"Use arrow keys to navigate, Enter to select"</p>
            
            <div
                class="focusable-list"
                tabindex="0"
                on:keydown=handle_keydown
            >
                {items.into_iter().enumerate().map(|(index, item)| {
                    let is_focused = index == focused_item();
                    view! {
                        <div
                            class=format!("focusable-item {}", if is_focused { "focused" } else { "" })
                            tabindex=if is_focused { "0" } else { "-1" }
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

## Conclusion

These interactive examples demonstrate the full power and flexibility of Radix-Leptos components. Each example includes:

- **Complete code**: Ready to copy and use
- **Best practices**: Following Radix-Leptos conventions
- **Accessibility**: Proper ARIA labels and keyboard navigation
- **Performance**: Optimized rendering and state management
- **Real-world patterns**: Practical usage scenarios

For more examples, see the [Component Gallery](component-gallery.md) and [Advanced Patterns](advanced-patterns.md).
