use leptos::prelude::*;
use leptos::*;
use radix_leptos_primitives::*;

#[component]
pub fn ToastExamples() -> impl IntoView {
    view! {
        <div class="toast-examples">
            <h1>"Toast Component Examples"</h1>

            <div class="example-section">
                <h2>"Basic Toast Examples"</h2>
                <p>"Different toast variants for different types of messages:"</p>

                <ToastProvider position=ToastPosition::TopRight>
                    <Toast
                        variant=ToastVariant::Success
                        position=ToastPosition::TopRight
                        duration=5000
                        dismissible=true
                    >
                        <ToastTitle>"Success!"</ToastTitle>
                        <ToastDescription>"Your action was completed successfully."</ToastDescription>
                        <ToastAction>
                            "Dismiss"
                        </ToastAction>
                    </Toast>

                    <Toast
                        variant=ToastVariant::Error
                        position=ToastPosition::TopRight
                        duration=8000
                        dismissible=true
                    >
                        <ToastTitle>"Error"</ToastTitle>
                        <ToastDescription>"Something went wrong. Please try again."</ToastDescription>
                        <ToastAction>
                            "Dismiss"
                        </ToastAction>
                    </Toast>

                    <Toast
                        variant=ToastVariant::Warning
                        position=ToastPosition::TopRight
                        duration=6000
                        dismissible=true
                    >
                        <ToastTitle>"Warning"</ToastTitle>
                        <ToastDescription>"Please review your input before proceeding."</ToastDescription>
                        <ToastAction>
                            "Dismiss"
                        </ToastAction>
                    </Toast>

                    <Toast
                        variant=ToastVariant::Info
                        position=ToastPosition::TopRight
                        duration=4000
                        dismissible=true
                    >
                        <ToastTitle>"Information"</ToastTitle>
                        <ToastDescription>"Here's some helpful information for you."</ToastDescription>
                        <ToastAction>
                            "Dismiss"
                        </ToastAction>
                    </Toast>
                </ToastProvider>
            </div>

            <div class="example-section">
                <h2>"Custom Toast"</h2>
                <p>"A custom toast with additional styling:"</p>

                <Toast
                    variant=ToastVariant::Default
                    position=ToastPosition::BottomRight
                    duration=10000
                    dismissible=true
                    class="custom-toast".to_string()
                >
                    <ToastTitle>"Custom Toast"</ToastTitle>
                    <ToastDescription>"This is a custom toast with additional styling and longer duration."</ToastDescription>
                    <ToastAction>
                        "Custom Action"
                    </ToastAction>
                </Toast>
            </div>
        </div>
    }
}
