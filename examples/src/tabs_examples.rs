use leptos::prelude::*;
use leptos::logging::log;
use radix_leptos_primitives::components::*;

#[component]
pub fn TabsExamples() -> impl IntoView {
    // State for different tabs examples
    let (active_tab, set_active_tab) = signal("account".to_string());
    let (settings_tab, set_settings_tab) = signal("general".to_string());
    let (profile_tab, set_profile_tab) = signal("personal".to_string());
    let (docs_tab, set_docs_tab) = signal("getting-started".to_string());

    // Callback functions
    let handle_tab_change = Callback::new(move |tab_value: String| {
        set_active_tab.set(tab_value.clone());
        log!("Tab changed to: {}", tab_value);
    });

    let handle_settings_change = Callback::new(move |tab_value: String| {
        set_settings_tab.set(tab_value.clone());
        log!("Settings tab changed to: {}", tab_value);
    });

    let handle_profile_change = Callback::new(move |tab_value: String| {
        set_profile_tab.set(tab_value.clone());
        log!("Profile tab changed to: {}", tab_value);
    });

    let handle_docs_change = Callback::new(move |tab_value: String| {
        set_docs_tab.set(tab_value.clone());
        log!("Docs tab changed to: {}", tab_value);
    });

    view! {
        <div class="min-h-screen bg-gray-50 p-8">
            <div class="max-w-6xl mx-auto space-y-8">
                <div class="text-center">
                    <h1 class="text-4xl font-bold text-gray-900 mb-4">"Tabs Component Examples"</h1>
                    <p class="text-lg text-gray-600">"Tabbed navigation with keyboard support and accessibility features"</p>
                </div>

                // Basic Tabs Example
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Basic Tabs"</h2>
                    <p class="text-gray-600 mb-4">"Simple horizontal tabs with default styling"</p>
                    
                    <Tabs class="w-full".to_string()>
                        <TabsList class="grid w-full grid-cols-3".to_string()>
                            <TabsTrigger 
                                value="account".to_string()
                                on_click=handle_tab_change.clone()
                                class="".to_string()
                            >
                                "Account"
                            </TabsTrigger>
                            <TabsTrigger 
                                value="password".to_string()
                                on_click=handle_tab_change.clone()
                                class="".to_string()
                            >
                                "Password"
                            </TabsTrigger>
                            <TabsTrigger 
                                value="notifications".to_string()
                                on_click=handle_tab_change.clone()
                                class="".to_string()
                            >
                                "Notifications"
                            </TabsTrigger>
                        </TabsList>
                        
                        <TabsContent value="account".to_string() class="".to_string()>
                            <div class="p-4">
                                <h3 class="text-lg font-medium mb-2">"Account Settings"</h3>
                                <p class="text-gray-600 mb-4">"Manage your account settings and preferences."</p>
                                <div class="space-y-3">
                                    <div class="flex items-center justify-between">
                                        <span class="text-sm font-medium">"Email"</span>
                                        <span class="text-sm text-gray-500">"user@example.com"</span>
                                    </div>
                                    <div class="flex items-center justify-between">
                                        <span class="text-sm font-medium">"Username"</span>
                                        <span class="text-sm text-gray-500">"johndoe"</span>
                                    </div>
                                    <div class="flex items-center justify-between">
                                        <span class="text-sm font-medium">"Member since"</span>
                                        <span class="text-sm text-gray-500">"January 2024"</span>
                                    </div>
                                </div>
                            </div>
                        </TabsContent>
                        
                        <TabsContent value="password".to_string() class="".to_string()>
                            <div class="p-4">
                                <h3 class="text-lg font-medium mb-2">"Password Settings"</h3>
                                <p class="text-gray-600 mb-4">"Update your password and security settings."</p>
                                <div class="space-y-3">
                                    <div class="flex items-center justify-between">
                                        <span class="text-sm font-medium">"Last changed"</span>
                                        <span class="text-sm text-gray-500">"2 weeks ago"</span>
                                    </div>
                                    <div class="flex items-center justify-between">
                                        <span class="text-sm font-medium">"Two-factor auth"</span>
                                        <span class="text-sm text-green-600">"Enabled"</span>
                                    </div>
                                    <div class="flex items-center justify-between">
                                        <span class="text-sm font-medium">"Login sessions"</span>
                                        <span class="text-sm text-gray-500">"3 active"</span>
                                    </div>
                                </div>
                            </div>
                        </TabsContent>
                        
                        <TabsContent value="notifications".to_string() class="".to_string()>
                            <div class="p-4">
                                <h3 class="text-lg font-medium mb-2">"Notification Preferences"</h3>
                                <p class="text-gray-600 mb-4">"Configure how you receive notifications."</p>
                                <div class="space-y-3">
                                    <div class="flex items-center justify-between">
                                        <span class="text-sm font-medium">"Email notifications"</span>
                                        <span class="text-sm text-green-600">"Enabled"</span>
                                    </div>
                                    <div class="flex items-center justify-between">
                                        <span class="text-sm font-medium">"Push notifications"</span>
                                        <span class="text-sm text-red-600">"Disabled"</span>
                                    </div>
                                    <div class="flex items-center justify-between">
                                        <span class="text-sm font-medium">"SMS notifications"</span>
                                        <span class="text-sm text-gray-500">"Not configured"</span>
                                    </div>
                                </div>
                            </div>
                        </TabsContent>
                    </Tabs>
                </div>

                // Tabs Variants
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Tabs Variants"</h2>
                    <p class="text-gray-600 mb-4">"Different tab styles: Default, Pills, and Underlined"</p>
                    
                    <div class="space-y-6">
                        // Default Variant
                        <div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"Default Variant"</h3>
                            <Tabs variant=TabsVariant::Default class="w-full".to_string()>
                                <TabsList class="grid w-full grid-cols-2".to_string()>
                                    <TabsTrigger 
                                        value="general".to_string()
                                        on_click=handle_settings_change.clone()
                                        class="".to_string()
                                    >
                                        "General"
                                    </TabsTrigger>
                                    <TabsTrigger 
                                        value="advanced".to_string()
                                        on_click=handle_settings_change.clone()
                                        class="".to_string()
                                    >
                                        "Advanced"
                                    </TabsTrigger>
                                </TabsList>
                                
                                <TabsContent value="general".to_string() class="".to_string()>
                                    <div class="p-4">
                                        <p class="text-gray-600">"General settings content goes here."</p>
                                    </div>
                                </TabsContent>
                                
                                <TabsContent value="advanced".to_string() class="".to_string()>
                                    <div class="p-4">
                                        <p class="text-gray-600">"Advanced settings content goes here."</p>
                                    </div>
                                </TabsContent>
                            </Tabs>
                        </div>

                        // Pills Variant
                        <div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"Pills Variant"</h3>
                            <Tabs variant=TabsVariant::Pills class="w-full".to_string()>
                                <TabsList class="grid w-full grid-cols-2".to_string()>
                                    <TabsTrigger 
                                        value="personal".to_string()
                                        on_click=handle_profile_change.clone()
                                        class="".to_string()
                                    >
                                        "Personal"
                                    </TabsTrigger>
                                    <TabsTrigger 
                                        value="professional".to_string()
                                        on_click=handle_profile_change.clone()
                                        class="".to_string()
                                    >
                                        "Professional"
                                    </TabsTrigger>
                                </TabsList>
                                
                                <TabsContent value="personal".to_string() class="".to_string()>
                                    <div class="p-4">
                                        <p class="text-gray-600">"Personal profile information."</p>
                                    </div>
                                </TabsContent>
                                
                                <TabsContent value="professional".to_string() class="".to_string()>
                                    <div class="p-4">
                                        <p class="text-gray-600">"Professional profile information."</p>
                                    </div>
                                </TabsContent>
                            </Tabs>
                        </div>

                        // Underlined Variant
                        <div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"Underlined Variant"</h3>
                            <Tabs variant=TabsVariant::Underlined class="w-full".to_string()>
                                <TabsList class="grid w-full grid-cols-3".to_string()>
                                    <TabsTrigger 
                                        value="getting-started".to_string()
                                        on_click=handle_docs_change.clone()
                                        class="".to_string()
                                    >
                                        "Getting Started"
                                    </TabsTrigger>
                                    <TabsTrigger 
                                        value="api".to_string()
                                        on_click=handle_docs_change.clone()
                                        class="".to_string()
                                    >
                                        "API"
                                    </TabsTrigger>
                                    <TabsTrigger 
                                        value="examples".to_string()
                                        on_click=handle_docs_change.clone()
                                        class="".to_string()
                                    >
                                        "Examples"
                                    </TabsTrigger>
                                </TabsList>
                                
                                <TabsContent value="getting-started".to_string() class="".to_string()>
                                    <div class="p-4">
                                        <p class="text-gray-600">"Getting started guide and tutorials."</p>
                                    </div>
                                </TabsContent>
                                
                                <TabsContent value="api".to_string() class="".to_string()>
                                    <div class="p-4">
                                        <p class="text-gray-600">"API documentation and reference."</p>
                                    </div>
                                </TabsContent>
                                
                                <TabsContent value="examples".to_string() class="".to_string()>
                                    <div class="p-4">
                                        <p class="text-gray-600">"Code examples and use cases."</p>
                                    </div>
                                </TabsContent>
                            </Tabs>
                        </div>
                    </div>
                </div>

                // Tabs Sizes
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Tabs Sizes"</h2>
                    <p class="text-gray-600 mb-4">"Different tab sizes: Small, Medium, and Large"</p>
                    
                    <div class="space-y-6">
                        <div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"Small Tabs"</h3>
                            <Tabs size=TabsSize::Small class="w-full".to_string()>
                                <TabsList class="grid w-full grid-cols-2".to_string()>
                                    <TabsTrigger value="tab1".to_string()>"Tab 1"</TabsTrigger>
                                    <TabsTrigger value="tab2".to_string()>"Tab 2"</TabsTrigger>
                                </TabsList>
                                <TabsContent value="tab1".to_string()>
                                    <div class="p-4">
                                        <p class="text-gray-600">"Small tab content."</p>
                                    </div>
                                </TabsContent>
                                <TabsContent value="tab2".to_string()>
                                    <div class="p-4">
                                        <p class="text-gray-600">"Small tab content."</p>
                                    </div>
                                </TabsContent>
                            </Tabs>
                        </div>

                        <div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"Medium Tabs (Default)"</h3>
                            <Tabs size=TabsSize::Medium class="w-full".to_string()>
                                <TabsList class="grid w-full grid-cols-2".to_string()>
                                    <TabsTrigger value="tab3".to_string()>"Tab 1"</TabsTrigger>
                                    <TabsTrigger value="tab4".to_string()>"Tab 2"</TabsTrigger>
                                </TabsList>
                                <TabsContent value="tab3".to_string()>
                                    <div class="p-4">
                                        <p class="text-gray-600">"Medium tab content."</p>
                                    </div>
                                </TabsContent>
                                <TabsContent value="tab4".to_string()>
                                    <div class="p-4">
                                        <p class="text-gray-600">"Medium tab content."</p>
                                    </div>
                                </TabsContent>
                            </Tabs>
                        </div>

                        <div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"Large Tabs"</h3>
                            <Tabs size=TabsSize::Large class="w-full".to_string()>
                                <TabsList class="grid w-full grid-cols-2".to_string()>
                                    <TabsTrigger value="tab5".to_string()>"Tab 1"</TabsTrigger>
                                    <TabsTrigger value="tab6".to_string()>"Tab 2"</TabsTrigger>
                                </TabsList>
                                <TabsContent value="tab5".to_string()>
                                    <div class="p-4">
                                        <p class="text-gray-600">"Large tab content."</p>
                                    </div>
                                </TabsContent>
                                <TabsContent value="tab6".to_string()>
                                    <div class="p-4">
                                        <p class="text-gray-600">"Large tab content."</p>
                                    </div>
                                </TabsContent>
                            </Tabs>
                        </div>
                    </div>
                </div>

                // Vertical Tabs
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Vertical Tabs"</h2>
                    <p class="text-gray-600 mb-4">"Tabs with vertical orientation"</p>
                    
                    <Tabs orientation=TabsOrientation::Vertical class="w-full".to_string()>
                        <TabsList class="w-48 h-auto flex-col".to_string()>
                            <TabsTrigger value="overview".to_string()>"Overview"</TabsTrigger>
                            <TabsTrigger value="analytics".to_string()>"Analytics"</TabsTrigger>
                            <TabsTrigger value="reports".to_string()>"Reports"</TabsTrigger>
                            <TabsTrigger value="notifications".to_string()>"Notifications"</TabsTrigger>
                        </TabsList>
                        
                        <div class="ml-6 flex-1">
                            <TabsContent value="overview".to_string()>
                                <div class="p-4">
                                    <h3 class="text-lg font-medium mb-2">"Overview"</h3>
                                    <p class="text-gray-600">"Dashboard overview and key metrics."</p>
                                </div>
                            </TabsContent>
                            
                            <TabsContent value="analytics".to_string()>
                                <div class="p-4">
                                    <h3 class="text-lg font-medium mb-2">"Analytics"</h3>
                                    <p class="text-gray-600">"Detailed analytics and insights."</p>
                                </div>
                            </TabsContent>
                            
                            <TabsContent value="reports".to_string()>
                                <div class="p-4">
                                    <h3 class="text-lg font-medium mb-2">"Reports"</h3>
                                    <p class="text-gray-600">"Generate and view reports."</p>
                                </div>
                            </TabsContent>
                            
                            <TabsContent value="notifications".to_string()>
                                <div class="p-4">
                                    <h3 class="text-lg font-medium mb-2">"Notifications"</h3>
                                    <p class="text-gray-600">"Manage notification preferences."</p>
                                </div>
                            </TabsContent>
                        </div>
                    </Tabs>
                </div>

                // Disabled Tabs
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Disabled Tabs"</h2>
                    <p class="text-gray-600 mb-4">"Tabs with disabled state"</p>
                    
                    <Tabs class="w-full".to_string()>
                        <TabsList class="grid w-full grid-cols-3".to_string()>
                            <TabsTrigger value="enabled".to_string()>"Enabled"</TabsTrigger>
                            <TabsTrigger value="disabled".to_string() disabled=true>"Disabled"</TabsTrigger>
                            <TabsTrigger value="another".to_string()>"Another"</TabsTrigger>
                        </TabsList>
                        
                        <TabsContent value="enabled".to_string()>
                            <div class="p-4">
                                <p class="text-gray-600">"This tab is enabled and functional."</p>
                            </div>
                        </TabsContent>
                        
                        <TabsContent value="disabled".to_string()>
                            <div class="p-4">
                                <p class="text-gray-600">"This content is not accessible due to disabled tab."</p>
                            </div>
                        </TabsContent>
                        
                        <TabsContent value="another".to_string()>
                            <div class="p-4">
                                <p class="text-gray-600">"Another enabled tab content."</p>
                            </div>
                        </TabsContent>
                    </Tabs>
                </div>

                // Current State Display
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Current State"</h2>
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                        <div class="bg-gray-50 p-3 rounded">
                            <div class="font-medium text-gray-900">"Active Tab"</div>
                            <div class="text-gray-600">{move || active_tab.get()}</div>
                        </div>
                        <div class="bg-gray-50 p-3 rounded">
                            <div class="font-medium text-gray-900">"Settings Tab"</div>
                            <div class="text-gray-600">{move || settings_tab.get()}</div>
                        </div>
                        <div class="bg-gray-50 p-3 rounded">
                            <div class="font-medium text-gray-900">"Profile Tab"</div>
                            <div class="text-gray-600">{move || profile_tab.get()}</div>
                        </div>
                        <div class="bg-gray-50 p-3 rounded">
                            <div class="font-medium text-gray-900">"Docs Tab"</div>
                            <div class="text-gray-600">{move || docs_tab.get()}</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

pub fn start_tabs_examples_fn() {
    mount_to_body(|| view! { <TabsExamples /> });
}
