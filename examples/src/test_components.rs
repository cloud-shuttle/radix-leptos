use leptos::*;
use leptos::prelude::*;
use leptos::callback::Callback;
use wasm_bindgen::JsCast;
use radix_leptos_primitives::*;

// Enhanced test suite with comprehensive coverage
#[component]
pub fn ComponentTestSuite() -> impl IntoView {
    // Static values for demonstration (since components expect actual values, not signals)
    let dialog_open = false;
    let checkbox_state = CheckboxState::Unchecked;
    let switch_state = SwitchState::Off;
    let radio_value = "option1".to_string();
    let input_value = String::new();
    let accordion_open = "item-1".to_string();
    let tabs_value = "tab1".to_string();
    let popover_open = false;
    let tooltip_open = false;
    let select_value = "option1".to_string();
    let select_value_display = select_value.clone();
    let select_value_summary = select_value.clone();
    
    // Phase 4: Complex Components - Reactive signals
    let (combobox_value, set_combobox_value) = signal("option1".to_string());
    let (date_picker_value, set_date_picker_value) = signal("".to_string());
    let (slider_value, set_slider_value) = signal(50.0);
    let (range_slider_value, set_range_slider_value) = signal((25.0, 75.0));
    let (progress_value, set_progress_value) = signal(65.0);
    
    // Phase 5: Layout & Navigation Components - Reactive signals
    let (navigation_active_item, set_navigation_active_item) = signal("home".to_string());
    let (navigation_collapsed, set_navigation_collapsed) = signal(false);
    let (breadcrumb_current_item, set_breadcrumb_current_item) = signal("page3".to_string());
    let (pagination_current_page, set_pagination_current_page) = signal(3);
    
    // Test counters for interaction tracking
    let (button_clicks, set_button_clicks) = signal(0);
    let (form_submissions, set_form_submissions) = signal(0);
    let (keyboard_events, set_keyboard_events) = signal(0);

    // Enhanced event handlers with comprehensive testing
    let handle_button_click = Callback::new(move |_| {
        set_button_clicks.update(|count| *count += 1);
        web_sys::console::log_1(&"Button clicked!".into());
    });

    let handle_dialog_open_change = Callback::new(move |_open: bool| {
        web_sys::console::log_1(&"Dialog state changed".into());
    });

    let handle_checkbox_change = Callback::new(move |_state: CheckboxState| {
        web_sys::console::log_1(&"Checkbox state changed".into());
    });

    let handle_switch_change = Callback::new(move |_state: SwitchState| {
        web_sys::console::log_1(&"Switch state changed".into());
    });

    let handle_radio_change = Callback::new(move |_value: String| {
        web_sys::console::log_1(&"Radio value changed".into());
    });

    let handle_text_input_change = Callback::new(move |_e: web_sys::Event| {
        web_sys::console::log_1(&"Text input changed".into());
    });

    let handle_accordion_toggle = Callback::new(move |_item: String| {
        web_sys::console::log_1(&"Accordion toggled".into());
    });

    let handle_tabs_change = Callback::new(move |_value: String| {
        web_sys::console::log_1(&"Tabs changed".into());
    });

    let handle_popover_toggle = Callback::new(move |_: web_sys::MouseEvent| {
        web_sys::console::log_1(&"Popover toggled".into());
    });

    let handle_tooltip_toggle = Callback::new(move |_: web_sys::MouseEvent| {
        web_sys::console::log_1(&"Tooltip toggled".into());
    });

    let handle_select_change = Callback::new(move |_value: String| {
        web_sys::console::log_1(&"Select changed".into());
    });

    // Phase 4: Complex Components - Event handlers
    let handle_combobox_change = Callback::new(move |value: String| {
        set_combobox_value.set(value);
        web_sys::console::log_1(&"Combobox changed".into());
    });

    let handle_date_picker_change = Callback::new(move |date: chrono::NaiveDate| {
        set_date_picker_value.set(date.format("%Y-%m-%d").to_string());
        web_sys::console::log_1(&"Date picker changed".into());
    });

    let handle_date_picker_open = Callback::new(move |_: ()| {
        web_sys::console::log_1(&"Date picker opened".into());
    });

    let handle_date_picker_close = Callback::new(move |_: ()| {
        web_sys::console::log_1(&"Date picker closed".into());
    });

    let handle_slider_change = Callback::new(move |value: f64| {
        set_slider_value.set(value);
        web_sys::console::log_1(&"Slider changed".into());
    });

    let handle_range_slider_change = Callback::new(move |values: (f64, f64)| {
        set_range_slider_value.set(values);
        web_sys::console::log_1(&"Range slider changed".into());
    });

        // Phase 5: Layout & Navigation Components - Event handlers
    let handle_navigation_item_click = Callback::new(move |item: NavigationItem| {
        set_navigation_active_item.set(item.id.clone());
        web_sys::console::log_1(&format!("Navigation item clicked: {}", item.id).into());
    });
    
    let handle_breadcrumb_item_click = Callback::new(move |item: BreadcrumbItem| {
        set_breadcrumb_current_item.set(item.id.clone());
        web_sys::console::log_1(&format!("Breadcrumb item clicked: {}", item.id).into());
    });
    
    let handle_pagination_page_change = Callback::new(move |page: usize| {
        set_pagination_current_page.set(page);
        web_sys::console::log_1(&format!("Pagination page changed to: {}", page).into());
    });

    // Phase 6: Data Display Components - Reactive signals
    let (table_sort_column, set_table_sort_column) = signal(Some("name".to_string()));
    let (table_sort_direction, set_table_sort_direction) = signal(SortDirection::Ascending);
    let (table_current_page, set_table_current_page) = signal(1);
    let (table_selected_rows, set_table_selected_rows) = signal(Vec::<String>::new());

    // Phase 6: Data Display Components - Event handlers
    let handle_table_sort = Callback::new(move |(column, direction): (String, SortDirection)| {
        set_table_sort_column.set(Some(column.clone()));
        set_table_sort_direction.set(direction.clone());
        web_sys::console::log_1(&format!("Table sorted by {}: {:?}", column, direction).into());
    });
    
    let handle_table_row_select = Callback::new(move |row: TableData| {
        let mut current_selected = table_selected_rows.get();
        if current_selected.contains(&row.id) {
            current_selected.retain(|id| id != &row.id);
        } else {
            current_selected.push(row.id.clone());
        }
        set_table_selected_rows.set(current_selected);
        web_sys::console::log_1(&format!("Table row selected: {}", row.id).into());
    });
    
    let handle_table_page_change = Callback::new(move |page: usize| {
        set_table_current_page.set(page);
        web_sys::console::log_1(&format!("Table page changed to: {}", page).into());
    });

    let handle_form_submit = Callback::new(move |_e: web_sys::SubmitEvent| {
        set_form_submissions.update(|count| *count += 1);
        web_sys::console::log_1(&"Form submitted!".into());
    });

    let handle_keyboard_event = Callback::new(move |_e: web_sys::KeyboardEvent| {
        set_keyboard_events.update(|count| *count += 1);
        web_sys::console::log_1(&"Keyboard event captured".into());
    });

    let handle_mouse_event = Callback::new(move |_: web_sys::MouseEvent| {
        set_keyboard_events.update(|count| *count += 1);
        web_sys::console::log_1(&"Mouse event captured".into());
    });

    view! {
        <div class="test-suite">
            <h1>"🧪 Enhanced Radix-Leptos Component Test Suite"</h1>
            <p>"Comprehensive testing of all components with edge cases, accessibility, and interactive scenarios"</p>
            
            // Test Statistics Dashboard
            <section class="test-dashboard">
                <h2>"📊 Test Statistics"</h2>
                <div class="stats-grid">
                    <div class="stat-item">
                        <h4>"Button Clicks"</h4>
                        <p>{button_clicks}</p>
                    </div>
                    <div class="stat-item">
                        <h4>"Form Submissions"</h4>
                        <p>{form_submissions}</p>
                    </div>
                    <div class="stat-item">
                        <h4>"Keyboard Events"</h4>
                        <p>{keyboard_events}</p>
                    </div>
                </div>
            </section>

            // Phase 1: Core Components - Enhanced Testing
            <section class="test-section">
                <h2>"🎯 Phase 1: Core Components - Enhanced Testing"</h2>
                
                <div class="component-group">
                    <h3>"Button Component - All Variants & States"</h3>
                    <div class="button-test-grid">
                        <Button on_click=handle_button_click.clone()>
                            "Primary Button (Clicks: " {button_clicks} ")"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_click=handle_button_click.clone()>
                            "Secondary Button"
                        </Button>
                        <Button variant=ButtonVariant::Outline on_click=handle_button_click.clone()>
                            "Outline Button"
                        </Button>
                        <Button variant=ButtonVariant::Ghost on_click=handle_button_click.clone()>
                            "Ghost Button"
                        </Button>
                        <Button variant=ButtonVariant::Destructive on_click=handle_button_click.clone()>
                            "Destructive Button"
                        </Button>
                        <Button disabled=true>
                            "Disabled Button"
                        </Button>
                        <Button on_click=handle_button_click.clone() style="background: linear-gradient(45deg, #ff6b6b, #4ecdc4); color: white; border: none; padding: 12px 24px; border-radius: 8px; font-weight: bold; box-shadow: 0 4px 15px rgba(0,0,0,0.2);".to_string()>
                            "Custom Styled Button"
                        </Button>
                    </div>
                </div>

                <div class="component-group">
                    <h3>"Label Component - Accessibility Testing"</h3>
                    <div class="label-test-grid">
                        <div class="form-group">
                            <Label for_control="test-input-1".to_string()>
                                "Standard Label"
                            </Label>
                            <input id="test-input-1" type="text" placeholder="Test input 1" />
                        </div>
                        <div class="form-group">
                            <Label for_control="test-input-2".to_string() class="required-label".to_string()>
                                "Required Field Label *"
                            </Label>
                            <input id="test-input-2" type="email" placeholder="Required email" required />
                        </div>
                        <div class="form-group">
                            <Label for_control="test-textarea".to_string()>
                                "Textarea Label"
                            </Label>
                            <textarea id="test-textarea" placeholder="Multi-line input" rows="3"></textarea>
                        </div>
                    </div>
                </div>

                <div class="component-group">
                    <h3>"Separator Component - All Orientations & Content"</h3>
                    <div class="separator-test-grid">
                        <div class="separator-test">
                            <p>"Content above horizontal separator"</p>
                            <Separator orientation=SeparatorOrientation::Horizontal>
                                "Horizontal separator with content"
                            </Separator>
                            <p>"Content below horizontal separator"</p>
                        </div>
                        <div class="separator-test">
                            <p>"Content above horizontal separator (no content)"</p>
                            <Separator orientation=SeparatorOrientation::Horizontal>
                                ""
                            </Separator>
                            <p>"Content below horizontal separator (no content)"</p>
                        </div>
                        <div class="separator-test vertical">
                            <span>"Left content"</span>
                            <Separator orientation=SeparatorOrientation::Vertical>
                                "Vertical separator with content"
                            </Separator>
                            <span>"Right content"</span>
                        </div>
                        <div class="separator-test vertical">
                            <span>"Left content"</span>
                            <Separator orientation=SeparatorOrientation::Vertical>
                                ""
                            </Separator>
                            <span>"Right content"</span>
                        </div>
                    </div>
                </div>

                <div class="component-group">
                    <h3>"Dialog Component - Enhanced Interaction Testing"</h3>
                    <div class="dialog-test-grid">
                        <DialogRoot default_open=dialog_open on_open_change=handle_dialog_open_change>
                            <DialogTrigger>
                                <Button>"Open Enhanced Dialog"</Button>
                            </DialogTrigger>
                            <DialogContent>
                                <DialogTitle>"Enhanced Dialog Title"</DialogTitle>
                                <DialogDescription>
                                    "This is an enhanced dialog with comprehensive testing. It includes form elements, 
                                    interactive content, and accessibility features."
                                </DialogDescription>
                                <div class="dialog-content-test">
                                    <div class="form-group">
                                        <Label for_control="dialog-input".to_string()>
                                            "Dialog Input Field"
                                        </Label>
                                        <input id="dialog-input" type="text" placeholder="Enter text in dialog" />
                                    </div>
                                    <div class="form-group">
                                        <Label for_control="dialog-select".to_string()>
                                            "Dialog Select Field"
                                        </Label>
                                        <select id="dialog-select">
                                            <option value="">Choose an option</option>
                                            <option value="option1">Option 1</option>
                                            <option value="option2">Option 2</option>
                                            <option value="option3">Option 3</option>
                                        </select>
                                    </div>
                                    <div class="form-group">
                                        <Label for_control="dialog-checkbox".to_string()>
                                            "Dialog Checkbox"
                                        </Label>
                                        <input id="dialog-checkbox" type="checkbox" />
                                    </div>
                                </div>
                                <div style="display: flex; gap: 10px; justify-content: flex-end; margin-top: 20px;">
                                    <DialogClose>
                                        <Button variant=ButtonVariant::Secondary>
                                            "Cancel"
                                        </Button>
                                    </DialogClose>
                                    <DialogClose>
                                        <Button on_click=handle_button_click.clone()>
                                            "Save & Close"
                                        </Button>
                                    </DialogClose>
                                </div>
                            </DialogContent>
                        </DialogRoot>
                    </div>
                </div>
            </section>

            // Phase 2: Form Components - Enhanced Testing
            <section class="test-section">
                <h2>"📝 Phase 2: Form Components - Enhanced Testing"</h2>
                
                <div class="component-group">
                    <h3>"Checkbox Component - All States & Interactions"</h3>
                    <div class="checkbox-test-grid">
                        <CheckboxWithLabel
                            state=checkbox_state
                        >
                            "Interactive Checkbox (State: " {format!("{:?}", checkbox_state)} ")"
                        </CheckboxWithLabel>
                        <Checkbox state=CheckboxState::Checked>
                            "Always Checked Checkbox"
                        </Checkbox>
                        <Checkbox state=CheckboxState::Indeterminate>
                            "Indeterminate Checkbox"
                        </Checkbox>
                        <Checkbox state=CheckboxState::Unchecked disabled=true>
                            "Disabled Unchecked Checkbox"
                        </Checkbox>
                        <Checkbox state=CheckboxState::Checked disabled=true>
                            "Disabled Checked Checkbox"
                        </Checkbox>
                        <Checkbox state=CheckboxState::Indeterminate disabled=true>
                            "Disabled Indeterminate Checkbox"
                        </Checkbox>
                    </div>
                </div>

                <div class="component-group">
                    <h3>"Switch Component - All States & Interactions"</h3>
                    <div class="switch-test-grid">
                        <SwitchWithLabel
                            state=switch_state
                        >
                            "Interactive Switch (State: " {format!("{:?}", switch_state)} ")"
                        </SwitchWithLabel>
                        <Switch state=SwitchState::On>
                            "Always On Switch"
                        </Switch>
                        <Switch state=SwitchState::Off>
                            "Always Off Switch"
                        </Switch>
                        <Switch state=SwitchState::On disabled=true>
                            "Disabled On Switch"
                        </Switch>
                        <Switch state=SwitchState::Off disabled=true>
                            "Disabled Off Switch"
                        </Switch>
                    </div>
                </div>

                <div class="component-group">
                    <h3>"RadioGroup Component - Enhanced Selection Testing"</h3>
                    <div class="radio-test-grid">
                        <RadioGroup value=radio_value.clone()>
                            <RadioGroupItemWithLabel value="option1".to_string()>
                                "Option 1 (Default)"
                            </RadioGroupItemWithLabel>
                            <RadioGroupItemWithLabel value="option2".to_string()>
                                "Option 2"
                            </RadioGroupItemWithLabel>
                            <RadioGroupItemWithLabel value="option3".to_string()>
                                "Option 3"
                            </RadioGroupItemWithLabel>
                            <RadioGroupItemWithLabel value="option4".to_string() disabled=true>
                                "Disabled Option 4"
                            </RadioGroupItemWithLabel>
                        </RadioGroup>
                        <p class="radio-status">"Selected: " {radio_value.clone()}</p>
                    </div>
                </div>

                <div class="component-group">
                    <h3>"TextInput Component - All Types & Validation"</h3>
                    <form class="form-test">
                        <div class="input-test-grid">
                            <TextInputWithLabel
                                input_type=TextInputType::Text
                                placeholder="Enter your full name".to_string()
                            >
                                "Full Name"
                            </TextInputWithLabel>
                            <TextInputWithLabel
                                input_type=TextInputType::Email
                                placeholder="Enter your email address".to_string()
                            >
                                "Email Address *"
                            </TextInputWithLabel>
                            <TextInputWithLabel
                                input_type=TextInputType::Password
                                placeholder="Enter your password".to_string()
                            >
                                "Password *"
                            </TextInputWithLabel>
                            <TextInputWithLabel
                                input_type=TextInputType::Number
                                placeholder="Enter your age".to_string()
                            >
                                "Age"
                            </TextInputWithLabel>
                            <TextInputWithLabel
                                input_type=TextInputType::Tel
                                placeholder="Enter your phone number".to_string()
                            >
                                "Phone Number"
                            </TextInputWithLabel>
                            <TextInputWithLabel
                                input_type=TextInputType::Url
                                placeholder="Enter your website URL".to_string()
                            >
                                "Website URL"
                            </TextInputWithLabel>
                            <TextInputWithLabel
                                input_type=TextInputType::Search
                                placeholder="Search for something...".to_string()
                            >
                                "Search"
                            </TextInputWithLabel>
                            <TextInputWithLabel
                                input_type=TextInputType::Date
                                placeholder="Select a date".to_string()
                            >
                                "Date"
                            </TextInputWithLabel>
                            <TextInputWithLabel
                                input_type=TextInputType::Time
                                placeholder="Select a time".to_string()
                            >
                                "Time"
                            </TextInputWithLabel>
                            <TextInputWithLabel
                                input_type=TextInputType::DateTimeLocal
                                placeholder="Select date and time".to_string()
                            >
                                "Date & Time"
                            </TextInputWithLabel>
                            <TextInputWithLabel
                                input_type=TextInputType::Month
                                placeholder="Select month".to_string()
                            >
                                "Month"
                            </TextInputWithLabel>
                            <TextInputWithLabel
                                input_type=TextInputType::Week
                                placeholder="Select week".to_string()
                            >
                                "Week"
                            </TextInputWithLabel>
                        </div>
                        <Button on_click=handle_button_click.clone()>
                            "Submit Form"
                        </Button>
                    </form>
                </div>
            </section>

            // Phase 3: Advanced Components - Enhanced Testing
            <section class="test-section">
                <h2>"🚀 Phase 3: Advanced Components - Enhanced Testing"</h2>
                
                <div class="component-group">
                    <h3>"Accordion Component - Multiple Types & Interactions"</h3>
                    <div class="accordion-test-grid">
                        <div class="accordion-section">
                            <h4>"Single Accordion (Default)"</h4>
                            <Accordion accordion_type=AccordionType::Single>
                                <AccordionItem value="item-1".to_string()>
                                    <AccordionHeader>
                                        <AccordionTrigger>
                                            "What is Radix-Leptos?"
                                        </AccordionTrigger>
                                    </AccordionHeader>
                                    <AccordionContent>
                                        "Radix-Leptos is a collection of accessible UI components built with Leptos and Rust. 
                                        It provides a comprehensive set of components that follow WCAG 2.1 AA guidelines."
                                    </AccordionContent>
                                </AccordionItem>
                                <AccordionItem value="item-2".to_string()>
                                    <AccordionHeader>
                                        <AccordionTrigger>
                                            "How to use these components?"
                                        </AccordionTrigger>
                                    </AccordionHeader>
                                    <AccordionContent>
                                        "Simply import the components you need and use them in your Leptos application. 
                                        All components are fully accessible and include proper ARIA attributes."
                                    </AccordionContent>
                                </AccordionItem>
                                <AccordionItem value="item-3".to_string() disabled=true>
                                    <AccordionHeader>
                                        <AccordionTrigger>
                                            "Disabled Accordion Item"
                                        </AccordionTrigger>
                                    </AccordionHeader>
                                    <AccordionContent>
                                        "This content is not accessible due to disabled state."
                                    </AccordionContent>
                                </AccordionItem>
                            </Accordion>
                        </div>
                    </div>
                </div>

                <div class="component-group">
                    <h3>"Tabs Component - All Orientations & States"</h3>
                    <div class="tabs-test-grid">
                        <div class="tabs-section">
                            <h4>"Horizontal Tabs (Default)"</h4>
                            <Tabs>
                                <TabsList>
                                    <TabsTrigger value="tab1".to_string()>
                                        "Account Settings"
                                    </TabsTrigger>
                                    <TabsTrigger value="tab2".to_string()>
                                        "Security"
                                    </TabsTrigger>
                                    <TabsTrigger value="tab3".to_string()>
                                        "Notifications"
                                    </TabsTrigger>
                                    <TabsTrigger value="tab4".to_string() disabled=true>
                                        "Disabled Tab"
                                    </TabsTrigger>
                                </TabsList>
                                <TabsContent value="tab1".to_string()>
                                    <div class="tab-content">
                                        <h5>"Account Settings"</h5>
                                        <p>"Manage your account preferences, profile information, and account settings here."</p>
                                        <div class="form-group">
                                            <Label for_control="username".to_string()>"Username"</Label>
                                            <input id="username" type="text" placeholder="Enter username" />
                                        </div>
                                        <div class="form-group">
                                            <Label for_control="email".to_string()>"Email"</Label>
                                            <input id="email" type="email" placeholder="Enter email" />
                                        </div>
                                    </div>
                                </TabsContent>
                                <TabsContent value="tab2".to_string()>
                                    <div class="tab-content">
                                        <h5>"Security Settings"</h5>
                                        <p>"Configure your security preferences, password settings, and authentication methods."</p>
                                        <div class="form-group">
                                            <Label for_control="current-password".to_string()>"Current Password"</Label>
                                            <input id="current-password" type="password" placeholder="Enter current password" />
                                        </div>
                                        <div class="form-group">
                                            <Label for_control="new-password".to_string()>"New Password"</Label>
                                            <input id="new-password" type="password" placeholder="Enter new password" />
                                        </div>
                                    </div>
                                </TabsContent>
                                <TabsContent value="tab3".to_string()>
                                    <div class="tab-content">
                                        <h5>"Notification Preferences"</h5>
                                        <p>"Manage your notification settings and preferences for different types of alerts."</p>
                                        <div class="form-group">
                                            <Checkbox state=CheckboxState::Checked>
                                                "Email notifications"
                                            </Checkbox>
                                        </div>
                                        <div class="form-group">
                                            <Checkbox state=CheckboxState::Unchecked>
                                                "Push notifications"
                                            </Checkbox>
                                        </div>
                                        <div class="form-group">
                                            <Checkbox state=CheckboxState::Checked>
                                                "SMS notifications"
                                            </Checkbox>
                                        </div>
                                    </div>
                                </TabsContent>
                                <TabsContent value="tab4".to_string()>
                                    <div class="tab-content">
                                        <h5>"Disabled Tab Content"</h5>
                                        <p>"This tab is disabled and not accessible."</p>
                                    </div>
                                </TabsContent>
                            </Tabs>
                        </div>
                    </div>
                </div>

                <div class="component-group">
                    <h3>"Popover Component - All Positions & Content Types"</h3>
                    <div class="popover-test-grid">
                        <div class="popover-section">
                            <h4>"Popover with Rich Content"</h4>
                            <Popover open=popover_open>
                                <PopoverTrigger>
                                    <Button>"Open Enhanced Popover"</Button>
                                </PopoverTrigger>
                                <PopoverContent side=PopoverSide::Bottom align=PopoverAlignment::Center>
                                    <div class="popover-content-rich">
                                        <h4>"Enhanced Popover Content"</h4>
                                        <p>"This popover contains rich content including form elements and interactive components."</p>
                                        <div class="popover-form">
                                            <div class="form-group">
                                                <Label for_control="popover-input".to_string()>"Quick Input"</Label>
                                                <input id="popover-input" type="text" placeholder="Enter text" />
                                            </div>
                                            <div class="form-group">
                                                <Checkbox state=CheckboxState::Unchecked>
                                                    "Accept terms"
                                                </Checkbox>
                                            </div>
                                            <div class="popover-actions">
                                                <PopoverClose>
                                                    <Button variant=ButtonVariant::Secondary>"Cancel"</Button>
                                                </PopoverClose>
                                                <PopoverClose>
                                                    <Button on_click=handle_button_click.clone()>"Save"</Button>
                                                </PopoverClose>
                                            </div>
                                        </div>
                                    </div>
                                    <PopoverArrow />
                                </PopoverContent>
                            </Popover>
                        </div>
                    </div>
                </div>

                <div class="component-group">
                    <h3>"Tooltip Component - All Positions & Triggers"</h3>
                    <div class="tooltip-test-grid">
                        <TooltipProvider>
                            <div class="tooltip-section">
                                <h4>"Tooltip with Different Triggers"</h4>
                                <div class="tooltip-triggers">
                                    <Tooltip open=tooltip_open>
                                        <TooltipTrigger>
                                            <Button>"Click for Tooltip"</Button>
                                        </TooltipTrigger>
                                        <TooltipContent side=TooltipSide::Top align=TooltipAlignment::Center>
                                            "This tooltip is triggered by clicking the button!"
                                            <TooltipArrow />
                                        </TooltipContent>
                                    </Tooltip>
                                    <Tooltip>
                                        <TooltipTrigger>
                                            <Button variant=ButtonVariant::Secondary>"Hover for Tooltip"</Button>
                                        </TooltipTrigger>
                                        <TooltipContent side=TooltipSide::Right align=TooltipAlignment::Center>
                                            "This tooltip appears on hover"
                                            <TooltipArrow />
                                        </TooltipContent>
                                    </Tooltip>
                                    <Tooltip>
                                        <TooltipTrigger>
                                            <span class="tooltip-text-trigger">"Text with tooltip"</span>
                                        </TooltipTrigger>
                                        <TooltipContent side=TooltipSide::Bottom align=TooltipAlignment::Center>
                                            "Tooltip on text element"
                                            <TooltipArrow />
                                        </TooltipContent>
                                    </Tooltip>
                                </div>
                            </div>
                        </TooltipProvider>
                    </div>
                </div>

                <div class="component-group">
                    <h3>"Select Component - Enhanced Options & Grouping"</h3>
                    <div class="select-test-grid">
                        <Select value=select_value.clone()>
                            <SelectTrigger>
                                <SelectValue placeholder="Choose your favorite fruit".to_string()>
                                    {select_value}
                                </SelectValue>
                            </SelectTrigger>
                            <SelectContent>
                                <SelectGroup>
                                    <SelectLabel>"Fruits"</SelectLabel>
                                    <SelectItem value="apple".to_string()>
                                        "🍎 Apple"
                                    </SelectItem>
                                    <SelectItem value="banana".to_string()>
                                        "🍌 Banana"
                                    </SelectItem>
                                    <SelectItem value="orange".to_string()>
                                        "🍊 Orange"
                                    </SelectItem>
                                    <SelectItem value="grape".to_string()>
                                        "🍇 Grape"
                                    </SelectItem>
                                    <SelectItem value="strawberry".to_string()>
                                        "🍓 Strawberry"
                                    </SelectItem>
                                </SelectGroup>
                                <SelectSeparator />
                                <SelectGroup>
                                    <SelectLabel>"Vegetables"</SelectLabel>
                                    <SelectItem value="carrot".to_string()>
                                        "🥕 Carrot"
                                    </SelectItem>
                                    <SelectItem value="broccoli".to_string()>
                                        "🥦 Broccoli"
                                    </SelectItem>
                                    <SelectItem value="spinach".to_string() disabled=true>
                                        "🥬 Spinach (Disabled)"
                                    </SelectItem>
                                    <SelectItem value="tomato".to_string()>
                                        "🍅 Tomato"
                                    </SelectItem>
                                </SelectGroup>
                                <SelectSeparator />
                                <SelectGroup>
                                    <SelectLabel>"Nuts & Seeds"</SelectLabel>
                                    <SelectItem value="almond".to_string()>
                                        "🥜 Almond"
                                    </SelectItem>
                                    <SelectItem value="walnut".to_string()>
                                        "🌰 Walnut"
                                    </SelectItem>
                                    <SelectItem value="sunflower".to_string()>
                                        "🌻 Sunflower Seeds"
                                    </SelectItem>
                                </SelectGroup>
                            </SelectContent>
                        </Select>
                        <p class="select-status">"Selected: " {select_value_display}</p>
                    </div>
                </div>
            </section>

            // Phase 4: Complex Components - Enhanced Testing
            <section class="test-section">
                <h2>"🚀 Phase 4: Complex Components - Enhanced Testing"</h2>
                
                // Combobox Component Testing
                <div class="component-group">
                    <h3>"🔍 Combobox Component - Full Functionality"</h3>
                    <div class="combobox-test">
                        <p>"Interactive searchable dropdown with keyboard navigation, filtering, and selection."</p>
                        
                        // Sample options for testing
                        {move || {
                            let options = vec![
                                ComboboxOption::new("apple".to_string(), "🍎 Apple".to_string()),
                                ComboboxOption::new("banana".to_string(), "🍌 Banana".to_string()),
                                ComboboxOption::new("cherry".to_string(), "🍒 Cherry".to_string()),
                                ComboboxOption::new("date".to_string(), "📅 Date".to_string()),
                                ComboboxOption::new("elderberry".to_string(), "🫐 Elderberry".to_string()),
                                ComboboxOption::new("fig".to_string(), "🌿 Fig".to_string()),
                                ComboboxOption::new("grape".to_string(), "🍇 Grape".to_string()),
                                ComboboxOption::new("honeydew".to_string(), "🍈 Honeydew".to_string()),
                                ComboboxOption::new("kiwi".to_string(), "🥝 Kiwi".to_string()),
                                ComboboxOption::new("lemon".to_string(), "🍋 Lemon".to_string()),
                            ];
                            
                            let current_value = combobox_value.clone();
                            
                            view! {
                                <div class="combobox-demo">
                                    <h4>"Searchable Fruit Combobox"</h4>
                                    <Combobox
                                        options=options
                                        value=current_value.get()
                                        placeholder="Search fruits...".to_string()
                                        on_change=handle_combobox_change.clone()
                                        on_search=Callback::new(|query: String| {
                                            web_sys::console::log_1(&format!("Search query: {}", query).into());
                                        })
                                    >
                                        <ComboboxTrigger>
                                            <ComboboxInput placeholder="Type to search...".to_string() />
                                        </ComboboxTrigger>
                                        <ComboboxContent>
                                            <ComboboxOptions />
                                        </ComboboxContent>
                                    </Combobox>
                                    <p class="combobox-status">"Selected: " {current_value}</p>
                                </div>
                            }
                        }}
                        
                        <div class="combobox-instructions">
                            <h4>"Test Instructions:"</h4>
                            <ul>
                                <li>"Click the input to open the dropdown"</li>
                                <li>"Type to filter options (e.g., 'ap' for Apple)"</li>
                                <li>"Use Arrow Up/Down to navigate"</li>
                                <li>"Press Enter to select, Escape to close"</li>
                                <li>"Click on any option to select it"</li>
                            </ul>
                        </div>
                    </div>
                </div>
                
                // DatePicker Test
                <div class="component-group">
                    <h3>"📅 DatePicker Component - Basic Functionality"</h3>
                    <div class="date-picker-test">
                        <p>"Interactive date picker with calendar navigation and selection."</p>
                        <div class="date-picker-demo">
                                                    <DatePicker
                            on_change=handle_date_picker_change.clone()
                        >
                            <DatePickerTrigger>
                                <DatePickerInput placeholder="Select a date...".to_string() />
                            </DatePickerTrigger>
                            <DatePickerCalendar>
                                <DatePickerGrid>
                                    <div>"Calendar grid content"</div>
                                </DatePickerGrid>
                            </DatePickerCalendar>
                        </DatePicker>
                        </div>
                        <div class="date-picker-instructions">
                            <h4>"Test Instructions:"</h4>
                            <ul>
                                <li>"Click the input to open the calendar"</li>
                                <li>"Use navigation arrows to change months"</li>
                                <li>"Type a date in YYYY-MM-DD format in the input field"</li>
                                <li>"Press Enter or click a date to select it"</li>
                            </ul>
                        </div>
                        <div class="date-picker-status">
                            "Selected: " {date_picker_value.get()}
                        </div>
                    </div>
                </div>

                // Progress Component Testing
                <div class="component-group">
                    <h3>"📊 Progress Component - Basic Functionality"</h3>
                    <div class="progress-test">
                        <p>"Interactive progress bar with different variants and sizes."</p>
                        
                        // Basic Progress Bar
                        <div class="progress-demo">
                            <h4>"Basic Progress Bar"</h4>
                            <Progress
                                value=progress_value.get()
                                max=100.0
                                class="progress".to_string()
                            >
                                <ProgressTrack class="progress-track".to_string()>
                                    <ProgressIndicator class="progress-indicator".to_string()>
                                        <div>"Progress"</div>
                                    </ProgressIndicator>
                                </ProgressTrack>
                                <ProgressValue format="percentage".to_string() />
                            </Progress>
                            <div class="progress-value">
                                "Value: " {progress_value.get()} "/ 100"
                            </div>
                        </div>

                        // Different Variants
                        <div class="progress-demo">
                            <h4>"Progress Variants"</h4>
                            <div class="progress-variants">
                                <Progress
                                    value=25.0
                                    variant=ProgressVariant::Default
                                    class="progress variant-default".to_string()
                                >
                                    <ProgressTrack class="progress-track".to_string()>
                                        <ProgressIndicator class="progress-indicator".to_string()>
                                            <div>"Default"</div>
                                        </ProgressIndicator>
                                    </ProgressTrack>
                                </Progress>
                                
                                <Progress
                                    value=50.0
                                    variant=ProgressVariant::Success
                                    class="progress variant-success".to_string()
                                >
                                    <ProgressTrack class="progress-track".to_string()>
                                        <ProgressIndicator class="progress-indicator".to_string()>
                                            <div>"Success"</div>
                                        </ProgressIndicator>
                                    </ProgressTrack>
                                </Progress>
                                
                                <Progress
                                    value=75.0
                                    variant=ProgressVariant::Warning
                                    class="progress variant-warning".to_string()
                                >
                                    <ProgressTrack class="progress-track".to_string()>
                                        <ProgressIndicator class="progress-indicator".to_string()>
                                            <div>"Warning"</div>
                                        </ProgressIndicator>
                                    </ProgressTrack>
                                </Progress>
                                
                                <Progress
                                    value=90.0
                                    variant=ProgressVariant::Error
                                    class="progress variant-error".to_string()
                                >
                                    <ProgressTrack class="progress-track".to_string()>
                                        <ProgressIndicator class="progress-indicator".to_string()>
                                            <div>"Error"</div>
                                        </ProgressIndicator>
                                    </ProgressTrack>
                                </Progress>
                            </div>
                        </div>

                        // Different Sizes
                        <div class="progress-demo">
                            <h4>"Progress Sizes"</h4>
                            <div class="progress-sizes">
                                <Progress
                                    value=60.0
                                    size=ProgressSize::Small
                                    class="progress size-small".to_string()
                                >
                                    <ProgressTrack class="progress-track".to_string()>
                                        <ProgressIndicator class="progress-indicator".to_string()>
                                            <div>"Small"</div>
                                        </ProgressIndicator>
                                    </ProgressTrack>
                                </Progress>
                                
                                <Progress
                                    value=60.0
                                    size=ProgressSize::Medium
                                    class="progress size-medium".to_string()
                                >
                                    <ProgressTrack class="progress-track".to_string()>
                                        <ProgressIndicator class="progress-indicator".to_string()>
                                            <div>"Medium"</div>
                                        </ProgressIndicator>
                                    </ProgressTrack>
                                </Progress>
                                
                                <Progress
                                    value=60.0
                                    size=ProgressSize::Large
                                    class="progress size-large".to_string()
                                >
                                    <ProgressTrack class="progress-track".to_string()>
                                        <ProgressIndicator class="progress-indicator".to_string()>
                                            <div>"Large"</div>
                                        </ProgressIndicator>
                                    </ProgressTrack>
                                </Progress>
                            </div>
                        </div>

                        // Animated Progress
                        <div class="progress-demo">
                            <h4>"Animated Progress"</h4>
                            <Progress
                                value=progress_value.get()
                                animated=true
                                striped=true
                                class="progress animated".to_string()
                            >
                                <ProgressTrack class="progress-track".to_string()>
                                    <ProgressIndicator class="progress-indicator".to_string()>
                                        <div>"Animated"</div>
                                    </ProgressIndicator>
                                </ProgressTrack>
                                <ProgressValue format="fraction".to_string() />
                            </Progress>
                        </div>

                        <div class="progress-instructions">
                            <h4>"Test Instructions:"</h4>
                            <ul>
                                <li>"Progress bars show different completion states"</li>
                                <li>"Variants have different colors (default, success, warning, error)"</li>
                                <li>"Sizes affect the height of the progress bar"</li>
                                <li>"Animated progress shows striped animation"</li>
                                <li>"Values are displayed in different formats"</li>
                            </ul>
                        </div>
                    </div>
                </div>

                // Slider Component Testing
                <div class="component-group">
                    <h3>"🎚️ Slider Component - Basic Functionality"</h3>
                    <div class="slider-test">
                        <p>"Interactive slider with click and keyboard navigation."</p>
                        <div class="slider-demo">
                            <Slider
                                value=slider_value.get()
                                on_change=handle_slider_change.clone()
                                min=0.0
                                max=100.0
                                step=5.0
                                class="slider".to_string()
                            >
                                <SliderTrack class="slider-track".to_string()>
                                    <SliderRange class="slider-range".to_string()>
                                        <div>"Range"</div>
                                    </SliderRange>
                                    <SliderThumb class="slider-thumb".to_string()>
                                        <div class="thumb-content">
                                            {slider_value.get()}
                                        </div>
                                    </SliderThumb>
                                </SliderTrack>
                            </Slider>
                            <div class="slider-value">
                                "Value: " {slider_value.get()}
                            </div>
                        </div>
                        <div class="slider-instructions">
                            <h4>"Test Instructions:"</h4>
                            <ul>
                                <li>"Click the thumb to increment by step"</li>
                                <li>"Use Arrow Up/Down to change value by step"</li>
                                <li>"Use Home/End to go to min/max"</li>
                                <li>"Use Page Up/Down to change by 10x step"</li>
                            </ul>
                        </div>
                    </div>
                </div>

                // RangeSlider Component Testing
                <div class="component-group">
                    <h3>"🎚️ RangeSlider Component - Basic Functionality"</h3>
                    <div class="range-slider-test">
                        <p>"Interactive range slider with two thumbs and value display."</p>
                        <div class="range-slider-demo">
                            <RangeSlider
                                value=range_slider_value.get()
                                on_change=handle_range_slider_change.clone()
                                min=0.0
                                max=100.0
                                step=5.0
                                class="range-slider".to_string()
                            >
                                <SliderTrack class="slider-track".to_string()>
                                    <SliderRange class="slider-range".to_string()>
                                        <div>"Range"</div>
                                    </SliderRange>
                                    <RangeSliderThumb thumb_index=0 class="slider-thumb min-thumb".to_string()>
                                        <div class="thumb-content">
                                            {range_slider_value.get().0}
                                        </div>
                                    </RangeSliderThumb>
                                    <RangeSliderThumb thumb_index=1 class="slider-thumb max-thumb".to_string()>
                                        <div class="thumb-content">
                                            {range_slider_value.get().1}
                                        </div>
                                    </RangeSliderThumb>
                                </SliderTrack>
                            </RangeSlider>
                            <div class="range-slider-value">
                                "Range: " {range_slider_value.get().0} " - " {range_slider_value.get().1}
                            </div>
                        </div>
                        <div class="range-slider-instructions">
                            <h4>"Test Instructions:"</h4>
                            <ul>
                                <li>"Click the thumbs to increment by step"</li>
                                <li>"Use Arrow Up/Down to change value by step"</li>
                                <li>"Thumbs should move independently"</li>
                                <li>"Min thumb cannot exceed max thumb"</li>
                            </ul>
                        </div>
                    </div>
                </div>

                // Phase 4 Status
                <div class="component-group">
                    <h3>"📋 Phase 4 Implementation Status"</h3>
                    <div class="phase4-status">
                        <div class="status-item">
                            <h4>"✅ Combobox"</h4>
                            <p>"Complete with full functionality, search, keyboard navigation, and accessibility"</p>
                        </div>
                        <div class="status-item">
                            <h4>"✅ DatePicker"</h4>
                            <p>"Basic implementation with calendar navigation and selection"</p>
                        </div>
                        <div class="status-item">
                            <h4>"✅ Slider"</h4>
                            <p>"Complete with click and keyboard navigation, basic functionality implemented"</p>
                        </div>
                        <div class="status-item">
                            <h4>"✅ Progress"</h4>
                            <p>"Complete with variants, sizes, animations, and value formatting"</p>
                        </div>
                    </div>
                </div>
            </section>

            // Phase 5: Layout & Navigation Components - Enhanced Testing
            <section class="test-section">
                <h2>"🧭 Phase 5: Layout & Navigation Components - Enhanced Testing"</h2>
                
                <div class="component-group">
                    <h3>"Navigation Component - Main Navigation"</h3>
                    <div class="navigation-test">
                        <Navigation 
                            active_item=navigation_active_item.get()
                            collapsed=navigation_collapsed.get()
                            collapsible=true
                            on_item_click=handle_navigation_item_click.clone()
                        >
                            <NavigationList>
                                <NavigationItem item=NavigationItem::new("home".to_string(), "Home".to_string()).with_href("#home".to_string()).with_icon("🏠".to_string())>
                                    <NavigationLink text="Home".to_string() icon="🏠".to_string() href="#home".to_string()>
                                        "Home"
                                    </NavigationLink>
                                </NavigationItem>
                                <NavigationItem item=NavigationItem::new("about".to_string(), "About".to_string()).with_href("#about".to_string()).with_icon("ℹ️".to_string())>
                                    <NavigationLink text="About".to_string() icon="ℹ️".to_string() href="#about".to_string()>
                                        "About"
                                    </NavigationLink>
                                </NavigationItem>
                                <NavigationItem item=NavigationItem::new("contact".to_string(), "Contact".to_string()).with_href("#contact".to_string()).with_icon("📧".to_string())>
                                    <NavigationLink text="Contact".to_string() icon="📧".to_string() href="#contact".to_string()>
                                        "Contact"
                                    </NavigationLink>
                                </NavigationItem>
                            </NavigationList>
                            <NavigationToggle text="Menu".to_string() icon="☰".to_string()>
                                "Toggle Navigation"
                            </NavigationToggle>
                        </Navigation>
                        <div class="navigation-instructions">
                            <h4>"Test Instructions:"</h4>
                            <ul>
                                <li>"Click navigation items to change active state"</li>
                                <li>"Use Tab to navigate between items"</li>
                                <li>"Click toggle button to collapse/expand"</li>
                                <li>"Active item: " {navigation_active_item}</li>
                                <li>"Collapsed: " {navigation_collapsed}</li>
                            </ul>
                        </div>
                    </div>
                </div>

                <div class="component-group">
                    <h3>"Breadcrumbs Component - Hierarchical Navigation"</h3>
                    <div class="breadcrumbs-test">
                        <Breadcrumbs 
                            on_item_click=handle_breadcrumb_item_click.clone()
                        >
                            <BreadcrumbList>
                                <BreadcrumbItem item=BreadcrumbItem::new("home".to_string(), "Home".to_string()).with_href("#home".to_string())>
                                    <BreadcrumbLink text="Home".to_string() href="#home".to_string()>
                                        "Home"
                                    </BreadcrumbLink>
                                </BreadcrumbItem>
                                <BreadcrumbSeparator>
                                    " / "
                                </BreadcrumbSeparator>
                                <BreadcrumbItem item=BreadcrumbItem::new("section".to_string(), "Section".to_string()).with_href("#section".to_string())>
                                    <BreadcrumbLink text="Section".to_string() href="#section".to_string()>
                                        "Section"
                                    </BreadcrumbLink>
                                </BreadcrumbItem>
                                <BreadcrumbSeparator>
                                    " / "
                                </BreadcrumbSeparator>
                                <BreadcrumbItem item=BreadcrumbItem::new("page3".to_string(), "Current Page".to_string()).with_current(true)>
                                    "Current Page"
                                </BreadcrumbItem>
                            </BreadcrumbList>
                        </Breadcrumbs>
                        <div class="breadcrumbs-instructions">
                            <h4>"Test Instructions:"</h4>
                            <ul>
                                <li>"Click breadcrumb items to navigate"</li>
                                <li>"Current item is highlighted and non-clickable"</li>
                                <li>"Current item: " {breadcrumb_current_item}</li>
                            </ul>
                        </div>
                    </div>
                </div>

                <div class="component-group">
                    <h3>"Pagination Component - Page Navigation"</h3>
                    <div class="pagination-test">
                        <Pagination 
                            current_page=pagination_current_page.get()
                            total_pages=10
                            on_page_change=handle_pagination_page_change.clone()
                        >
                            <PaginationList>
                                <PaginationFirst text="First".to_string() icon="⏮️".to_string()>
                                    "First"
                                </PaginationFirst>
                                <PaginationPrevious text="Previous".to_string() icon="◀️".to_string()>
                                    "Previous"
                                </PaginationPrevious>
                                <PaginationItem page=PaginationPage::new(1).with_current(pagination_current_page.get() == 1)>
                                    "1"
                                </PaginationItem>
                                <PaginationItem page=PaginationPage::new(2).with_current(pagination_current_page.get() == 2)>
                                    "2"
                                </PaginationItem>
                                <PaginationItem page=PaginationPage::new(3).with_current(pagination_current_page.get() == 3)>
                                    "3"
                                </PaginationItem>
                                <PaginationEllipsis>
                                    "…"
                                </PaginationEllipsis>
                                <PaginationItem page=PaginationPage::new(9).with_current(pagination_current_page.get() == 9)>
                                    "9"
                                </PaginationItem>
                                <PaginationItem page=PaginationPage::new(10).with_current(pagination_current_page.get() == 10)>
                                    "10"
                                </PaginationItem>
                                <PaginationNext text="Next".to_string() icon="▶️".to_string()>
                                    "Next"
                                </PaginationNext>
                                <PaginationLast text="Last".to_string() icon="⏭️".to_string()>
                                    "Last"
                                </PaginationLast>
                            </PaginationList>
                        </Pagination>
                        <div class="pagination-instructions">
                            <h4>"Test Instructions:"</h4>
                            <ul>
                                <li>"Click page numbers to navigate"</li>
                                <li>"Use First/Previous/Next/Last buttons"</li>
                                <li>"Current page is highlighted"</li>
                                <li>"Current page: " {pagination_current_page}</li>
                            </ul>
                        </div>
                    </div>
                </div>

                // Phase 5 Status
                <div class="component-group">
                    <h3>"📋 Phase 5 Implementation Status"</h3>
                    <div class="phase5-status">
                        <div class="status-item">
                            <h4>"✅ Navigation"</h4>
                            <p>"Complete with collapsible support, active item tracking, and keyboard navigation"</p>
                        </div>
                        <div class="status-item">
                            <h4>"✅ Breadcrumbs"</h4>
                            <p>"Complete with hierarchical navigation, current item indication, and click handling"</p>
                        </div>
                        <div class="status-item">
                            <h4>"✅ Pagination"</h4>
                            <p>"Complete with first/last/prev/next buttons, page numbers, and ellipsis support"</p>
                        </div>
                    </div>
                </div>
            </section>

            // Phase 6: Data Display Components Section
            <section class="test-section">
                <h2>"📊 Phase 6: Data Display Components"</h2>
                
                <div class="component-group">
                    <h3>"Table Component - Sortable Data Table"</h3>
                    <div class="table-test">
                        <Table 
                            columns=vec![
                                TableColumn::new("name".to_string(), "Name".to_string()).with_sortable(true),
                                TableColumn::new("email".to_string(), "Email".to_string()).with_sortable(true),
                                TableColumn::new("role".to_string(), "Role".to_string()).with_sortable(true),
                                TableColumn::new("status".to_string(), "Status".to_string()).with_sortable(true),
                            ]
                            data=vec![
                                TableData::new("1".to_string(), vec!["John Doe".to_string(), "john@example.com".to_string(), "Admin".to_string(), "Active".to_string()]),
                                TableData::new("2".to_string(), vec!["Jane Smith".to_string(), "jane@example.com".to_string(), "User".to_string(), "Active".to_string()]),
                                TableData::new("3".to_string(), vec!["Bob Johnson".to_string(), "bob@example.com".to_string(), "Editor".to_string(), "Inactive".to_string()]),
                                TableData::new("4".to_string(), vec!["Alice Brown".to_string(), "alice@example.com".to_string(), "User".to_string(), "Active".to_string()]),
                                TableData::new("5".to_string(), vec!["Charlie Wilson".to_string(), "charlie@example.com".to_string(), "Admin".to_string(), "Active".to_string()]),
                            ]
                            sort_column=table_sort_column.get().unwrap_or_default()
                            sort_direction=table_sort_direction.get()
                            current_page=table_current_page.get()
                            page_size=3
                            selectable=true
                            multi_select=true
                            on_sort=handle_table_sort.clone()
                            on_row_select=handle_table_row_select.clone()
                            on_page_change=handle_table_page_change.clone()
                        >
                            <TableHeader>
                                <TableRow>
                                    <TableHeaderCell content="Name".to_string()>
                                        <TableSortButton column_id="name".to_string() header_text="Name".to_string()>
                                            "Name"
                                        </TableSortButton>
                                    </TableHeaderCell>
                                    <TableHeaderCell content="Email".to_string()>
                                        <TableSortButton column_id="email".to_string() header_text="Email".to_string()>
                                            "Email"
                                        </TableSortButton>
                                    </TableHeaderCell>
                                    <TableHeaderCell content="Role".to_string()>
                                        <TableSortButton column_id="role".to_string() header_text="Role".to_string()>
                                            "Role"
                                        </TableSortButton>
                                    </TableHeaderCell>
                                    <TableHeaderCell content="Status".to_string()>
                                        <TableSortButton column_id="status".to_string() header_text="Status".to_string()>
                                            "Status"
                                        </TableSortButton>
                                    </TableHeaderCell>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                <TableRow row=TableData::new("1".to_string(), vec!["John Doe".to_string(), "john@example.com".to_string(), "Admin".to_string(), "Active".to_string()])>
                                    <TableCell content="John Doe".to_string()>"John Doe"</TableCell>
                                    <TableCell content="john@example.com".to_string()>"john@example.com"</TableCell>
                                    <TableCell content="Admin".to_string()>"Admin"</TableCell>
                                    <TableCell content="Active".to_string()>"Active"</TableCell>
                                </TableRow>
                                <TableRow row=TableData::new("2".to_string(), vec!["Jane Smith".to_string(), "jane@example.com".to_string(), "User".to_string(), "Active".to_string()])>
                                    <TableCell content="Jane Smith".to_string()>"Jane Smith"</TableCell>
                                    <TableCell content="jane@example.com".to_string()>"jane@example.com"</TableCell>
                                    <TableCell content="User".to_string()>"User"</TableCell>
                                    <TableCell content="Active".to_string()>"Active"</TableCell>
                                </TableRow>
                                <TableRow row=TableData::new("3".to_string(), vec!["Bob Johnson".to_string(), "bob@example.com".to_string(), "Editor".to_string(), "Inactive".to_string()])>
                                    <TableCell content="Bob Johnson".to_string()>"Bob Johnson"</TableCell>
                                    <TableCell content="bob@example.com".to_string()>"bob@example.com"</TableCell>
                                    <TableCell content="Editor".to_string()>"Editor"</TableCell>
                                    <TableCell content="Inactive".to_string()>"Inactive"</TableCell>
                                </TableRow>
                            </TableBody>
                            <TablePagination>
                                <TableInfo>
                                    "Table pagination info"
                                </TableInfo>
                            </TablePagination>
                        </Table>
                        <div class="table-instructions">
                            <h4>"Test Instructions:"</h4>
                            <ul>
                                <li>"Click column headers to sort data"</li>
                                <li>"Click rows to select/deselect them"</li>
                                <li>"Use pagination controls to navigate pages"</li>
                                <li>"Sort column: " {table_sort_column.get().unwrap_or_default()}</li>
                                <li>"Sort direction: " {format!("{:?}", table_sort_direction.get())}</li>
                                <li>"Current page: " {table_current_page}</li>
                                <li>"Selected rows: " {format!("{:?}", table_selected_rows.get())}</li>
                            </ul>
                        </div>
                    </div>
                </div>

                // Phase 6 Status
                <div class="component-group">
                    <h3>"📋 Phase 6 Implementation Status"</h3>
                    <div class="phase6-status">
                        <div class="status-item">
                            <h4>"✅ Table"</h4>
                            <p>"Complete with sorting, pagination, row selection, and accessibility features"</p>
                        </div>
                        <div class="status-item">
                            <h4>"🔄 List"</h4>
                            <p>"In progress - Virtualized lists for large datasets"</p>
                        </div>
                        <div class="status-item">
                            <h4>"⏳ Tree"</h4>
                            <p>"Planned - Hierarchical data display with expand/collapse"</p>
                        </div>
                        <div class="status-item">
                            <h4>"⏳ Timeline"</h4>
                            <p>"Planned - Chronological data presentation"</p>
                        </div>
                    </div>
                </div>
            </section>

            // Accessibility Testing Section
            <section class="test-section">
                <h2>"♿ Accessibility Testing"</h2>
                <div class="accessibility-test">
                    <h3>"Keyboard Navigation Test"</h3>
                    <p>"Use Tab, Enter, Space, and Arrow keys to navigate through all components."</p>
                    <div class="keyboard-test-grid">
                        <Button on_click=handle_mouse_event.clone()>
                            "Tab to me and press Enter/Space"
                        </Button>
                        <Checkbox state=CheckboxState::Unchecked>
                            "Tab to me and press Space"
                        </Checkbox>
                        <Switch state=SwitchState::Off>
                            "Tab to me and press Space"
                        </Switch>
                        <input type="text" placeholder="Tab to me and type" />
                    </div>
                </div>
            </section>

            // Enhanced Test Summary
            <section class="test-section">
                <h2>"📈 Enhanced Test Summary"</h2>
                <div class="summary-grid">
                    <div class="summary-item">
                        <h4>"Dialog State"</h4>
                        <p>{if dialog_open { "Open" } else { "Closed" }}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Checkbox State"</h4>
                        <p>{format!("{:?}", checkbox_state)}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Switch State"</h4>
                        <p>{format!("{:?}", switch_state)}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Radio Value"</h4>
                        <p>{radio_value.clone()}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Input Value"</h4>
                        <p>{input_value.clone()}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Accordion Open"</h4>
                        <p>{accordion_open.clone()}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Tabs Value"</h4>
                        <p>{tabs_value.clone()}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Popover State"</h4>
                        <p>{if popover_open { "Open" } else { "Closed" }}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Tooltip State"</h4>
                        <p>{if tooltip_open { "Open" } else { "Closed" }}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Select Value"</h4>
                        <p>{select_value_summary}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Navigation Active"</h4>
                        <p>{navigation_active_item}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Navigation Collapsed"</h4>
                        <p>{navigation_collapsed}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Breadcrumb Current"</h4>
                        <p>{breadcrumb_current_item}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Pagination Page"</h4>
                        <p>{pagination_current_page}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Table Sort Column"</h4>
                        <p>{table_sort_column.get().unwrap_or_default()}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Table Sort Direction"</h4>
                        <p>{format!("{:?}", table_sort_direction.get())}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Table Current Page"</h4>
                        <p>{table_current_page}</p>
                    </div>
                    <div class="summary-item">
                        <h4>"Table Selected Rows"</h4>
                        <p>{format!("{:?}", table_selected_rows.get())}</p>
                    </div>
                </div>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <ComponentTestSuite /> })
}
