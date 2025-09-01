use leptos::*;
use leptos::prelude::*;
use leptos::logging::log;
use radix_leptos_primitives::components::video::*;

#[component]
pub fn VideoExamples() -> impl IntoView {
    let handle_video_click = move |_| {
        log!("Video clicked!");
    };
    
    view! {
        <div class="video-examples">
            <h1>"Video Component Examples"</h1>
            
            <div class="api-docs">
                <h2>"API Documentation"</h2>
                <ul>
                    <li><strong>"Video"</strong> - Basic video component with variants and sizing</li>
                    <li><strong>"VideoWithCustomControls"</strong> - Video with custom control interface</li>
                    <li><strong>"VideoPlayer"</strong> - Video player with built-in controls and overlay</li>
                </ul>
                
                <h3>"Video Sizes"</h3>
                <ul>
                    <li><strong>"Small"</strong> - Compact video player</li>
                    <li><strong>"Medium"</strong> - Standard video player (default)</li>
                    <li><strong>"Large"</strong> - Large video player</li>
                    <li><strong>"FullWidth"</strong> - Full width video player</li>
                </ul>
                
                <h3>"Video Controls"</h3>
                <ul>
                    <li><strong>"Minimal"</strong> - Basic playback controls</li>
                    <li><strong>"Standard"</strong> - Standard video controls (default)</li>
                    <li><strong>"Full"</strong> - Full set of video controls</li>
                    <li><strong>"None"</strong> - No visible controls</li>
                </ul>
                
                <h3>"Video Aspect Ratios"</h3>
                <ul>
                    <li><strong>"Square"</strong> - 1:1 aspect ratio</li>
                    <li><strong>"Landscape"</strong> - 16:9 aspect ratio (default)</li>
                    <li><strong>"Portrait"</strong> - 9:16 aspect ratio</li>
                    <li><strong>"Wide"</strong> - 21:9 aspect ratio</li>
                    <li><strong>"UltraWide"</strong> - 32:9 aspect ratio</li>
                </ul>
            </div>
            
            <div class="example-section">
                <h2>"Basic Video Examples"</h2>
                <p>"Different video sizes and aspect ratios."</p>
                
                <div class="video-grid">
                    <Video
                        src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                        poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                        size=VideoSize::Medium
                        aspect_ratio=VideoAspectRatio::Landscape
                        controls=VideoControls::Standard
                    />
                    
                    <Video
                        src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                        poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=300&h=300&fit=crop".to_string()
                        size=VideoSize::Small
                        aspect_ratio=VideoAspectRatio::Square
                        controls=VideoControls::Minimal
                    />
                    
                    <Video
                        src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                        poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=300&h=400&fit=crop".to_string()
                        size=VideoSize::Large
                        aspect_ratio=VideoAspectRatio::Portrait
                        controls=VideoControls::Full
                    />
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Video Controls Variants"</h2>
                <p>"Different control styles for video playback."</p>
                
                <div class="video-grid">
                    <div class="control-example">
                        <h4>"Standard Controls"</h4>
                        <Video
                            src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                            poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                            size=VideoSize::Medium
                            controls=VideoControls::Standard
                        />
                    </div>
                    
                    <div class="control-example">
                        <h4>"Minimal Controls"</h4>
                        <Video
                            src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                            poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                            size=VideoSize::Medium
                            controls=VideoControls::Minimal
                        />
                    </div>
                    
                    <div class="control-example">
                        <h4>"No Controls"</h4>
                        <Video
                            src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                            poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                            size=VideoSize::Medium
                            controls=VideoControls::None
                        />
                    </div>
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Interactive Videos"</h2>
                <p>"Videos that can be clicked for user interaction."</p>
                
                <div class="video-grid">
                    <Video
                        src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                        poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                        size=VideoSize::Medium
                        interactive=true
                        on_click=Callback::new(handle_video_click)
                    />
                    
                    <Video
                        src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                        poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                        size=VideoSize::Medium
                        interactive=true
                        disabled=true
                        on_click=Callback::new(handle_video_click)
                    />
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Video with Custom Controls"</h2>
                <p>"Videos with custom control interfaces."</p>
                
                <div class="video-grid">
                    <VideoWithCustomControls
                        src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                        poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                        size=VideoSize::Medium
                    >
                        <div class="custom-controls">
                            <button class="custom-play-button">"▶️ Play"</button>
                            <div class="custom-progress">
                                <div class="custom-progress-bar"></div>
                            </div>
                            <button class="custom-volume-button">"🔊"</button>
                        </div>
                    </VideoWithCustomControls>
                    
                    <VideoWithCustomControls
                        src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                        poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                        size=VideoSize::Large
                    >
                        <div class="custom-controls-large">
                            <div class="custom-controls-header">
                                <h4>"Custom Video Player"</h4>
                                <p>"With custom control interface"</p>
                            </div>
                            <div class="custom-controls-buttons">
                                <button class="custom-btn">"⏮️"</button>
                                <button class="custom-btn">"⏯️"</button>
                                <button class="custom-btn">"⏭️"</button>
                                <button class="custom-btn">"🔊"</button>
                            </div>
                        </div>
                    </VideoWithCustomControls>
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Video Player with Overlay"</h2>
                <p>"Video player with built-in controls and overlay information."</p>
                
                <div class="video-grid">
                    <VideoPlayer
                        src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                        poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                        title="Sample Video".to_string()
                        description="A beautiful sample video for demonstration".to_string()
                        size=VideoSize::Large
                        aspect_ratio=VideoAspectRatio::Landscape
                    />
                    
                    <VideoPlayer
                        src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                        poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                        title="Featured Content".to_string()
                        description="Premium video content with enhanced controls".to_string()
                        size=VideoSize::FullWidth
                        aspect_ratio=VideoAspectRatio::Wide
                    />
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Video Playback Options"</h2>
                <p>"Different video playback configurations."</p>
                
                <div class="video-grid">
                    <div class="playback-example">
                        <h4>"Autoplay (Muted)"</h4>
                        <Video
                            src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                            poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                            size=VideoSize::Medium
                            autoplay=true
                            muted=true
                            controls=VideoControls::Standard
                        />
                        <p>"Autoplays when loaded (muted for user experience)"</p>
                    </div>
                    
                    <div class="playback-example">
                        <h4>"Looping Video"</h4>
                        <Video
                            src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                            poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                            size=VideoSize::Medium
                            loop_video=true
                            controls=VideoControls::Minimal
                        />
                        <p>"Continuously loops the video"</p>
                    </div>
                    
                    <div class="playback-example">
                        <h4>"No Controls"</h4>
                        <Video
                            src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                            poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                            size=VideoSize::Medium
                            show_controls=false
                        />
                        <p>"Video without visible controls"</p>
                    </div>
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Video in Context"</h2>
                <p>"Videos used in realistic UI contexts."</p>
                
                <div class="context-examples">
                    <div class="video-card">
                        <Video
                            src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                            poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop".to_string()
                            size=VideoSize::Medium
                            interactive=true
                            on_click=Callback::new(handle_video_click)
                        />
                        <div class="video-card-content">
                            <h3>"Featured Video"</h3>
                            <p>"Click to watch this amazing video content"</p>
                            <button class="btn btn-primary">"Watch Now"</button>
                        </div>
                    </div>
                    
                    <div class="video-gallery">
                        <h3>"Video Gallery"</h3>
                        <div class="gallery-grid">
                            <Video
                                src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                                poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=300&h=200&fit=crop".to_string()
                                size=VideoSize::Small
                                controls=VideoControls::Minimal
                            />
                            <Video
                                src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                                poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=300&h=200&fit=crop".to_string()
                                size=VideoSize::Small
                                controls=VideoControls::Minimal
                            />
                            <Video
                                src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                                poster="https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=300&h=200&fit=crop".to_string()
                                size=VideoSize::Small
                                controls=VideoControls::Minimal
                            />
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Video Features"</h2>
                <ul>
                    <li>"Multiple sizes (small, medium, large, full-width)"</li>
                    <li>"Four control variants (minimal, standard, full, none)"</li>
                    <li>"Five aspect ratios (square, landscape, portrait, wide, ultra-wide)"</li>
                    <li>"Interactive videos with click handlers"</li>
                    <li>"Disabled state for interactive videos"</li>
                    <li>"Autoplay and looping options"</li>
                    <li>"Muted playback for autoplay"</li>
                    <li>"Custom control interfaces"</li>
                    <li>"Built-in video player with overlay"</li>
                    <li>"Poster image support"</li>
                    <li>"Accessible with proper ARIA attributes"</li>
                    <li>"Responsive design"</li>
                    <li>"Flexible styling with CSS classes"</li>
                </ul>
            </div>
        </div>
    }
}

pub fn start_video_examples() {
    mount_to_body(|| {
        view! {
            <VideoExamples />
        }
    });
}
