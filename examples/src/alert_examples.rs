use leptos::*;
use leptos::prelude::*;
use leptos::logging::log;
use radix_leptos_primitives::components::alert::*;

#[component]
pub fn AlertExamples() -> impl IntoView {
    let (dismissed_alerts, set_dismissed_alerts) = signal(std::collections::HashSet::<String>::new());
    
    let handle_dismiss = move |alert_id: String| {
        let alert_id_clone = alert_id.clone();
        let mut current = dismissed_alerts.get();
        current.insert(alert_id);
        set_dismissed_alerts.set(current);
        log!("Alert dismissed: {}", alert_id_clone);
    };
    
    let is_dismissed = move |alert_id: &str| {
        dismissed_alerts.get().contains(alert_id)
    };
    
    view! {
        <div class="alert-examples">
            <h1>"Alert Component Examples"</h1>
            
            <div class="api-docs">
                <h2>"API Documentation"</h2>
                <ul>
                    <li><strong>"Alert"</strong> - Root alert component with variants and sizing</li>
                    <li><strong>"AlertTitle"</strong> - Title component for alerts</li>
                    <li><strong>"AlertDescription"</strong> - Description component for alerts</li>
                    <li><strong>"AlertAction"</strong> - Action button component for alerts</li>
                </ul>
                
                <h3>"Alert Variants"</h3>
                <ul>
                    <li><strong>"Default"</strong> - Standard alert styling</li>
                    <li><strong>"Success"</strong> - Green styling for successful actions</li>
                    <li><strong>"Error"</strong> - Red styling for error messages</li>
                    <li><strong>"Warning"</strong> - Yellow styling for warnings</li>
                    <li><strong>"Info"</strong> - Blue styling for informational messages</li>
                </ul>
                
                <h3>"Alert Sizes"</h3>
                <ul>
                    <li><strong>"Small"</strong> - Compact alert styling</li>
                    <li><strong>"Medium"</strong> - Standard alert styling (default)</li>
                    <li><strong>"Large"</strong> - Large alert styling</li>
                </ul>
            </div>
            
            <div class="example-section">
                <h2>"Basic Alert Examples"</h2>
                <p>"Different alert variants with various content types."</p>
                
                <div class="alert-grid">
                    <Alert variant=AlertVariant::Default>
                        <AlertTitle>"Default Alert"</AlertTitle>
                        <AlertDescription>
                            "This is a default alert with informational content."
                        </AlertDescription>
                    </Alert>
                    
                    <Alert variant=AlertVariant::Success>
                        <AlertTitle>"Success Alert"</AlertTitle>
                        <AlertDescription>
                            "Your action was completed successfully!"
                        </AlertDescription>
                    </Alert>
                    
                    <Alert variant=AlertVariant::Error>
                        <AlertTitle>"Error Alert"</AlertTitle>
                        <AlertDescription>
                            "Something went wrong. Please try again."
                        </AlertDescription>
                    </Alert>
                    
                    <Alert variant=AlertVariant::Warning>
                        <AlertTitle>"Warning Alert"</AlertTitle>
                        <AlertDescription>
                            "Please review your input before proceeding."
                        </AlertDescription>
                    </Alert>
                    
                    <Alert variant=AlertVariant::Info>
                        <AlertTitle>"Info Alert"</AlertTitle>
                        <AlertDescription>
                            "Here's some helpful information for you."
                        </AlertDescription>
                    </Alert>
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Alert Sizes"</h2>
                <p>"Different alert sizes for various use cases."</p>
                
                <div class="alert-grid">
                    <Alert variant=AlertVariant::Info size=AlertSize::Small>
                        <AlertTitle>"Small Alert"</AlertTitle>
                        <AlertDescription>
                            "This is a small alert for compact spaces."
                        </AlertDescription>
                    </Alert>
                    
                    <Alert variant=AlertVariant::Info size=AlertSize::Medium>
                        <AlertTitle>"Medium Alert"</AlertTitle>
                        <AlertDescription>
                            "This is a medium alert (default size)."
                        </AlertDescription>
                    </Alert>
                    
                    <Alert variant=AlertVariant::Info size=AlertSize::Large>
                        <AlertTitle>"Large Alert"</AlertTitle>
                        <AlertDescription>
                            "This is a large alert for prominent messaging."
                        </AlertDescription>
                    </Alert>
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Dismissible Alerts"</h2>
                <p>"Alerts that can be dismissed by the user."</p>
                
                <div class="alert-grid">
                    {move || {
                        if !is_dismissed("dismissible-1") {
                            view! {
                                <Alert
                                    variant=AlertVariant::Warning
                                    dismissible=true
                                    on_dismiss=Callback::new(move |_| handle_dismiss("dismissible-1".to_string()))
                                >
                                    <AlertTitle>"Dismissible Warning"</AlertTitle>
                                    <AlertDescription>
                                        "This alert can be dismissed by clicking the close button."
                                    </AlertDescription>
                                </Alert>
                            }
                        } else {
                            view! {
                                <Alert variant=AlertVariant::Default class="alert-placeholder".to_string()>
                                    <AlertDescription>
                                        "Alert dismissed - click 'Reset Alerts' to show again"
                                    </AlertDescription>
                                </Alert>
                            }
                        }
                    }}
                    
                    {move || {
                        if !is_dismissed("dismissible-2") {
                            view! {
                                <Alert
                                    variant=AlertVariant::Info
                                    dismissible=true
                                    on_dismiss=Callback::new(move |_| handle_dismiss("dismissible-2".to_string()))
                                >
                                    <AlertTitle>"Another Dismissible Alert"</AlertTitle>
                                    <AlertDescription>
                                        "This is another dismissible alert with different content."
                                    </AlertDescription>
                                </Alert>
                            }
                        } else {
                            view! {
                                <Alert variant=AlertVariant::Default class="alert-placeholder".to_string()>
                                    <AlertDescription>
                                        "Alert dismissed - click 'Reset Alerts' to show again"
                                    </AlertDescription>
                                </Alert>
                            }
                        }
                    }}
                </div>
                
                <button
                    class="btn btn-reset"
                    on:click=move |_| {
                        set_dismissed_alerts.set(std::collections::HashSet::new());
                        log!("Alerts reset");
                    }
                >
                    "Reset Alerts"
                </button>
            </div>
            
            <div class="example-section">
                <h2>"Alerts with Actions"</h2>
                <p>"Alerts that include action buttons for user interaction."</p>
                
                <div class="alert-grid">
                    <Alert variant=AlertVariant::Success>
                        <AlertTitle>"Action Required"</AlertTitle>
                        <AlertDescription>
                            "Your profile has been updated. Would you like to view the changes?"
                        </AlertDescription>
                        <div class="alert-actions">
                            <AlertAction
                                class="btn btn-primary".to_string()
                                on_click=Callback::new(move |_| log!("View changes clicked"))
                            >
                                "View Changes"
                            </AlertAction>
                            <AlertAction
                                class="btn btn-secondary".to_string()
                                on_click=Callback::new(move |_| log!("Dismiss clicked"))
                            >
                                "Dismiss"
                            </AlertAction>
                        </div>
                    </Alert>
                    
                    <Alert variant=AlertVariant::Error>
                        <AlertTitle>"Connection Error"</AlertTitle>
                        <AlertDescription>
                            "Unable to connect to the server. Please check your internet connection."
                        </AlertDescription>
                        <div class="alert-actions">
                            <AlertAction
                                class="btn btn-retry".to_string()
                                on_click=Callback::new(move |_| log!("Retry connection clicked"))
                            >
                                "Retry Connection"
                            </AlertAction>
                        </div>
                    </Alert>
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Simple Alerts"</h2>
                <p>"Alerts with minimal content for simple messaging."</p>
                
                <div class="alert-grid">
                    <Alert variant=AlertVariant::Info>
                        <AlertDescription>
                            "This is a simple alert with just a description."
                        </AlertDescription>
                    </Alert>
                    
                    <Alert variant=AlertVariant::Success>
                        <AlertDescription>
                            "Operation completed successfully!"
                        </AlertDescription>
                    </Alert>
                    
                    <Alert variant=AlertVariant::Warning>
                        <AlertDescription>
                            "Please save your work before continuing."
                        </AlertDescription>
                    </Alert>
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Alert Features"</h2>
                <ul>
                    <li>"Multiple variants (default, success, error, warning, info)"</li>
                    <li>"Three sizes (small, medium, large)"</li>
                    <li>"Dismissible alerts with close button"</li>
                    <li>"Action buttons for user interaction"</li>
                    <li>"Flexible content structure with title and description"</li>
                    <li>"Accessible with proper ARIA attributes"</li>
                    <li>"Responsive design"</li>
                    <li>"Icon indicators for each variant"</li>
                </ul>
            </div>
        </div>
    }
}

pub fn start_alert_examples() {
    mount_to_body(|| {
        view! {
            <AlertExamples />
        }
    });
}
