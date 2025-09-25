#!/bin/bash

# Fix unused generate_id imports
echo "Fixing unused generate_id imports..."

# List of files with unused generate_id imports
files=(
    "crates/radix-leptos-primitives/src/components/alert_dialog.rs"
    "crates/radix-leptos-primitives/src/components/aspect_ratio.rs"
    "crates/radix-leptos-primitives/src/components/avatar.rs"
    "crates/radix-leptos-primitives/src/components/bar_chart.rs"
    "crates/radix-leptos-primitives/src/components/calendar.rs"
    "crates/radix-leptos-primitives/src/components/chart.rs"
    "crates/radix-leptos-primitives/src/components/code_editor.rs"
    "crates/radix-leptos-primitives/src/components/collapsible.rs"
    "crates/radix-leptos-primitives/src/components/color_picker.rs"
    "crates/radix-leptos-primitives/src/components/combobox.rs"
    "crates/radix-leptos-primitives/src/components/command_palette.rs"
    "crates/radix-leptos-primitives/src/components/context_menu.rs"
    "crates/radix-leptos-primitives/src/components/data_table.rs"
    "crates/radix-leptos-primitives/src/components/date_picker.rs"
    "crates/radix-leptos-primitives/src/components/drag_drop.rs"
    "crates/radix-leptos-primitives/src/components/file_upload.rs"
    "crates/radix-leptos-primitives/src/components/gauge.rs"
    "crates/radix-leptos-primitives/src/components/hover_card.rs"
    "crates/radix-leptos-primitives/src/components/image_viewer.rs"
    "crates/radix-leptos-primitives/src/components/infinite_scroll.rs"
    "crates/radix-leptos-primitives/src/components/label.rs"
    "crates/radix-leptos-primitives/src/components/lazy_loading_optimized.rs"
    "crates/radix-leptos-primitives/src/components/lazy_loading.rs"
    "crates/radix-leptos-primitives/src/components/line_chart.rs"
    "crates/radix-leptos-primitives/src/components/menubar.rs"
    "crates/radix-leptos-primitives/src/components/multi_select.rs"
    "crates/radix-leptos-primitives/src/components/navigation_menu.rs"
    "crates/radix-leptos-primitives/src/components/otp_field.rs"
    "crates/radix-leptos-primitives/src/components/password_toggle_field.rs"
    "crates/radix-leptos-primitives/src/components/pie_chart.rs"
    "crates/radix-leptos-primitives/src/components/popover.rs"
    "crates/radix-leptos-primitives/src/components/pull_to_refresh.rs"
    "crates/radix-leptos-primitives/src/components/resizable.rs"
    "crates/radix-leptos-primitives/src/components/rich_text_editor.rs"
    "crates/radix-leptos-primitives/src/components/scatter_plot.rs"
    "crates/radix-leptos-primitives/src/components/scroll_area.rs"
    "crates/radix-leptos-primitives/src/components/search.rs"
    "crates/radix-leptos-primitives/src/components/separator.rs"
    "crates/radix-leptos-primitives/src/components/sheet.rs"
    "crates/radix-leptos-primitives/src/components/skeleton.rs"
    "crates/radix-leptos-primitives/src/components/split_pane.rs"
    "crates/radix-leptos-primitives/src/components/swipe_gestures.rs"
    "crates/radix-leptos-primitives/src/components/time_picker.rs"
    "crates/radix-leptos-primitives/src/components/timeline.rs"
    "crates/radix-leptos-primitives/src/components/toast.rs"
    "crates/radix-leptos-primitives/src/components/toggle_group.rs"
    "crates/radix-leptos-primitives/src/components/toggle.rs"
    "crates/radix-leptos-primitives/src/components/toolbar.rs"
    "crates/radix-leptos-primitives/src/components/touch_button.rs"
    "crates/radix-leptos-primitives/src/components/tree_view.rs"
    "crates/radix-leptos-primitives/src/components/virtual_list.rs"
)

# Fix each file
for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        echo "Fixing $file..."
        # Remove generate_id from imports
        sed -i '' 's/use crate::utils::{merge_classes, generate_id};/use crate::utils::merge_classes;/g' "$file"
        sed -i '' 's/use crate::utils::{merge_optional_classes, generate_id};/use crate::utils::merge_optional_classes;/g' "$file"
        sed -i '' 's/use crate::utils::{merge_classes, merge_optional_classes, generate_id};/use crate::utils::{merge_classes, merge_optional_classes};/g' "$file"
    fi
done

echo "Done fixing unused generate_id imports!"
