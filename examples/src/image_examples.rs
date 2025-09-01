use leptos::*;
use leptos::prelude::*;
use leptos::logging::log;
use radix_leptos_primitives::components::image::*;

#[component]
pub fn ImageExamples() -> impl IntoView {
    let handle_image_click = move |_| {
        log!("Image clicked!");
    };
    
    view! {
        <div class="image-examples">
            <h1>"Image Component Examples"</h1>
            
            <div class="api-docs">
                <h2>"API Documentation"</h2>
                <ul>
                    <li><strong>"Image"</strong> - Basic image component with variants and sizing</li>
                    <li><strong>"ResponsiveImage"</strong> - Image with multiple sources for different screen sizes</li>
                    <li><strong>"ImageWithPlaceholder"</strong> - Image with placeholder/skeleton loading</li>
                </ul>
                
                <h3>"Image Aspect Ratios"</h3>
                <ul>
                    <li><strong>"Square"</strong> - 1:1 aspect ratio</li>
                    <li><strong>"Landscape"</strong> - 16:9 aspect ratio (default)</li>
                    <li><strong>"Portrait"</strong> - 9:16 aspect ratio</li>
                    <li><strong>"Wide"</strong> - 21:9 aspect ratio</li>
                    <li><strong>"UltraWide"</strong> - 32:9 aspect ratio</li>
                </ul>
                
                <h3>"Image Fit Behaviors"</h3>
                <ul>
                    <li><strong>"Cover"</strong> - Scale to cover container (default)</li>
                    <li><strong>"Contain"</strong> - Scale to fit within container</li>
                    <li><strong>"Fill"</strong> - Stretch to fill container</li>
                    <li><strong>"None"</strong> - No scaling</li>
                    <li><strong>"ScaleDown"</strong> - Scale down only if larger than container</li>
                </ul>
                
                <h3>"Loading Strategies"</h3>
                <ul>
                    <li><strong>"Lazy"</strong> - Load when near viewport (default)</li>
                    <li><strong>"Eager"</strong> - Load immediately</li>
                </ul>
            </div>
            
            <div class="example-section">
                <h2>"Basic Image Examples"</h2>
                <p>"Different image aspect ratios and fit behaviors."</p>
                
                <div class="image-grid">
                    <Image
                        src="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                        alt="Mountain landscape".to_string()
                        aspect_ratio=ImageAspectRatio::Landscape
                        fit=ImageFit::Cover
                    />
                    
                    <Image
                        src="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=300&h=300&fit=crop".to_string()
                        alt="Square mountain view".to_string()
                        aspect_ratio=ImageAspectRatio::Square
                        fit=ImageFit::Cover
                    />
                    
                    <Image
                        src="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=300&h=400&fit=crop".to_string()
                        alt="Portrait mountain view".to_string()
                        aspect_ratio=ImageAspectRatio::Portrait
                        fit=ImageFit::Cover
                    />
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Image Fit Behaviors"</h2>
                <p>"Different ways images can fit within their containers."</p>
                
                <div class="image-grid">
                    <div class="fit-example">
                        <h4>"Cover"</h4>
                        <Image
                            src="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                            alt="Cover fit".to_string()
                            aspect_ratio=ImageAspectRatio::Landscape
                            fit=ImageFit::Cover
                        />
                    </div>
                    
                    <div class="fit-example">
                        <h4>"Contain"</h4>
                        <Image
                            src="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                            alt="Contain fit".to_string()
                            aspect_ratio=ImageAspectRatio::Landscape
                            fit=ImageFit::Contain
                        />
                    </div>
                    
                    <div class="fit-example">
                        <h4>"Fill"</h4>
                        <Image
                            src="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                            alt="Fill fit".to_string()
                            aspect_ratio=ImageAspectRatio::Landscape
                            fit=ImageFit::Fill
                        />
                    </div>
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Interactive Images"</h2>
                <p>"Images that can be clicked for user interaction."</p>
                
                <div class="image-grid">
                    <Image
                        src="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                        alt="Clickable mountain view".to_string()
                        aspect_ratio=ImageAspectRatio::Landscape
                        interactive=true
                        on_click=Callback::new(handle_image_click)
                    />
                    
                    <Image
                        src="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                        alt="Disabled interactive image".to_string()
                        aspect_ratio=ImageAspectRatio::Landscape
                        interactive=true
                        disabled=true
                        on_click=Callback::new(handle_image_click)
                    />
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Responsive Images"</h2>
                <p>"Images that adapt to different screen sizes."</p>
                
                <div class="responsive-examples">
                    <ResponsiveImage
                        sources=vec![
                            ("https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=800&h=600&fit=crop".to_string(), "(min-width: 768px)".to_string()),
                            ("https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=600&h=450&fit=crop".to_string(), "(min-width: 480px)".to_string()),
                        ]
                        src="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                        alt="Responsive mountain view".to_string()
                        aspect_ratio=ImageAspectRatio::Landscape
                    />
                    
                    <ResponsiveImage
                        sources=vec![
                            ("https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=600&h=600&fit=crop".to_string(), "(min-width: 768px)".to_string()),
                            ("https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=400&fit=crop".to_string(), "(min-width: 480px)".to_string()),
                        ]
                        src="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=300&h=300&fit=crop".to_string()
                        alt="Responsive square view".to_string()
                        aspect_ratio=ImageAspectRatio::Square
                    />
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Images with Placeholders"</h2>
                <p>"Images with placeholder loading states."</p>
                
                <div class="image-grid">
                    <ImageWithPlaceholder
                        src="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                        alt="Mountain with placeholder".to_string()
                        aspect_ratio=ImageAspectRatio::Landscape
                        loading=ImageLoading::Lazy
                    />
                    
                    <ImageWithPlaceholder
                        src="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=300&h=300&fit=crop".to_string()
                        alt="Square with placeholder".to_string()
                        aspect_ratio=ImageAspectRatio::Square
                        loading=ImageLoading::Eager
                    />
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Image Loading Strategies"</h2>
                <p>"Different loading strategies for performance optimization."</p>
                
                <div class="image-grid">
                    <div class="loading-example">
                        <h4>"Lazy Loading"</h4>
                        <Image
                            src="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                            alt="Lazy loaded image".to_string()
                            aspect_ratio=ImageAspectRatio::Landscape
                            loading=ImageLoading::Lazy
                        />
                        <p>"Loads when near viewport"</p>
                    </div>
                    
                    <div class="loading-example">
                        <h4>"Eager Loading"</h4>
                        <Image
                            src="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                            alt="Eager loaded image".to_string()
                            aspect_ratio=ImageAspectRatio::Landscape
                            loading=ImageLoading::Eager
                        />
                        <p>"Loads immediately"</p>
                    </div>
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Image in Context"</h2>
                <p>"Images used in realistic UI contexts."</p>
                
                <div class="context-examples">
                    <div class="gallery-item">
                        <Image
                            src="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                            alt="Gallery item 1".to_string()
                            aspect_ratio=ImageAspectRatio::Landscape
                            interactive=true
                            on_click=Callback::new(handle_image_click)
                        />
                        <div class="gallery-caption">
                            <h3>"Mountain Landscape"</h3>
                            <p>"Beautiful mountain view at sunset"</p>
                        </div>
                    </div>
                    
                    <div class="card-layout">
                        <Image
                            src="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                            alt="Card image".to_string()
                            aspect_ratio=ImageAspectRatio::Landscape
                            fit=ImageFit::Cover
                        />
                        <div class="card-content">
                            <h3>"Featured Destination"</h3>
                            <p>"Explore the breathtaking mountain ranges"</p>
                            <button class="btn btn-primary">"Learn More"</button>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Image Features"</h2>
                <ul>
                    <li>"Multiple aspect ratios (square, landscape, portrait, wide, ultra-wide)"</li>
                    <li>"Five fit behaviors (cover, contain, fill, none, scale-down)"</li>
                    <li>"Two loading strategies (lazy, eager)"</li>
                    <li>"Interactive images with click handlers"</li>
                    <li>"Disabled state for interactive images"</li>
                    <li>"Responsive images with multiple sources"</li>
                    <li>"Placeholder loading states"</li>
                    <li>"Accessible with proper alt text"</li>
                    <li>"Responsive design"</li>
                    <li>"Flexible styling with CSS classes"</li>
                </ul>
            </div>
        </div>
    }
}

pub fn start_image_examples() {
    mount_to_body(|| {
        view! {
            <ImageExamples />
        }
    });
}
