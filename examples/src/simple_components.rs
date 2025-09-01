use leptos::*;
use radix_leptos_primitives::*;

#[component]
pub fn SimpleComponentsExample() -> impl IntoView {
    let (count, set_count) = signal(0);
    
    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif;">
            <h1>"Simple Components Example"</h1>
            
            <div style="margin: 20px 0;">
                <h2>"Button Component"</h2>
                <Button 
                    class="btn btn-primary".to_string()
                    on_click=Callback::new(move |_| set_count.update(|c| *c += 1))
                >
                    "Click me! Count: " {move || count.get()}
                </Button>
            </div>
            
            <Separator orientation=SeparatorOrientation::Horizontal>
                "Separator content"
            </Separator>
            
            <div style="margin: 20px 0;">
                <h2>"Label Component"</h2>
                <Label for_control="test-input".to_string() class="form-label".to_string()>
                    "Test Input Label"
                </Label>
                <input 
                    id="test-input" 
                    type="text" 
                    placeholder="Type something..."
                    style="margin-left: 10px; padding: 5px;"
                />
            </div>
            
            <Separator orientation=SeparatorOrientation::Horizontal>
                "Another separator"
            </Separator>
            
            <div style="margin: 20px 0;">
                <h2>"Separator Components"</h2>
                <div style="display: flex; align-items: center; gap: 20px;">
                    <span>"Left content"</span>
                    <VerticalSeparator>
                        "Vertical separator"
                    </VerticalSeparator>
                    <span>"Right content"</span>
                </div>
                
                <div style="margin-top: 20px;">
                    <p>"Content above"</p>
                    <HorizontalSeparator>
                        "Horizontal separator"
                    </HorizontalSeparator>
                    <p>"Content below"</p>
                </div>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <SimpleComponentsExample/> })
}
