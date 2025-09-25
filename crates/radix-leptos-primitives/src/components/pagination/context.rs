use leptos::callback::Callback;
use leptos::prelude::*;
use crate::utils::generate_id;

/// Pagination page information
#[derive(Clone, Debug, PartialEq)]
pub struct PaginationPage {
    pub number: usize,
    pub label: Option<String>,
    pub disabled: bool,
    pub _current: bool,
}

impl PaginationPage {
    pub fn new(number: usize) -> Self {
        Self {
            number,
            label: None,
            disabled: false,
            _current: false,
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn withdisabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn withcurrent(mut self, current: bool) -> Self {
        self._current = current;
        self
    }
}

/// Pagination size
#[derive(Clone, Debug, PartialEq)]
pub enum PaginationSize {
    Small,
    Medium,
    Large,
}

impl PaginationSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            PaginationSize::Small => "small",
            PaginationSize::Medium => "medium",
            PaginationSize::Large => "large",
        }
    }
}

/// Pagination variant
#[derive(Clone, Debug, PartialEq)]
pub enum PaginationVariant {
    Default,
    Compact,
    Detailed,
}

impl PaginationVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            PaginationVariant::Default => "default",
            PaginationVariant::Compact => "compact",
            PaginationVariant::Detailed => "detailed",
        }
    }
}

/// Pagination context for state management
#[derive(Clone)]
pub struct PaginationContext {
    pub current_page: ReadSignal<usize>,
    pub _current_page_write: WriteSignal<usize>,
    pub total_pages: usize,
    pub page_size: usize,
    pub total_items: usize,
    pub size: PaginationSize,
    pub variant: PaginationVariant,
    pub _show_first_last: bool,
    pub _show_prev_next: bool,
    pub _show_page_numbers: bool,
    pub pagination_id: String,
    pub on_page_change: Option<Callback<usize>>,
}

impl PaginationContext {
    pub fn new(
        current_page: usize,
        total_pages: usize,
        page_size: usize,
        total_items: usize,
        size: PaginationSize,
        variant: PaginationVariant,
        on_page_change: Option<Callback<usize>>,
    ) -> Self {
        let (read_signal, write_signal) = signal(current_page);
        Self {
            current_page: read_signal,
            _current_page_write: write_signal,
            total_pages,
            page_size,
            total_items,
            size,
            variant,
            _show_first_last: true,
            _show_prev_next: true,
            _show_page_numbers: true,
            pagination_id: generate_id("pagination"),
            on_page_change,
        }
    }

    pub fn can_go_previous(&self) -> bool {
        self.current_page.get() > 1
    }

    pub fn can_go_next(&self) -> bool {
        self.current_page.get() < self.total_pages
    }

    pub fn go_to_page(&self, page: usize) {
        if page >= 1 && page <= self.total_pages && page != self.current_page.get() {
            self._current_page_write.set(page);
            if let Some(callback) = &self.on_page_change {
                callback.run(page);
            }
        }
    }

    pub fn go_to_previous(&self) {
        if self.can_go_previous() {
            self.go_to_page(self.current_page.get() - 1);
        }
    }

    pub fn go_to_next(&self) {
        if self.can_go_next() {
            self.go_to_page(self.current_page.get() + 1);
        }
    }

    pub fn go_to_first(&self) {
        self.go_to_page(1);
    }

    pub fn go_to_last(&self) {
        self.go_to_page(self.total_pages);
    }
}

/// Pagination provider component for context management
#[component]
pub fn PaginationProvider(
    #[prop(optional, default = 1)] current_page: usize,
    #[prop(optional, default = 1)] total_pages: usize,
    #[prop(optional, default = 10)] page_size: usize,
    #[prop(optional, default = 0)] total_items: usize,
    #[prop(optional, default = PaginationSize::Medium)] size: PaginationSize,
    #[prop(optional, default = PaginationVariant::Default)] variant: PaginationVariant,
    #[prop(optional)] on_page_change: Option<Callback<usize>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = PaginationContext::new(
        current_page,
        total_pages,
        page_size,
        total_items,
        size,
        variant,
        on_page_change,
    );

    provide_context(context);

    view! {
        <div class="pagination-provider">
            {children.map(|c| c())}
        </div>
    }
}

/// Hook to use pagination context
pub fn use_pagination_context() -> PaginationContext {
    use_context::<PaginationContext>()
        .expect("PaginationProvider must be used within a PaginationProvider")
}

#[cfg(test)]
mod context_tests {
    use super::*;
use crate::utils::{merge_optional_classes, generate_id};

    #[test]
    fn test_pagination_page() {
        let page = PaginationPage::new(1)
            .with_label("First".to_string())
            .withcurrent(true)
            .withdisabled(false);

        assert_eq!(page.number, 1);
        assert_eq!(page.label, Some("First".to_string()));
        assert_eq!(page._current, true);
        assert_eq!(page.disabled, false);
    }

    #[test]
    fn test_pagination_size() {
        assert_eq!(PaginationSize::Small.as_str(), "small");
        assert_eq!(PaginationSize::Medium.as_str(), "medium");
        assert_eq!(PaginationSize::Large.as_str(), "large");
    }

    #[test]
    fn test_pagination_variant() {
        assert_eq!(PaginationVariant::Default.as_str(), "default");
        assert_eq!(PaginationVariant::Compact.as_str(), "compact");
        assert_eq!(PaginationVariant::Detailed.as_str(), "detailed");
    }

    #[test]
    fn test_pagination_context() {
        let context = PaginationContext::new(
            1,
            10,
            10,
            100,
            PaginationSize::Medium,
            PaginationVariant::Default,
            None,
        );

        assert_eq!(context.current_page.get(), 1);
        assert_eq!(context.total_pages, 10);
        assert_eq!(context.page_size, 10);
        assert_eq!(context.total_items, 100);
        assert!(context.can_go_next());
        assert!(!context.can_go_previous());
    }

    #[test]
    fn test_pagination_context_navigation() {
        let context = PaginationContext::new(
            5,
            10,
            10,
            100,
            PaginationSize::Medium,
            PaginationVariant::Default,
            None,
        );

        assert!(context.can_go_previous());
        assert!(context.can_go_next());

        context.go_to_previous();
        assert_eq!(context.current_page.get(), 4);

        context.go_to_next();
        assert_eq!(context.current_page.get(), 5);

        context.go_to_first();
        assert_eq!(context.current_page.get(), 1);

        context.go_to_last();
        assert_eq!(context.current_page.get(), 10);
    }
}
