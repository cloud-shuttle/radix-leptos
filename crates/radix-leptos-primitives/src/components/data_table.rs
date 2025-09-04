use leptos::*;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Column definition for data table
pub struct TableColumn<T> {
    pub key: String,
    pub header: String,
    pub sortable: bool,
    pub filterable: bool,
    pub width: Option<String>,
    pub render: Option<Box<dyn Fn(&T) -> String + Send + Sync>>,
}

impl<T> TableColumn<T> {
    pub fn new(key: String, header: String) -> Self {
        Self {
            key,
            header,
            sortable: true,
            filterable: true,
            width: None,
            render: None,
        }
    }

    pub fn with_sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    pub fn with_filterable(mut self, filterable: bool) -> Self {
        self.filterable = filterable;
        self
    }

    pub fn with_width(mut self, width: String) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_render<F>(mut self, render: F) -> Self
    where
        F: Fn(&T) -> String + Send + Sync + 'static,
    {
        self.render = Some(Box::new(render));
        self
    }
}

impl<T> Clone for TableColumn<T> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            header: self.header.clone(),
            sortable: self.sortable,
            filterable: self.filterable,
            width: self.width.clone(),
            render: None, // Can't clone the render function, so we'll set it to None
        }
    }
}

/// Sort direction for table columns
#[derive(Clone, Debug, PartialEq)]
pub enum SortDirection {
    None,
    Ascending,
    Descending,
}

impl SortDirection {
    pub fn next(&self) -> Self {
        match self {
            SortDirection::None => SortDirection::Ascending,
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::None,
        }
    }
}

/// Data table component with sorting and filtering
#[component]
pub fn DataTable<T>(
    /// Table columns configuration
    #[prop(into)]
    columns: Vec<TableColumn<T>>,
    /// Table data
    #[prop(into)]
    data: Vec<T>,
    /// Whether to show search/filter input
    #[prop(optional, default = true)]
    searchable: bool,
    /// Whether to show pagination
    #[prop(optional, default = true)]
    paginated: bool,
    /// Page size for pagination
    #[prop(optional, default = 10)]
    page_size: usize,
    /// CSS classes to apply
    #[prop(optional)]
    class: Option<String>,
    /// Callback when data changes (filtered/sorted)
    #[prop(optional)]
    on_data_change: Option<Callback<Vec<T>>>,
) -> impl IntoView
where
    T: Clone + Send + Sync + std::fmt::Debug + 'static,
{
    let (search_query, set_search_query) = signal(String::new());
    let (sort_column, set_sort_column) = signal(String::new());
    let (sort_direction, set_sort_direction) = signal(SortDirection::None);
    let (current_page, set_current_page) = signal(1);
    let (filters, set_filters) = signal(std::collections::HashMap::<String, String>::new());

    // Filter and sort data
    let processed_data = move || {
        let mut filtered = data.clone();
        
        // Apply search filter
        let query = search_query.get();
        if !query.is_empty() {
            filtered.retain(|item| {
                columns.iter().any(|col| {
                    if let Some(render) = &col.render {
                        render(item).to_lowercase().contains(&query.to_lowercase())
                    } else {
                        // For now, we'll need a way to access item fields
                        // This is a simplified version
                        true
                    }
                })
            });
        }

        // Apply column filters
        for (col_key, filter_value) in filters.get() {
            if !filter_value.is_empty() {
                filtered.retain(|item| {
                    // Simplified filtering - in a real implementation,
                    // you'd need reflection or a trait to access item fields
                    true
                });
            }
        }

        // Apply sorting
        if !sort_column.get().is_empty() && sort_direction.get() != SortDirection::None {
            // Simplified sorting - in a real implementation,
            // you'd need reflection or a trait to access item fields
            // For now, we'll just return the filtered data
        }

        filtered
    };

    // Paginate data
    let paginated_data = move || {
        let processed = processed_data();
        let start = (current_page.get() - 1) * page_size;
        let end = start + page_size;
        processed.into_iter().skip(start).take(page_size).collect::<Vec<_>>()
    };

    // Total pages
    let total_pages = move || {
        let total = processed_data().len();
        (total + page_size - 1) / page_size
    };
    
    // Get total pages for current state - avoid moving the closure
    let get_total_pages = move || {
        let total = processed_data().len();
        (total + page_size - 1) / page_size
    };

    // Handle column sorting
    let handle_sort = move |column_key: String| {
        if sort_column.get() == column_key {
            let new_direction = sort_direction.get().next();
            set_sort_direction.set(new_direction);
            if new_direction == SortDirection::None {
                set_sort_column.set(String::new());
            }
        } else {
            set_sort_column.set(column_key);
            set_sort_direction.set(SortDirection::Ascending);
        }
        set_current_page.set(1); // Reset to first page when sorting
    };

    // Handle search
    let handle_search = move |event: web_sys::Event| {
        let target = event.target().unwrap().unchecked_ref::<web_sys::HtmlInputElement>();
        set_search_query.set(target.value());
        set_current_page.set(1); // Reset to first page when searching
    };

    // Handle page change
    let handle_page_change = move |page: usize| {
        set_current_page.set(page);
    };

    // Handle filter change
    let handle_filter_change = move |column_key: String, value: String| {
        let mut current_filters = filters.get();
        if value.is_empty() {
            current_filters.remove(&column_key);
        } else {
            current_filters.insert(column_key, value);
        }
        set_filters.set(current_filters);
        set_current_page.set(1); // Reset to first page when filtering
    };

    // Notify parent of data changes
    Effect::new(move |_| {
        let processed = processed_data();
        if let Some(callback) = on_data_change {
            callback.run(processed);
        }
    });

    view! {
        <div class={format!("data-table {}", class.unwrap_or_default())}>
            // Search and filters
            <Show when=move || searchable>
                <div class="table-controls">
                    <div class="table-search">
                        <input
                            type="text"
                            class="search-input"
                            placeholder="Search all columns..."
                            value=move || search_query.get()
                            on:input=handle_search
                        />
                    </div>
                    
                    <div class="table-filters">
                        {columns.iter().filter(|col| col.filterable).map(|col| {
                            let col_key = col.key.clone();
                            let filter_value = move || filters.get().get(&col_key).cloned().unwrap_or_default();
                            
                            view! {
                                <div class="table-filter">
                                    <label class="filter-label">{col.header.clone()}</label>
                                    <input
                                        type="text"
                                        class="filter-input"
                                        placeholder="Filter..."
                                        value=filter_value()
                                        on:input=move |event| {
                                            let target = event.target().unwrap().unchecked_ref::<web_sys::HtmlInputElement>();
                                            handle_filter_change(col_key.clone(), target.value());
                                        }
                                    />
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </Show>

            // Table
            <div class="table-container">
                <table class="data-table-table">
                    <thead>
                        <tr>
                            {columns.iter().map(|col| {
                                let col_key = col.key.clone();
                                let is_sorted = move || sort_column.get() == col_key;
                                let sort_direction_class = move || {
                                    if sort_column.get() == col_key {
                                        match sort_direction.get() {
                                            SortDirection::Ascending => "sort-asc",
                                            SortDirection::Descending => "sort-desc",
                                            SortDirection::None => "",
                                        }
                                    } else {
                                        ""
                                    }
                                };
                                
                                view! {
                                    <th
                                        class={format!(
                                            "table-header {} {}",
                                            if col.sortable { "sortable" } else { "" },
                                            sort_direction_class()
                                        )}
                                        style={col.width.as_ref().map(|w| format!("width: {}", w))}
                                        on:click=move |_| {
                                            if col.sortable {
                                                handle_sort(col_key.clone());
                                            }
                                        }
                                    >
                                        <span class="header-content">{col.header.clone()}</span>
                                        <Show when=move || col.sortable>
                                            <span class="sort-indicator">
                                                {if is_sorted() { "▼" } else { "▽" }}
                                            </span>
                                        </Show>
                                    </th>
                                }
                            }).collect::<Vec<_>>()}
                        </tr>
                    </thead>
                    <tbody>
                        {move || paginated_data().into_iter().map(|item| {
                            view! {
                                <tr class="table-row">
                                    {columns.iter().map(|col| {
                                        let cell_content = if let Some(render) = &col.render {
                                            render(&item)
                                        } else {
                                            // Simplified - in real implementation, you'd access item fields
                                            format!("{:?}", item)
                                        };
                                        
                                        view! {
                                            <td class="table-cell">
                                                {cell_content}
                                            </td>
                                        }
                                    }).collect::<Vec<_>>()}
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>

            // Pagination - simplified to avoid type issues
            {move || {
                let total = processed_data().len();
                let total_pages = (total + page_size - 1) / page_size;
                
                if paginated && total_pages > 1 {
                    view! {
                        <div class="table-pagination">
                            <div class="pagination-info">
                                {format!(
                                    "Showing {} to {} of {} entries",
                                    (current_page.get() - 1) * page_size + 1,
                                    (current_page.get() * page_size).min(processed_data().len()),
                                    processed_data().len()
                                )}
                            </div>
                            
                            <div class="pagination-controls">
                                <button
                                    type="button"
                                    class="pagination-button"
                                    disabled=move || current_page.get() == 1
                                    on:click=move |_| handle_page_change(current_page.get() - 1)
                                >
                                    "Previous"
                                </button>
                                
                                {(1..=total_pages).map(|page| {
                                    let page_num = page;
                                    let is_current = move || current_page.get() == page_num;
                                    
                                    view! {
                                        <button
                                            type="button"
                                            class={format!(
                                                "pagination-button {}",
                                                if is_current() { "current" } else { "" }
                                            )}
                                            on:click=move |_| handle_page_change(page_num)
                                        >
                                            {page_num}
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                                
                                <button
                                    type="button"
                                    class="pagination-button"
                                    disabled=move || current_page.get() == total_pages
                                    on:click=move |_| handle_page_change(current_page.get() + 1)
                                >
                                    "Next"
                                </button>
                            </div>
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}
        </div>
    }
}

/// Simplified data table for basic use cases
#[component]
pub fn SimpleDataTable(
    /// Table headers
    #[prop(into)]
    headers: Vec<String>,
    /// Table rows (each row is a vector of strings)
    #[prop(into)]
    rows: Vec<Vec<String>>,
    /// Whether to show search
    #[prop(optional, default = true)]
    searchable: bool,
    /// Whether to show pagination
    #[prop(optional, default = true)]
    paginated: bool,
    /// Page size
    #[prop(optional, default = 10)]
    page_size: usize,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let columns = move || {
        headers
            .iter()
            .enumerate()
            .map(|(i, header)| {
                TableColumn::new(i.to_string(), header.clone())
                    .with_render(move |row: &Vec<String>| {
                        row.get(i).cloned().unwrap_or_default()
                    })
            })
            .collect::<Vec<_>>()
    };

    let data = move || rows.clone();

    view! {
        <DataTable
            columns=columns()
            data=data()
            searchable=searchable
            paginated=paginated
            page_size=page_size
            class=class.unwrap_or_default()
        />
    }
}
