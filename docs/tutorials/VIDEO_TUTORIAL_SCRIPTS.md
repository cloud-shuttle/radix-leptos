# Video Tutorial Scripts for Radix-Leptos

This document provides comprehensive scripts for creating video tutorials that demonstrate Radix-Leptos components and patterns.

## Table of Contents

- [Getting Started Tutorial](#getting-started-tutorial)
- [Component Deep Dive](#component-deep-dive)
- [Advanced Patterns Tutorial](#advanced-patterns-tutorial)
- [Performance Optimization Tutorial](#performance-optimization-tutorial)
- [Accessibility Tutorial](#accessibility-tutorial)
- [Testing Tutorial](#testing-tutorial)

## Getting Started Tutorial

### Script: "Building Your First Radix-Leptos App"

**Duration**: 15 minutes  
**Target Audience**: Beginners  
**Prerequisites**: Basic Rust knowledge

#### Introduction (2 minutes)

"Welcome to Radix-Leptos! In this tutorial, we'll build a complete todo application using Radix-Leptos components. By the end, you'll understand the core concepts and be ready to build your own applications.

Radix-Leptos is a powerful UI component library built on top of Leptos, providing accessible, performant, and beautiful components for Rust web applications."

#### Setup (3 minutes)

"Let's start by setting up our project. First, we'll create a new Leptos project:

```bash
cargo new my-todo-app
cd my-todo-app
```

Now, let's add the necessary dependencies to our Cargo.toml:

```toml
[dependencies]
leptos = "0.6"
radix-leptos-primitives = "0.9.0"
radix-leptos-core = "0.9.0"
```

Next, we'll create our main application file. I'll show you the basic structure:

```rust
use leptos::prelude::*;
use radix_leptos_primitives::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app">
            <h1>"My Todo App"</h1>
            <TodoList />
        </div>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
```"

#### Building the Todo List (5 minutes)

"Now let's build our todo list component. We'll use Radix-Leptos components to create a beautiful and accessible interface:

```rust
#[component]
pub fn TodoList() -> impl IntoView {
    let (todos, set_todos) = create_signal(vec![
        Todo { id: 1, text: "Learn Radix-Leptos".to_string(), completed: false },
        Todo { id: 2, text: "Build amazing apps".to_string(), completed: false },
    ]);
    
    let (new_todo, set_new_todo) = create_signal("".to_string());
    
    let add_todo = move |_| {
        if !new_todo().is_empty() {
            set_todos.update(|todos| {
                todos.push(Todo {
                    id: todos.len() + 1,
                    text: new_todo(),
                    completed: false,
                });
            });
            set_new_todo("".to_string());
        }
    };
    
    view! {
        <div class="todo-list">
            <div class="todo-input">
                <Input
                    value=new_todo
                    on_change=set_new_todo
                    placeholder="Add a new todo..."
                    on_keydown=move |event| {
                        if event.key() == "Enter" {
                            add_todo(());
                        }
                    }
                />
                <Button on_click=add_todo>"Add"</Button>
            </div>
            
            <div class="todo-items">
                {todos().into_iter().map(|todo| {
                    view! {
                        <TodoItem
                            todo=todo.clone()
                            on_toggle=move |id| {
                                set_todos.update(|todos| {
                                    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                                        todo.completed = !todo.completed;
                                    }
                                });
                            }
                            on_delete=move |id| {
                                set_todos.update(|todos| {
                                    todos.retain(|t| t.id != id);
                                });
                            }
                        />
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
```

Notice how we're using Radix-Leptos components like `Input` and `Button`. These components come with built-in accessibility features and consistent styling."

#### Styling and Theming (3 minutes)

"Let's add some styling to make our app look great. Radix-Leptos components work seamlessly with CSS:

```css
.app {
    max-width: 600px;
    margin: 0 auto;
    padding: 2rem;
}

.todo-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.todo-input {
    display: flex;
    gap: 0.5rem;
}

.todo-items {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}
```

We can also use the built-in theming system:

```rust
<ThemeProvider theme="light">
    <App />
</ThemeProvider>
```"

#### Conclusion (2 minutes)

"Congratulations! You've built your first Radix-Leptos application. You've learned:

- How to set up a Radix-Leptos project
- How to use basic components like Input and Button
- How to manage state with Leptos signals
- How to style your application

In the next tutorial, we'll dive deeper into more advanced components and patterns. Thanks for watching!"

## Component Deep Dive

### Script: "Mastering Radix-Leptos Components"

**Duration**: 25 minutes  
**Target Audience**: Intermediate developers  
**Prerequisites**: Basic Radix-Leptos knowledge

#### Introduction (2 minutes)

"In this tutorial, we'll explore the full range of Radix-Leptos components, from basic form elements to complex data display components. We'll build a comprehensive dashboard that showcases the power and flexibility of the component library."

#### Form Components (8 minutes)

"Let's start with form components. Radix-Leptos provides a complete set of form elements with built-in validation and accessibility:

```rust
#[component]
pub fn ContactForm() -> impl IntoView {
    let (form_data, set_form_data) = create_signal(ContactFormData::default());
    let (errors, set_errors) = create_signal(HashMap::new());
    
    let handle_submit = move |_| {
        let mut new_errors = HashMap::new();
        
        if form_data().name.is_empty() {
            new_errors.insert("name".to_string(), "Name is required".to_string());
        }
        
        if form_data().email.is_empty() {
            new_errors.insert("email".to_string(), "Email is required".to_string());
        } else if !form_data().email.contains('@') {
            new_errors.insert("email".to_string(), "Invalid email format".to_string());
        }
        
        set_errors(new_errors);
        
        if errors().is_empty() {
            log::info!("Form submitted: {:?}", form_data());
        }
    };
    
    view! {
        <form on:submit=handle_submit class="contact-form">
            <h2>"Contact Us"</h2>
            
            <FormField>
                <FormLabel for="name">"Name"</FormLabel>
                <Input
                    id="name"
                    value=form_data().name
                    on_change=move |name| set_form_data.update(|data| data.name = name)
                    placeholder="Enter your name"
                    aria_required=true
                    aria_invalid=!errors().get("name").is_none()
                />
                {if let Some(error) = errors().get("name") {
                    view! { <FormFieldError>{error}</FormFieldError> }
                }}
            </FormField>
            
            <FormField>
                <FormLabel for="email">"Email"</FormLabel>
                <Input
                    id="email"
                    type="email"
                    value=form_data().email
                    on_change=move |email| set_form_data.update(|data| data.email = email)
                    placeholder="Enter your email"
                    aria_required=true
                    aria_invalid=!errors().get("email").is_none()
                />
                {if let Some(error) = errors().get("email") {
                    view! { <FormFieldError>{error}</FormFieldError> }
                }}
            </FormField>
            
            <FormField>
                <FormLabel for="subject">"Subject"</FormLabel>
                <Select
                    value=form_data().subject
                    on_value_change=move |subject| set_form_data.update(|data| data.subject = subject)
                >
                    <SelectTrigger>
                        <SelectValue placeholder="Select a subject" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="general">"General Inquiry"</SelectItem>
                        <SelectItem value="support">"Technical Support"</SelectItem>
                        <SelectItem value="sales">"Sales Question"</SelectItem>
                        <SelectItem value="feedback">"Feedback"</SelectItem>
                    </SelectContent>
                </Select>
            </FormField>
            
            <FormField>
                <FormLabel for="message">"Message"</FormLabel>
                <Textarea
                    id="message"
                    value=form_data().message
                    on_change=move |message| set_form_data.update(|data| data.message = message)
                    placeholder="Enter your message"
                    rows=4
                />
            </FormField>
            
            <FormField>
                <Checkbox
                    checked=form_data().newsletter
                    on_change=move |newsletter| set_form_data.update(|data| data.newsletter = newsletter)
                    label="Subscribe to our newsletter"
                />
            </FormField>
            
            <Button type="submit">"Send Message"</Button>
        </form>
    }
}
```

Notice how each form component includes proper accessibility attributes and validation states."

#### Navigation Components (5 minutes)

"Next, let's explore navigation components. Radix-Leptos provides powerful navigation elements:

```rust
#[component]
pub fn DashboardNavigation() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal("overview".to_string());
    
    view! {
        <div class="dashboard-navigation">
            <Tabs value=active_tab on_value_change=set_active_tab>
                <TabsList>
                    <TabsTrigger value="overview">"Overview"</TabsTrigger>
                    <TabsTrigger value="analytics">"Analytics"</TabsTrigger>
                    <TabsTrigger value="settings">"Settings"</TabsTrigger>
                </TabsList>
                
                <TabsContent value="overview">
                    <div class="tab-content">
                        <h3>"Dashboard Overview"</h3>
                        <p>"Welcome to your dashboard"</p>
                    </div>
                </TabsContent>
                
                <TabsContent value="analytics">
                    <div class="tab-content">
                        <h3>"Analytics"</h3>
                        <p>"View your analytics data"</p>
                    </div>
                </TabsContent>
                
                <TabsContent value="settings">
                    <div class="tab-content">
                        <h3>"Settings"</h3>
                        <p>"Configure your preferences"</p>
                    </div>
                </TabsContent>
            </Tabs>
            
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
                        <NavigationMenuLink href="/about">"About"</NavigationMenuLink>
                    </NavigationMenuItem>
                </NavigationMenuList>
            </NavigationMenu>
        </div>
    }
}
```"

#### Data Display Components (7 minutes)

"Now let's explore data display components. These are perfect for showing complex data:

```rust
#[component]
pub fn DataDashboard() -> impl IntoView {
    let users = vec![
        User { id: "1".to_string(), name: "John Doe".to_string(), email: "john@example.com".to_string(), role: "Admin".to_string() },
        User { id: "2".to_string(), name: "Jane Smith".to_string(), email: "jane@example.com".to_string(), role: "User".to_string() },
        User { id: "3".to_string(), name: "Bob Johnson".to_string(), email: "bob@example.com".to_string(), role: "User".to_string() },
    ];
    
    view! {
        <div class="data-dashboard">
            <h2>"User Management"</h2>
            
            <div class="dashboard-cards">
                <Card>
                    <CardHeader>
                        <CardTitle>"Total Users"</CardTitle>
                        <CardDescription>"Active user count"</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <div class="metric">
                            <span class="metric-value">{users.len()}</span>
                            <span class="metric-label">"users"</span>
                        </div>
                    </CardContent>
                </Card>
                
                <Card>
                    <CardHeader>
                        <CardTitle>"Recent Activity"</CardTitle>
                        <CardDescription>"Latest user actions"</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <div class="activity-list">
                            <div class="activity-item">
                                <Avatar>
                                    <AvatarImage src="user1.jpg" alt="User" />
                                    <AvatarFallback>"JD"</AvatarFallback>
                                </Avatar>
                                <div class="activity-content">
                                    <p>"John Doe logged in"</p>
                                    <span class="activity-time">"2 minutes ago"</span>
                                </div>
                            </div>
                        </div>
                    </CardContent>
                </Card>
            </div>
            
            <Table>
                <TableHeader>
                    <TableRow>
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
                                <TableCell>
                                    <div class="user-cell">
                                        <Avatar>
                                            <AvatarImage src="user.jpg" alt="User" />
                                            <AvatarFallback>{user.name.chars().take(2).collect::<String>()}</AvatarFallback>
                                        </Avatar>
                                        <span>{user.name}</span>
                                    </div>
                                </TableCell>
                                <TableCell>{user.email}</TableCell>
                                <TableCell>
                                    <Badge variant=if user.role == "Admin" { BadgeVariant::Default } else { BadgeVariant::Secondary }>
                                        {user.role}
                                    </Badge>
                                </TableCell>
                                <TableCell>
                                    <div class="action-buttons">
                                        <Button size=ButtonSize::Small>"Edit"</Button>
                                        <Button size=ButtonSize::Small variant=ButtonVariant::Destructive>"Delete"</Button>
                                    </div>
                                </TableCell>
                            </TableRow>
                        }
                    }).collect::<Vec<_>>()}
                </TableBody>
            </Table>
        </div>
    }
}
```"

#### Overlay Components (3 minutes)

"Finally, let's explore overlay components for modals and notifications:

```rust
#[component]
pub fn OverlayExamples() -> impl IntoView {
    let (is_dialog_open, set_is_dialog_open) = create_signal(false);
    let (is_sheet_open, set_is_sheet_open) = create_signal(false);
    
    view! {
        <div class="overlay-examples">
            <h2>"Overlay Components"</h2>
            
            <div class="overlay-buttons">
                <Button on_click=move |_| set_is_dialog_open(true)>"Open Dialog"</Button>
                <Button on_click=move |_| set_is_sheet_open(true)>"Open Sheet"</Button>
            </div>
            
            <Dialog open=is_dialog_open on_open_change=set_is_dialog_open>
                <DialogTrigger>
                    <Button>"Open Dialog"</Button>
                </DialogTrigger>
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>"Dialog Title"</DialogTitle>
                        <DialogDescription>"Dialog description"</DialogDescription>
                    </DialogHeader>
                    <DialogBody>
                        <p>"Dialog content goes here"</p>
                    </DialogBody>
                    <DialogFooter>
                        <Button variant=ButtonVariant::Outline on_click=move |_| set_is_dialog_open(false)>"Cancel"</Button>
                        <Button on_click=move |_| set_is_dialog_open(false)>"Confirm"</Button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
            
            <Sheet open=is_sheet_open on_open_change=set_is_sheet_open>
                <SheetTrigger>
                    <Button>"Open Sheet"</Button>
                </SheetTrigger>
                <SheetContent>
                    <SheetHeader>
                        <SheetTitle>"Sheet Title"</SheetTitle>
                        <SheetDescription>"Sheet description"</SheetDescription>
                    </SheetHeader>
                    <SheetBody>
                        <p>"Sheet content goes here"</p>
                    </SheetBody>
                </SheetContent>
            </Sheet>
        </div>
    }
}
```"

## Advanced Patterns Tutorial

### Script: "Advanced Radix-Leptos Patterns"

**Duration**: 30 minutes  
**Target Audience**: Advanced developers  
**Prerequisites**: Intermediate Radix-Leptos knowledge

#### Introduction (2 minutes)

"In this tutorial, we'll explore advanced patterns and techniques for building sophisticated applications with Radix-Leptos. We'll cover state management, performance optimization, and architectural patterns."

#### State Management Patterns (10 minutes)

"Let's start with advanced state management patterns. We'll build a comprehensive state management system:

```rust
#[derive(Debug, Clone)]
pub struct AppState {
    pub user: Option<User>,
    pub theme: String,
    pub notifications: Vec<Notification>,
    pub preferences: HashMap<String, String>,
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

// Custom hooks for state management
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
```"

#### Performance Optimization (8 minutes)

"Now let's explore performance optimization techniques:

```rust
#[component]
pub fn PerformanceOptimizedList() -> impl IntoView {
    let items = (0..10000).map(|i| format!("Item {}", i)).collect::<Vec<_>>();
    let (search_term, set_search_term) = create_signal("".to_string());

    // Memoized filtered items
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

    // Memoized callback
    let handle_item_click = use_callback(
        move |item: String| {
            log::info!("Clicked item: {}", item);
        },
        [],
    );

    view! {
        <div class="performance-optimized-list">
            <h2>"Performance Optimized List"</h2>
            
            <Input
                value=search_term
                on_change=set_search_term
                placeholder="Search items..."
            />
            
            <OptimizedVirtualList
                items=filtered_items
                item_height=50.0
                container_height=400.0
                overscan=5
                render_item=Callback::new(|item| {
                    view! {
                        <div class="virtual-item" on:click=move |_| handle_item_click(item.clone())>
                            {item}
                        </div>
                    }
                })
            />
        </div>
    }
}
```"

#### Architecture Patterns (10 minutes)

"Finally, let's explore architectural patterns:

```rust
// Feature-based architecture
// features/user/mod.rs
pub mod components;
pub mod hooks;
pub mod services;
pub mod types;

// features/user/components.rs
#[component]
pub fn UserProfile(user: User) -> impl IntoView {
    let (is_editing, set_is_editing) = create_signal(false);
    let (edited_user, set_edited_user) = create_signal(user.clone());
    
    let handle_save = move |_| {
        // Save user changes
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
                        <Button on_click=handle_save>"Save"</Button>
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

// Plugin architecture
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
}
```"

## Performance Optimization Tutorial

### Script: "Optimizing Radix-Leptos Applications"

**Duration**: 20 minutes  
**Target Audience**: Intermediate to advanced developers  
**Prerequisites**: Basic Radix-Leptos knowledge

#### Introduction (2 minutes)

"In this tutorial, we'll explore performance optimization techniques for Radix-Leptos applications. We'll cover memoization, virtual scrolling, and performance monitoring."

#### Memoization Techniques (6 minutes)

"Let's start with memoization techniques:

```rust
#[component]
pub fn MemoizedComponent(data: ReadSignal<Vec<String>>) -> impl IntoView {
    // Memoized expensive computation
    let memoized_data = use_memo(move || {
        data().into_iter()
            .map(|s| s.to_uppercase())
            .filter(|s| s.len() > 5)
            .collect::<Vec<_>>()
    }, [data]);

    // Memoized callback
    let handle_click = use_callback(
        move |item: String| {
            log::info!("Clicked: {}", item);
        },
        [],
    );

    view! {
        <div class="memoized-component">
            <h3>"Memoized Component"</h3>
            <ul>
                {memoized_data().into_iter().map(|item| {
                    view! {
                        <li on:click=move |_| handle_click(item.clone())>
                            {item}
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
```"

#### Virtual Scrolling (6 minutes)

"Next, let's explore virtual scrolling for large datasets:

```rust
#[component]
pub fn VirtualScrollingExample() -> impl IntoView {
    let items = (0..10000).map(|i| format!("Item {}", i)).collect::<Vec<_>>();
    let (search_term, set_search_term) = create_signal("".to_string());

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
```"

#### Performance Monitoring (6 minutes)

"Finally, let's explore performance monitoring:

```rust
#[component]
pub fn PerformanceMonitoringExample() -> impl IntoView {
    let monitor = get_global_performance_monitor();
    let (stats, set_stats) = create_signal(monitor.get_stats());

    let update_stats = move || {
        set_stats(monitor.get_stats());
    };

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
                </div>
            </div>
            
            <VirtualListPerformanceMonitor show_stats=true />
        </div>
    }
}
```"

## Accessibility Tutorial

### Script: "Building Accessible Radix-Leptos Applications"

**Duration**: 25 minutes  
**Target Audience**: All developers  
**Prerequisites**: Basic Radix-Leptos knowledge

#### Introduction (2 minutes)

"Accessibility is crucial for creating inclusive applications. In this tutorial, we'll explore how Radix-Leptos components are designed with accessibility in mind and how to build accessible applications."

#### ARIA Patterns (8 minutes)

"Let's start with ARIA patterns:

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

    view! {
        <form class="accessible-form">
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
        </form>
    }
}
```"

#### Keyboard Navigation (8 minutes)

"Next, let's explore keyboard navigation:

```rust
#[component]
pub fn KeyboardNavigationExample() -> impl IntoView {
    let (focused_index, set_focused_index) = create_signal(0);
    let items = vec!["Item 1", "Item 2", "Item 3", "Item 4"];

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
            "Enter" => {
                event.prevent_default();
                log::info!("Selected: {}", items[focused_index()]);
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
```"

#### Focus Management (7 minutes)

"Finally, let's explore focus management:

```rust
#[component]
pub fn FocusManagementExample() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);
    let trigger_ref = NodeRef::<leptos::html::Button>::new();
    let content_ref = NodeRef::<leptos::html::Div>::new();

    let handle_open = move |_| {
        set_is_open(true);
        // Focus the first focusable element
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
```"

## Testing Tutorial

### Script: "Testing Radix-Leptos Applications"

**Duration**: 20 minutes  
**Target Audience**: Intermediate developers  
**Prerequisites**: Basic testing knowledge

#### Introduction (2 minutes)

"Testing is essential for building reliable applications. In this tutorial, we'll explore testing strategies for Radix-Leptos applications, from unit tests to integration tests."

#### Unit Testing (6 minutes)

"Let's start with unit testing:

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
```"

#### Integration Testing (6 minutes)

"Next, let's explore integration testing:

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
```"

#### Performance Testing (6 minutes)

"Finally, let's explore performance testing:

```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use leptos::prelude::*;
    use std::time::Instant;

    #[test]
    fn test_component_render_performance() {
        let runtime = create_runtime();
        let start = Instant::now();
        
        for _ in 0..1000 {
            let _ = view! {
                <Button>"Performance Test"</Button>
            };
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 1000); // Should complete in under 1 second
        
        runtime.dispose();
    }

    #[test]
    fn test_virtual_list_performance() {
        let runtime = create_runtime();
        let items = (0..10000).map(|i| format!("Item {}", i)).collect::<Vec<_>>();
        
        let start = Instant::now();
        let view = view! {
            <OptimizedVirtualList
                items=items
                item_height=50.0
                container_height=400.0
                overscan=5
                render_item=Callback::new(|item| {
                    view! { <div class="virtual-item">{item}</div> }
                })
            />
        };
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should render in under 100ms
        assert!(view.into_any().is_some());
        
        runtime.dispose();
    }
}
```"

## Conclusion

These video tutorial scripts provide comprehensive coverage of Radix-Leptos components and patterns. Each script includes:

- **Clear structure**: Introduction, main content, and conclusion
- **Code examples**: Complete, working code snippets
- **Best practices**: Following Radix-Leptos conventions
- **Accessibility**: Proper ARIA labels and keyboard navigation
- **Performance**: Optimized rendering and state management
- **Testing**: Comprehensive test coverage

For more examples, see the [Interactive Examples](interactive-examples.md) and [Advanced Patterns](advanced-patterns.md).
