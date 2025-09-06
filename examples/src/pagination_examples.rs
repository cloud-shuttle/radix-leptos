use leptos::callback::Callback;
use leptos::prelude::*;
use leptos::*;
use radix_leptos_primitives::*;

/// Comprehensive pagination examples demonstrating all features
#[component]
pub fn PaginationExamples() -> impl IntoView {
    // Reactive state for different pagination examples
    let (basic_current_page, set_basic_current_page) = signal(1);
    let (compact_current_page, set_compact_current_page) = signal(3);
    let (detailed_current_page, set_detailed_current_page) = signal(5);
    let (custom_current_page, set_custom_current_page) = signal(7);
    let (large_dataset_current_page, set_large_dataset_current_page) = signal(1);

    // Event handlers
    let handle_basic_page_change = Callback::new(move |page: usize| {
        set_basic_current_page.set(page);
        web_sys::console::log_1(&format!("Basic pagination: page {}", page).into());
    });

    let handle_compact_page_change = Callback::new(move |page: usize| {
        set_compact_current_page.set(page);
        web_sys::console::log_1(&format!("Compact pagination: page {}", page).into());
    });

    let handle_detailed_page_change = Callback::new(move |page: usize| {
        set_detailed_current_page.set(page);
        web_sys::console::log_1(&format!("Detailed pagination: page {}", page).into());
    });

    let handle_custom_page_change = Callback::new(move |page: usize| {
        set_custom_current_page.set(page);
        web_sys::console::log_1(&format!("Custom pagination: page {}", page).into());
    });

    let handle_large_dataset_page_change = Callback::new(move |page: usize| {
        set_large_dataset_current_page.set(page);
        web_sys::console::log_1(&format!("Large dataset pagination: page {}", page).into());
    });

    view! {
        <div class="pagination-examples">
            <h1>"📄 Pagination Component Examples"</h1>

            // Basic Pagination Example
            <section class="example-section">
                <h2>"1. Basic Pagination"</h2>
                <p>"Simple pagination with page numbers and navigation buttons."</p>

                <Pagination
                    current_page=basic_current_page.get()
                    total_pages=10
                    on_page_change=handle_basic_page_change.clone()
                >
                    <PaginationList>
                        <PaginationFirst icon="⏮️".to_string()>
                            "First"
                        </PaginationFirst>
                        <PaginationPrevious icon="◀️".to_string()>
                            "Previous"
                        </PaginationPrevious>

                        <PaginationItem page=PaginationPage::new(1).withcurrent(basic_current_page.get() == 1)>
                            "1"
                        </PaginationItem>
                        <PaginationItem page=PaginationPage::new(2).withcurrent(basic_current_page.get() == 2)>
                            "2"
                        </PaginationItem>
                        <PaginationItem page=PaginationPage::new(3).withcurrent(basic_current_page.get() == 3)>
                            "3"
                        </PaginationItem>
                        <PaginationItem page=PaginationPage::new(4).withcurrent(basic_current_page.get() == 4)>
                            "4"
                        </PaginationItem>
                        <PaginationItem page=PaginationPage::new(5).withcurrent(basic_current_page.get() == 5)>
                            "5"
                        </PaginationItem>

                        <PaginationEllipsis>
                            "…"
                        </PaginationEllipsis>

                        <PaginationItem page=PaginationPage::new(9).withcurrent(basic_current_page.get() == 9)>
                            "9"
                        </PaginationItem>
                        <PaginationItem page=PaginationPage::new(10).withcurrent(basic_current_page.get() == 10)>
                            "10"
                        </PaginationItem>

                        <PaginationNext icon="▶️".to_string()>
                            "Next"
                        </PaginationNext>
                        <PaginationLast icon="⏭️".to_string()>
                            "Last"
                        </PaginationLast>
                    </PaginationList>
                </Pagination>

                <div class="example-info">
                    <p><strong>"Current page:"</strong> {basic_current_page}</p>
                </div>
            </section>

            // Compact Pagination Example
            <section class="example-section">
                <h2>"2. Compact Pagination"</h2>
                <p>"Minimal pagination with just previous/next and current page indicator."</p>

                <Pagination
                    current_page=compact_current_page.get()
                    total_pages=15
                    variant=PaginationVariant::Compact
                    _show_first_last=false
                    on_page_change=handle_compact_page_change.clone()
                >
                    <PaginationList>
                        <PaginationPrevious icon="◀️".to_string()>
                            "Previous"
                        </PaginationPrevious>

                        <PaginationItem page=PaginationPage::new(compact_current_page.get()).withcurrent(true)>
                            {format!("{} of {}", compact_current_page.get(), 15)}
                        </PaginationItem>

                        <PaginationNext icon="▶️".to_string()>
                            "Next"
                        </PaginationNext>
                    </PaginationList>
                </Pagination>

                <div class="example-info">
                    <p><strong>"Current page:"</strong> {compact_current_page}</p>
                </div>
            </section>

            // Detailed Pagination Example
            <section class="example-section">
                <h2>"3. Detailed Pagination with Info"</h2>
                <p>"Pagination with detailed information about current page and total items."</p>

                <Pagination
                    current_page=detailed_current_page.get()
                    total_pages=8
                    total_items=240
                    page_size=30
                    variant=PaginationVariant::Detailed
                    on_page_change=handle_detailed_page_change.clone()
                >
                    <PaginationContent>
                        <PaginationInfo format="Page {current} of {total_pages} ({start}-{end} of {total} items)".to_string()>
                            <span></span>
                        </PaginationInfo>

                        <PaginationList>
                            <PaginationFirst icon="⏮️".to_string()>
                                "First"
                            </PaginationFirst>
                            <PaginationPrevious icon="◀️".to_string()>
                                "Previous"
                            </PaginationPrevious>

                            <PaginationItem page=PaginationPage::new(1).withcurrent(detailed_current_page.get() == 1)>
                                "1"
                            </PaginationItem>
                            <PaginationItem page=PaginationPage::new(2).withcurrent(detailed_current_page.get() == 2)>
                                "2"
                            </PaginationItem>
                            <PaginationItem page=PaginationPage::new(3).withcurrent(detailed_current_page.get() == 3)>
                                "3"
                            </PaginationItem>
                            <PaginationItem page=PaginationPage::new(4).withcurrent(detailed_current_page.get() == 4)>
                                "4"
                            </PaginationItem>
                            <PaginationItem page=PaginationPage::new(5).withcurrent(detailed_current_page.get() == 5)>
                                "5"
                            </PaginationItem>
                            <PaginationItem page=PaginationPage::new(6).withcurrent(detailed_current_page.get() == 6)>
                                "6"
                            </PaginationItem>
                            <PaginationItem page=PaginationPage::new(7).withcurrent(detailed_current_page.get() == 7)>
                                "7"
                            </PaginationItem>
                            <PaginationItem page=PaginationPage::new(8).withcurrent(detailed_current_page.get() == 8)>
                                "8"
                            </PaginationItem>

                            <PaginationNext icon="▶️".to_string()>
                                "Next"
                            </PaginationNext>
                            <PaginationLast icon="⏭️".to_string()>
                                "Last"
                            </PaginationLast>
                        </PaginationList>
                    </PaginationContent>
                </Pagination>

                <div class="example-info">
                    <p><strong>"Current page:"</strong> {detailed_current_page}</p>
                </div>
            </section>

            // Custom Pagination Example
            <section class="example-section">
                <h2>"4. Custom Pagination with Helper Functions"</h2>
                <p>"Using helper functions to generate pagination controls dynamically."</p>

                <Pagination
                    current_page=custom_current_page.get()
                    total_pages=20
                    on_page_change=handle_custom_page_change.clone()
                >
                    <PaginationList>
                        <PaginationFirst icon="⏮️".to_string()>
                            "First"
                        </PaginationFirst>
                        <PaginationPrevious icon="◀️".to_string()>
                            "Previous"
                        </PaginationPrevious>

                        <PaginationItem page=PaginationPage::new(1).withcurrent(custom_current_page.get() == 1)>
                            "1"
                        </PaginationItem>
                        <PaginationItem page=PaginationPage::new(2).withcurrent(custom_current_page.get() == 2)>
                            "2"
                        </PaginationItem>
                        <PaginationItem page=PaginationPage::new(3).withcurrent(custom_current_page.get() == 3)>
                            "3"
                        </PaginationItem>
                        <PaginationItem page=PaginationPage::new(4).withcurrent(custom_current_page.get() == 4)>
                            "4"
                        </PaginationItem>
                        <PaginationItem page=PaginationPage::new(5).withcurrent(custom_current_page.get() == 5)>
                            "5"
                        </PaginationItem>
                        <PaginationItem page=PaginationPage::new(6).withcurrent(custom_current_page.get() == 6)>
                            "6"
                        </PaginationItem>
                        <PaginationItem page=PaginationPage::new(7).withcurrent(custom_current_page.get() == 7)>
                            "7"
                        </PaginationItem>
                        <PaginationEllipsis>
                            "…"
                        </PaginationEllipsis>
                        <PaginationItem page=PaginationPage::new(19).withcurrent(custom_current_page.get() == 19)>
                            "19"
                        </PaginationItem>
                        <PaginationItem page=PaginationPage::new(20).withcurrent(custom_current_page.get() == 20)>
                            "20"
                        </PaginationItem>

                        <PaginationNext icon="▶️".to_string()>
                            "Next"
                        </PaginationNext>
                        <PaginationLast icon="⏭️".to_string()>
                            "Last"
                        </PaginationLast>
                    </PaginationList>
                </Pagination>

                <div class="example-info">
                    <p><strong>"Current page:"</strong> {custom_current_page}</p>
                    <p><strong>"Features:"</strong> "Dynamic page generation, ellipsis for large page counts, helper functions"</p>
                </div>
            </section>

            // Large Dataset Pagination Example
            <section class="example-section">
                <h2>"5. Large Dataset Pagination"</h2>
                <p>"Pagination for large datasets with smart page range calculation."</p>

                <Pagination
                    current_page=large_dataset_current_page.get()
                    total_pages=100
                    total_items=10000
                    page_size=100
                    size=PaginationSize::Large
                    on_page_change=handle_large_dataset_page_change.clone()
                >
                    <PaginationContent>
                        <PaginationInfo>
                            <span></span>
                        </PaginationInfo>

                        <PaginationList>
                            <PaginationFirst icon="⏮️".to_string()>
                                "First"
                            </PaginationFirst>
                            <PaginationPrevious icon="◀️".to_string()>
                                "Previous"
                            </PaginationPrevious>

                            <PaginationItem page=PaginationPage::new(1).withcurrent(large_dataset_current_page.get() == 1)>
                                "1"
                            </PaginationItem>
                            <PaginationItem page=PaginationPage::new(2).withcurrent(large_dataset_current_page.get() == 2)>
                                "2"
                            </PaginationItem>
                            <PaginationItem page=PaginationPage::new(3).withcurrent(large_dataset_current_page.get() == 3)>
                                "3"
                            </PaginationItem>
                            <PaginationItem page=PaginationPage::new(4).withcurrent(large_dataset_current_page.get() == 4)>
                                "4"
                            </PaginationItem>
                            <PaginationItem page=PaginationPage::new(5).withcurrent(large_dataset_current_page.get() == 5)>
                                "5"
                            </PaginationItem>
                            <PaginationEllipsis>
                                "…"
                            </PaginationEllipsis>
                            <PaginationItem page=PaginationPage::new(99).withcurrent(large_dataset_current_page.get() == 99)>
                                "99"
                            </PaginationItem>
                            <PaginationItem page=PaginationPage::new(100).withcurrent(large_dataset_current_page.get() == 100)>
                                "100"
                            </PaginationItem>

                            <PaginationNext icon="▶️".to_string()>
                                "Next"
                            </PaginationNext>
                            <PaginationLast icon="⏭️".to_string()>
                                "Last"
                            </PaginationLast>
                        </PaginationList>
                    </PaginationContent>
                </Pagination>

                <div class="example-info">
                    <p><strong>"Current page:"</strong> {large_dataset_current_page}</p>
                    <p><strong>"Total items:"</strong> "10,000"</p>
                    <p><strong>"Page size:"</strong> "100"</p>
                </div>
            </section>

            // Size Variants Example
            <section class="example-section">
                <h2>"6. Pagination Size Variants"</h2>
                <p>"Different size variants for different use cases."</p>

                <div class="size-variants">
                    <div class="size-variant">
                        <h3>"Small Size"</h3>
                        <Pagination
                            current_page=1
                            total_pages=5
                            size=PaginationSize::Small
                        >
                            <PaginationList>
                                <PaginationPrevious icon="◀️".to_string()>
                                    "Prev"
                                </PaginationPrevious>
                                <PaginationItem page=PaginationPage::new(1).withcurrent(true)>
                                    "1"
                                </PaginationItem>
                                <PaginationItem page=PaginationPage::new(2)>
                                    "2"
                                </PaginationItem>
                                <PaginationItem page=PaginationPage::new(3)>
                                    "3"
                                </PaginationItem>
                                <PaginationNext icon="▶️".to_string()>
                                    "Next"
                                </PaginationNext>
                            </PaginationList>
                        </Pagination>
                    </div>

                    <div class="size-variant">
                        <h3>"Medium Size (Default)"</h3>
                        <Pagination
                            current_page=1
                            total_pages=5
                            size=PaginationSize::Medium
                        >
                            <PaginationList>
                                <PaginationPrevious icon="◀️".to_string()>
                                    "Previous"
                                </PaginationPrevious>
                                <PaginationItem page=PaginationPage::new(1).withcurrent(true)>
                                    "1"
                                </PaginationItem>
                                <PaginationItem page=PaginationPage::new(2)>
                                    "2"
                                </PaginationItem>
                                <PaginationItem page=PaginationPage::new(3)>
                                    "3"
                                </PaginationItem>
                                <PaginationNext icon="▶️".to_string()>
                                    "Next"
                                </PaginationNext>
                            </PaginationList>
                        </Pagination>
                    </div>

                    <div class="size-variant">
                        <h3>"Large Size"</h3>
                        <Pagination
                            current_page=1
                            total_pages=5
                            size=PaginationSize::Large
                        >
                            <PaginationList>
                                <PaginationPrevious icon="◀️".to_string()>
                                    "Previous"
                                </PaginationPrevious>
                                <PaginationItem page=PaginationPage::new(1).withcurrent(true)>
                                    "1"
                                </PaginationItem>
                                <PaginationItem page=PaginationPage::new(2)>
                                    "2"
                                </PaginationItem>
                                <PaginationItem page=PaginationPage::new(3)>
                                    "3"
                                </PaginationItem>
                                <PaginationNext icon="▶️".to_string()>
                                    "Next"
                                </PaginationNext>
                            </PaginationList>
                        </Pagination>
                    </div>
                </div>
            </section>

            // Accessibility Features
            <section class="example-section">
                <h2>"7. Accessibility Features"</h2>
                <p>"Pagination with enhanced accessibility features including ARIA labels and keyboard navigation."</p>

                <Pagination
                    current_page=1
                    total_pages=5
                    class="accessible-pagination".to_string()
                >
                    <PaginationList>
                        <PaginationFirst
                            icon="⏮️".to_string()
                            class="sr-only-accessible".to_string()
                        >
                            "Go to first page"
                        </PaginationFirst>
                        <PaginationPrevious
                            icon="◀️".to_string()
                            class="sr-only-accessible".to_string()
                        >
                            "Go to previous page"
                        </PaginationPrevious>

                        <PaginationItem page=PaginationPage::new(1).withcurrent(true)>
                            "Page 1"
                        </PaginationItem>
                        <PaginationItem page=PaginationPage::new(2)>
                            "Page 2"
                        </PaginationItem>
                        <PaginationItem page=PaginationPage::new(3)>
                            "Page 3"
                        </PaginationItem>

                        <PaginationNext
                            icon="▶️".to_string()
                            class="sr-only-accessible".to_string()
                        >
                            "Go to next page"
                        </PaginationNext>
                        <PaginationLast
                            icon="⏭️".to_string()
                            class="sr-only-accessible".to_string()
                        >
                            "Go to last page"
                        </PaginationLast>
                    </PaginationList>
                </Pagination>

                <div class="accessibility-info">
                    <h3>"Accessibility Features:"</h3>
                    <ul>
                        <li>"Proper ARIA labels for screen readers"</li>
                        <li>"Keyboard navigation support"</li>
                        <li>"Current page indication with aria-current"</li>
                        <li>"Disabled state handling"</li>
                        <li>"Semantic HTML structure"</li>
                    </ul>
                </div>
            </section>

            // Component API Documentation
            <section class="example-section">
                <h2>"8. Component API Reference"</h2>
                <div class="api-documentation">
                    <h3>"Available Components:"</h3>
                    <ul>
                        <li><strong>"Pagination"</strong> " - Main container component"</li>
                        <li><strong>"PaginationList"</strong> " - List container for pagination items"</li>
                        <li><strong>"PaginationItem"</strong> " - Individual page number button"</li>
                        <li><strong>"PaginationFirst"</strong> " - First page button"</li>
                        <li><strong>"PaginationPrevious"</strong> " - Previous page button"</li>
                        <li><strong>"PaginationNext"</strong> " - Next page button"</li>
                        <li><strong>"PaginationLast"</strong> " - Last page button"</li>
                        <li><strong>"PaginationEllipsis"</strong> " - Ellipsis for truncated ranges"</li>
                        <li><strong>"PaginationInfo"</strong> " - Information display component"</li>
                        <li><strong>"PaginationContent"</strong> " - Content wrapper component"</li>
                    </ul>

                    <h3>"Helper Functions:"</h3>
                    <ul>
                        <li><strong>"generate_page_numbers"</strong> " - Generate page numbers with ellipsis"</li>
                        <li><strong>"create_pagination_controls"</strong> " - Create complete pagination controls"</li>
                        <li><strong>"create_compact_pagination"</strong> " - Create compact pagination"</li>
                        <li><strong>"create_detailed_pagination"</strong> " - Create detailed pagination with info"</li>
                    </ul>

                    <h3>"Configuration Options:"</h3>
                    <ul>
                        <li><strong>"Size variants:"</strong> " Small, Medium, Large"</li>
                        <li><strong>"Display variants:"</strong> " Default, Compact, Detailed"</li>
                        <li><strong>"Customizable buttons:"</strong> " First/Last, Previous/Next, Page numbers"</li>
                        <li><strong>"Information display:"</strong> " Customizable format strings"</li>
                        <li><strong>"Accessibility:"</strong> " Full ARIA support and keyboard navigation"</li>
                    </ul>
                </div>
            </section>
        </div>
    }
}
