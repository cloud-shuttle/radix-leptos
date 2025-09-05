#!/bin/bash

# Fix variables that are actually used but were incorrectly prefixed with underscore

echo "Fixing variables that are actually used..."

# Button component
sed -i '' 's/let _button_id = generate_id/let button_id = generate_id/g' crates/radix-leptos-primitives/src/components/button.rs

# Checkbox component
sed -i '' 's/let _checkbox_id = generate_id/let checkbox_id = generate_id/g' crates/radix-leptos-primitives/src/components/checkbox.rs
sed -i '' 's/let _label_id = generate_id/let label_id = generate_id/g' crates/radix-leptos-primitives/src/components/checkbox.rs

# Dialog component
sed -i '' 's/let _title_id = generate_id/let title_id = generate_id/g' crates/radix-leptos-primitives/src/components/dialog.rs
sed -i '' 's/let _description_id = generate_id/let description_id = generate_id/g' crates/radix-leptos-primitives/src/components/dialog.rs

# Form component
sed -i '' 's/let _form_id = generate_id/let form_id = generate_id/g' crates/radix-leptos-primitives/src/components/form.rs
sed -i '' 's/let _input_id = generate_id/let input_id = generate_id/g' crates/radix-leptos-primitives/src/components/form.rs

# Accordion component
sed -i '' 's/let _trigger_id = generate_id/let trigger_id = generate_id/g' crates/radix-leptos-primitives/src/components/accordion.rs
sed -i '' 's/let _content_id = generate_id/let content_id = generate_id/g' crates/radix-leptos-primitives/src/components/accordion.rs

# Tooltip component
sed -i '' 's/let _trigger_id = generate_id/let trigger_id = generate_id/g' crates/radix-leptos-primitives/src/components/tooltip.rs
sed -i '' 's/let _content_id = generate_id/let content_id = generate_id/g' crates/radix-leptos-primitives/src/components/tooltip.rs

# Switch component
sed -i '' 's/let _switch_id = generate_id/let switch_id = generate_id/g' crates/radix-leptos-primitives/src/components/switch.rs
sed -i '' 's/let _thumb_id = generate_id/let thumb_id = generate_id/g' crates/radix-leptos-primitives/src/components/switch.rs

# List component
sed -i '' 's/let _list_id = generate_id/let list_id = generate_id/g' crates/radix-leptos-primitives/src/components/list.rs
sed -i '' 's/let _item_id = generate_id/let item_id = generate_id/g' crates/radix-leptos-primitives/src/components/list.rs
sed -i '' 's/let _header_id = generate_id/let header_id = generate_id/g' crates/radix-leptos-primitives/src/components/list.rs
sed -i '' 's/let _footer_id = generate_id/let footer_id = generate_id/g' crates/radix-leptos-primitives/src/components/list.rs
sed -i '' 's/let _empty_id = generate_id/let empty_id = generate_id/g' crates/radix-leptos-primitives/src/components/list.rs
sed -i '' 's/let _loading_id = generate_id/let loading_id = generate_id/g' crates/radix-leptos-primitives/src/components/list.rs

# Pagination component
sed -i '' 's/let _pagination_id = generate_id/let pagination_id = generate_id/g' crates/radix-leptos-primitives/src/components/pagination.rs
sed -i '' 's/let _list_id = generate_id/let list_id = generate_id/g' crates/radix-leptos-primitives/src/components/pagination.rs
sed -i '' 's/let _item_id = generate_id/let item_id = generate_id/g' crates/radix-leptos-primitives/src/components/pagination.rs
sed -i '' 's/let _first_id = generate_id/let first_id = generate_id/g' crates/radix-leptos-primitives/src/components/pagination.rs
sed -i '' 's/let _prev_id = generate_id/let prev_id = generate_id/g' crates/radix-leptos-primitives/src/components/pagination.rs
sed -i '' 's/let _next_id = generate_id/let next_id = generate_id/g' crates/radix-leptos-primitives/src/components/pagination.rs
sed -i '' 's/let _last_id = generate_id/let last_id = generate_id/g' crates/radix-leptos-primitives/src/components/pagination.rs
sed -i '' 's/let _ellipsis_id = generate_id/let ellipsis_id = generate_id/g' crates/radix-leptos-primitives/src/components/pagination.rs
sed -i '' 's/let _info_id = generate_id/let info_id = generate_id/g' crates/radix-leptos-primitives/src/components/pagination.rs
sed -i '' 's/let _content_id = generate_id/let content_id = generate_id/g' crates/radix-leptos-primitives/src/components/pagination.rs

echo "Used variables fixed!"
