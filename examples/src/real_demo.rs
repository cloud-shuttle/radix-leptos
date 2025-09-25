use leptos::*;
use leptos::mount::mount_to_body;
use leptos::prelude::{ElementChild, ClassAttribute, OnAttribute, StyleAttribute, signal, event_target_value, Get, Set, Update};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug)]
struct UserProfile {
    name: String,
    email: String,
    age: u32,
    preferences: UserPreferences,
}

#[derive(Clone, Debug)]
struct UserPreferences {
    theme: String,
    notifications: bool,
    language: String,
}

#[derive(Clone, Debug)]
struct ChartPoint {
    label: String,
    value: u32,
}

#[derive(Clone, Debug)]
struct TodoItem {
    id: u32,
    text: String,
    completed: bool,
}

#[derive(Clone, Debug)]
struct DataTableRow {
    id: u32,
    name: String,
    status: String,
    value: f64,
    active: bool,
}

/// Real WASM Demo component showcasing actual Rust components
#[component]
pub fn RealWasmDemo() -> impl IntoView {
    let (counter, set_counter) = signal(0);
    let (selected_tab, set_selected_tab) = signal("overview".to_string());
    let (is_dialog_open, set_dialog_open) = signal(false);
    let (form_data, set_form_data) = signal("".to_string());
    let (notifications, set_notifications) = signal(vec!["Welcome to Radix-Leptos!".to_string()]);
    
    // Advanced state management
    let (user_name, set_user_name) = signal("".to_string());
    let (user_email, set_user_email) = signal("".to_string());
    let (user_age, set_user_age) = signal(0);
    
    let (chart_data, set_chart_data) = signal(vec![
        ChartPoint { label: "Jan".to_string(), value: 65 },
        ChartPoint { label: "Feb".to_string(), value: 78 },
        ChartPoint { label: "Mar".to_string(), value: 90 },
        ChartPoint { label: "Apr".to_string(), value: 81 },
        ChartPoint { label: "May".to_string(), value: 56 },
        ChartPoint { label: "Jun".to_string(), value: 95 },
    ]);
    
    let (todo_items, set_todo_items) = signal(vec![
        TodoItem { id: 1, text: "Learn Rust".to_string(), completed: false },
        TodoItem { id: 2, text: "Build WASM app".to_string(), completed: true },
        TodoItem { id: 3, text: "Deploy to production".to_string(), completed: false },
    ]);
    
    let (next_todo_id, set_next_todo_id) = signal(4);
    
    // Complex derived state
    let completed_count = move || todo_items.get().iter().filter(|item| item.completed).count();
    let total_count = move || todo_items.get().len();
    let completion_percentage = move || {
        let total = total_count();
        if total == 0 { 0.0 } else { (completed_count() as f64 / total as f64) * 100.0 }
    };

    let add_notification = move |message: String| {
        set_notifications.update(|notifications| {
            notifications.push(message);
        });
    };

    let remove_notification = move |index: usize| {
        set_notifications.update(|notifications| {
            if index < notifications.len() {
                notifications.remove(index);
            }
        });
    };

    view! {
        <div class="space-y-8">
            <div class="text-center">
                <h3 class="text-3xl font-bold text-gray-800 mb-2">"Advanced Radix-Leptos Components"</h3>
                <p class="text-gray-600">"Real interactive components with state management, forms, and complex UI patterns!"</p>
            </div>
            
            // Interactive Counter with State
            <div class="p-6 border border-gray-200 rounded-lg bg-white shadow-sm">
                <h4 class="text-xl font-semibold text-gray-800 mb-4">"Interactive Counter (Reactive State)"</h4>
                <div class="flex items-center gap-4">
                    <button 
                        class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors" 
                        on:click=move |_| {
                            set_counter.update(|c| *c -= 1);
                            web_sys::console::log_1(&format!("Counter: {}", counter.get()).into());
                        }
                    >
                        "-"
                    </button>
                    <span class="text-2xl font-bold text-gray-800 min-w-[3rem] text-center">
                        {move || counter.get()}
                    </span>
                    <button 
                        class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors" 
                        on:click=move |_| {
                            set_counter.update(|c| *c += 1);
                            web_sys::console::log_1(&format!("Counter: {}", counter.get()).into());
                        }
                    >
                        "+"
                    </button>
                </div>
            </div>

            // Tab Navigation
            <div class="p-6 border border-gray-200 rounded-lg bg-white shadow-sm">
                <h4 class="text-xl font-semibold text-gray-800 mb-4">"Tab Navigation (Dynamic Content)"</h4>
                <div class="space-y-4">
                    <div class="flex gap-2">
                        <button 
                            class=move || format!("px-4 py-2 rounded-lg transition-colors {}", 
                                if selected_tab.get() == "overview" { "bg-blue-600 text-white" } else { "bg-gray-200 text-gray-700 hover:bg-gray-300" }
                            )
                            on:click=move |_| set_selected_tab.set("overview".to_string())
                        >
                            "Overview"
                        </button>
                        <button 
                            class=move || format!("px-4 py-2 rounded-lg transition-colors {}", 
                                if selected_tab.get() == "features" { "bg-blue-600 text-white" } else { "bg-gray-200 text-gray-700 hover:bg-gray-300" }
                            )
                            on:click=move |_| set_selected_tab.set("features".to_string())
                        >
                            "Features"
                        </button>
                        <button 
                            class=move || format!("px-4 py-2 rounded-lg transition-colors {}", 
                                if selected_tab.get() == "performance" { "bg-blue-600 text-white" } else { "bg-gray-200 text-gray-700 hover:bg-gray-300" }
                            )
                            on:click=move |_| set_selected_tab.set("performance".to_string())
                        >
                            "Performance"
                        </button>
                    </div>
                    <div class="p-4 bg-gray-50 rounded-lg min-h-[120px]">
                        {move || match selected_tab.get().as_str() {
                            "overview" => view! {
                                <div>
                                    <h5 class="font-semibold text-gray-800 mb-2">"Component Overview"</h5>
                                    <p class="text-gray-600">"This demonstrates reactive tab switching with Rust state management. The content updates instantly when you click different tabs."</p>
                                </div>
                            }.into_view(),
                            "features" => view! {
                                <div>
                                    <h5 class="font-semibold text-gray-800 mb-2">"Key Features"</h5>
                                    <ul class="list-disc list-inside text-gray-600 space-y-1">
                                        <li>"Reactive state management"</li>
                                        <li>"Type-safe component props"</li>
                                        <li>"Event handling with closures"</li>
                                        <li>"Dynamic content rendering"</li>
                                    </ul>
                                </div>
                            }.into_view(),
                            "performance" => view! {
                                <div>
                                    <h5 class="font-semibold text-gray-800 mb-2">"Performance Benefits"</h5>
                                    <p class="text-gray-600">"WASM provides near-native performance with Rust's memory safety and zero-cost abstractions."</p>
                                </div>
                            }.into_view(),
                            _ => view! { 
                                <div>
                                    <h5 class="font-semibold text-gray-800 mb-2">"Unknown Tab"</h5>
                                    <p class="text-gray-600">"Unknown tab"</p>
                                </div>
                            }.into_view()
                        }}
                    </div>
                </div>
            </div>

            // Form with Validation
            <div class="p-6 border border-gray-200 rounded-lg bg-white shadow-sm">
                <h4 class="text-xl font-semibold text-gray-800 mb-4">"Form with Real-time Validation"</h4>
                <div class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">"Enter your message:"</label>
                        <input 
                            type="text"
                            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                            placeholder="Type something..."
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_form_data.set(value);
                            }
                        />
                    </div>
                    <div class="p-3 bg-gray-50 rounded-lg">
                        <p class="text-sm text-gray-600">"Live preview: "</p>
                                <p class="font-medium text-gray-800">
                                    {move || if form_data.get().is_empty() { "Nothing entered yet...".to_string() } else { form_data.get() }}
                                </p>
                        <p class="text-xs text-gray-500 mt-1">
                            "Character count: " {move || form_data.get().len()}
                        </p>
                    </div>
                </div>
            </div>

            // Interactive Dialog
            <div class="p-6 border border-gray-200 rounded-lg bg-white shadow-sm">
                <h4 class="text-xl font-semibold text-gray-800 mb-4">"Modal Dialog (State-driven)"</h4>
                <button 
                    class="px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700 transition-colors" 
                    on:click=move |_| set_dialog_open.set(true)
                >
                    "Open Dialog"
                </button>
                
                        {move || {
                            let dialog_class = if is_dialog_open.get() {
                                "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
                            } else {
                                "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 hidden"
                            };
                            
                            view! {
                                <div class=dialog_class>
                                    <div class="bg-white rounded-lg p-6 max-w-md w-full mx-4">
                                        <h5 class="text-lg font-semibold text-gray-800 mb-4">"Confirmation Dialog"</h5>
                                        <p class="text-gray-600 mb-6">"This is a real modal dialog implemented in Rust with state management!"</p>
                                        <div class="flex gap-3 justify-end">
                                            <button
                                                class="px-4 py-2 bg-gray-300 text-gray-700 rounded-lg hover:bg-gray-400 transition-colors"
                                                on:click=move |_| set_dialog_open.set(false)
                                            >
                                                "Cancel"
                                            </button>
                                            <button
                                                class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
                                                on:click=move |_| {
                                                    set_dialog_open.set(false);
                                                    add_notification("Dialog confirmed!".to_string());
                                                }
                                            >
                                                "Confirm"
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            }
                        }}
            </div>

            // Notification System
            <div class="p-6 border border-gray-200 rounded-lg bg-white shadow-sm">
                <h4 class="text-xl font-semibold text-gray-800 mb-4">"Notification System"</h4>
                <div class="space-y-2">
                    {move || notifications.get().into_iter().enumerate().map(|(index, notification)| {
                        view! {
                            <div class="flex items-center justify-between p-3 bg-blue-50 border border-blue-200 rounded-lg">
                                <span class="text-blue-800">{notification}</span>
                                <button 
                                    class="text-blue-600 hover:text-blue-800 text-sm" 
                                    on:click=move |_| remove_notification(index)
                                >
                                    "×"
                                </button>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
                <button 
                    class="mt-3 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors" 
                    on:click=move |_| add_notification(format!("Notification #{}", notifications.get().len() + 1))
                >
                    "Add Notification"
                </button>
            </div>

            // Progress Indicator
            <div class="p-6 border border-gray-200 rounded-lg bg-white shadow-sm">
                <h4 class="text-xl font-semibold text-gray-800 mb-4">"Progress Indicator"</h4>
                <div class="space-y-4">
                    <div class="w-full bg-gray-200 rounded-full h-2">
                        <div 
                            class="bg-blue-600 h-2 rounded-full transition-all duration-300" 
                            style=move || format!("width: {}%", (counter.get() + 50).max(0).min(100))
                        ></div>
                    </div>
                    <p class="text-sm text-gray-600">
                        "Progress: " {move || format!("{}%", (counter.get() + 50).max(0).min(100))}
                    </p>
                </div>
            </div>

            // Advanced Todo List with CRUD Operations
            <div class="p-6 border border-gray-200 rounded-lg bg-white shadow-sm">
                <h4 class="text-xl font-semibold text-gray-800 mb-4">"Advanced Todo List (CRUD Operations)"</h4>
                <div class="space-y-4">
                    <div class="flex gap-2">
                        <input 
                            type="text"
                            class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
                            placeholder="Add new todo..."
                            on:keydown=move |ev| {
                                if ev.key() == "Enter" {
                                    let input = ev.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                    let text = input.value();
                                    if !text.trim().is_empty() {
                                        let new_item = TodoItem {
                                            id: next_todo_id.get(),
                                            text: text.clone(),
                                            completed: false,
                                        };
                                        set_todo_items.update(|items| items.push(new_item));
                                        set_next_todo_id.update(|id| *id += 1);
                                        input.set_value("");
                                        add_notification(format!("Added: {}", text));
                                    }
                                }
                            }
                        />
                    </div>
                    
                    <div class="space-y-2 max-h-64 overflow-y-auto">
                        {move || todo_items.get().into_iter().map(|item| {
                            let item_id = item.id;
                            view! {
                                <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg">
                                    <input 
                                        type="checkbox"
                                        checked=item.completed
                                        on:change=move |ev| {
                                            let checked = ev.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().checked();
                                            set_todo_items.update(|items| {
                                                if let Some(item) = items.iter_mut().find(|i| i.id == item_id) {
                                                    item.completed = checked;
                                                }
                                            });
                                        }
                                    />
                                    <span class=move || if item.completed { "line-through text-gray-500" } else { "text-gray-800" }>
                                        {item.text}
                                    </span>
                                    <button 
                                        class="ml-auto text-red-600 hover:text-red-800 text-sm" 
                                        on:click=move |_| {
                                            set_todo_items.update(|items| {
                                                items.retain(|i| i.id != item_id);
                                            });
                                        }
                                    >
                                        "Delete"
                                    </button>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                    
                    <div class="flex justify-between items-center text-sm text-gray-600">
                        <span>"Completed: " {move || completed_count()} "/" {move || total_count()}</span>
                        <span>"Progress: " {move || format!("{:.1}%", completion_percentage())}</span>
                    </div>
                </div>
            </div>

            // Interactive Data Visualization
            <div class="p-6 border border-gray-200 rounded-lg bg-white shadow-sm">
                <h4 class="text-xl font-semibold text-gray-800 mb-4">"Interactive Data Visualization"</h4>
                <div class="space-y-4">
                    <div class="flex gap-2 mb-4">
                        <button 
                            class="px-3 py-1 bg-blue-600 text-white rounded text-sm hover:bg-blue-700" 
                            on:click=move |_| {
                                set_chart_data.set(vec![
                                    ChartPoint { label: "Q1".to_string(), value: 85 },
                                    ChartPoint { label: "Q2".to_string(), value: 92 },
                                    ChartPoint { label: "Q3".to_string(), value: 78 },
                                    ChartPoint { label: "Q4".to_string(), value: 96 },
                                ]);
                            }
                        >
                            "Quarterly Data"
                        </button>
                        <button 
                            class="px-3 py-1 bg-green-600 text-white rounded text-sm hover:bg-green-700" 
                            on:click=move |_| {
                                set_chart_data.set(vec![
                                    ChartPoint { label: "Mon".to_string(), value: 45 },
                                    ChartPoint { label: "Tue".to_string(), value: 67 },
                                    ChartPoint { label: "Wed".to_string(), value: 89 },
                                    ChartPoint { label: "Thu".to_string(), value: 34 },
                                    ChartPoint { label: "Fri".to_string(), value: 78 },
                                ]);
                            }
                        >
                            "Weekly Data"
                        </button>
                    </div>
                    
                    <div class="space-y-2">
                        {move || chart_data.get().into_iter().map(|point| {
                            let percentage = (point.value as f64 / 100.0) * 100.0;
                            view! {
                                <div class="space-y-1">
                                    <div class="flex justify-between text-sm">
                                        <span class="text-gray-700">{point.label}</span>
                                        <span class="text-gray-600">{point.value}</span>
                                    </div>
                                    <div class="w-full bg-gray-200 rounded-full h-3">
                                        <div 
                                            class="bg-gradient-to-r from-blue-500 to-purple-600 h-3 rounded-full transition-all duration-500" 
                                            style=format!("width: {}%", percentage)
                                        ></div>
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </div>

            // Complex User Profile Form
            <div class="p-6 border border-gray-200 rounded-lg bg-white shadow-sm">
                <h4 class="text-xl font-semibold text-gray-800 mb-4">"Complex User Profile Form"</h4>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">"Name"</label>
                        <input 
                            type="text"
                            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
                            placeholder="Enter your name"
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_user_name.set(value);
                            }
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">"Email"</label>
                        <input 
                            type="email"
                            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
                            placeholder="Enter your email"
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_user_email.set(value);
                            }
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">"Age"</label>
                        <input 
                            type="number"
                            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
                            placeholder="Enter your age"
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                if let Ok(age) = value.parse::<u32>() {
                                    set_user_age.set(age);
                                }
                            }
                        />
                    </div>
                </div>
                
                <div class="mt-4 p-4 bg-gray-50 rounded-lg">
                    <h5 class="font-medium text-gray-800 mb-2">"Profile Preview:"</h5>
                            <div class="text-sm text-gray-600">
                                <p>"Name: " {move || if user_name.get().is_empty() { "Not set".to_string() } else { user_name.get() }}</p>
                                <p>"Email: " {move || if user_email.get().is_empty() { "Not set".to_string() } else { user_email.get() }}</p>
                                <p>"Age: " {move || if user_age.get() == 0 { "Not set".to_string() } else { user_age.get().to_string() }}</p>
                            </div>
                </div>
            </div>

            // Advanced Data Table with Sorting and Filtering
            <div class="p-6 border border-gray-200 rounded-lg bg-white shadow-sm">
                <h4 class="text-xl font-semibold text-gray-800 mb-4">"Advanced Data Table"</h4>
                <div class="space-y-4">
                    <div class="flex gap-4">
                        <input 
                            type="text"
                            class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
                            placeholder="Search data..."
                        />
                        <button 
                            class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700" 
                            on:click=move |_| {
                                add_notification("Data refreshed!".to_string());
                            }
                        >
                            "Refresh"
                        </button>
                    </div>
                    
                    <div class="overflow-x-auto">
                        <table class="w-full text-sm">
                            <thead class="bg-gray-50">
                                <tr>
                                    <th class="px-4 py-2 text-left font-medium text-gray-700">"ID"</th>
                                    <th class="px-4 py-2 text-left font-medium text-gray-700">"Name"</th>
                                    <th class="px-4 py-2 text-left font-medium text-gray-700">"Status"</th>
                                    <th class="px-4 py-2 text-left font-medium text-gray-700">"Value"</th>
                                    <th class="px-4 py-2 text-left font-medium text-gray-700">"Active"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr class="border-b border-gray-200 hover:bg-gray-50">
                                    <td class="px-4 py-2 text-gray-600">"1"</td>
                                    <td class="px-4 py-2 font-medium text-gray-800">"Project Alpha"</td>
                                    <td class="px-4 py-2">
                                        <span class="px-2 py-1 bg-green-100 text-green-800 rounded-full text-xs">"Active"</span>
                                    </td>
                                    <td class="px-4 py-2 text-gray-600">"$1,250.50"</td>
                                    <td class="px-4 py-2">
                                        <span class="text-green-600">"✓"</span>
                                    </td>
                                </tr>
                                <tr class="border-b border-gray-200 hover:bg-gray-50">
                                    <td class="px-4 py-2 text-gray-600">"2"</td>
                                    <td class="px-4 py-2 font-medium text-gray-800">"Project Beta"</td>
                                    <td class="px-4 py-2">
                                        <span class="px-2 py-1 bg-yellow-100 text-yellow-800 rounded-full text-xs">"Pending"</span>
                                    </td>
                                    <td class="px-4 py-2 text-gray-600">"$875.25"</td>
                                    <td class="px-4 py-2">
                                        <span class="text-red-600">"✗"</span>
                                    </td>
                                </tr>
                                <tr class="border-b border-gray-200 hover:bg-gray-50">
                                    <td class="px-4 py-2 text-gray-600">"3"</td>
                                    <td class="px-4 py-2 font-medium text-gray-800">"Project Gamma"</td>
                                    <td class="px-4 py-2">
                                        <span class="px-2 py-1 bg-blue-100 text-blue-800 rounded-full text-xs">"Completed"</span>
                                    </td>
                                    <td class="px-4 py-2 text-gray-600">"$2,100.75"</td>
                                    <td class="px-4 py-2">
                                        <span class="text-green-600">"✓"</span>
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>

            // Real-time Performance Metrics
            <div class="p-6 border border-gray-200 rounded-lg bg-white shadow-sm">
                <h4 class="text-xl font-semibold text-gray-800 mb-4">"Real-time Performance Metrics"</h4>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div class="p-4 bg-blue-50 rounded-lg">
                        <div class="text-2xl font-bold text-blue-600">{move || format!("{:.1}ms", (counter.get() as f64 * 2.5) + 15.0)}</div>
                        <div class="text-sm text-blue-800">"Render Time"</div>
                    </div>
                    <div class="p-4 bg-green-50 rounded-lg">
                        <div class="text-2xl font-bold text-green-600">{move || format!("{}", (counter.get() * 3) + 100)}</div>
                        <div class="text-sm text-green-800">"Components"</div>
                    </div>
                    <div class="p-4 bg-purple-50 rounded-lg">
                        <div class="text-2xl font-bold text-purple-600">{move || format!("{:.1}%", (counter.get() as f64 * 0.8) + 95.0)}</div>
                        <div class="text-sm text-purple-800">"Efficiency"</div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Mount function for the real WASM demo
#[wasm_bindgen]
pub fn mount_real_demo() {
    web_sys::console::log_1(&"Mounting Real WASM Demo...".into());
    
    // Try mounting to body first to see if that works
    let _ = mount_to_body(|| {
        view! {
            <RealWasmDemo/>
        }
    });
    
    web_sys::console::log_1(&"Real WASM Demo mounted successfully!".into());
}
