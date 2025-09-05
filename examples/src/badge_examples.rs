use leptos::logging::log;
use leptos::prelude::*;
use leptos::*;
use radix_leptos_primitives::components::badge::*;

#[component]
pub fn BadgeExamples() -> impl IntoView {
    let (count, set_count) = signal(5u32);
    let (notification_count, set_notification_count) = signal(12u32);
    let (message_count, set_message_count) = signal(0u32);

    let handle_badge_click = move |_| {
        log!("Badge clicked!");
    };

    let increment_count = move |_| {
        set_count.update(|c| *c += 1);
    };

    let decrement_count = move |_| {
        if count.get() > 0 {
            set_count.update(|c| *c -= 1);
        }
    };

    let reset_count = move |_| {
        set_count.set(0);
    };

    let increment_notifications = move |_| {
        set_notification_count.update(|c| *c += 1);
    };

    let increment_messages = move |_| {
        set_message_count.update(|c| *c += 1);
    };

    view! {
        <div class="badge-examples">
            <h1>"Badge Component Examples"</h1>

            <div class="api-docs">
                <h2>"API Documentation"</h2>
                <ul>
                    <li><strong>"Badge"</strong> - Root badge component with variants and sizing</li>
                    <li><strong>"BadgeCount"</strong> - Badge with count/number display</li>
                    <li><strong>"BadgeDot"</strong> - Badge with dot indicator</li>
                </ul>

                <h3>"Badge Variants"</h3>
                <ul>
                    <li><strong>"Default"</strong> - Standard badge styling</li>
                    <li><strong>"Primary"</strong> - Primary brand color</li>
                    <li><strong>"Secondary"</strong> - Secondary brand color</li>
                    <li><strong>"Success"</strong> - Green styling for success states</li>
                    <li><strong>"Error"</strong> - Red styling for error states</li>
                    <li><strong>"Warning"</strong> - Yellow styling for warning states</li>
                    <li><strong>"Info"</strong> - Blue styling for informational states</li>
                    <li><strong>"Outline"</strong> - Outlined badge styling</li>
                </ul>

                <h3>"Badge Sizes"</h3>
                <ul>
                    <li><strong>"Small"</strong> - Compact badge styling</li>
                    <li><strong>"Medium"</strong> - Standard badge styling (default)</li>
                    <li><strong>"Large"</strong> - Large badge styling</li>
                </ul>
            </div>

            <div class="example-section">
                <h2>"Basic Badge Examples"</h2>
                <p>"Different badge variants with various content types."</p>

                <div class="badge-grid">
                    <Badge variant=BadgeVariant::Default>
                        "Default"
                    </Badge>

                    <Badge variant=BadgeVariant::Primary>
                        "Primary"
                    </Badge>

                    <Badge variant=BadgeVariant::Secondary>
                        "Secondary"
                    </Badge>

                    <Badge variant=BadgeVariant::Success>
                        "Success"
                    </Badge>

                    <Badge variant=BadgeVariant::Error>
                        "Error"
                    </Badge>

                    <Badge variant=BadgeVariant::Warning>
                        "Warning"
                    </Badge>

                    <Badge variant=BadgeVariant::Info>
                        "Info"
                    </Badge>

                    <Badge variant=BadgeVariant::Outline>
                        "Outline"
                    </Badge>
                </div>
            </div>

            <div class="example-section">
                <h2>"Badge Sizes"</h2>
                <p>"Different badge sizes for various use cases."</p>

                <div class="badge-grid">
                    <Badge variant=BadgeVariant::Primary size=BadgeSize::Small>
                        "Small"
                    </Badge>

                    <Badge variant=BadgeVariant::Primary size=BadgeSize::Medium>
                        "Medium"
                    </Badge>

                    <Badge variant=BadgeVariant::Primary size=BadgeSize::Large>
                        "Large"
                    </Badge>
                </div>
            </div>

            <div class="example-section">
                <h2>"Interactive Badges"</h2>
                <p>"Badges that can be clicked for user interaction."</p>

                <div class="badge-grid">
                    <Badge
                        variant=BadgeVariant::Primary
                        interactive=true
                        on_click=Callback::new(handle_badge_click)
                    >
                        "Clickable Badge"
                    </Badge>

                    <Badge
                        variant=BadgeVariant::Success
                        interactive=true
                        on_click=Callback::new(handle_badge_click)
                    >
                        "Interactive Success"
                    </Badge>

                    <Badge
                        variant=BadgeVariant::Error
                        interactive=true
                        disabled=true
                        on_click=Callback::new(handle_badge_click)
                    >
                        "Disabled Badge"
                    </Badge>
                </div>
            </div>

            <div class="example-section">
                <h2>"Badge Count Examples"</h2>
                <p>"Badges that display counts with various configurations."</p>

                <div class="badge-grid">
                    <BadgeCount count=(move || count.get())() variant=BadgeVariant::Error />

                    <BadgeCount
                        count=(move || notification_count.get())()
                        variant=BadgeVariant::Primary
                        size=BadgeSize::Medium
                    />

                    <BadgeCount
                        count=(move || message_count.get())()
                        variant=BadgeVariant::Info
                        show_zero=true
                    />

                    <BadgeCount
                        count=150
                        max_count=99
                        variant=BadgeVariant::Warning
                    />
                </div>

                <div class="controls">
                    <button class="btn btn-primary" on:click=increment_count>
                        "Increment Count"
                    </button>
                    <button class="btn btn-secondary" on:click=decrement_count>
                        "Decrement Count"
                    </button>
                    <button class="btn btn-reset" on:click=reset_count>
                        "Reset Count"
                    </button>
                    <button class="btn btn-info" on:click=increment_notifications>
                        "Add Notification"
                    </button>
                    <button class="btn btn-success" on:click=increment_messages>
                        "Add Message"
                    </button>
                </div>
            </div>

            <div class="example-section">
                <h2>"Badge Dot Examples"</h2>
                <p>"Dot indicators for status and notifications."</p>

                <div class="badge-grid">
                    <BadgeDot variant=BadgeVariant::Success />
                    <BadgeDot variant=BadgeVariant::Error />
                    <BadgeDot variant=BadgeVariant::Warning />
                    <BadgeDot variant=BadgeVariant::Info />
                    <BadgeDot variant=BadgeVariant::Primary />
                    <BadgeDot variant=BadgeVariant::Success pulsing=true />
                </div>
            </div>

            <div class="example-section">
                <h2>"Badge in Context"</h2>
                <p>"Badges used in realistic UI contexts."</p>

                <div class="context-examples">
                    <div class="notification-item">
                        <span class="notification-text">"New message from John"</span>
                        <BadgeCount count=3 variant=BadgeVariant::Error />
                    </div>

                    <div class="notification-item">
                        <span class="notification-text">"System update available"</span>
                        <BadgeDot variant=BadgeVariant::Info />
                    </div>

                    <div class="notification-item">
                        <span class="notification-text">"Task completed"</span>
                        <Badge variant=BadgeVariant::Success>
                            "Done"
                        </Badge>
                    </div>

                    <div class="notification-item">
                        <span class="notification-text">"Critical alert"</span>
                        <BadgeDot variant=BadgeVariant::Error pulsing=true />
                    </div>
                </div>
            </div>

            <div class="example-section">
                <h2>"Badge Features"</h2>
                <ul>
                    <li>"Multiple variants (default, primary, secondary, success, error, warning, info, outline)"</li>
                    <li>"Three sizes (small, medium, large)"</li>
                    <li>"Interactive badges with click handlers"</li>
                    <li>"Disabled state for interactive badges"</li>
                    <li>"Count badges with max count limits"</li>
                    <li>"Dot indicators with pulsing animation"</li>
                    <li>"Accessible with proper ARIA attributes"</li>
                    <li>"Responsive design"</li>
                    <li>"Flexible content structure"</li>
                </ul>
            </div>
        </div>
    }
}

pub fn start_badge_examples() {
    mount_to_body(|| {
        view! {
            <BadgeExamples />
        }
    });
}
