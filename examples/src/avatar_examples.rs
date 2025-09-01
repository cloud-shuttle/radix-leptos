use leptos::*;
use leptos::prelude::*;
use leptos::logging::log;
use radix_leptos_primitives::components::avatar::*;

#[component]
pub fn AvatarExamples() -> impl IntoView {
    let handle_avatar_click = move |_| {
        log!("Avatar clicked!");
    };
    
    view! {
        <div class="avatar-examples">
            <h1>"Avatar Component Examples"</h1>
            
            <div class="api-docs">
                <h2>"API Documentation"</h2>
                <ul>
                    <li><strong>"Avatar"</strong> - Root avatar component with variants and sizing</li>
                    <li><strong>"AvatarImage"</strong> - Avatar with image source</li>
                    <li><strong>"AvatarFallback"</strong> - Avatar fallback content (initials, icons)</li>
                    <li><strong>"AvatarGroup"</strong> - Group of avatars with overlap</li>
                    <li><strong>"AvatarWithInitials"</strong> - Avatar with automatic initials generation</li>
                </ul>
                
                <h3>"Avatar Sizes"</h3>
                <ul>
                    <li><strong>"Small"</strong> - 24px avatar</li>
                    <li><strong>"Medium"</strong> - 32px avatar (default)</li>
                    <li><strong>"Large"</strong> - 40px avatar</li>
                    <li><strong>"ExtraLarge"</strong> - 48px avatar</li>
                </ul>
                
                <h3>"Avatar Shapes"</h3>
                <ul>
                    <li><strong>"Circle"</strong> - Circular avatar (default)</li>
                    <li><strong>"Square"</strong> - Square avatar</li>
                    <li><strong>"Rounded"</strong> - Rounded square avatar</li>
                </ul>
            </div>
            
            <div class="example-section">
                <h2>"Basic Avatar Examples"</h2>
                <p>"Different avatar sizes and shapes with initials."</p>
                
                <div class="avatar-grid">
                    <AvatarWithInitials
                        name="John Doe".to_string()
                        size=AvatarSize::Small
                        shape=AvatarShape::Circle
                    />
                    
                    <AvatarWithInitials
                        name="Jane Smith".to_string()
                        size=AvatarSize::Medium
                        shape=AvatarShape::Circle
                    />
                    
                    <AvatarWithInitials
                        name="Bob Johnson".to_string()
                        size=AvatarSize::Large
                        shape=AvatarShape::Circle
                    />
                    
                    <AvatarWithInitials
                        name="Alice Brown".to_string()
                        size=AvatarSize::ExtraLarge
                        shape=AvatarShape::Circle
                    />
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Avatar Shapes"</h2>
                <p>"Different avatar shapes for various design needs."</p>
                
                <div class="avatar-grid">
                    <AvatarWithInitials
                        name="Circle User".to_string()
                        size=AvatarSize::Large
                        shape=AvatarShape::Circle
                    />
                    
                    <AvatarWithInitials
                        name="Square User".to_string()
                        size=AvatarSize::Large
                        shape=AvatarShape::Square
                    />
                    
                    <AvatarWithInitials
                        name="Rounded User".to_string()
                        size=AvatarSize::Large
                        shape=AvatarShape::Rounded
                    />
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Interactive Avatars"</h2>
                <p>"Avatars that can be clicked for user interaction."</p>
                
                <div class="avatar-grid">
                    <Avatar
                        size=AvatarSize::Large
                        shape=AvatarShape::Circle
                        interactive=true
                        on_click=Callback::new(handle_avatar_click)
                    >
                        <AvatarFallback>
                            <span class="avatar-icon">"👤"</span>
                        </AvatarFallback>
                    </Avatar>
                    
                    <Avatar
                        size=AvatarSize::Large
                        shape=AvatarShape::Circle
                        interactive=true
                        on_click=Callback::new(handle_avatar_click)
                    >
                        <AvatarFallback>
                            <span class="avatar-icon">"🎭"</span>
                        </AvatarFallback>
                    </Avatar>
                    
                    <Avatar
                        size=AvatarSize::Large
                        shape=AvatarShape::Circle
                        interactive=true
                        disabled=true
                        on_click=Callback::new(handle_avatar_click)
                    >
                        <AvatarFallback>
                            <span class="avatar-icon">"🔒"</span>
                        </AvatarFallback>
                    </Avatar>
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Avatar with Images"</h2>
                <p>"Avatars with image sources and fallback handling."</p>
                
                <div class="avatar-grid">
                    <Avatar size=AvatarSize::Large shape=AvatarShape::Circle>
                        <AvatarImage
                            src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=150&h=150&fit=crop&crop=face".to_string()
                            alt="Professional headshot".to_string()
                        />
                        <AvatarFallback>
                            <span class="radix-avatar-initials">"JD"</span>
                        </AvatarFallback>
                    </Avatar>
                    
                    <Avatar size=AvatarSize::Large shape=AvatarShape::Circle>
                        <AvatarImage
                            src="https://images.unsplash.com/photo-1494790108755-2616b612b786?w=150&h=150&fit=crop&crop=face".to_string()
                            alt="Professional headshot".to_string()
                        />
                        <AvatarFallback>
                            <span class="radix-avatar-initials">"JS"</span>
                        </AvatarFallback>
                    </Avatar>
                    
                    <Avatar size=AvatarSize::Large shape=AvatarShape::Circle>
                        <AvatarImage
                            src="invalid-url".to_string()
                            alt="Invalid image".to_string()
                        />
                        <AvatarFallback>
                            <span class="radix-avatar-initials">"??"</span>
                        </AvatarFallback>
                    </Avatar>
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Avatar Groups"</h2>
                <p>"Groups of avatars with overlap and team display."</p>
                
                <div class="avatar-group-examples">
                    <AvatarGroup size=AvatarSize::Medium shape=AvatarShape::Circle>
                        <AvatarWithInitials name="Alice Johnson".to_string() size=AvatarSize::Medium />
                        <AvatarWithInitials name="Bob Smith".to_string() size=AvatarSize::Medium />
                        <AvatarWithInitials name="Carol Davis".to_string() size=AvatarSize::Medium />
                        <AvatarWithInitials name="David Wilson".to_string() size=AvatarSize::Medium />
                        <AvatarWithInitials name="Eve Brown".to_string() size=AvatarSize::Medium />
                    </AvatarGroup>
                    
                    <AvatarGroup size=AvatarSize::Large shape=AvatarShape::Rounded>
                        <Avatar size=AvatarSize::Large shape=AvatarShape::Rounded>
                            <AvatarImage
                                src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=150&h=150&fit=crop&crop=face".to_string()
                                alt="Team member 1".to_string()
                            />
                            <AvatarFallback>
                                <span class="radix-avatar-initials">"AJ"</span>
                            </AvatarFallback>
                        </Avatar>
                        <Avatar size=AvatarSize::Large shape=AvatarShape::Rounded>
                            <AvatarImage
                                src="https://images.unsplash.com/photo-1494790108755-2616b612b786?w=150&h=150&fit=crop&crop=face".to_string()
                                alt="Team member 2".to_string()
                            />
                            <AvatarFallback>
                                <span class="radix-avatar-initials">"BS"</span>
                            </AvatarFallback>
                        </Avatar>
                        <Avatar size=AvatarSize::Large shape=AvatarShape::Rounded>
                            <AvatarFallback>
                                <span class="radix-avatar-initials">"CD"</span>
                            </AvatarFallback>
                        </Avatar>
                    </AvatarGroup>
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Avatar in Context"</h2>
                <p>"Avatars used in realistic UI contexts."</p>
                
                <div class="context-examples">
                    <div class="user-profile">
                        <div class="profile-header">
                            <AvatarWithInitials
                                name="Sarah Wilson".to_string()
                                size=AvatarSize::ExtraLarge
                                shape=AvatarShape::Circle
                            />
                            <div class="profile-info">
                                <h3>"Sarah Wilson"</h3>
                                <p>"Senior Developer"</p>
                                <p>"sarah.wilson@company.com"</p>
                            </div>
                        </div>
                    </div>
                    
                    <div class="team-list">
                        <div class="team-member">
                            <AvatarWithInitials name="Mike Chen".to_string() size=AvatarSize::Medium />
                            <div class="member-info">
                                <span class="member-name">"Mike Chen"</span>
                                <span class="member-role">"Designer"</span>
                            </div>
                        </div>
                        
                        <div class="team-member">
                            <Avatar size=AvatarSize::Medium>
                                <AvatarImage
                                    src="https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=150&h=150&fit=crop&crop=face".to_string()
                                    alt="Lisa Park".to_string()
                                />
                                <AvatarFallback>
                                    <span class="radix-avatar-initials">"LP"</span>
                                </AvatarFallback>
                            </Avatar>
                            <div class="member-info">
                                <span class="member-name">"Lisa Park"</span>
                                <span class="member-role">"Product Manager"</span>
                            </div>
                        </div>
                        
                        <div class="team-member">
                            <Avatar size=AvatarSize::Medium>
                                <AvatarFallback>
                                    <span class="avatar-icon">"👨‍💻"</span>
                                </AvatarFallback>
                            </Avatar>
                            <div class="member-info">
                                <span class="member-name">"Alex Kumar"</span>
                                <span class="member-role">"Frontend Developer"</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="example-section">
                <h2>"Avatar Features"</h2>
                <ul>
                    <li>"Multiple sizes (small, medium, large, extra-large)"</li>
                    <li>"Three shapes (circle, square, rounded)"</li>
                    <li>"Interactive avatars with click handlers"</li>
                    <li>"Disabled state for interactive avatars"</li>
                    <li>"Image support with fallback handling"</li>
                    <li>"Automatic initials generation from names"</li>
                    <li>"Avatar groups for team displays"</li>
                    <li>"Accessible with proper ARIA attributes"</li>
                    <li>"Responsive design"</li>
                    <li>"Flexible content structure"</li>
                </ul>
            </div>
        </div>
    }
}

pub fn start_avatar_examples() {
    mount_to_body(|| {
        view! {
            <AvatarExamples />
        }
    });
}
