use leptos::*;
use radix_leptos_primitives::*;

/// Test component that validates all working components
#[component]
pub fn ComponentTestSuite() -> impl IntoView {
    let (button_click_count, set_button_click_count) = create_signal(0);
    let (dialog_open, set_dialog_open) = create_signal(false);
    let (input_value, set_input_value) = create_signal("".to_string());
    
    // Test button functionality
    let handle_button_click = Callback::new(move |_| {
        set_button_click_count.update(|count| *count += 1);
        println!("Button clicked! Count: {}", button_click_count.get());
    });
    
    // Test dialog functionality
    let handle_dialog_open = Callback::new(move |_| {
        set_dialog_open.set(true);
        println!("Dialog opened!");
    });
    
    let handle_dialog_close = Callback::new(move |_| {
        set_dialog_open.set(false);
        println!("Dialog closed!");
    });
    
    // Test input functionality
    let handle_input_change = Callback::new(move |event: web_sys::Event| {
        if let Some(target) = event.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                set_input_value.set(input.value());
                println!("Input value changed: {}", input.value());
            }
        }
    });
    
    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif;">
            <h1>"🧪 Radix-Leptos Component Test Suite"</h1>
            
            <div class="section">
                <h2>"✅ Button Component Test"</h2>
                <div class="test-area">
                    <Button 
                        class="btn btn-primary".to_string()
                        on_click=handle_button_click
                    >
                        "Click me! Count: " {move || button_click_count.get()}
                    </Button>
                    <p>"Expected: Button should increment counter on click"</p>
                    <p>"Status: " {move || if button_click_count.get() > 0 { "✅ Working" } else { "⏳ Click to test" }}</p>
                </div>
            </div>
            
            <div class="section">
                <h2>"✅ Label Component Test"</h2>
                <div class="test-area">
                    <Label for_control="test-input".to_string() class="form-label".to_string()>
                        "Test Input Label (click to focus input)"
                    </Label>
                    <input 
                        id="test-input" 
                        type="text" 
                        placeholder="Type something..."
                        value=move || input_value.get()
                        on:input=handle_input_change
                        style="margin-left: 10px; padding: 5px; border: 1px solid #ccc; border-radius: 4px;"
                    />
                    <p>"Expected: Clicking label should focus input, typing should update value"</p>
                    <p>"Input value: " {move || if input_value.get().is_empty() { "⏳ Type to test".to_string() } else { format!("✅ {}", input_value.get()) }}</p>
                </div>
            </div>
            
            <div class="section">
                <h2>"✅ Separator Component Test"</h2>
                <div class="test-area">
                    <p>"Content above separator"</p>
                    <Separator orientation=SeparatorOrientation::Horizontal>
                        "Horizontal separator content"
                    </Separator>
                    <p>"Content below separator"</p>
                    
                    <div style="display: flex; align-items: center; margin-top: 20px;">
                        <span>"Left content"</span>
                        <VerticalSeparator>
                            "Vertical separator content"
                        </VerticalSeparator>
                        <span>"Right content"</span>
                    </div>
                    <p>"Expected: Separators should render with proper orientation and content"</p>
                    <p>"Status: ✅ Visual test - check if separators appear correctly"</p>
                </div>
            </div>
            
            <div class="section">
                <h2>"✅ Dialog Component Test"</h2>
                <div class="test-area">
                    <DialogRoot default_open=dialog_open>
                        <DialogTrigger class="btn btn-primary".to_string() on_click=handle_dialog_open>
                            "Open Test Dialog"
                        </DialogTrigger>
                        
                        <DialogContent class="dialog-content".to_string()>
                            <DialogTitle level=2 class="dialog-title".to_string()>
                                "Test Dialog"
                            </DialogTitle>
                            
                            <DialogDescription class="dialog-description".to_string()>
                                "This is a test dialog to validate the dialog component functionality."
                            </DialogDescription>
                            
                            <div style="margin: 20px 0;">
                                <p>"Dialog content area"</p>
                                <p>"Test features:"</p>
                                <ul>
                                    <li>"Modal behavior"</li>
                                    <li>"Focus management"</li>
                                    <li>"Keyboard navigation (Escape to close)"</li>
                                    <li>"Close button functionality"</li>
                                </ul>
                            </div>
                            
                            <div style="text-align: right;">
                                <DialogClose class="btn btn-secondary".to_string() on_click=handle_dialog_close>
                                    "Close Dialog"
                                </DialogClose>
                            </div>
                        </DialogContent>
                    </DialogRoot>
                    <p>"Expected: Click 'Open Test Dialog' to open modal, use Escape or Close button to close"</p>
                    <p>"Status: " {move || if dialog_open.get() { "✅ Dialog open" } else { "⏳ Click to open dialog" }}</p>
                </div>
            </div>
            
            <div class="section">
                <h2>"📊 Test Summary"</h2>
                <div class="test-area">
                    <p><strong>"Components Tested:"</strong></p>
                    <ul>
                        <li>"Button - Click handling and state management"</li>
                        <li>"Label - Form association and focus management"</li>
                        <li>"Separator - Visual rendering and orientation"</li>
                        <li>"Dialog - Modal behavior and accessibility"</li>
                    </ul>
                    <p><strong>"All components should be fully functional with proper accessibility features."</strong></p>
                </div>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <ComponentTestSuite/> })
}
