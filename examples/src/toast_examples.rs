use leptos::*;
use leptos::prelude::*;
// Removed unused import
use radix_leptos_primitives::components::toast::*;

#[component]
pub fn ToastExamples() -> impl IntoView {
    let show_toast = use_toast();
    
    let success_toast = create_toast(
        "Success!",
        Some("Your action was completed successfully."),
        ToastVariant::Success,
        Some(5000),
    );
    
    let error_toast = create_toast(
        "Error",
        Some("Something went wrong. Please try again."),
        ToastVariant::Error,
        Some(8000),
    );
    
    let warning_toast = create_toast(
        "Warning",
        Some("Please review your input before proceeding."),
        ToastVariant::Warning,
        Some(6000),
    );
    
    let info_toast = create_toast(
        "Information",
        Some("Here's some helpful information for you."),
        ToastVariant::Info,
        Some(4000),
    );
    
    let persistent_toast = create_toast(
        "Persistent",
        Some("This toast will stay until you dismiss it."),
        ToastVariant::Default,
        None,
    );
    
    view! {
        <div class="toast-examples">
            <h1>"Toast Component Examples"</h1>
            
            <div class="example-section">
                <h2>"Basic Toast Examples"</h2>
                <p>"Click the buttons below to see different types of toasts in action."</p>
                
                <div class="button-grid">
                    <ToastAction
                        toast=success_toast.clone()
                        class="btn btn-success".to_string()
                    >
                        "Show Success Toast"
                    </ToastAction>
                    
                    <ToastAction
                        toast=error_toast.clone()
                        class="btn btn-error".to_string()
                    >
                        "Show Error Toast"
                    </ToastAction>
                    
                    <ToastAction
                        toast=warning_toast.clone()
                        class="btn btn-warning".to_string()
                    >
                        "Show Warning Toast"
                    </ToastAction>
                    
                    <ToastAction
                        toast=info_toast.clone()
                        class="btn btn-info".to_string()
                    >
                        "Show Info Toast"
                    </ToastAction>
                    
                    <ToastAction
                        toast=persistent_toast.clone()
                        class="btn btn-default".to_string()
                    >
                        "Show Persistent Toast"
                    </ToastAction>
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Programmatic Usage"</h2>
                <p>"Demonstrates using the use_toast hook to programmatically show toasts."</p>
                
                <button
                    class="btn btn-programmatic"
                    on:click=move |_| {
                        let custom_toast = create_toast(
                            "Custom Toast",
                            Some("This toast was created programmatically!"),
                            ToastVariant::Info,
                            Some(7000),
                        );
                        show_toast(custom_toast);
                    }
                >
                    "Create Custom Toast"
                </button>
            </div>
        </div>
    }
}

pub fn start_toast_examples() {
    mount_to_body(|| {
        view! {
            <ToastRoot position=ToastPosition::TopRight max_toasts=5>
                <ToastExamples />
                <ToastViewport />
            </ToastRoot>
        }
    });
}
