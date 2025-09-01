use leptos::*;
use leptos::prelude::*;
use leptos::callback::Callback;
use radix_leptos_primitives::*;

/// Comprehensive list examples demonstrating all features
#[component]
pub fn ListExamples() -> impl IntoView {
    // Reactive state for different list examples
    let (basic_selected_items, set_basic_selected_items) = signal(Vec::<String>::new());
    let (multi_selected_items, set_multi_selected_items) = signal(Vec::<String>::new());
    let (large_dataset_selected_items, set_large_dataset_selected_items) = signal(Vec::<String>::new());
    let (virtualized_selected_items, set_virtualized_selected_items) = signal(Vec::<String>::new());
    let (focused_item, set_focused_item) = signal(None::<String>);
    
    // Event handlers
    let handle_basic_selection = Callback::new(move |selected: Vec<String>| {
        let selected_clone = selected.clone();
        set_basic_selected_items.set(selected);
        web_sys::console::log_1(&format!("Basic list selection: {:?}", selected_clone).into());
    });
    
    let handle_multi_selection = Callback::new(move |selected: Vec<String>| {
        let selected_clone = selected.clone();
        set_multi_selected_items.set(selected);
        web_sys::console::log_1(&format!("Multi-select list selection: {:?}", selected_clone).into());
    });
    
    let handle_large_dataset_selection = Callback::new(move |selected: Vec<String>| {
        let selected_clone = selected.clone();
        set_large_dataset_selected_items.set(selected);
        web_sys::console::log_1(&format!("Large dataset selection: {:?}", selected_clone).into());
    });
    
    let handle_virtualized_selection = Callback::new(move |selected: Vec<String>| {
        let selected_clone = selected.clone();
        set_virtualized_selected_items.set(selected);
        web_sys::console::log_1(&format!("Virtualized list selection: {:?}", selected_clone).into());
    });
    
    let handle_item_click = Callback::new(move |item: ListItem<String>| {
        web_sys::console::log_1(&format!("Item clicked: {}", item.data).into());
    });
    
    let handle_item_focus = Callback::new(move |item: ListItem<String>| {
        set_focused_item.set(Some(item.id.clone()));
        web_sys::console::log_1(&format!("Item focused: {}", item.data).into());
    });

    // Sample data - create separate instances for each example
    let basic_items_1 = vec![
        create_list_item("1", "Apple".to_string()),
        create_list_item("2", "Banana".to_string()),
        create_list_item("3", "Cherry".to_string()),
        create_list_item("4", "Date".to_string()),
        create_list_item("5", "Elderberry".to_string()),
    ];
    
    let basic_items_2 = vec![
        create_list_item("1", "Apple".to_string()),
        create_list_item("2", "Banana".to_string()),
        create_list_item("3", "Cherry".to_string()),
        create_list_item("4", "Date".to_string()),
        create_list_item("5", "Elderberry".to_string()),
    ];
    
    let basic_items_3 = vec![
        create_list_item("1", "Apple".to_string()),
        create_list_item("2", "Banana".to_string()),
        create_list_item("3", "Cherry".to_string()),
        create_list_item("4", "Date".to_string()),
        create_list_item("5", "Elderberry".to_string()),
    ];
    
    let basic_items_4 = vec![
        create_list_item("1", "Apple".to_string()),
        create_list_item("2", "Banana".to_string()),
        create_list_item("3", "Cherry".to_string()),
        create_list_item("4", "Date".to_string()),
        create_list_item("5", "Elderberry".to_string()),
    ];
    
    let multi_items_1 = vec![
        create_list_item("m1", "Red".to_string()),
        create_list_item("m2", "Green".to_string()),
        create_list_item("m3", "Blue".to_string()),
        create_list_item("m4", "Yellow".to_string()),
        create_list_item("m5", "Purple".to_string()),
        create_list_item("m6", "Orange".to_string()),
    ];
    
    let multi_items_2 = vec![
        create_list_item("m1", "Red".to_string()),
        create_list_item("m2", "Green".to_string()),
        create_list_item("m3", "Blue".to_string()),
        create_list_item("m4", "Yellow".to_string()),
        create_list_item("m5", "Purple".to_string()),
        create_list_item("m6", "Orange".to_string()),
    ];
    
    let multi_items_3 = vec![
        create_list_item("m1", "Red".to_string()),
        create_list_item("m2", "Green".to_string()),
        create_list_item("m3", "Blue".to_string()),
        create_list_item("m4", "Yellow".to_string()),
        create_list_item("m5", "Purple".to_string()),
        create_list_item("m6", "Orange".to_string()),
    ];
    
    let multi_items_4 = vec![
        create_list_item("m1", "Red".to_string()),
        create_list_item("m2", "Green".to_string()),
        create_list_item("m3", "Blue".to_string()),
        create_list_item("m4", "Yellow".to_string()),
        create_list_item("m5", "Purple".to_string()),
        create_list_item("m6", "Orange".to_string()),
    ];
    
    let multi_items_5 = vec![
        create_list_item("m1", "Red".to_string()),
        create_list_item("m2", "Green".to_string()),
        create_list_item("m3", "Blue".to_string()),
        create_list_item("m4", "Yellow".to_string()),
        create_list_item("m5", "Purple".to_string()),
        create_list_item("m6", "Orange".to_string()),
    ];
    
    let basic_items_5 = vec![
        create_list_item("1", "Apple".to_string()),
        create_list_item("2", "Banana".to_string()),
        create_list_item("3", "Cherry".to_string()),
        create_list_item("4", "Date".to_string()),
        create_list_item("5", "Elderberry".to_string()),
    ];
    
    // Generate large dataset
    let large_dataset_items: Vec<ListItem<String>> = (1..=1000)
        .map(|i| create_list_item(&format!("large-{}", i), format!("Item {}", i)))
        .collect();
    
    // Generate virtualized dataset
    let virtualized_items: Vec<ListItem<String>> = (1..=10000)
        .map(|i| create_list_item(&format!("virtual-{}", i), format!("Virtual Item {}", i)))
        .collect();

    // Empty list for examples
    let empty_list: Vec<ListItem<String>> = vec![];

    view! {
        <div class="list-examples">
            <h1>"📋 List Component Examples"</h1>
            
            // Basic List Example
            <section class="example-section">
                <h2>"1. Basic List"</h2>
                <p>"Simple list with single selection and basic styling."</p>
                
                <List 
                    items=basic_items_1.clone()
                    selected_items=basic_selected_items.get()
                    on_selection_change=handle_basic_selection.clone()
                    on_item_click=handle_item_click.clone()
                    on_item_focus=handle_item_focus.clone()
                >
                    {basic_items_1.clone().into_iter().map(|item| {
                        let item_data = item.data.clone();
                        let item_id = item.id.clone();
                        view! {
                            <ListItem item=item>
                                <div class="list-item-content">
                                    <span class="item-text">{item_data}</span>
                                    <span class="item-id">"ID: " {item_id}</span>
                                </div>
                            </ListItem>
                        }
                    }).collect::<Vec<_>>()}
                </List>
                
                <div class="example-info">
                    <p><strong>"Selected items:"</strong> {format!("{:?}", basic_selected_items.get())}</p>
                    <p><strong>"Focused item:"</strong> {focused_item.get().unwrap_or_default()}</p>
                </div>
            </section>

            // Multi-Select List Example
            <section class="example-section">
                <h2>"2. Multi-Select List"</h2>
                <p>"List with multiple selection support and different styling variants."</p>
                
                <List 
                    items=multi_items_1.clone()
                    selected_items=multi_selected_items.get()
                    multi_select=true
                    variant=ListVariant::Bordered
                    on_selection_change=handle_multi_selection.clone()
                    on_item_click=handle_item_click.clone()
                    on_item_focus=handle_item_focus.clone()
                >
                    {multi_items_1.clone().into_iter().map(|item| {
                        let item_data = item.data.clone();
                        let item_id = item.id.clone();
                        view! {
                            <ListItem item=item>
                                <div class="list-item-content">
                                    <span class="item-text">{item_data}</span>
                                    <span class="item-checkbox">
                                        {if multi_selected_items.get().contains(&item_id) {
                                            "☑️"
                                        } else {
                                            "☐"
                                        }}
                                    </span>
                                </div>
                            </ListItem>
                        }
                    }).collect::<Vec<_>>()}
                </List>
                
                <div class="example-info">
                    <p><strong>"Selected items:"</strong> {format!("{:?}", multi_selected_items.get())}</p>
                    <p><strong>"Selection count:"</strong> {multi_selected_items.get().len()}</p>
                </div>
            </section>

            // Large Dataset List Example
            <section class="example-section">
                <h2>"3. Large Dataset List"</h2>
                <p>"List with 1,000 items demonstrating performance with large datasets."</p>
                
                <List 
                    items=large_dataset_items.clone()
                    selected_items=large_dataset_selected_items.get()
                    variant=ListVariant::Striped
                    size=ListSize::Small
                    on_selection_change=handle_large_dataset_selection.clone()
                    on_item_click=handle_item_click.clone()
                    on_item_focus=handle_item_focus.clone()
                >
                    {large_dataset_items.clone().into_iter().take(50).map(|item| {
                        let item_data = item.data.clone();
                        let item_id = item.id.clone();
                        let index_str = item_id.split('-').last().unwrap_or("0").to_string();
                        let item_clone = item.clone();
                        view! {
                            <ListItem item=item_clone>
                                <div class="list-item-content">
                                    <span class="item-text">{item_data}</span>
                                    <span class="item-index">"Index: " {index_str}</span>
                                </div>
                            </ListItem>
                        }
                    }).collect::<Vec<_>>()}
                </List>
                
                <div class="example-info">
                    <p><strong>"Total items:"</strong> "1,000"</p>
                    <p><strong>"Selected items:"</strong> {format!("{:?}", large_dataset_selected_items.get())}</p>
                    <p><strong>"Note:"</strong> "Only first 50 items shown for performance"</p>
                </div>
            </section>

            // Virtualized List Example
            <section class="example-section">
                <h2>"4. Virtualized List"</h2>
                <p>"High-performance virtualized list with 10,000 items using virtualization."</p>
                
                <List 
                    items=virtualized_items.clone()
                    selected_items=virtualized_selected_items.get()
                    variant=ListVariant::Compact
                    on_selection_change=handle_virtualized_selection.clone()
                    on_item_click=handle_item_click.clone()
                    on_item_focus=handle_item_focus.clone()
                >
                    {virtualized_items.clone().into_iter().take(100).map(|item| {
                        let item_data = item.data.clone();
                        let item_id = item.id.clone();
                        let id_str = item_id.split('-').last().unwrap_or("0").to_string();
                        let item_clone = item.clone();
                        view! {
                            <ListItem item=item_clone>
                                <div class="list-item-content">
                                    <span class="item-text">{item_data}</span>
                                    <span class="item-details">
                                        "Virtual Item • ID: " {id_str}
                                    </span>
                                </div>
                            </ListItem>
                        }
                    }).collect::<Vec<_>>()}
                </List>
                
                <div class="example-info">
                    <p><strong>"Total items:"</strong> "10,000"</p>
                    <p><strong>"Virtualization:"</strong> "Enabled with 50px item height"</p>
                    <p><strong>"Overscan:"</strong> "10 items"</p>
                    <p><strong>"Selected items:"</strong> {format!("{:?}", virtualized_selected_items.get())}</p>
                </div>
            </section>

            // Size Variants Example
            <section class="example-section">
                <h2>"5. List Size Variants"</h2>
                <p>"Different size variants for different use cases."</p>
                
                <div class="size-variants">
                    <div class="size-variant">
                        <h3>"Small Size"</h3>
                        <List 
                            items=basic_items_2.clone()
                            size=ListSize::Small
                            variant=ListVariant::Compact
                        >
                            {basic_items_2.clone().into_iter().map(|item| {
                                let item_data = item.data.clone();
                                view! {
                                    <ListItem item=item>
                                        <span class="item-text">{item_data}</span>
                                    </ListItem>
                                }
                            }).collect::<Vec<_>>()}
                        </List>
                    </div>
                    
                    <div class="size-variant">
                        <h3>"Medium Size (Default)"</h3>
                        <List 
                            items=basic_items_3.clone()
                            size=ListSize::Medium
                            variant=ListVariant::Default
                        >
                            {basic_items_3.clone().into_iter().map(|item| {
                                let item_data = item.data.clone();
                                view! {
                                    <ListItem item=item>
                                        <span class="item-text">{item_data}</span>
                                    </ListItem>
                                }
                            }).collect::<Vec<_>>()}
                        </List>
                    </div>
                    
                    <div class="size-variant">
                        <h3>"Large Size"</h3>
                        <List 
                            items=basic_items_4.clone()
                            size=ListSize::Large
                            variant=ListVariant::Bordered
                        >
                            {basic_items_4.clone().into_iter().map(|item| {
                                let item_data = item.data.clone();
                                view! {
                                    <ListItem item=item>
                                        <span class="item-text">{item_data}</span>
                                    </ListItem>
                                }
                            }).collect::<Vec<_>>()}
                        </List>
                    </div>
                </div>
            </section>

            // Variant Styles Example
            <section class="example-section">
                <h2>"6. List Variant Styles"</h2>
                <p>"Different styling variants for different visual needs."</p>
                
                <div class="variant-styles">
                    <div class="variant-style">
                        <h3>"Default Variant"</h3>
                        <List 
                            items=multi_items_2.clone()
                            variant=ListVariant::Default
                        >
                            {multi_items_2.clone().into_iter().take(3).map(|item| {
                                let item_data = item.data.clone();
                                view! {
                                    <ListItem item=item>
                                        <span class="item-text">{item_data}</span>
                                    </ListItem>
                                }
                            }).collect::<Vec<_>>()}
                        </List>
                    </div>
                    
                    <div class="variant-style">
                        <h3>"Bordered Variant"</h3>
                        <List 
                            items=multi_items_3.clone()
                            variant=ListVariant::Bordered
                        >
                            {multi_items_3.clone().into_iter().take(3).map(|item| {
                                let item_data = item.data.clone();
                                view! {
                                    <ListItem item=item>
                                        <span class="item-text">{item_data}</span>
                                    </ListItem>
                                }
                            }).collect::<Vec<_>>()}
                        </List>
                    </div>
                    
                    <div class="variant-style">
                        <h3>"Striped Variant"</h3>
                        <List 
                            items=multi_items_4.clone()
                            variant=ListVariant::Striped
                        >
                            {multi_items_4.clone().into_iter().take(3).map(|item| {
                                let item_data = item.data.clone();
                                view! {
                                    <ListItem item=item>
                                        <span class="item-text">{item_data}</span>
                                    </ListItem>
                                }
                            }).collect::<Vec<_>>()}
                        </List>
                    </div>
                    
                    <div class="variant-style">
                        <h3>"Compact Variant"</h3>
                        <List 
                            items=multi_items_5.clone()
                            variant=ListVariant::Compact
                        >
                            {multi_items_5.clone().into_iter().take(3).map(|item| {
                                let item_data = item.data.clone();
                                view! {
                                    <ListItem item=item>
                                        <span class="item-text">{item_data}</span>
                                    </ListItem>
                                }
                            }).collect::<Vec<_>>()}
                        </List>
                    </div>
                </div>
            </section>

            // Empty and Loading States
            <section class="example-section">
                <h2>"7. Empty and Loading States"</h2>
                <p>"Handling empty lists and loading states."</p>
                
                <div class="state-examples">
                    <div class="state-example">
                        <h3>"Empty State"</h3>
                        <List 
                            items=empty_list.clone()
                            variant=ListVariant::Bordered
                        >
                            <ListEmpty message="No items available".to_string()>
                                <p>"Try adding some items to see them here."</p>
                            </ListEmpty>
                        </List>
                    </div>
                    
                    <div class="state-example">
                        <h3>"Loading State"</h3>
                        <List 
                            items=empty_list.clone()
                            variant=ListVariant::Bordered
                        >
                            <ListLoading message="Loading items...".to_string()>
                                <div class="loading-spinner">"⏳"</div>
                            </ListLoading>
                        </List>
                    </div>
                </div>
            </section>

            // Accessibility Features
            <section class="example-section">
                <h2>"8. Accessibility Features"</h2>
                <p>"List with enhanced accessibility features including keyboard navigation and ARIA support."</p>
                
                <List 
                    items=basic_items_5.clone()
                    selected_items=basic_selected_items.get()
                    class="accessible-list".to_string()
                >
                    {basic_items_5.clone().into_iter().map(|item| {
                        let item_data = item.data.clone();
                        let item_clone = item.clone();
                        view! {
                            <ListItem item=item_clone>
                                <div class="list-item-content">
                                    <span class="item-text">{item_data}</span>
                                    <span class="item-description">
                                        "Press Enter or Space to select this item"
                                    </span>
                                </div>
                            </ListItem>
                        }
                    }).collect::<Vec<_>>()}
                </List>
                
                <div class="accessibility-info">
                    <h3>"Accessibility Features:"</h3>
                    <ul>
                        <li>"Proper ARIA roles (listbox, option)"</li>
                        <li>"Keyboard navigation support (Arrow keys, Enter, Space)"</li>
                        <li>"Focus management and tracking"</li>
                        <li>"Selection state indication"</li>
                        <li>"Screen reader announcements"</li>
                    </ul>
                </div>
            </section>

            // Component API Documentation
            <section class="example-section">
                <h2>"9. Component API Reference"</h2>
                <div class="api-documentation">
                    <h3>"Available Components:"</h3>
                    <ul>
                        <li><strong>"List"</strong> " - Main container with virtualization support"</li>
                        <li><strong>"ListItem"</strong> " - Individual list item with selection"</li>
                        <li><strong>"ListHeader"</strong> " - List header section"</li>
                        <li><strong>"ListFooter"</strong> " - List footer section"</li>
                        <li><strong>"ListEmpty"</strong> " - Empty state component"</li>
                        <li><strong>"ListLoading"</strong> " - Loading state component"</li>
                    </ul>
                    
                    <h3>"Configuration Options:"</h3>
                    <ul>
                        <li><strong>"Size variants:"</strong> " Small, Medium, Large"</li>
                        <li><strong>"Style variants:"</strong> " Default, Bordered, Striped, Compact"</li>
                        <li><strong>"Selection modes:"</strong> " Single, Multi-select"</li>
                        <li><strong>"Virtualization:"</strong> " Configurable item height, overscan, container height"</li>
                        <li><strong>"Event handlers:"</strong> " Selection change, item click, item focus"</li>
                    </ul>
                    
                    <h3>"Performance Features:"</h3>
                    <ul>
                        <li><strong>"Virtualization:"</strong> " Only renders visible items for large datasets"</li>
                        <li><strong>"Overscan:"</strong> " Pre-renders items outside viewport for smooth scrolling"</li>
                        <li><strong>"Efficient updates:"</strong> " Reactive updates with minimal re-renders"</li>
                        <li><strong>"Memory optimization:"</strong> " Minimal DOM nodes for large lists"</li>
                    </ul>
                </div>
            </section>
        </div>
    }
}
