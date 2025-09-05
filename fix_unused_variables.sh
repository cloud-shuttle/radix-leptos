#!/bin/bash

# Fix unused variables by prefixing with underscore
echo "Fixing unused variables..."

# Find all files with unused variable warnings and fix them
find crates/radix-leptos-primitives/src -name "*.rs" -exec sed -i '' 's/let \([a-zA-Z_][a-zA-Z0-9_]*\) = generate_id/let _\1 = generate_id/g' {} \;

# Fix specific unused variables in form.rs
sed -i '' 's/let field_id = generate_id/let _field_id = generate_id/g' crates/radix-leptos-primitives/src/components/form.rs

# Fix unused variables in select.rs
sed -i '' 's/let select_id = generate_id/let _select_id = generate_id/g' crates/radix-leptos-primitives/src/components/select.rs
sed -i '' 's/let trigger_id = generate_id/let _trigger_id = generate_id/g' crates/radix-leptos-primitives/src/components/select.rs
sed -i '' 's/let content_id = generate_id/let _content_id = generate_id/g' crates/radix-leptos-primitives/src/components/select.rs

# Fix unused variables in accordion.rs
sed -i '' 's/let accordion_id = generate_id/let _accordion_id = generate_id/g' crates/radix-leptos-primitives/src/components/accordion.rs
sed -i '' 's/let item_id = generate_id/let _item_id = generate_id/g' crates/radix-leptos-primitives/src/components/accordion.rs

# Fix unused variables in tooltip.rs
sed -i '' 's/let tooltip_id = generate_id/let _tooltip_id = generate_id/g' crates/radix-leptos-primitives/src/components/tooltip.rs
sed -i '' 's/let trigger_id = generate_id/let _trigger_id = generate_id/g' crates/radix-leptos-primitives/src/components/tooltip.rs
sed -i '' 's/let content_id = generate_id/let _content_id = generate_id/g' crates/radix-leptos-primitives/src/components/tooltip.rs

# Fix unused variables in radio_group.rs
sed -i '' 's/let radio_group_id = generate_id/let _radio_group_id = generate_id/g' crates/radix-leptos-primitives/src/components/radio_group.rs
sed -i '' 's/let item_id = generate_id/let _item_id = generate_id/g' crates/radix-leptos-primitives/src/components/radio_group.rs

# Fix unused variables in slider.rs
sed -i '' 's/let slider_id = generate_id/let _slider_id = generate_id/g' crates/radix-leptos-primitives/src/components/slider.rs
sed -i '' 's/let track_id = generate_id/let _track_id = generate_id/g' crates/radix-leptos-primitives/src/components/slider.rs
sed -i '' 's/let range_id = generate_id/let _range_id = generate_id/g' crates/radix-leptos-primitives/src/components/slider.rs
sed -i '' 's/let thumb_id = generate_id/let _thumb_id = generate_id/g' crates/radix-leptos-primitives/src/components/slider.rs

# Fix unused variables in progress.rs
sed -i '' 's/let progress_id = generate_id/let _progress_id = generate_id/g' crates/radix-leptos-primitives/src/components/progress.rs

# Fix unused variables in tabs.rs
sed -i '' 's/let tabs_id = generate_id/let _tabs_id = generate_id/g' crates/radix-leptos-primitives/src/components/tabs.rs
sed -i '' 's/let trigger_id = generate_id/let _trigger_id = generate_id/g' crates/radix-leptos-primitives/src/components/tabs.rs
sed -i '' 's/let content_id = generate_id/let _content_id = generate_id/g' crates/radix-leptos-primitives/src/components/tabs.rs

echo "Unused variables fixed!"
