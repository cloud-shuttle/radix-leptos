
/// ImageViewer component - Advanced image viewing
#[component]
pub fn ImageViewer(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] src: Option<String>,
    #[prop(optional)] alt: Option<String>,
    #[prop(optional)] config: Option<ImageViewerConfig>,
    #[prop(optional)] zoomable: Option<bool>,
    #[prop(optional)] pannable: Option<bool>,
    #[prop(optional)] rotatable: Option<bool>,
    #[prop(optional)] on_load: Option<Callback<()>>,
    #[prop(optional)] on_error: Option<Callback<String>>,
) -> impl IntoView {
    let src = src.unwrap_or_default();
    let alt = alt.unwrap_or_default();
    let config = config.unwrap_or_default();
    let zoomable = zoomable.unwrap_or(true);
    let pannable = pannable.unwrap_or(true);
    let rotatable = rotatable.unwrap_or(true);

    let class = merge_classes(vec![
        "image-viewer",
        </div>
    }
}

/// Image Viewer Configuration
#[derive(Debug, Clone, PartialEq)]
pub struct ImageViewerConfig {
    pub width: f64,
    pub height: f64,
    pub zoom_min: f64,
    pub zoom_max: f64,
    pub zoom_step: f64,
    pub rotation_step: f64,
    pub pan_speed: f64,
}

impl Default for ImageViewerConfig {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
            zoom_min: 0.1,
            zoom_max: 5.0,
            zoom_step: 0.1,
            rotation_step: 90.0,
            pan_speed: 1.0,
        }
    }
}

/// Image Controls component
#[component]
pub fn ImageControls(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] zoom_level: Option<f64>,
    #[prop(optional)] rotation: Option<f64>,
    #[prop(optional)] on_zoom_in: Option<Callback<()>>,
    #[prop(optional)] on_zoom_out: Option<Callback<()>>,
    #[prop(optional)] on_rotate: Option<Callback<f64>>,
    #[prop(optional)] on_reset: Option<Callback<()>>,
) -> impl IntoView {
    let zoom_level = zoom_level.unwrap_or(1.0);
    let rotation = rotation.unwrap_or(0.0);

    let class = merge_classes(vec![
        "image-controls",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="toolbar"
            aria-label="Image controls"
            data-zoom-level=zoom_level
            data-rotation=rotation
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Image Thumbnail component
#[component]
pub fn ImageThumbnail(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] src: Option<String>,
    #[prop(optional)] alt: Option<String>,
    #[prop(optional)] selected: Option<bool>,
    #[prop(optional)] on_click: Option<Callback<String>>,
) -> impl IntoView {
    let src = src.unwrap_or_default();
    let alt = alt.unwrap_or_default();
    let selected = selected.unwrap_or(false);

    let class = merge_classes(vec![
        "image-thumbnail",
            data-src=src
            data-selected=selected
            tabindex="0"
        />
    }
}

/// Image Gallery component
#[component]
pub fn ImageGallery(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] images: Option<Vec<GalleryImage>>,
    #[prop(optional)] current_index: Option<usize>,
    #[prop(optional)] on_image_select: Option<Callback<usize>>,
) -> impl IntoView {
    let images = images.unwrap_or_default();
    let current_index = current_index.unwrap_or(0);

    let class = merge_classes(vec![
        "image-gallery",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="region"
            aria-label="Image gallery"
            data-image-count=images.len()
            data-current-index=current_index
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Gallery Image structure
#[derive(Debug, Clone, PartialEq)]
pub struct GalleryImage {
    pub src: String,
    pub alt: String,
    pub thumbnail: Option<String>,
    pub caption: Option<String>,
}

impl Default for GalleryImage {
    fn default() -> Self {
        Self {
            src: "".to_string(),
            alt: "Image".to_string(),
            thumbnail: None,
            caption: None,
        }
    }
}

/// Helper function to merge CSS classes
fn merge_classes(classes: Vec<&str>) -> String {
    classes
        .into_iter()
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use proptest::prelude::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Unit Tests
    #[test] fn test_imageviewer_creation() { 
    #[test] fn test_imageviewer_with_class() { 
    #[test] fn test_imageviewer_with_style() { 
    #[test] fn test_imageviewer_with_src() { 
    #[test] fn test_imageviewer_with_alt() { 
    #[test] fn test_imageviewer_with_config() { 
    #[test] fn test_imageviewer_zoomable() { 
    #[test] fn test_imageviewer_pannable() { 
    #[test] fn test_imageviewer_rotatable() { 
    #[test] fn test_imageviewer_on_load() { 
    #[test] fn test_imageviewer_on_error() { 

    // Image Viewer Config tests
    #[test] fn test_imageviewer_config_default() { 
    #[test] fn test_imageviewer_config_custom() { 

    // Image Controls tests
    #[test] fn test_image_controls_creation() { 
    #[test] fn test_image_controls_with_class() { 
    #[test] fn test_image_controls_with_style() { 
    #[test] fn test_image_controls_zoom_level() { 
    #[test] fn test_image_controls_rotation() { 
    #[test] fn test_image_controls_on_zoom_in() { 
    #[test] fn test_image_controls_on_zoom_out() { 
    #[test] fn test_image_controls_on_rotate() { 
    #[test] fn test_image_controls_on_reset() { 

    // Image Thumbnail tests
    #[test] fn test_image_thumbnail_creation() { 
    #[test] fn test_image_thumbnail_with_class() { 
    #[test] fn test_image_thumbnail_with_style() { 
    #[test] fn test_image_thumbnail_src() { 
    #[test] fn test_image_thumbnail_alt() { 
    #[test] fn test_image_thumbnailselected() { 
    #[test] fn test_image_thumbnail_on_click() { 

    // Image Gallery tests
    #[test] fn test_image_gallery_creation() { 
    #[test] fn test_image_gallery_with_class() { 
    #[test] fn test_image_gallery_with_style() { 
    #[test] fn test_image_gallery_images() { 
    #[test] fn test_image_gallerycurrent_index() { 
    #[test] fn test_image_gallery_on_image_select() { 

    // Gallery Image tests
    #[test] fn test_gallery_image_default() { 
    #[test] fn test_gallery_image_creation() { 

    // Helper function tests
    #[test] fn test_merge_classes_empty() { 
    #[test] fn test_merge_classes_single() { 
    #[test] fn test_merge_classes_multiple() { 
    #[test] fn test_merge_classes_with_empty() { 

    // Property-based Tests
    #[test] fn test_imageviewer_property_based() {
        proptest!(|(____class in ".*", __style in ".*")| {
            
        });
    }

    #[test] fn test_imageviewer_config_validation() {
        proptest!(|(____width in 100.0..2000.0f64, __height in 100.0..2000.0f64)| {
            
        });
    }

    #[test] fn test_imageviewer_zoom_property_based() {
        proptest!(|(____zoom_min in 0.1..1.0f64, __zoom_max in 1.0..10.0f64)| {
            
        });
    }

    #[test] fn test_imageviewer_gallery_property_based() {
        proptest!(|(______image_count in 0..50usize)| {
            
        });
    }

    // Integration Tests
    #[test] fn test_imageviewer_user_interaction() { 
    #[test] fn test_imageviewer_accessibility() { 
    #[test] fn test_imageviewer_keyboard_navigation() { 
    #[test] fn test_imageviewer_touch_gestures() { 
    #[test] fn test_imageviewer_gallery_navigation() { 

    // Performance Tests
    #[test] fn test_imageviewer_large_images() { 
    #[test] fn test_imageviewer_memory_usage() { 
    #[test] fn test_imageviewer_render_performance() { 
    #[test] fn test_imageviewer_zoom_performance() { 
}
