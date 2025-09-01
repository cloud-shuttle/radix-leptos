use leptos::*;
use leptos::prelude::*;
use leptos::logging::log;
use radix_leptos_primitives::*;

#[component]
pub fn ContextMenuExamples() -> impl IntoView {
    let (selected_theme, set_selected_theme) = signal("light".to_string());
    let (show_grid, set_show_grid) = signal(true);
    let (show_rulers, set_show_rulers) = signal(false);
    let (zoom_level, set_zoom_level) = signal("100%".to_string());

    let handle_copy = Callback::new(move |_e: web_sys::MouseEvent| {
        log!("Copy action triggered");
    });

    let handle_paste = Callback::new(move |_e: web_sys::MouseEvent| {
        log!("Paste action triggered");
    });

    let handle_cut = Callback::new(move |_e: web_sys::MouseEvent| {
        log!("Cut action triggered");
    });

    let handle_delete = Callback::new(move |_e: web_sys::MouseEvent| {
        log!("Delete action triggered");
    });

    let handle_theme_change = Callback::new(move |theme: String| {
        let theme_clone = theme.clone();
        set_selected_theme.set(theme);
        log!("Theme changed to: {}", theme_clone);
    });

    let handle_grid_toggle = Callback::new(move |show: bool| {
        set_show_grid.set(show);
        log!("Grid visibility: {}", show);
    });

    let handle_rulers_toggle = Callback::new(move |show: bool| {
        set_show_rulers.set(show);
        log!("Rulers visibility: {}", show);
    });

    let handle_zoom_change = Callback::new(move |zoom: String| {
        let zoom_clone = zoom.clone();
        set_zoom_level.set(zoom);
        log!("Zoom level changed to: {}", zoom_clone);
    });

    view! {
        <div class="context-menu-examples">
            <h1>"Context Menu Component Examples"</h1>
            
            <section class="example-section">
                <h2>"Basic Context Menu"</h2>
                <p>"Right-click on the area below to see a basic context menu."</p>
                <div class="example-container">
                    <ContextMenu size=ContextMenuSize::Medium>
                        <div class="context-menu-trigger-area">
                            <p>"Right-click here for context menu"</p>
                            <p>"This area demonstrates a basic context menu with common actions."</p>
                        </div>
                        <ContextMenuContent>
                            <ContextMenuItem on_click=handle_copy.clone()>
                                "Copy"
                            </ContextMenuItem>
                            <ContextMenuItem on_click=handle_paste.clone()>
                                "Paste"
                            </ContextMenuItem>
                            <ContextMenuItem on_click=handle_cut.clone()>
                                "Cut"
                            </ContextMenuItem>
                            <ContextMenuSeparator />
                            <ContextMenuItem variant=ContextMenuItemVariant::Destructive on_click=handle_delete.clone()>
                                "Delete"
                            </ContextMenuItem>
                        </ContextMenuContent>
                    </ContextMenu>
                </div>
            </section>

            <section class="example-section">
                <h2>"Context Menu with Checkboxes"</h2>
                <p>"Context menu with checkbox items for toggling features."</p>
                <div class="example-container">
                    <ContextMenu size=ContextMenuSize::Medium>
                        <div class="context-menu-trigger-area">
                            <p>"Right-click here for checkbox menu"</p>
                            <p>"Current settings: Grid: " {move || if show_grid.get() { "On" } else { "Off" }} ", Rulers: " {move || if show_rulers.get() { "On" } else { "Off" }}</p>
                        </div>
                        <ContextMenuContent>
                            <ContextMenuLabel>
                                "View Options"
                            </ContextMenuLabel>
                            <ContextMenuCheckboxItem 
                                checked=(move || show_grid.get())()
                                on_change=handle_grid_toggle.clone()
                            >
                                "Show Grid"
                            </ContextMenuCheckboxItem>
                            <ContextMenuCheckboxItem 
                                checked=(move || show_rulers.get())()
                                on_change=handle_rulers_toggle.clone()
                            >
                                "Show Rulers"
                            </ContextMenuCheckboxItem>
                            <ContextMenuSeparator />
                            <ContextMenuLabel>
                                "Actions"
                            </ContextMenuLabel>
                            <ContextMenuItem on_click=handle_copy.clone()>
                                "Copy Selection"
                            </ContextMenuItem>
                        </ContextMenuContent>
                    </ContextMenu>
                </div>
            </section>

            <section class="example-section">
                <h2>"Context Menu with Radio Items"</h2>
                <p>"Context menu with radio items for theme selection."</p>
                <div class="example-container">
                    <ContextMenu size=ContextMenuSize::Medium>
                        <div class="context-menu-trigger-area">
                            <p>"Right-click here for theme selection"</p>
                            <p>"Current theme: " {selected_theme}</p>
                        </div>
                        <ContextMenuContent>
                            <ContextMenuLabel>
                                "Theme Selection"
                            </ContextMenuLabel>
                            <ContextMenuRadioItem 
                                value="light".to_string()
                                checked=(move || selected_theme.get() == "light")()
                                on_change=handle_theme_change.clone()
                            >
                                "Light Theme"
                            </ContextMenuRadioItem>
                            <ContextMenuRadioItem 
                                value="dark".to_string()
                                checked=(move || selected_theme.get() == "dark")()
                                on_change=handle_theme_change.clone()
                            >
                                "Dark Theme"
                            </ContextMenuRadioItem>
                            <ContextMenuRadioItem 
                                value="auto".to_string()
                                checked=(move || selected_theme.get() == "auto")()
                                on_change=handle_theme_change.clone()
                            >
                                "Auto (System)"
                            </ContextMenuRadioItem>
                        </ContextMenuContent>
                    </ContextMenu>
                </div>
            </section>

            <section class="example-section">
                <h2>"Context Menu with Zoom Options"</h2>
                <p>"Context menu with radio items for zoom level selection."</p>
                <div class="example-container">
                    <ContextMenu size=ContextMenuSize::Medium>
                        <div class="context-menu-trigger-area">
                            <p>"Right-click here for zoom options"</p>
                            <p>"Current zoom: " {zoom_level}</p>
                        </div>
                        <ContextMenuContent>
                            <ContextMenuLabel>
                                "Zoom Level"
                            </ContextMenuLabel>
                            <ContextMenuRadioItem 
                                value="50%".to_string()
                                checked=(move || zoom_level.get() == "50%")()
                                on_change=handle_zoom_change.clone()
                            >
                                "50%"
                            </ContextMenuRadioItem>
                            <ContextMenuRadioItem 
                                value="75%".to_string()
                                checked=(move || zoom_level.get() == "75%")()
                                on_change=handle_zoom_change.clone()
                            >
                                "75%"
                            </ContextMenuRadioItem>
                            <ContextMenuRadioItem 
                                value="100%".to_string()
                                checked=(move || zoom_level.get() == "100%")()
                                on_change=handle_zoom_change.clone()
                            >
                                "100%"
                            </ContextMenuRadioItem>
                            <ContextMenuRadioItem 
                                value="125%".to_string()
                                checked=(move || zoom_level.get() == "125%")()
                                on_change=handle_zoom_change.clone()
                            >
                                "125%"
                            </ContextMenuRadioItem>
                            <ContextMenuRadioItem 
                                value="150%".to_string()
                                checked=(move || zoom_level.get() == "150%")()
                                on_change=handle_zoom_change.clone()
                            >
                                "150%"
                            </ContextMenuRadioItem>
                        </ContextMenuContent>
                    </ContextMenu>
                </div>
            </section>

            <section class="example-section">
                <h2>"Large Context Menu"</h2>
                <p>"A larger context menu with more options."</p>
                <div class="example-container">
                    <ContextMenu size=ContextMenuSize::Large>
                        <div class="context-menu-trigger-area">
                            <p>"Right-click here for large menu"</p>
                            <p>"This demonstrates a larger context menu with more space."</p>
                        </div>
                        <ContextMenuContent>
                            <ContextMenuItem on_click=handle_copy.clone()>
                                "Copy"
                            </ContextMenuItem>
                            <ContextMenuItem on_click=handle_paste.clone()>
                                "Paste"
                            </ContextMenuItem>
                            <ContextMenuItem on_click=handle_cut.clone()>
                                "Cut"
                            </ContextMenuItem>
                            <ContextMenuSeparator />
                            <ContextMenuItem>
                                "Select All"
                            </ContextMenuItem>
                            <ContextMenuItem>
                                "Find and Replace"
                            </ContextMenuItem>
                            <ContextMenuSeparator />
                            <ContextMenuItem>
                                "Format Document"
                            </ContextMenuItem>
                            <ContextMenuItem>
                                "Organize Imports"
                            </ContextMenuItem>
                            <ContextMenuSeparator />
                            <ContextMenuItem variant=ContextMenuItemVariant::Destructive on_click=handle_delete.clone()>
                                "Delete"
                            </ContextMenuItem>
                        </ContextMenuContent>
                    </ContextMenu>
                </div>
            </section>

            <section class="example-section">
                <h2>"Small Context Menu"</h2>
                <p>"A compact context menu for limited space."</p>
                <div class="example-container">
                    <ContextMenu size=ContextMenuSize::Small>
                        <div class="context-menu-trigger-area">
                            <p>"Right-click here for small menu"</p>
                            <p>"Compact menu for tight spaces."</p>
                        </div>
                        <ContextMenuContent>
                            <ContextMenuItem on_click=handle_copy.clone()>
                                "Copy"
                            </ContextMenuItem>
                            <ContextMenuItem on_click=handle_paste.clone()>
                                "Paste"
                            </ContextMenuItem>
                            <ContextMenuSeparator />
                            <ContextMenuItem variant=ContextMenuItemVariant::Destructive on_click=handle_delete.clone()>
                                "Delete"
                            </ContextMenuItem>
                        </ContextMenuContent>
                    </ContextMenu>
                </div>
            </section>

            <section class="example-section">
                <h2>"Disabled Context Menu"</h2>
                <p>"A disabled context menu that doesn't respond to right-clicks."</p>
                <div class="example-container">
                    <ContextMenu disabled=true size=ContextMenuSize::Medium>
                        <div class="context-menu-trigger-area disabled">
                            <p>"Right-click here (disabled)"</p>
                            <p>"This context menu is disabled and won't respond to clicks."</p>
                        </div>
                        <ContextMenuContent>
                            <ContextMenuItem>
                                "This won't work"
                            </ContextMenuItem>
                        </ContextMenuContent>
                    </ContextMenu>
                </div>
            </section>

            <section class="example-section">
                <h2>"Complex Context Menu"</h2>
                <p>"A complex context menu combining all features."</p>
                <div class="example-container">
                    <ContextMenu size=ContextMenuSize::Large>
                        <div class="context-menu-trigger-area">
                            <p>"Right-click here for complex menu"</p>
                            <p>"This menu demonstrates all context menu features together."</p>
                        </div>
                        <ContextMenuContent>
                            <ContextMenuLabel>
                                "Edit Actions"
                            </ContextMenuLabel>
                            <ContextMenuItem on_click=handle_copy.clone()>
                                "Copy"
                            </ContextMenuItem>
                            <ContextMenuItem on_click=handle_paste.clone()>
                                "Paste"
                            </ContextMenuItem>
                            <ContextMenuItem on_click=handle_cut.clone()>
                                "Cut"
                            </ContextMenuItem>
                            <ContextMenuSeparator />
                            <ContextMenuLabel>
                                "View Settings"
                            </ContextMenuLabel>
                            <ContextMenuCheckboxItem 
                                checked=(move || show_grid.get())()
                                on_change=handle_grid_toggle.clone()
                            >
                                "Show Grid"
                            </ContextMenuCheckboxItem>
                            <ContextMenuCheckboxItem 
                                checked=(move || show_rulers.get())()
                                on_change=handle_rulers_toggle.clone()
                            >
                                "Show Rulers"
                            </ContextMenuCheckboxItem>
                            <ContextMenuSeparator />
                            <ContextMenuLabel>
                                "Theme"
                            </ContextMenuLabel>
                            <ContextMenuRadioItem 
                                value="light".to_string()
                                checked=(move || selected_theme.get() == "light")()
                                on_change=handle_theme_change.clone()
                            >
                                "Light"
                            </ContextMenuRadioItem>
                            <ContextMenuRadioItem 
                                value="dark".to_string()
                                checked=(move || selected_theme.get() == "dark")()
                                on_change=handle_theme_change.clone()
                            >
                                "Dark"
                            </ContextMenuRadioItem>
                            <ContextMenuSeparator />
                            <ContextMenuItem variant=ContextMenuItemVariant::Destructive on_click=handle_delete.clone()>
                                "Delete"
                            </ContextMenuItem>
                        </ContextMenuContent>
                    </ContextMenu>
                </div>
            </section>
        </div>
    }
}

pub fn start_context_menu_examples_fn() {
    mount_to_body(|| view! { <ContextMenuExamples /> });
}
