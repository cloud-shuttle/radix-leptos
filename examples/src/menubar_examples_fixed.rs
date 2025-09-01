use leptos::prelude::*;
use leptos::logging::log;
use radix_leptos_primitives::components::*;

#[component]
pub fn MenubarExamples() -> impl IntoView {
    // State for different menubar examples
    let (selected_theme, set_selected_theme) = signal("dark".to_string());
    let (show_grid, set_show_grid) = signal(false);
    let (show_rulers, set_show_rulers) = signal(true);
    let (zoom_level, set_zoom_level) = signal("100%".to_string());
    let (selected_file, set_selected_file) = signal("untitled.txt".to_string());
    let (auto_save, set_auto_save) = signal(true);
    let (spell_check, set_spell_check) = signal(false);

    // Computed signals for radio items
    let zoom_50_checked = Memo::new(move |_| zoom_level.get() == "50%");
    let zoom_100_checked = Memo::new(move |_| zoom_level.get() == "100%");
    let zoom_150_checked = Memo::new(move |_| zoom_level.get() == "150%");
    let theme_light_checked = Memo::new(move |_| selected_theme.get() == "light");
    let theme_dark_checked = Memo::new(move |_| selected_theme.get() == "dark");
    let theme_auto_checked = Memo::new(move |_| selected_theme.get() == "auto");

    // Callback functions
    let handle_new_file = Callback::new(move |_| {
        log!("New file created");
    });

    let handle_open_file = Callback::new(move |_| {
        log!("Open file dialog");
    });

    let handle_save_file = Callback::new(move |_| {
        log!("File saved");
    });

    let handle_save_as = Callback::new(move |_| {
        log!("Save as dialog");
    });

    let handle_export = Callback::new(move |_| {
        log!("Export dialog");
    });

    let handle_print = Callback::new(move |_| {
        log!("Print dialog");
    });

    let handle_undo = Callback::new(move |_| {
        log!("Undo action");
    });

    let handle_redo = Callback::new(move |_| {
        log!("Redo action");
    });

    let handle_cut = Callback::new(move |_| {
        log!("Cut action");
    });

    let handle_copy = Callback::new(move |_| {
        log!("Copy action");
    });

    let handle_paste = Callback::new(move |_| {
        log!("Paste action");
    });

    let handle_find = Callback::new(move |_| {
        log!("Find dialog");
    });

    let handle_replace = Callback::new(move |_| {
        log!("Replace dialog");
    });

    let handle_theme_change = Callback::new(move |theme: String| {
        set_selected_theme.set(theme);
        log!("Theme changed to: {}", theme);
    });

    let handle_grid_toggle = Callback::new(move |checked: bool| {
        set_show_grid.set(checked);
        log!("Grid visibility: {}", checked);
    });

    let handle_rulers_toggle = Callback::new(move |checked: bool| {
        set_show_rulers.set(checked);
        log!("Rulers visibility: {}", checked);
    });

    let handle_zoom_change = Callback::new(move |zoom: String| {
        set_zoom_level.set(zoom.clone());
        log!("Zoom level: {}", zoom);
    });

    let handle_auto_save_toggle = Callback::new(move |checked: bool| {
        set_auto_save.set(checked);
        log!("Auto save: {}", checked);
    });

    let handle_spell_check_toggle = Callback::new(move |checked: bool| {
        set_spell_check.set(checked);
        log!("Spell check: {}", checked);
    });

    view! {
        <div class="min-h-screen bg-gray-50 p-8">
            <div class="max-w-6xl mx-auto space-y-8">
                <div class="text-center">
                    <h1 class="text-4xl font-bold text-gray-900 mb-4">"Menubar Component Examples"</h1>
                    <p class="text-lg text-gray-600">"Horizontal menu bars with dropdown menus for application navigation"</p>
                </div>

                // Basic Menubar Example
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Basic Menubar"</h2>
                    <p class="text-gray-600 mb-4">"A simple menubar with File, Edit, and View menus"</p>
                    
                    <Menubar class="w-full max-w-2xl".to_string()>
                        <MenubarTrigger on_click=handle_new_file>
                            "File"
                        </MenubarTrigger>
                        <MenubarContent>
                            <MenubarItem on_click=handle_new_file>
                                "New File"
                            </MenubarItem>
                            <MenubarItem on_click=handle_open_file>
                                "Open..."
                            </MenubarItem>
                            <MenubarItem on_click=handle_save_file>
                                "Save"
                            </MenubarItem>
                            <MenubarSeparator />
                            <MenubarItem on_click=handle_save_as>
                                "Save As..."
                            </MenubarItem>
                            <MenubarItem on_click=handle_export>
                                "Export..."
                            </MenubarItem>
                            <MenubarSeparator />
                            <MenubarItem on_click=handle_print>
                                "Print..."
                            </MenubarItem>
                        </MenubarContent>

                        <MenubarTrigger on_click=handle_undo>
                            "Edit"
                        </MenubarTrigger>
                        <MenubarContent>
                            <MenubarItem on_click=handle_undo>
                                "Undo"
                            </MenubarItem>
                            <MenubarItem on_click=handle_redo>
                                "Redo"
                            </MenubarItem>
                            <MenubarSeparator />
                            <MenubarItem on_click=handle_cut>
                                "Cut"
                            </MenubarItem>
                            <MenubarItem on_click=handle_copy>
                                "Copy"
                            </MenubarItem>
                            <MenubarItem on_click=handle_paste>
                                "Paste"
                            </MenubarItem>
                        </MenubarContent>

                        <MenubarTrigger on_click=handle_find>
                            "View"
                        </MenubarTrigger>
                        <MenubarContent>
                            <MenubarCheckboxItem 
                                checked=show_grid 
                                on_checked_change=handle_grid_toggle
                            >
                                "Show Grid"
                            </MenubarCheckboxItem>
                            <MenubarCheckboxItem 
                                checked=show_rulers 
                                on_checked_change=handle_rulers_toggle
                            >
                                "Show Rulers"
                            </MenubarCheckboxItem>
                            <MenubarSeparator />
                            <MenubarLabel>
                                "Zoom"
                            </MenubarLabel>
                            <MenubarRadioItem 
                                value="50%".to_string()
                                checked=zoom_50_checked 
                                on_value_change=handle_zoom_change
                            >
                                "50%"
                            </MenubarRadioItem>
                            <MenubarRadioItem 
                                value="100%".to_string()
                                checked=zoom_100_checked 
                                on_value_change=handle_zoom_change
                            >
                                "100%"
                            </MenubarRadioItem>
                            <MenubarRadioItem 
                                value="150%".to_string()
                                checked=zoom_150_checked 
                                on_value_change=handle_zoom_change
                            >
                                "150%"
                            </MenubarRadioItem>
                        </MenubarContent>
                    </Menubar>
                </div>

                // Advanced Menubar Example
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Advanced Menubar"</h2>
                    <p class="text-gray-600 mb-4">"A more complex menubar with themes, settings, and nested menus"</p>
                    
                    <Menubar class="w-full max-w-4xl".to_string()>
                        <MenubarTrigger on_click=handle_new_file>
                            "File"
                        </MenubarTrigger>
                        <MenubarContent>
                            <MenubarItem on_click=handle_new_file>
                                "New File"
                            </MenubarItem>
                            <MenubarItem on_click=handle_open_file>
                                "Open..."
                            </MenubarItem>
                            <MenubarItem on_click=handle_save_file>
                                "Save"
                            </MenubarItem>
                            <MenubarSeparator />
                            <MenubarItem on_click=handle_save_as>
                                "Save As..."
                            </MenubarItem>
                            <MenubarItem on_click=handle_export>
                                "Export..."
                            </MenubarItem>
                            <MenubarSeparator />
                            <MenubarItem on_click=handle_print>
                                "Print..."
                            </MenubarItem>
                        </MenubarContent>

                        <MenubarTrigger on_click=handle_undo>
                            "Edit"
                        </MenubarTrigger>
                        <MenubarContent>
                            <MenubarItem on_click=handle_undo>
                                "Undo"
                            </MenubarItem>
                            <MenubarItem on_click=handle_redo>
                                "Redo"
                            </MenubarItem>
                            <MenubarSeparator />
                            <MenubarItem on_click=handle_cut>
                                "Cut"
                            </MenubarItem>
                            <MenubarItem on_click=handle_copy>
                                "Copy"
                            </MenubarItem>
                            <MenubarItem on_click=handle_paste>
                                "Paste"
                            </MenubarItem>
                            <MenubarSeparator />
                            <MenubarItem on_click=handle_find>
                                "Find..."
                            </MenubarItem>
                            <MenubarItem on_click=handle_replace>
                                "Replace..."
                            </MenubarItem>
                        </MenubarContent>

                        <MenubarTrigger on_click=handle_find>
                            "View"
                        </MenubarTrigger>
                        <MenubarContent>
                            <MenubarCheckboxItem 
                                checked=show_grid 
                                on_checked_change=handle_grid_toggle
                            >
                                "Show Grid"
                            </MenubarCheckboxItem>
                            <MenubarCheckboxItem 
                                checked=show_rulers 
                                on_checked_change=handle_rulers_toggle
                            >
                                "Show Rulers"
                            </MenubarCheckboxItem>
                            <MenubarSeparator />
                            <MenubarLabel>
                                "Zoom"
                            </MenubarLabel>
                            <MenubarRadioItem 
                                value="50%".to_string()
                                checked=zoom_50_checked 
                                on_value_change=handle_zoom_change
                            >
                                "50%"
                            </MenubarRadioItem>
                            <MenubarRadioItem 
                                value="100%".to_string()
                                checked=zoom_100_checked 
                                on_value_change=handle_zoom_change
                            >
                                "100%"
                            </MenubarRadioItem>
                            <MenubarRadioItem 
                                value="150%".to_string()
                                checked=zoom_150_checked 
                                on_value_change=handle_zoom_change
                            >
                                "150%"
                            </MenubarRadioItem>
                        </MenubarContent>

                        <MenubarTrigger on_click=handle_find>
                            "Theme"
                        </MenubarTrigger>
                        <MenubarContent>
                            <MenubarLabel>
                                "Color Theme"
                            </MenubarLabel>
                            <MenubarRadioItem 
                                value="light".to_string()
                                checked=theme_light_checked
                                on_value_change=handle_theme_change
                            >
                                "Light"
                            </MenubarRadioItem>
                            <MenubarRadioItem 
                                value="dark".to_string()
                                checked=theme_dark_checked
                                on_value_change=handle_theme_change
                            >
                                "Dark"
                            </MenubarRadioItem>
                            <MenubarRadioItem 
                                value="auto".to_string()
                                checked=theme_auto_checked
                                on_value_change=handle_theme_change
                            >
                                "Auto"
                            </MenubarRadioItem>
                        </MenubarContent>

                        <MenubarTrigger on_click=handle_find>
                            "Settings"
                        </MenubarTrigger>
                        <MenubarContent>
                            <MenubarCheckboxItem 
                                checked=auto_save 
                                on_checked_change=handle_auto_save_toggle
                            >
                                "Auto Save"
                            </MenubarCheckboxItem>
                            <MenubarCheckboxItem 
                                checked=spell_check 
                                on_checked_change=handle_spell_check_toggle
                            >
                                "Spell Check"
                            </MenubarCheckboxItem>
                            <MenubarSeparator />
                            <MenubarItem variant=MenubarItemVariant::Destructive on_click=handle_new_file>
                                "Reset Settings"
                            </MenubarItem>
                        </MenubarContent>
                    </Menubar>
                </div>

                // Different Sizes Example
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Menubar Sizes"</h2>
                    <p class="text-gray-600 mb-4">"Menubars in different sizes: Small, Medium, and Large"</p>
                    
                    <div class="space-y-4">
                        <div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"Small"</h3>
                            <Menubar size=MenubarSize::Small class="w-full max-w-2xl".to_string()>
                                <MenubarTrigger on_click=handle_new_file>
                                    "File"
                                </MenubarTrigger>
                                <MenubarContent>
                                    <MenubarItem on_click=handle_new_file>
                                        "New"
                                    </MenubarItem>
                                    <MenubarItem on_click=handle_open_file>
                                        "Open"
                                    </MenubarItem>
                                    <MenubarItem on_click=handle_save_file>
                                        "Save"
                                    </MenubarItem>
                                </MenubarContent>
                            </Menubar>
                        </div>

                        <div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"Medium (Default)"</h3>
                            <Menubar size=MenubarSize::Medium class="w-full max-w-2xl".to_string()>
                                <MenubarTrigger on_click=handle_new_file>
                                    "File"
                                </MenubarTrigger>
                                <MenubarContent>
                                    <MenubarItem on_click=handle_new_file>
                                        "New"
                                    </MenubarItem>
                                    <MenubarItem on_click=handle_open_file>
                                        "Open"
                                    </MenubarItem>
                                    <MenubarItem on_click=handle_save_file>
                                        "Save"
                                    </MenubarItem>
                                </MenubarContent>
                            </Menubar>
                        </div>

                        <div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"Large"</h3>
                            <Menubar size=MenubarSize::Large class="w-full max-w-2xl".to_string()>
                                <MenubarTrigger on_click=handle_new_file>
                                    "File"
                                </MenubarTrigger>
                                <MenubarContent>
                                    <MenubarItem on_click=handle_new_file>
                                        "New"
                                    </MenubarItem>
                                    <MenubarItem on_click=handle_open_file>
                                        "Open"
                                    </MenubarItem>
                                    <MenubarItem on_click=handle_save_file>
                                        "Save"
                                    </MenubarItem>
                                </MenubarContent>
                            </Menubar>
                        </div>
                    </div>
                </div>

                // Item Variants Example
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Item Variants"</h2>
                    <p class="text-gray-600 mb-4">"Different types of menubar items: Default, Destructive, and Disabled"</p>
                    
                    <Menubar class="w-full max-w-2xl".to_string()>
                        <MenubarTrigger on_click=handle_new_file>
                            "Actions"
                        </MenubarTrigger>
                        <MenubarContent>
                            <MenubarItem on_click=handle_new_file>
                                "Normal Action"
                            </MenubarItem>
                            <MenubarItem variant=MenubarItemVariant::Destructive on_click=handle_new_file>
                                "Destructive Action"
                            </MenubarItem>
                            <MenubarItem variant=MenubarItemVariant::Disabled disabled=true on_click=handle_new_file>
                                "Disabled Action"
                            </MenubarItem>
                        </MenubarContent>
                    </Menubar>
                </div>

                // State Display
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Current State"</h2>
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                        <div class="bg-gray-50 p-3 rounded">
                            <div class="font-medium text-gray-900">"Selected Theme"</div>
                            <div class="text-gray-600">{move || selected_theme.get()}</div>
                        </div>
                        <div class="bg-gray-50 p-3 rounded">
                            <div class="font-medium text-gray-900">"Show Grid"</div>
                            <div class="text-gray-600">{move || if show_grid.get() { "Yes" } else { "No" }}</div>
                        </div>
                        <div class="bg-gray-50 p-3 rounded">
                            <div class="font-medium text-gray-900">"Show Rulers"</div>
                            <div class="text-gray-600">{move || if show_rulers.get() { "Yes" } else { "No" }}</div>
                        </div>
                        <div class="bg-gray-50 p-3 rounded">
                            <div class="font-medium text-gray-900">"Zoom Level"</div>
                            <div class="text-gray-600">{move || zoom_level.get()}</div>
                        </div>
                        <div class="bg-gray-50 p-3 rounded">
                            <div class="font-medium text-gray-900">"Auto Save"</div>
                            <div class="text-gray-600">{move || if auto_save.get() { "Enabled" } else { "Disabled" }}</div>
                        </div>
                        <div class="bg-gray-50 p-3 rounded">
                            <div class="font-medium text-gray-900">"Spell Check"</div>
                            <div class="text-gray-600">{move || if spell_check.get() { "Enabled" } else { "Disabled" }}</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

pub fn start_menubar_examples_fn() {
    mount_to_body(|| view! { <MenubarExamples /> });
}
