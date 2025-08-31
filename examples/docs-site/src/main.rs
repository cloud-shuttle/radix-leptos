use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod pages;

use components::*;
use pages::*;

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/radix-leptos-docs.css"/>
        <Title text="Radix-Leptos Documentation"/>
        <Meta name="description" content="Accessible, unstyled UI components for Leptos applications"/>
        
        <Router>
            <div class="docs-layout">
                <nav class="docs-sidebar">
                    <Sidebar/>
                </nav>
                <main class="docs-main">
                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route path="/getting-started" view=GettingStartedPage/>
                        <Route path="/components" view=ComponentsPage/>
                        <Route path="/components/:component" view=ComponentPage/>
                        <Route path="/playground" view=PlaygroundPage/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}