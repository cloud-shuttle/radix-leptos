use leptos::prelude::*;
use leptos::*;
use radix_leptos_primitives::*;

#[component]
pub fn TimelineExamples() -> impl IntoView {
    // Create timeline events for different scenarios
    let project_event1 = TimelineEvent {
        id: "1".to_string(),
        title: "Project Started".to_string(),
        description: Some("Initial project setup and planning phase completed".to_string()),
        date: "2024-01-15".to_string(),
        icon: Some("🚀".to_string()),
        color: Some("green".to_string()),
        category: Some("milestone".to_string()),
    };

    let project_event2 = TimelineEvent {
        id: "2".to_string(),
        title: "Design Phase".to_string(),
        description: Some("UI/UX design and wireframes created".to_string()),
        date: "2024-01-20".to_string(),
        icon: Some("🎨".to_string()),
        color: Some("blue".to_string()),
        category: Some("design".to_string()),
    };

    let project_event3 = TimelineEvent {
        id: "3".to_string(),
        title: "Development Phase".to_string(),
        description: Some("Core functionality implementation".to_string()),
        date: "2024-02-01".to_string(),
        icon: Some("💻".to_string()),
        color: Some("green".to_string()),
        category: Some("development".to_string()),
    };

    let project_timeline_events = vec![
        project_event1.clone(),
        project_event2.clone(),
        project_event3.clone(),
    ];

    let empty_timeline_events: Vec<TimelineEvent> = vec![];

    view! {
        <div class="timeline-examples">
            <h1>"Timeline Component Examples"</h1>

            <div class="example-section">
                <h2>"Project Timeline"</h2>
                <p>"A vertical timeline showing project milestones:"</p>
                <Timeline
                    events=project_timeline_events.clone()
                    orientation=TimelineOrientation::Vertical
                    show_dates=true
                    show_icons=true
                >
                    <TimelineItem event=project_event1.clone()>
                        <div class="timeline-content">
                            <div class="timeline-header">
                                <span class="timeline-icon">{project_event1.icon.clone().unwrap_or_default()}</span>
                                <h3 class="timeline-title">{project_event1.title.clone()}</h3>
                                <span class="timeline-date">{project_event1.date.clone()}</span>
                            </div>
                            <p class="timeline-description">{project_event1.description.clone().unwrap_or_default()}</p>
                        </div>
                    </TimelineItem>
                    <TimelineItem event=project_event2.clone()>
                        <div class="timeline-content">
                            <div class="timeline-header">
                                <span class="timeline-icon">{project_event2.icon.clone().unwrap_or_default()}</span>
                                <h3 class="timeline-title">{project_event2.title.clone()}</h3>
                                <span class="timeline-date">{project_event2.date.clone()}</span>
                            </div>
                            <p class="timeline-description">{project_event2.description.clone().unwrap_or_default()}</p>
                        </div>
                    </TimelineItem>
                    <TimelineItem event=project_event3.clone()>
                        <div class="timeline-content">
                            <div class="timeline-header">
                                <span class="timeline-icon">{project_event3.icon.clone().unwrap_or_default()}</span>
                                <h3 class="timeline-title">{project_event3.title.clone()}</h3>
                                <span class="timeline-date">{project_event3.date.clone()}</span>
                            </div>
                            <p class="timeline-description">{project_event3.description.clone().unwrap_or_default()}</p>
                        </div>
                    </TimelineItem>
                </Timeline>
            </div>

            <div class="example-section">
                <h2>"Empty Timeline"</h2>
                <p>"A timeline with no events:"</p>
                <Timeline
                    events=empty_timeline_events.clone()
                    orientation=TimelineOrientation::Vertical
                    show_dates=true
                    show_icons=true
                >
                    <div class="timeline-empty">
                        <p>"No events to display"</p>
                    </div>
                </Timeline>
            </div>

            <div class="example-section">
                <h2>"Loading Timeline"</h2>
                <p>"A timeline in loading state:"</p>
                <Timeline
                    events=empty_timeline_events
                    orientation=TimelineOrientation::Vertical
                    show_dates=true
                    show_icons=true
                >
                    <div class="timeline-loading">
                        <div class="loading-spinner"></div>
                        <p>"Loading timeline events..."</p>
                    </div>
                </Timeline>
            </div>
        </div>
    }
}
