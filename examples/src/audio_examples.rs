use leptos::*;
use leptos::prelude::*;
use leptos::logging::log;
use radix_leptos_primitives::components::audio::*;

#[component]
pub fn AudioExamples() -> impl IntoView {
    let handle_audio_click = move |_| {
        log!("Audio clicked!");
    };

    view! {
        <div class="audio-examples">
            <h1>"Audio Component Examples"</h1>

            <div class="api-docs">
                <h2>"API Documentation"</h2>
                <ul>
                    <li><strong>"Audio"</strong> - Basic audio component with variants and sizing</li>
                    <li><strong>"AudioWithCustomControls"</strong> - Audio with custom control interface</li>
                    <li><strong>"AudioPlayer"</strong> - Audio player with built-in controls and metadata</li>
                </ul>

                <h3>"Audio Sizes"</h3>
                <ul>
                    <li><strong>"Small"</strong> - Compact audio player</li>
                    <li><strong>"Medium"</strong> - Standard audio player (default)</li>
                    <li><strong>"Large"</strong> - Large audio player</li>
                    <li><strong>"FullWidth"</strong> - Full width audio player</li>
                </ul>

                <h3>"Audio Controls"</h3>
                <ul>
                    <li><strong>"Minimal"</strong> - Basic playback controls</li>
                    <li><strong>"Standard"</strong> - Standard audio controls (default)</li>
                    <li><strong>"Full"</strong> - Full set of audio controls</li>
                    <li><strong>"None"</strong> - No visible controls</li>
                </ul>

                <h3>"Audio Themes"</h3>
                <ul>
                    <li><strong>"Default"</strong> - Default theme</li>
                    <li><strong>"Dark"</strong> - Dark theme</li>
                    <li><strong>"Light"</strong> - Light theme</li>
                    <li><strong>"Custom"</strong> - Custom theme</li>
                </ul>
            </div>

            <div class="example-section">
                <h2>"Basic Audio Examples"</h2>
                <p>"Different audio sizes and themes."</p>

                <div class="audio-grid">
                    <Audio
                        src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                        title="Bell Ringing".to_string()
                        artist="Sound Effects".to_string()
                        size=AudioSize::Medium
                        controls=AudioControls::Standard
                        theme=AudioTheme::Default
                    />

                    <Audio
                        src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                        title="Bell Ringing".to_string()
                        artist="Sound Effects".to_string()
                        size=AudioSize::Small
                        controls=AudioControls::Minimal
                        theme=AudioTheme::Dark
                    />

                    <Audio
                        src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                        title="Bell Ringing".to_string()
                        artist="Sound Effects".to_string()
                        size=AudioSize::Large
                        controls=AudioControls::Full
                        theme=AudioTheme::Light
                    />
                </div>
            </div>

            <div class="example-section">
                <h2>"Audio Controls Variants"</h2>
                <p>"Different control styles for audio playback."</p>

                <div class="audio-grid">
                    <div class="control-example">
                        <h4>"Standard Controls"</h4>
                        <Audio
                            src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                            title="Bell Ringing".to_string()
                            artist="Sound Effects".to_string()
                            size=AudioSize::Medium
                            controls=AudioControls::Standard
                        />
                    </div>

                    <div class="control-example">
                        <h4>"Minimal Controls"</h4>
                        <Audio
                            src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                            title="Bell Ringing".to_string()
                            artist="Sound Effects".to_string()
                            size=AudioSize::Medium
                            controls=AudioControls::Minimal
                        />
                    </div>

                    <div class="control-example">
                        <h4>"No Controls"</h4>
                        <Audio
                            src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                            title="Bell Ringing".to_string()
                            artist="Sound Effects".to_string()
                            size=AudioSize::Medium
                            controls=AudioControls::None
                        />
                    </div>
                </div>
            </div>

            <div class="example-section">
                <h2>"Interactive Audio"</h2>
                <p>"Audio that can be clicked for user interaction."</p>

                <div class="audio-grid">
                    <Audio
                        src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                        title="Bell Ringing".to_string()
                        artist="Sound Effects".to_string()
                        size=AudioSize::Medium
                        interactive=true
                        on_click=Callback::new(handle_audio_click)
                    />

                    <Audio
                        src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                        title="Bell Ringing".to_string()
                        artist="Sound Effects".to_string()
                        size=AudioSize::Medium
                        interactive=true
                        disabled=true
                        on_click=Callback::new(handle_audio_click)
                    />
                </div>
            </div>

            <div class="example-section">
                <h2>"Audio with Custom Controls"</h2>
                <p>"Audio with custom control interfaces."</p>

                <div class="audio-grid">
                    <AudioWithCustomControls
                        src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                        title="Bell Ringing".to_string()
                        artist="Sound Effects".to_string()
                        size=AudioSize::Medium
                    >
                        <div class="custom-controls">
                            <button class="custom-play-button">"▶️ Play"</button>
                            <div class="custom-progress">
                                <div class="custom-progress-bar"></div>
                            </div>
                            <button class="custom-volume-button">"🔊"</button>
                        </div>
                    </AudioWithCustomControls>

                    <AudioWithCustomControls
                        src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                        title="Bell Ringing".to_string()
                        artist="Sound Effects".to_string()
                        size=AudioSize::Large
                    >
                        <div class="custom-controls-large">
                            <div class="custom-controls-header">
                                <h4>"Custom Audio Player"</h4>
                                <p>"With custom control interface"</p>
                            </div>
                            <div class="custom-controls-buttons">
                                <button class="custom-btn">"⏮️"</button>
                                <button class="custom-btn">"⏯️"</button>
                                <button class="custom-btn">"⏭️"</button>
                                <button class="custom-btn">"🔊"</button>
                            </div>
                        </div>
                    </AudioWithCustomControls>
                </div>
            </div>

            <div class="example-section">
                <h2>"Audio Player with Metadata"</h2>
                <p>"Audio player with built-in controls and track information."</p>

                <div class="audio-grid">
                    <AudioPlayer
                        src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                        size=AudioSize::Large
                        theme=AudioTheme::Default
                    />

                    <AudioPlayer
                        src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                        size=AudioSize::FullWidth
                        theme=AudioTheme::Dark
                    />
                </div>
            </div>

            <div class="example-section">
                <h2>"Audio Playback Options"</h2>
                <p>"Different audio playback configurations."</p>

                <div class="audio-grid">
                    <div class="playback-example">
                        <h4>"Autoplay (Muted)"</h4>
                        <Audio
                            src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                            title="Bell Ringing".to_string()
                            artist="Sound Effects".to_string()
                            size=AudioSize::Medium
                            autoplay=true
                            muted=true
                            controls=AudioControls::Standard
                        />
                        <p>"Autoplays when loaded (muted for user experience)"</p>
                    </div>

                    <div class="playback-example">
                        <h4>"Looping Audio"</h4>
                        <Audio
                            src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                            title="Bell Ringing".to_string()
                            artist="Sound Effects".to_string()
                            size=AudioSize::Medium
                            loop_audio=true
                            controls=AudioControls::Minimal
                        />
                        <p>"Continuously loops the audio"</p>
                    </div>

                    <div class="playback-example">
                        <h4>"No Controls"</h4>
                        <Audio
                            src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                            title="Bell Ringing".to_string()
                            artist="Sound Effects".to_string()
                            size=AudioSize::Medium
                            show_controls=false
                        />
                        <p>"Audio without visible controls"</p>
                    </div>
                </div>
            </div>

            <div class="example-section">
                <h2>"Audio in Context"</h2>
                <p>"Audio used in realistic UI contexts."</p>

                <div class="context-examples">
                    <div class="audio-card">
                        <Audio
                            src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                            title="Bell Ringing".to_string()
                            artist="Sound Effects".to_string()
                            size=AudioSize::Medium
                            interactive=true
                            on_click=Callback::new(handle_audio_click)
                        />
                        <div class="audio-card-content">
                            <h3>"Featured Audio"</h3>
                            <p>"Click to play this amazing audio content"</p>
                            <button class="btn btn-primary">"Play Now"</button>
                        </div>
                    </div>

                    <div class="audio-gallery">
                        <h3>"Audio Gallery"</h3>
                        <div class="gallery-grid">
                            <Audio
                                src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                                title="Bell Ringing".to_string()
                                artist="Sound Effects".to_string()
                                size=AudioSize::Small
                                controls=AudioControls::Minimal
                            />
                            <Audio
                                src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                                title="Bell Ringing".to_string()
                                artist="Sound Effects".to_string()
                                size=AudioSize::Small
                                controls=AudioControls::Minimal
                            />
                            <Audio
                                src="https://www.soundjay.com/misc/sounds/bell-ringing-05.wav".to_string()
                                title="Bell Ringing".to_string()
                                artist="Sound Effects".to_string()
                                size=AudioSize::Small
                                controls=AudioControls::Minimal
                            />
                        </div>
                    </div>
                </div>
            </div>

            <div class="example-section">
                <h2>"Audio Features"</h2>
                <ul>
                    <li>"Multiple sizes (small, medium, large, full-width)"</li>
                    <li>"Four control variants (minimal, standard, full, none)"</li>
                    <li>"Four themes (default, dark, light, custom)"</li>
                    <li>"Interactive audio with click handlers"</li>
                    <li>"Disabled state for interactive audio"</li>
                    <li>"Autoplay and looping options"</li>
                    <li>"Muted playback for autoplay"</li>
                    <li>"Custom control interfaces"</li>
                    <li>"Built-in audio player with metadata"</li>
                    <li>"Track information display (title, artist, album, duration)"</li>
                    <li>"Accessible with proper ARIA attributes"</li>
                    <li>"Responsive design"</li>
                    <li>"Flexible styling with CSS classes"</li>
                </ul>
            </div>
        </div>
    }
}

pub fn start_audio_examples() {
    mount_to_body(|| {
        view! {
            <AudioExamples />
        }
    });
}
