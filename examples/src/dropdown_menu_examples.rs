use leptos::*;
use leptos::prelude::*;
use leptos::logging::log;
use radix_leptos_primitives::*;

#[component]
pub fn DropdownMenuExamples() -> impl IntoView {
    let (selected_theme, set_selected_theme) = signal("light".to_string());
    let (show_grid, set_show_grid) = signal(false);
    let (show_rulers, set_show_rulers) = signal(true);
    let (zoom_level, set_zoom_level) = signal("100%".to_string());

    let handle_copy = Callback::new(move |_| {
        log!("Copy action triggered");
    });

    let handle_paste = Callback::new(move |_| {
        log!("Paste action triggered");
    });

    let handle_cut = Callback::new(move |_| {
        log!("Cut action triggered");
    });

    let handle_delete = Callback::new(move |_| {
        log!("Delete action triggered");
    });

    let handle_theme_change = Callback::new(move |theme: String| {
        let theme_clone = theme.clone();
        log!("Theme changed to: {}", theme_clone);
        set_selected_theme.set(theme);
    });

    let handle_grid_toggle = Callback::new(move |checked: bool| {
        log!("Grid visibility toggled: {}", checked);
        set_show_grid.set(checked);
    });

    let handle_rulers_toggle = Callback::new(move |checked: bool| {
        log!("Rulers visibility toggled: {}", checked);
        set_show_rulers.set(checked);
    });

    let handle_zoom_change = Callback::new(move |zoom: String| {
        let zoom_clone = zoom.clone();
        log!("Zoom level changed to: {}", zoom_clone);
        set_zoom_level.set(zoom);
    });

    view! {
        <div class="min-h-screen bg-gray-50 p-8">
            <div class="max-w-4xl mx-auto">
                <h1 class="text-3xl font-bold text-gray-900 mb-8">"DropdownMenu Examples"</h1>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    
                    // Basic Dropdown Menu
                    <div class="bg-white rounded-lg shadow-md p-6">
                        <h2 class="text-xl font-semibold text-gray-900 mb-4">"Basic Dropdown Menu"</h2>
                        <DropdownMenu>
                            <DropdownMenuTrigger class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2".to_string()>
                                "Open Menu"
                                <svg class="ml-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                                </svg>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent>
                                <DropdownMenuItem on_click=handle_copy>
                                    "Copy"
                                </DropdownMenuItem>
                                <DropdownMenuItem on_click=handle_paste>
                                    "Paste"
                                </DropdownMenuItem>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem on_click=handle_cut>
                                    "Cut"
                                </DropdownMenuItem>
                                <DropdownMenuItem variant=DropdownMenuItemVariant::Destructive on_click=handle_delete>
                                    "Delete"
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </div>

                    // Theme Selection
                    <div class="bg-white rounded-lg shadow-md p-6">
                        <h2 class="text-xl font-semibold text-gray-900 mb-4">"Theme Selection"</h2>
                        <DropdownMenu>
                            <DropdownMenuTrigger class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2".to_string()>
                                {move || format!("Theme: {}", selected_theme.get())}
                                <svg class="ml-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                                </svg>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent>
                                <DropdownMenuLabel>"Select Theme"</DropdownMenuLabel>
                                <DropdownMenuSeparator />
                                <DropdownMenuRadioItem 
                                    value="light".to_string()
                                    checked=(move || selected_theme.get() == "light")()
                                    on_value_change=handle_theme_change
                                >
                                    "Light"
                                </DropdownMenuRadioItem>
                                <DropdownMenuRadioItem 
                                    value="dark".to_string()
                                    checked=(move || selected_theme.get() == "dark")()
                                    on_value_change=handle_theme_change
                                >
                                    "Dark"
                                </DropdownMenuRadioItem>
                                <DropdownMenuRadioItem 
                                    value="system".to_string()
                                    checked=(move || selected_theme.get() == "system")()
                                    on_value_change=handle_theme_change
                                >
                                    "System"
                                </DropdownMenuRadioItem>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </div>

                    // View Options
                    <div class="bg-white rounded-lg shadow-md p-6">
                        <h2 class="text-xl font-semibold text-gray-900 mb-4">"View Options"</h2>
                        <DropdownMenu>
                            <DropdownMenuTrigger class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2".to_string()>
                                "View Options"
                                <svg class="ml-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                                </svg>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent>
                                <DropdownMenuLabel>"Display Options"</DropdownMenuLabel>
                                <DropdownMenuSeparator />
                                <DropdownMenuCheckboxItem 
                                    checked=(move || show_grid.get())()
                                    on_checked_change=handle_grid_toggle
                                >
                                    "Show Grid"
                                </DropdownMenuCheckboxItem>
                                <DropdownMenuCheckboxItem 
                                    checked=(move || show_rulers.get())()
                                    on_checked_change=handle_rulers_toggle
                                >
                                    "Show Rulers"
                                </DropdownMenuCheckboxItem>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem disabled=true>
                                    "Snap to Grid"
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </div>

                    // Zoom Controls
                    <div class="bg-white rounded-lg shadow-md p-6">
                        <h2 class="text-xl font-semibold text-gray-900 mb-4">"Zoom Controls"</h2>
                        <DropdownMenu>
                            <DropdownMenuTrigger class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2".to_string()>
                                {move || format!("Zoom: {}", zoom_level.get())}
                                <svg class="ml-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                                </svg>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent>
                                <DropdownMenuLabel>"Zoom Level"</DropdownMenuLabel>
                                <DropdownMenuSeparator />
                                <DropdownMenuRadioItem 
                                    value="50%".to_string()
                                    checked=(move || zoom_level.get() == "50%")()
                                    on_value_change=handle_zoom_change
                                >
                                    "50%"
                                </DropdownMenuRadioItem>
                                <DropdownMenuRadioItem 
                                    value="75%".to_string()
                                    checked=(move || zoom_level.get() == "75%")()
                                    on_value_change=handle_zoom_change
                                >
                                    "75%"
                                </DropdownMenuRadioItem>
                                <DropdownMenuRadioItem 
                                    value="100%".to_string()
                                    checked=(move || zoom_level.get() == "100%")()
                                    on_value_change=handle_zoom_change
                                >
                                    "100%"
                                </DropdownMenuRadioItem>
                                <DropdownMenuRadioItem 
                                    value="125%".to_string()
                                    checked=(move || zoom_level.get() == "125%")()
                                    on_value_change=handle_zoom_change
                                >
                                    "125%"
                                </DropdownMenuRadioItem>
                                <DropdownMenuRadioItem 
                                    value="150%".to_string()
                                    checked=(move || zoom_level.get() == "150%")()
                                    on_value_change=handle_zoom_change
                                >
                                    "150%"
                                </DropdownMenuRadioItem>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </div>

                    // Complex Menu with Icons
                    <div class="bg-white rounded-lg shadow-md p-6">
                        <h2 class="text-xl font-semibold text-gray-900 mb-4">"Complex Menu with Icons"</h2>
                        <DropdownMenu>
                            <DropdownMenuTrigger class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2".to_string()>
                                "More Options"
                                <svg class="ml-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z"/>
                                </svg>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent>
                                <DropdownMenuItem on_click=handle_copy>
                                    <svg class="mr-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/>
                                    </svg>
                                    "Copy"
                                </DropdownMenuItem>
                                <DropdownMenuItem on_click=handle_paste>
                                    <svg class="mr-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"/>
                                    </svg>
                                    "Paste"
                                </DropdownMenuItem>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem on_click=handle_cut>
                                    <svg class="mr-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 4v16a2 2 0 002 2h8a2 2 0 002-2V7.242a2 2 0 00-.586-1.414l-4.242-4.242A2 2 0 0010.242 4H6z"/>
                                    </svg>
                                    "Cut"
                                </DropdownMenuItem>
                                <DropdownMenuItem variant=DropdownMenuItemVariant::Destructive on_click=handle_delete>
                                    <svg class="mr-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                                    </svg>
                                    "Delete"
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </div>

                    // Disabled Menu
                    <div class="bg-white rounded-lg shadow-md p-6">
                        <h2 class="text-xl font-semibold text-gray-900 mb-4">"Disabled Menu"</h2>
                        <DropdownMenu>
                            <DropdownMenuTrigger disabled=true class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2".to_string()>
                                "Disabled Menu"
                                <svg class="ml-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                                </svg>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent>
                                <DropdownMenuItem disabled=true>
                                    "This menu is disabled"
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </div>

                </div>

                // Current State Display
                <div class="mt-8 bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-xl font-semibold text-gray-900 mb-4">"Current State"</h2>
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                        <div>
                            <span class="font-medium">"Theme: "</span>
                            <span class="text-blue-600">{selected_theme}</span>
                        </div>
                        <div>
                            <span class="font-medium">"Show Grid: "</span>
                            <span class="text-blue-600">{move || if show_grid.get() { "Yes" } else { "No" }}</span>
                        </div>
                        <div>
                            <span class="font-medium">"Show Rulers: "</span>
                            <span class="text-blue-600">{move || if show_rulers.get() { "Yes" } else { "No" }}</span>
                        </div>
                        <div>
                            <span class="font-medium">"Zoom Level: "</span>
                            <span class="text-blue-600">{zoom_level}</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

pub fn start_dropdown_menu_examples_fn() {
    mount_to_body(|| view! { <DropdownMenuExamples/> })
}
