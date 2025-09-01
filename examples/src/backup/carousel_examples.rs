use leptos::*;
use leptos::prelude::*;
use radix_leptos_primitives::*;

#[component]
pub fn CarouselExamples() -> impl IntoView {
    let (current_slide, set_current_slide) = signal(0);
    
    let _handle_slide_change = Callback::new(move |new_slide: usize| {
        set_current_slide.set(new_slide);
    });

    view! {
        <div class="carousel-examples">
            <h1>"Carousel Component Examples"</h1>
            
            <section class="example-section">
                <h2>"Basic Carousel"</h2>
                <p>"A simple carousel with navigation arrows and dots."</p>
                <div class="example-container">
                    <Carousel
                        navigation=CarouselNavigation::Both
                        autoplay=CarouselAutoplay::None
                        size=CarouselSize::Medium
                    >
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=1" alt="Slide 1" />
                                <div class="slide-overlay">
                                    <h3>"Beautiful Landscape"</h3>
                                    <p>"Discover the wonders of nature"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=2" alt="Slide 2" />
                                <div class="slide-overlay">
                                    <h3>"Urban Architecture"</h3>
                                    <p>"Modern cityscapes and design"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=3" alt="Slide 3" />
                                <div class="slide-overlay">
                                    <h3>"Ocean Views"</h3>
                                    <p>"Peaceful coastal scenes"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                    </Carousel>
                </div>
            </section>

            <section class="example-section">
                <h2>"Carousel with Autoplay"</h2>
                <p>"A carousel that automatically advances slides every 3 seconds."</p>
                <div class="example-container">
                    <Carousel
                        navigation=CarouselNavigation::Both
                        autoplay=CarouselAutoplay::Medium
                        size=CarouselSize::Medium
                    >
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=4" alt="Slide 1" />
                                <div class="slide-overlay">
                                    <h3>"Mountain Peaks"</h3>
                                    <p>"Reach new heights"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=5" alt="Slide 2" />
                                <div class="slide-overlay">
                                    <h3>"Forest Paths"</h3>
                                    <p>"Explore the wilderness"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=6" alt="Slide 3" />
                                <div class="slide-overlay">
                                    <h3>"Desert Sands"</h3>
                                    <p>"Vast open spaces"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                    </Carousel>
                </div>
            </section>

            <section class="example-section">
                <h2>"Carousel with Arrows Only"</h2>
                <p>"Navigation using only arrow buttons."</p>
                <div class="example-container">
                    <Carousel
                        navigation=CarouselNavigation::Arrows
                        autoplay=CarouselAutoplay::None
                        size=CarouselSize::Medium
                    >
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=7" alt="Slide 1" />
                                <div class="slide-overlay">
                                    <h3>"Sunset Views"</h3>
                                    <p>"Golden hour magic"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=8" alt="Slide 2" />
                                <div class="slide-overlay">
                                    <h3>"Waterfall"</h3>
                                    <p>"Nature's power"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=9" alt="Slide 3" />
                                <div class="slide-overlay">
                                    <h3>"Aurora Borealis"</h3>
                                    <p>"Northern lights dance"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                    </Carousel>
                </div>
            </section>

            <section class="example-section">
                <h2>"Carousel with Dots Only"</h2>
                <p>"Navigation using only dot indicators."</p>
                <div class="example-container">
                    <Carousel
                        navigation=CarouselNavigation::Dots
                        autoplay=CarouselAutoplay::None
                        size=CarouselSize::Medium
                    >
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=10" alt="Slide 1" />
                                <div class="slide-overlay">
                                    <h3>"Tropical Paradise"</h3>
                                    <p>"Island getaway dreams"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=11" alt="Slide 2" />
                                <div class="slide-overlay">
                                    <h3>"Alpine Meadows"</h3>
                                    <p>"High altitude beauty"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=12" alt="Slide 3" />
                                <div class="slide-overlay">
                                    <h3>"Canyon Views"</h3>
                                    <p>"Deep earth formations"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                    </Carousel>
                </div>
            </section>

            <section class="example-section">
                <h2>"Large Carousel"</h2>
                <p>"A larger carousel for more prominent display."</p>
                <div class="example-container">
                    <Carousel
                        navigation=CarouselNavigation::Both
                        autoplay=CarouselAutoplay::None
                        size=CarouselSize::Large
                    >
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/1000/500?random=13" alt="Slide 1" />
                                <div class="slide-overlay">
                                    <h3>"Majestic Mountains"</h3>
                                    <p>"Towering peaks and valleys"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/1000/500?random=14" alt="Slide 2" />
                                <div class="slide-overlay">
                                    <h3>"Serene Lakes"</h3>
                                    <p>"Reflections of tranquility"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/1000/500?random=15" alt="Slide 3" />
                                <div class="slide-overlay">
                                    <h3>"Ancient Forests"</h3>
                                    <p>"Timeless natural beauty"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                    </Carousel>
                </div>
            </section>

            <section class="example-section">
                <h2>"Small Carousel"</h2>
                <p>"A compact carousel for smaller spaces."</p>
                <div class="example-container">
                    <Carousel
                        navigation=CarouselNavigation::Both
                        autoplay=CarouselAutoplay::None
                        size=CarouselSize::Small
                    >
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/400/200?random=16" alt="Slide 1" />
                                <div class="slide-overlay">
                                    <h3>"Mini Adventure"</h3>
                                    <p>"Small but mighty"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/400/200?random=17" alt="Slide 2" />
                                <div class="slide-overlay">
                                    <h3>"Tiny Treasures"</h3>
                                    <p>"Little wonders"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/400/200?random=18" alt="Slide 3" />
                                <div class="slide-overlay">
                                    <h3>"Petite Perfection"</h3>
                                    <p>"Compact beauty"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                    </Carousel>
                </div>
            </section>

            <section class="example-section">
                <h2>"Carousel with Custom Navigation"</h2>
                <p>"A carousel with custom navigation controls."</p>
                <div class="example-container">
                    <CarouselWithCustomNavigation
                        size=CarouselSize::Medium
                    >
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=19" alt="Slide 1" />
                                <div class="slide-overlay">
                                    <h3>"Custom Controls"</h3>
                                    <p>"Tailored navigation experience"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=20" alt="Slide 2" />
                                <div class="slide-overlay">
                                    <h3>"Enhanced UX"</h3>
                                    <p>"Better user interaction"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=21" alt="Slide 3" />
                                <div class="slide-overlay">
                                    <h3>"Flexible Design"</h3>
                                    <p>"Adaptable to your needs"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                    </CarouselWithCustomNavigation>
                </div>
            </section>

            <section class="example-section">
                <h2>"Interactive Carousel"</h2>
                <p>"Current slide: " {move || current_slide.get() + 1} " of 3"</p>
                <div class="example-container">
                    <Carousel
                        navigation=CarouselNavigation::Both
                        autoplay=CarouselAutoplay::None
                        size=CarouselSize::Medium
                    >
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=22" alt="Slide 1" />
                                <div class="slide-overlay">
                                    <h3>"Interactive Slide 1"</h3>
                                    <p>"Track your progress"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=23" alt="Slide 2" />
                                <div class="slide-overlay">
                                    <h3>"Interactive Slide 2"</h3>
                                    <p>"Stay engaged"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                        <CarouselSlide>
                            <div class="slide-content">
                                <img src="https://picsum.photos/800/400?random=24" alt="Slide 3" />
                                <div class="slide-overlay">
                                    <h3>"Interactive Slide 3"</h3>
                                    <p>"Complete the journey"</p>
                                </div>
                            </div>
                        </CarouselSlide>
                    </Carousel>
                </div>
            </section>
        </div>
    }
}

pub fn start_carousel_examples_fn() {
    mount_to_body(|| view! { <CarouselExamples /> });
}
