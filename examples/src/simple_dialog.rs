use leptos::*;
use radix_leptos_primitives::*;

#[component]
pub fn SimpleDialogExample() -> impl IntoView {
    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif;">
            <h1>"Simple Dialog Example"</h1>
            
            <DialogRoot default_open=false>
                <DialogTrigger class="btn btn-primary".to_string()>
                    "Open Dialog"
                </DialogTrigger>
                
                <DialogContent class="dialog-content".to_string()>
                    <DialogTitle level=2 class="dialog-title".to_string()>
                        "Dialog Title"
                    </DialogTitle>
                    
                    <DialogDescription class="dialog-description".to_string()>
                        "This is a simple dialog example using Radix-Leptos primitives."
                    </DialogDescription>
                    
                    <div style="margin-top: 20px;">
                        <p>"This dialog demonstrates the basic functionality of the dialog components."</p>
                        <p>"You can close it using the button below."</p>
                    </div>
                    
                    <div style="margin-top: 20px; text-align: right;">
                        <DialogClose class="btn btn-secondary".to_string()>
                            "Close"
                        </DialogClose>
                    </div>
                </DialogContent>
            </DialogRoot>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <SimpleDialogExample/> })
}
