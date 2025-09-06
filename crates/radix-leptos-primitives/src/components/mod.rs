//! # Component Primitives
//!
//! Individual component implementations

// Component modules
pub mod accordion;
pub mod alert;
pub mod badge;
pub mod button;
pub mod checkbox;
pub mod dialog;
pub mod form;
pub mod progress;
pub mod radio_group;
pub mod select;
pub mod slider;
pub mod switch;
pub mod tooltip;
// pub mod data_table;  // Temporarily disabled due to view! macro type issues
// pub mod date_picker;  // Temporarily disabled due to view! macro type issues
pub mod dropdown_menu;
pub mod hover_card;
pub mod menubar;
pub mod navigation_menu;
pub mod popover;
pub mod scroll_area;
pub mod toggle;
pub mod toggle_group;
pub mod toolbar;
// pub mod chart;
// pub mod data_table;
// pub mod virtual_list;
// pub mod split_pane;
// pub mod line_chart;
// pub mod bar_chart;
// pub mod pie_chart;
// pub mod scatter_plot;
// pub mod drag_drop;
// pub mod rich_text_editor;
// pub mod color_picker;
// pub mod image_viewer;
// pub mod code_editor;
pub mod timeline;
// pub mod gauge;
// pub mod command_palette;
// pub mod touch_button;
// pub mod swipe_gestures;
// pub mod pull_to_refresh;
pub mod aspect_ratio;
pub mod avatar;
pub mod calendar;
pub mod collapsible;
pub mod combobox;
pub mod context_menu;
pub mod date_picker;
pub mod file_upload;
pub mod label;
pub mod list;
pub mod multi_select;
pub mod otp_field;
pub mod pagination;
pub mod password_toggle_field;
pub mod resizable;
pub mod search;
pub mod separator;
pub mod tabs;
pub mod toast;
pub mod tree_view;
// pub mod infinite_scroll;
// pub mod lazyloading;
// pub mod lazyloading_optimized;
pub mod alert_dialog;
pub mod sheet;
pub mod skeleton;
pub mod time_picker; // TDD: GREEN phase - enabling component
                     // pub mod range_slider;  // TDD: Need to fix tests first
                     // pub mod form_validation;  // TDD: Need to fix tests first

// Test modules - temporarily disabled
// #[cfg(test)]
// mod alert_dialog_tests;
// #[cfg(test)]
// mod sheet_tests;
// #[cfg(test)]
// mod skeleton_tests;

// Re-export components
pub use accordion::*;
pub use alert::*;
pub use badge::*;
pub use button::*;
pub use checkbox::*;
pub use dialog::*;
pub use form::*;
pub use progress::*;
pub use radio_group::*;
pub use select::*;
pub use slider::*;
pub use switch::*;
pub use tooltip::*;
// pub use data_table::*;  // Temporarily disabled
pub use date_picker::*; // Temporarily disabled
pub use dropdown_menu::*;
pub use hover_card::*;
pub use list::*;
pub use menubar::*;
pub use navigation_menu::*;
pub use pagination::*;
pub use popover::*;
pub use scroll_area::*;
pub use timeline::*;
pub use toast::*;
pub use toggle::*;
pub use toggle_group::*;
pub use toolbar::*;
// pub use chart::*;
// pub use data_table::*;
// pub use virtual_list::*;
// pub use split_pane::*;
// pub use line_chart::*;
// pub use bar_chart::*;
// pub use pie_chart::*;
// pub use scatter_plot::*;
// pub use drag_drop::*;
// pub use rich_text_editor::*;
// pub use color_picker::*;
// pub use image_viewer::*;
// pub use code_editor::*;
// pub use timeline::*;
// pub use gauge::*;
// pub use command_palette::*;
// pub use touch_button::*;
// pub use swipe_gestures::*;
// pub use pull_to_refresh::*;
// pub use list::*;
// pub use pagination::*;
pub use tabs::*;
// pub use timeline::*;
// pub use toast::*;
pub use aspect_ratio::*;
pub use avatar::*;
pub use calendar::*;
pub use collapsible::*;
pub use combobox::*;
pub use context_menu::*;
pub use file_upload::*;
pub use label::*;
pub use multi_select::*;
pub use otp_field::*;
pub use password_toggle_field::*;
pub use resizable::*;
pub use search::*;
pub use separator::*;
pub use tree_view::*;
// pub use infinite_scroll::*;
// pub use lazyloading::*;
// pub use lazyloading_optimized::*;
pub use alert_dialog::*;
pub use sheet::*;
pub use skeleton::*;
pub use time_picker::*; // TDD: GREEN phase - enabling component
                        // pub use range_slider::*;  // TDD: Need to fix tests first
                        // pub use form_validation::*;  // TDD: Need to fix tests first
