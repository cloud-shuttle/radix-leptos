use leptos::*;
use leptos::prelude::*;
use leptos::logging::log;
use radix_leptos_primitives::*;

#[component]
pub fn TimelineExamples() -> impl IntoView {
    // Create timeline items for different scenarios
    let project_item1 = create_timeline_item(
        "1",
        "Project Started".to_string(),
        "Project Started",
        Some("Initial project setup and planning phase completed"),
        Some("2024-01-15"),
        Some("🚀"),
        TimelineStatus::Success,
    );
    
    let project_item2 = create_timeline_item(
        "2",
        "Design Phase".to_string(),
        "Design Phase",
        Some("UI/UX design and wireframes created"),
        Some("2024-01-20"),
        Some("🎨"),
        TimelineStatus::Info,
    );
    
    let project_item3 = create_timeline_item(
        "3",
        "Development Started".to_string(),
        "Development Started",
        Some("Core development work began"),
        Some("2024-02-01"),
        Some("💻"),
        TimelineStatus::Success,
    );

    let project_timeline_items = vec![project_item1.clone(), project_item2.clone(), project_item3.clone()];

    let activity_item1 = create_timeline_item(
        "a1",
        "User Login".to_string(),
        "User Login",
        Some("User john.doe logged in successfully"),
        Some("10:30 AM"),
        Some("👤"),
        TimelineStatus::Success,
    );
    
    let activity_item2 = create_timeline_item(
        "a2",
        "File Upload".to_string(),
        "File Upload",
        Some("Document 'report.pdf' uploaded"),
        Some("10:35 AM"),
        Some("📁"),
        TimelineStatus::Info,
    );

    let activity_timeline_items = vec![activity_item1.clone(), activity_item2.clone()];

    let empty_timeline_items: Vec<TimelineItem<String>> = vec![];

    // Event handlers
    let handle_item_click = Callback::new(move |item: TimelineItem<String>| {
        log!("Timeline item clicked: {}", item.title);
    });

    let handle_item_focus = Callback::new(move |item: TimelineItem<String>| {
        log!("Timeline item focused: {}", item.title);
    });

    view! {
        <div class="timeline-examples">
            <h1>"Timeline Component Examples"</h1>
            
            <section class="example-section">
                <h2>"Project Timeline"</h2>
                <p>"A comprehensive project timeline with different status indicators."</p>
                
                <Timeline 
                    items=project_timeline_items
                    on_item_click=handle_item_click.clone()
                    on_item_focus=handle_item_focus.clone()
                    class="project-timeline".to_string()
                >
                    <TimelineItemComponent item=project_item1.clone()>
                        <div class="timeline-item-content">
                            <span class="timeline-item-meta">
                                {"Status: "} {format!("{:?}", project_item1.status)}
                            </span>
                        </div>
                    </TimelineItemComponent>
                    <TimelineItemComponent item=project_item2.clone()>
                        <div class="timeline-item-content">
                            <span class="timeline-item-meta">
                                {"Status: "} {format!("{:?}", project_item2.status)}
                            </span>
                        </div>
                    </TimelineItemComponent>
                    <TimelineItemComponent item=project_item3.clone()>
                        <div class="timeline-item-content">
                            <span class="timeline-item-meta">
                                {"Status: "} {format!("{:?}", project_item3.status)}
                            </span>
                        </div>
                    </TimelineItemComponent>
                </Timeline>
            </section>

            <section class="example-section">
                <h2>"Activity Timeline"</h2>
                <p>"Real-time activity feed with timestamps and status indicators."</p>
                
                <Timeline 
                    items=activity_timeline_items
                    size=TimelineSize::Small
                    variant=TimelineVariant::Compact
                    on_item_click=handle_item_click.clone()
                    on_item_focus=handle_item_focus.clone()
                    class="activity-timeline".to_string()
                >
                    <TimelineItemComponent item=activity_item1.clone()>
                        <div class="timeline-item-content">
                            <span class="timeline-item-meta">
                                {"Activity: "} {activity_item1.data.clone()}
                            </span>
                        </div>
                    </TimelineItemComponent>
                    <TimelineItemComponent item=activity_item2.clone()>
                        <div class="timeline-item-content">
                            <span class="timeline-item-meta">
                                {"Activity: "} {activity_item2.data.clone()}
                            </span>
                        </div>
                    </TimelineItemComponent>
                </Timeline>
            </section>

            <section class="example-section">
                <h2>"Empty Timeline"</h2>
                <p>"Timeline with no items showing empty state."</p>
                
                <Timeline 
                    items=empty_timeline_items.clone()
                    class="empty-timeline".to_string()
                >
                    <TimelineEmpty 
                        message="No timeline items available".to_string()
                    >
                        <p>"Add some items to see the timeline in action!"</p>
                    </TimelineEmpty>
                </Timeline>
            </section>

            <section class="example-section">
                <h2>"Loading Timeline"</h2>
                <p>"Timeline in loading state."</p>
                
                <Timeline 
                    items=empty_timeline_items
                    class="loading-timeline".to_string()
                >
                    <TimelineLoading 
                        message="Loading timeline data...".to_string()
                    >
                        <p>"Please wait while we fetch your timeline data."</p>
                    </TimelineLoading>
                </Timeline>
            </section>

            <section class="example-section">
                <h2>"API Documentation"</h2>
                <div class="api-docs">
                    <h3>"Timeline Component Props"</h3>
                    <ul>
                        <li><strong>"items"</strong> - Vector of TimelineItem&lt;T&gt; to display</li>
                        <li><strong>"size"</strong> - TimelineSize (Small, Medium, Large)</li>
                        <li><strong>"variant"</strong> - TimelineVariant (Default, Bordered, Compact, Vertical, Horizontal)</li>
                        <li><strong>"class"</strong> - Optional CSS class</li>
                        <li><strong>"on_item_click"</strong> - Callback when item is clicked</li>
                        <li><strong>"on_item_focus"</strong> - Callback when item receives focus</li>
                    </ul>

                    <h3>"TimelineItem Structure"</h3>
                    <ul>
                        <li><strong>"id"</strong> - Unique identifier</li>
                        <li><strong>"data"</strong> - Generic data payload</li>
                        <li><strong>"title"</strong> - Item title</li>
                        <li><strong>"description"</strong> - Optional description</li>
                        <li><strong>"date"</strong> - Optional date/timestamp</li>
                        <li><strong>"icon"</strong> - Optional custom icon</li>
                        <li><strong>"status"</strong> - TimelineStatus (Default, Success, Warning, Error, Info)</li>
                        <li><strong>"disabled"</strong> - Whether item is disabled</li>
                    </ul>

                    <h3>"Helper Functions"</h3>
                    <ul>
                        <li><strong>"create_timeline_item"</strong> - Create a new timeline item</li>
                        <li><strong>"create_disabled_timeline_item"</strong> - Create a disabled timeline item</li>
                    </ul>
                </div>
            </section>
        </div>
    }
}
