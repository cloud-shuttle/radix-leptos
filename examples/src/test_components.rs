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
                            <Tabs value=tabs_value.clone()>
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
                </div>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <ComponentTestSuite /> })
}
