use leptos::*;
use leptos::prelude::*;

/// Table data structure
#[derive(Clone, Debug, PartialEq)]
pub struct TableData {
    pub id: String,
    pub cells: Vec<String>,
    pub selected: bool,
    pub disabled: bool,
}

impl TableData {
    pub fn new(id: String, cells: Vec<String>) -> Self {
        Self {
            id,
            cells,
            selected: false,
            disabled: false,
        }
    }

    pub fn with_selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Table column structure
#[derive(Clone, Debug, PartialEq)]
pub struct TableColumn {
    pub id: String,
    pub header: String,
    pub sortable: bool,
    pub width: Option<String>,
    pub align: TableAlign,
}

impl TableColumn {
    pub fn new(id: String, header: String) -> Self {
        Self {
            id,
            header,
            sortable: false,
            width: None,
            align: TableAlign::Left,
        }
    }

    pub fn with_sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    pub fn with_width(mut self, width: String) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_align(mut self, align: TableAlign) -> Self {
        self.align = align;
        self
    }
}

/// Table alignment options
#[derive(Clone, Debug, PartialEq)]
pub enum TableAlign {
    Left,
    Center,
    Right,
}

impl TableAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            TableAlign::Left => "left",
            TableAlign::Center => "center",
            TableAlign::Right => "right",
        }
    }
}

/// Table size options
#[derive(Clone, Debug, PartialEq)]
pub enum TableSize {
    Small,
    Medium,
    Large,
}

impl TableSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            TableSize::Small => "small",
            TableSize::Medium => "medium",
            TableSize::Large => "large",
        }
    }
}

/// Table variant options
#[derive(Clone, Debug, PartialEq)]
pub enum TableVariant {
    Default,
    Bordered,
    Striped,
    Compact,
}

impl TableVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            TableVariant::Default => "default",
            TableVariant::Bordered => "bordered",
            TableVariant::Striped => "striped",
            TableVariant::Compact => "compact",
        }
    }
}

/// Sort direction
#[derive(Clone, Debug, PartialEq)]
pub enum SortDirection {
    None,
    Ascending,
    Descending,
}

impl SortDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortDirection::None => "none",
            SortDirection::Ascending => "ascending",
            SortDirection::Descending => "descending",
        }
    }
}

/// Table context for state management
#[derive(Clone)]
pub struct TableContext {
    pub columns: Signal<Vec<TableColumn>>,
    pub data: Signal<Vec<TableData>>,
    pub sort_column: Signal<Option<String>>,
    pub sort_direction: Signal<SortDirection>,
    pub current_page: Signal<usize>,
    pub page_size: Signal<usize>,
    pub total_pages: Signal<usize>,
    pub size: TableSize,
    pub variant: TableVariant,
    pub selectable: bool,
    pub multi_select: bool,
    pub selected_rows: Signal<Vec<String>>,
    pub table_id: String,
    pub on_sort: Option<Callback<(String, SortDirection)>>,
    pub on_row_select: Option<Callback<TableData>>,
    pub on_page_change: Option<Callback<usize>>,
}

/// Table component for displaying tabular data
#[component]
pub fn Table(
    /// Table columns configuration
    #[prop(optional)]
    columns: Vec<TableColumn>,
    /// Table data rows
    #[prop(optional)]
    data: Vec<TableData>,
    /// Current sort column
    #[prop(optional)]
    sort_column: Option<String>,
    /// Current sort direction
    #[prop(optional, default = SortDirection::None)]
    sort_direction: SortDirection,
    /// Current page (1-based)
    #[prop(optional, default = 1)]
    current_page: usize,
    /// Number of rows per page
    #[prop(optional, default = 10)]
    page_size: usize,
    /// Table size
    #[prop(optional, default = TableSize::Medium)]
    size: TableSize,
    /// Table variant
    #[prop(optional, default = TableVariant::Default)]
    variant: TableVariant,
    /// Whether rows are selectable
    #[prop(optional, default = false)]
    selectable: bool,
    /// Whether multiple rows can be selected
    #[prop(optional, default = false)]
    multi_select: bool,
    /// Sort event handler
    #[prop(optional)]
    on_sort: Option<Callback<(String, SortDirection)>>,
    /// Row selection event handler
    #[prop(optional)]
    on_row_select: Option<Callback<TableData>>,
    /// Page change event handler
    #[prop(optional)]
    on_page_change: Option<Callback<usize>>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child content (table headers, rows, etc.)
    children: Children,
) -> impl IntoView {
    let table_id = generate_id("table");
    
    // Reactive state
    let (columns_signal, _set_columns_signal) = signal(columns);
    let (data_signal, _set_data_signal) = signal(data);
    let (sort_column_signal, _set_sort_column_signal) = signal(sort_column);
    let (sort_direction_signal, _set_sort_direction_signal) = signal(sort_direction);
    let (current_page_signal, _set_current_page_signal) = signal(current_page);
    let (page_size_signal, _set_page_size_signal) = signal(page_size);
    let (selected_rows_signal, _set_selected_rows_signal) = signal(Vec::<String>::new());
    
    // Calculate total pages
    let total_pages = Memo::new(move |_| {
        let data_len = data_signal.get().len();
        if data_len == 0 {
            1
        } else {
            (data_len + page_size_signal.get() - 1) / page_size_signal.get()
        }
    });
    
    // Create context
    let context = TableContext {
        columns: columns_signal.into(),
        data: data_signal.into(),
        sort_column: sort_column_signal.into(),
        sort_direction: sort_direction_signal.into(),
        current_page: current_page_signal.into(),
        page_size: page_size_signal.into(),
        total_pages: total_pages.into(),
        size: size.clone(),
        variant: variant.clone(),
        selectable,
        multi_select,
        selected_rows: selected_rows_signal.into(),
        table_id: table_id.clone(),
        on_sort: on_sort.clone(),
        on_row_select: on_row_select.clone(),
        on_page_change: on_page_change.clone(),
    };
    
    // Build base classes
    let base_classes = "radix-table";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Provide the context
    provide_context(context);
    
    view! {
        <div
            id=table_id
            class=combined_class
            data-size=size.as_str()
            data-variant=variant.as_str()
            data-selectable=selectable
            data-multi-select=multi_select
            data-current-page=current_page_signal.get()
            data-total-pages=total_pages.get()
            data-page-size=page_size_signal.get()
            role="table"
            aria-label="Data table"
        >
            {children()}
        </div>
    }
}

/// TableHeader component for the table header section
#[component]
pub fn TableHeader(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (table header rows)
    children: Children,
) -> impl IntoView {
    let _context = use_context::<TableContext>().expect("TableHeader must be used within Table");
    let header_id = generate_id("table-header");
    
    // Build base classes
    let base_classes = "radix-table-header";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <thead
            id=header_id
            class=combined_class
            style=style.unwrap_or_default()
        >
            {children()}
        </thead>
    }
}

/// TableBody component for the table body section
#[component]
pub fn TableBody(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (table data rows)
    children: Children,
) -> impl IntoView {
    let _context = use_context::<TableContext>().expect("TableBody must be used within Table");
    let body_id = generate_id("table-body");
    
    // Build base classes
    let base_classes = "radix-table-body";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <tbody
            id=body_id
            class=combined_class
            style=style.unwrap_or_default()
        >
            {children()}
        </tbody>
    }
}

/// TableRow component for individual table rows
#[component]
pub fn TableRow(
    /// The table row data this component represents
    #[prop(optional)]
    row: Option<TableData>,
    /// Whether this row is selected
    #[prop(optional)]
    selected: Option<bool>,
    /// Whether this row is disabled
    #[prop(optional)]
    disabled: Option<bool>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (table cells)
    children: Children,
) -> impl IntoView {
    let context = use_context::<TableContext>().expect("TableRow must be used within Table");
    let row_id = generate_id("table-row");
    
    let row_for_click = row.clone();
    let row_for_selected = row.clone();
    let row_for_disabled = row.clone();
    
    let handle_click = move |event: web_sys::MouseEvent| {
        if let Some(row) = row_for_click.clone() {
            if !row.disabled {
                // Call the row selection handler
                if let Some(callback) = context.on_row_select.clone() {
                    callback.run(row);
                }
            } else {
                event.prevent_default();
            }
        }
    };
    
    // Determine if this row is selected
    let is_selected = Memo::new(move |_| {
        if let Some(selected) = selected {
            selected
        } else if let Some(row) = row_for_selected.as_ref() {
            row.selected
        } else {
            false
        }
    });
    
    // Determine if this row is disabled
    let is_disabled = Memo::new(move |_| {
        if let Some(disabled) = disabled {
            disabled
        } else if let Some(row) = row_for_disabled.as_ref() {
            row.disabled
        } else {
            false
        }
    });
    
    // Build base classes
    let base_classes = "radix-table-row";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <tr
            id=row_id
            class=combined_class
            style=style.unwrap_or_default()
            data-selected=is_selected.get()
            data-disabled=is_disabled.get()
            role="row"
            aria-selected=is_selected.get()
            aria-disabled=is_disabled.get()
            on:click=handle_click
        >
            {children()}
        </tr>
    }
}

/// TableHeaderCell component for table header cells
#[component]
pub fn TableHeaderCell(
    /// Cell content
    #[prop(optional)]
    content: Option<String>,
    /// Column alignment
    #[prop(optional)]
    align: Option<TableAlign>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let _context = use_context::<TableContext>().expect("TableHeaderCell must be used within Table");
    let cell_id = generate_id("table-header-cell");
    
    // Build base classes
    let base_classes = "radix-table-header-cell";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <th
            id=cell_id
            class=combined_class
            style=style.unwrap_or_default()
            data-align=align.map(|a| a.as_str()).unwrap_or("left")
            role="columnheader"
        >
            {content.map(|c| view! { <span>{c}</span> })}
            {children()}
        </th>
    }
}

/// TableCell component for individual table data cells
#[component]
pub fn TableCell(
    /// Cell content
    #[prop(optional)]
    content: Option<String>,
    /// Column alignment
    #[prop(optional)]
    align: Option<TableAlign>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let _context = use_context::<TableContext>().expect("TableCell must be used within Table");
    let cell_id = generate_id("table-cell");
    
    // Build base classes
    let base_classes = "radix-table-cell";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <td
            id=cell_id
            class=combined_class
            style=style.unwrap_or_default()
            data-align=align.map(|a| a.as_str()).unwrap_or("left")
            role="cell"
        >
            {content.map(|c| view! { <span>{c}</span> })}
            {children()}
        </td>
    }
}

/// TableSortButton component for sortable column headers
#[component]
pub fn TableSortButton(
    /// Column ID to sort by
    column_id: String,
    /// Column header text
    header_text: String,
    /// Current sort direction for this column
    #[prop(optional)]
    sort_direction: Option<SortDirection>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<TableContext>().expect("TableSortButton must be used within Table");
    let button_id = generate_id("table-sort-button");
    
    let column_id_clone = column_id.clone();
    let header_text_clone = header_text.clone();
    
    let handle_click = move |_event: web_sys::MouseEvent| {
        let current_direction = context.sort_direction.get();
        let current_column = context.sort_column.get();
        
        let new_direction = if current_column.as_ref() == Some(&column_id_clone) {
            match current_direction {
                SortDirection::None => SortDirection::Ascending,
                SortDirection::Ascending => SortDirection::Descending,
                SortDirection::Descending => SortDirection::Ascending,
            }
        } else {
            SortDirection::Ascending
        };
        
        // Call the sort handler
        if let Some(callback) = context.on_sort.clone() {
            callback.run((column_id_clone.clone(), new_direction));
        }
    };
    
    // Build base classes
    let base_classes = "radix-table-sort-button";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    let current_sort_direction = context.sort_direction.get();
    let current_sort_column = context.sort_column.get();
    let is_current_column = current_sort_column.as_ref() == Some(&column_id);
    
    view! {
        <button
            id=button_id
            class=combined_class
            style=style.unwrap_or_default()
            data-sort-direction=if is_current_column { current_sort_direction.as_str() } else { "none" }
            type="button"
            role="button"
            aria-label=format!("Sort by {}", header_text_clone)
            aria-sort=if is_current_column { current_sort_direction.as_str() } else { "none" }
            on:click=handle_click
        >
            <span class="radix-table-sort-text">{header_text}</span>
            <span class="radix-table-sort-icon">
                {if is_current_column {
                    match current_sort_direction {
                        SortDirection::Ascending => "↑",
                        SortDirection::Descending => "↓",
                        SortDirection::None => "↕",
                    }
                } else {
                    "↕"
                }}
            </span>
            {children()}
        </button>
    }
}

/// TablePagination component for table pagination
#[component]
pub fn TablePagination(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (pagination controls)
    children: Children,
) -> impl IntoView {
    let context = use_context::<TableContext>().expect("TablePagination must be used within Table");
    let pagination_id = generate_id("table-pagination");
    
    // Build base classes
    let base_classes = "radix-table-pagination";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=pagination_id
            class=combined_class
            style=style.unwrap_or_default()
            data-current-page=context.current_page.get()
            data-total-pages=context.total_pages.get()
            data-page-size=context.page_size.get()
            role="navigation"
            aria-label="Table pagination"
        >
            {children()}
        </div>
    }
}

/// TableInfo component for displaying table information
#[component]
pub fn TableInfo(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<TableContext>().expect("TableInfo must be used within Table");
    let info_id = generate_id("table-info");
    
    // Build base classes
    let base_classes = "radix-table-info";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    let current_page = context.current_page.get();
    let total_pages = context.total_pages.get();
    let page_size = context.page_size.get();
    let total_items = context.data.get().len();
    let start_item = if total_items == 0 { 0 } else { (current_page - 1) * page_size + 1 };
    let end_item = std::cmp::min(current_page * page_size, total_items);
    
    view! {
        <div
            id=info_id
            class=combined_class
            style=style.unwrap_or_default()
            role="status"
            aria-live="polite"
        >
            <span class="radix-table-info-text">
                "Showing " {start_item} " to " {end_item} " of " {total_items} " entries"
            </span>
            {children()}
        </div>
    }
}

/// Helper function to generate unique IDs
fn generate_id(prefix: &str) -> String {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}-{}", prefix, id)
}

/// Helper function to merge CSS classes
fn merge_classes(base: Option<&str>, additional: Option<&str>) -> Option<String> {
    match (base, additional) {
        (Some(base), Some(additional)) => Some(format!("{} {}", base, additional)),
        (Some(base), None) => Some(base.to_string()),
        (None, Some(additional)) => Some(additional.to_string()),
        (None, None) => None,
    }
}
