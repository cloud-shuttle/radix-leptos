use leptos::logging::log;
use leptos::prelude::*;
use leptos::*;
use radix_leptos_primitives::*;

#[component]
pub fn AlertExamples() -> impl IntoView {
    view! {
        <div class="alert-examples">
            <h1>"Alert Component Examples"</h1>

            <div class="api-docs">
                <h2>"API Documentation"</h2>
                <ul>
                    <li><strong>"Alert"</strong> - Root alert component with variants and sizing</li>
                    <li><strong>"AlertTitle"</strong> - Title component for alerts</li>
                    <li><strong>"AlertDescription"</strong> - Description component for alerts</li>
                </ul>

                <h3>"Alert Variants"</h3>
                <ul>
                    <li><strong>"Default"</strong> - Standard alert styling</li>
                    <li><strong>"Success"</strong> - Green styling for successful actions</li>
                    <li><strong>"Destructive"</strong> - Red styling for error messages</li>
                    <li><strong>"Warning"</strong> - Yellow styling for warnings</li>
                    <li><strong>"Info"</strong> - Blue styling for informational messages</li>
                </ul>

                <h3>"Alert Sizes"</h3>
                <ul>
                    <li><strong>"Default"</strong> - Standard alert styling</li>
                    <li><strong>"Sm"</strong> - Small alert styling</li>
                    <li><strong>"Lg"</strong> - Large alert styling</li>
                </ul>
            </div>

            <div class="example-section">
                <h2>"Basic Alert Examples"</h2>
                <p>"Different alert variants for different types of messages:"</p>

                <div class="alert-grid">
                    <Alert variant=AlertVariant::Default>
                        <AlertTitle>"Default Alert"</AlertTitle>
                        <AlertDescription>"This is a default alert with standard styling."</AlertDescription>
                    </Alert>

                    <Alert variant=AlertVariant::Success>
                        <AlertTitle>"Success Alert"</AlertTitle>
                        <AlertDescription>"Your action was completed successfully!"</AlertDescription>
                    </Alert>

                    <Alert variant=AlertVariant::Destructive>
                        <AlertTitle>"Destructive Alert"</AlertTitle>
                        <AlertDescription>"Something went wrong. Please try again."</AlertDescription>
                    </Alert>

                    <Alert variant=AlertVariant::Warning>
                        <AlertTitle>"Warning Alert"</AlertTitle>
                        <AlertDescription>"Please review your input before proceeding."</AlertDescription>
                    </Alert>

                    <Alert variant=AlertVariant::Info>
                        <AlertTitle>"Info Alert"</AlertTitle>
                        <AlertDescription>"Here's some helpful information for you."</AlertDescription>
                    </Alert>
                </div>
            </div>

            <div class="example-section">
                <h2>"Alert Sizes"</h2>
                <p>"Different alert sizes for different use cases:"</p>

                <div class="alert-grid">
                    <Alert variant=AlertVariant::Default size=AlertSize::Sm>
                        <AlertTitle>"Small Alert"</AlertTitle>
                        <AlertDescription>"This is a small alert with compact styling."</AlertDescription>
                    </Alert>

                    <Alert variant=AlertVariant::Success size=AlertSize::Default>
                        <AlertTitle>"Default Size Alert"</AlertTitle>
                        <AlertDescription>"This is a default size alert with standard styling."</AlertDescription>
                    </Alert>

                    <Alert variant=AlertVariant::Info size=AlertSize::Lg>
                        <AlertTitle>"Large Alert"</AlertTitle>
                        <AlertDescription>"This is a large alert with expanded styling for important messages."</AlertDescription>
                    </Alert>
                </div>
            </div>

            <div class="example-section">
                <h2>"Dismissible Alerts"</h2>
                <p>"Alerts that can be dismissed by the user:"</p>

                <div class="alert-grid">
                    <Alert variant=AlertVariant::Default dismissible=true>
                        <AlertTitle>"Dismissible Alert"</AlertTitle>
                        <AlertDescription>"This alert can be dismissed by clicking the close button."</AlertDescription>
                    </Alert>

                    <Alert variant=AlertVariant::Success dismissible=true>
                        <AlertTitle>"Success Alert"</AlertTitle>
                        <AlertDescription>"This success alert can also be dismissed."</AlertDescription>
                    </Alert>

                    <Alert variant=AlertVariant::Warning dismissible=true>
                        <AlertTitle>"Warning Alert"</AlertTitle>
                        <AlertDescription>"This warning alert can be dismissed as well."</AlertDescription>
                    </Alert>
                </div>
            </div>

            <div class="example-section">
                <h2>"Custom Styled Alerts"</h2>
                <p>"Alerts with custom CSS classes and styles:"</p>

                <div class="alert-grid">
                    <Alert
                        variant=AlertVariant::Default
                        class="custom-alert".to_string()
                        style="border-left: 4px solid #3b82f6;".to_string()
                    >
                        <AlertTitle>"Custom Styled Alert"</AlertTitle>
                        <AlertDescription>"This alert has custom styling applied."</AlertDescription>
                    </Alert>

                    <Alert
                        variant=AlertVariant::Info
                        class="custom-alert-info".to_string()
                        style="background: linear-gradient(135deg, #dbeafe 0%, #bfdbfe 100%);".to_string()
                    >
                        <AlertTitle>"Gradient Alert"</AlertTitle>
                        <AlertDescription>"This alert has a custom gradient background."</AlertDescription>
                    </Alert>
                </div>
            </div>

            <div class="example-section">
                <h2>"Alert with Actions"</h2>
                <p>"Alerts that include action buttons:"</p>

                <div class="alert-grid">
                    <Alert variant=AlertVariant::Success>
                        <AlertTitle>"Action Required"</AlertTitle>
                        <AlertDescription>"Your account has been created successfully. Please verify your email address."</AlertDescription>
                        <div class="alert-actions">
                            <Button variant=ButtonVariant::Default size=ButtonSize::Small>
                                "Verify Email"
                            </Button>
                            <Button variant=ButtonVariant::Outline size=ButtonSize::Small>
                                "Skip for Now"
                            </Button>
                        </div>
                    </Alert>

                    <Alert variant=AlertVariant::Destructive>
                        <AlertTitle>"Account Suspended"</AlertTitle>
                        <AlertDescription>"Your account has been suspended due to policy violations. Please contact support."</AlertDescription>
                        <div class="alert-actions">
                            <Button variant=ButtonVariant::Destructive size=ButtonSize::Small>
                                "Contact Support"
                            </Button>
                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Small>
                                "Learn More"
                            </Button>
                        </div>
                    </Alert>
                </div>
            </div>
        </div>
    }
}
