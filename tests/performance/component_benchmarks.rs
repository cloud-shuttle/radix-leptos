use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leptos::*;
use radix_leptos::*;
use std::time::Duration;

// Benchmark Button component rendering
fn benchmark_button_rendering(c: &mut Criterion) {
    let runtime = create_runtime();
    
    c.bench_function("button_rendering", |b| {
        b.iter(|| {
            let _ = view! {
                <Button>"Benchmark Button"</Button>
            };
            black_box(());
        });
    });
    
    runtime.dispose();
}

// Benchmark Form component with validation
fn benchmark_form_validation(c: &mut Criterion) {
    let runtime = create_runtime();
    
    c.bench_function("form_validation", |b| {
        b.iter(|| {
            let _ = view! {
                <Form>
                    <FormField>
                        <FormLabel>"Email"</FormLabel>
                        <FormControl>
                            <Input type="email" placeholder="Enter email" />
                        </FormControl>
                        <FormMessage>"Invalid email"</FormMessage>
                    </FormField>
                    <Button type="submit">"Submit"</Button>
                </Form>
            };
            black_box(());
        });
    });
    
    runtime.dispose();
}

// Benchmark DataTable with large dataset
fn benchmark_data_table(c: &mut Criterion) {
    let runtime = create_runtime();
    
    c.bench_function("data_table_large_dataset", |b| {
        b.iter(|| {
            let data = (0..1000).map(|i| format!("Row {}", i)).collect::<Vec<_>>();
            let _ = view! {
                <DataTable>
                    {data.into_iter().map(|row| {
                        view! {
                            <tr>
                                <td>{row}</td>
                                <td>"Data"</td>
                                <td>"More Data"</td>
                            </tr>
                        }
                    }).collect::<Vec<_>>()}
                </DataTable>
            };
            black_box(());
        });
    });
    
    runtime.dispose();
}

// Benchmark Modal component with animation
fn benchmark_modal_animation(c: &mut Criterion) {
    let runtime = create_runtime();
    
    c.bench_function("modal_animation", |b| {
        b.iter(|| {
            let _ = view! {
                <Dialog>
                    <DialogTrigger>
                        <Button>"Open Modal"</Button>
                    </DialogTrigger>
                    <DialogContent>
                        <DialogHeader>
                            <DialogTitle>"Benchmark Modal"</DialogTitle>
                            <DialogDescription>"This is a benchmark modal"</DialogDescription>
                        </DialogHeader>
                        <div>"Modal content here"</div>
                        <DialogFooter>
                            <Button variant="outline">"Cancel"</Button>
                            <Button>"Confirm"</Button>
                        </DialogFooter>
                    </DialogContent>
                </Dialog>
            };
            black_box(());
        });
    });
    
    runtime.dispose();
}

// Benchmark List component with virtualization
fn benchmark_virtual_list(c: &mut Criterion) {
    let runtime = create_runtime();
    
    c.bench_function("virtual_list_rendering", |b| {
        b.iter(|| {
            let items = (0..10000).map(|i| format!("Item {}", i)).collect::<Vec<_>>();
            let _ = view! {
                <VirtualList
                    items=items
                    item_height=50
                    container_height=400
                    render_item=move |item| {
                        view! {
                            <div class="list-item">
                                {item}
                            </div>
                        }
                    }
                />
            };
            black_box(());
        });
    });
    
    runtime.dispose();
}

// Benchmark component state updates
fn benchmark_state_updates(c: &mut Criterion) {
    let runtime = create_runtime();
    
    c.bench_function("state_updates", |b| {
        b.iter(|| {
            let (count, set_count) = create_signal(0);
            let _ = view! {
                <div>
                    <span>{move || count()}</span>
                    <Button on:click=move |_| set_count.update(|c| *c += 1)>
                        "Increment"
                    </Button>
                </div>
            };
            black_box(());
        });
    });
    
    runtime.dispose();
}

// Benchmark memory usage
fn benchmark_memory_usage(c: &mut Criterion) {
    let runtime = create_runtime();
    
    c.bench_function("memory_usage", |b| {
        b.iter(|| {
            let mut components = Vec::new();
            for i in 0..1000 {
                let component = view! {
                    <div>
                        <Button>"Button {i}"</Button>
                        <Input placeholder="Input {i}" />
                        <Card>
                            <CardHeader>
                                <CardTitle>"Card {i}"</CardTitle>
                            </CardHeader>
                            <CardContent>"Content {i}"</CardContent>
                        </Card>
                    </div>
                };
                components.push(component);
            }
            black_box(components);
        });
    });
    
    runtime.dispose();
}

// Benchmark component mounting and unmounting
fn benchmark_mount_unmount(c: &mut Criterion) {
    let runtime = create_runtime();
    
    c.bench_function("mount_unmount", |b| {
        b.iter(|| {
            let (show, set_show) = create_signal(true);
            let _ = view! {
                <div>
                    <Button on:click=move |_| set_show(!show())>
                        "Toggle"
                    </Button>
                    {move || if show() {
                        view! {
                            <div>
                                <Button>"Mounted Button"</Button>
                                <Input placeholder="Mounted Input" />
                                <Card>
                                    <CardContent>"Mounted Card"</CardContent>
                                </Card>
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }}
                </div>
            };
            black_box(());
        });
    });
    
    runtime.dispose();
}

// Benchmark CSS class application
fn benchmark_css_classes(c: &mut Criterion) {
    let runtime = create_runtime();
    
    c.bench_function("css_classes", |b| {
        b.iter(|| {
            let (variant, set_variant) = create_signal("default");
            let _ = view! {
                <div>
                    <Button 
                        variant=move || variant()
                        class="benchmark-button"
                        on:click=move |_| {
                            set_variant(if variant() == "default" { "destructive" } else { "default" });
                        }
                    >
                        "Dynamic Button"
                    </Button>
                    <div class="benchmark-container">
                        <span class="benchmark-text">"Text with classes"</span>
                        <div class="benchmark-nested">
                            <p class="benchmark-paragraph">"Nested content"</p>
                        </div>
                    </div>
                </div>
            };
            black_box(());
        });
    });
    
    runtime.dispose();
}

// Benchmark event handling
fn benchmark_event_handling(c: &mut Criterion) {
    let runtime = create_runtime();
    
    c.bench_function("event_handling", |b| {
        b.iter(|| {
            let (clicks, set_clicks) = create_signal(0);
            let (hovered, set_hovered) = create_signal(false);
            let (focused, set_focused) = create_signal(false);
            
            let _ = view! {
                <div>
                    <Button 
                        on:click=move |_| set_clicks.update(|c| *c += 1)
                        on:mouseenter=move |_| set_hovered(true)
                        on:mouseleave=move |_| set_hovered(false)
                    >
                        "Click me"
                    </Button>
                    <Input 
                        on:focus=move |_| set_focused(true)
                        on:blur=move |_| set_focused(false)
                        placeholder="Focus me"
                    />
                    <div>
                        "Clicks: " {move || clicks()}
                        " | Hovered: " {move || hovered()}
                        " | Focused: " {move || focused()}
                    </div>
                </div>
            };
            black_box(());
        });
    });
    
    runtime.dispose();
}

// Configure benchmark group
criterion_group!(
    name = component_benchmarks;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .warm_up_time(Duration::from_secs(3))
        .sample_size(100);
    targets = 
        benchmark_button_rendering,
        benchmark_form_validation,
        benchmark_data_table,
        benchmark_modal_animation,
        benchmark_virtual_list,
        benchmark_state_updates,
        benchmark_memory_usage,
        benchmark_mount_unmount,
        benchmark_css_classes,
        benchmark_event_handling
);

criterion_main!(component_benchmarks);

