use leptos::*;
use radix_leptos_primitives::*;
use wasm_bindgen::prelude::*;
use leptos::mount::mount_to_body;
use leptos::prelude::{ElementChild, ClassAttribute, GlobalAttributes};

#[component]
pub fn ComponentShowcase() -> impl IntoView {
    view! {
        <div class="space-y-16">
            // Form Components Section
            <div id="form-components" class="component-section">
                <div class="text-center mb-8">
                    <h2 class="text-3xl font-bold text-gray-900 mb-2">Form Components</h2>
                    <p class="text-lg text-gray-600">Essential form elements with accessibility and validation</p>
                </div>
                
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    // Button Component
                    <div id="button" class="component-card bg-white rounded-lg shadow-md p-6">
                        <h3 class="text-xl font-semibold text-gray-900 mb-4">Button</h3>
                        <div class="space-y-3">
                            <Button variant=ButtonVariant::Default>
                                "Default Button"
                            </Button>
                            <Button variant=ButtonVariant::Destructive>
                                "Destructive"
                            </Button>
                            <Button variant=ButtonVariant::Outline>
                                "Outline"
                            </Button>
                        </div>
                    </div>

                    // TextInput Component
                    <div id="text-input" class="component-card bg-white rounded-lg shadow-md p-6">
                        <h3 class="text-xl font-semibold text-gray-900 mb-4">TextInput</h3>
                        <div class="space-y-3">
                            <TextInput placeholder="Enter text...".to_string() />
                            <TextInput placeholder="Disabled input".to_string() />
                        </div>
                    </div>

                    // Select Component
                    <div id="select" class="component-card bg-white rounded-lg shadow-md p-6">
                        <h3 class="text-xl font-semibold text-gray-900 mb-4">Select</h3>
                        <Select>
                            <SelectTrigger>
                                <SelectValue placeholder="Choose an option".to_string()>
                                "Choose an option"
                            </SelectValue>
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="option1".to_string()>"Option 1"</SelectItem>
                                <SelectItem value="option2".to_string()>"Option 2"</SelectItem>
                                <SelectItem value="option3".to_string()>"Option 3"</SelectItem>
                            </SelectContent>
                        </Select>
                    </div>
                </div>
            </div>

            // Feedback Components Section
            <div id="feedback-components" class="component-section">
                <div class="text-center mb-8">
                    <h2 class="text-3xl font-bold text-gray-900 mb-2">Feedback Components</h2>
                    <p class="text-lg text-gray-600">User feedback and status indicators</p>
                </div>
                
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    // Alert Component
                    <div id="alert" class="component-card bg-white rounded-lg shadow-md p-6">
                        <h3 class="text-xl font-semibold text-gray-900 mb-4">Alert</h3>
                        <div class="space-y-3">
                            <Alert variant=AlertVariant::Info>
                                <AlertTitle>"Information"</AlertTitle>
                                <AlertDescription>"This is an informational alert."</AlertDescription>
                            </Alert>
                        </div>
                    </div>

                    // Badge Component
                    <div id="badge" class="component-card bg-white rounded-lg shadow-md p-6">
                        <h3 class="text-xl font-semibold text-gray-900 mb-4">Badge</h3>
                        <div class="space-y-3">
                            <Badge variant=BadgeVariant::Default>"Default"</Badge>
                            <Badge variant=BadgeVariant::Secondary>"Secondary"</Badge>
                            <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
                        </div>
                    </div>

                    // Avatar Component
                    <div id="avatar" class="component-card bg-white rounded-lg shadow-md p-6">
                        <h3 class="text-xl font-semibold text-gray-900 mb-4">Avatar</h3>
                        <div class="space-y-3">
                            <Avatar>
                                <AvatarImage src="https://github.com/shadcn.png".to_string() alt="User".to_string() />
                                <AvatarFallback>"CN"</AvatarFallback>
                            </Avatar>
                        </div>
                    </div>
                </div>
            </div>

            // Media Components Section
            <div id="media-components" class="component-section">
                <div class="text-center mb-8">
                    <h2 class="text-3xl font-bold text-gray-900 mb-2">Media Components</h2>
                    <p class="text-lg text-gray-600">Rich media display and interaction</p>
                </div>
                
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    // Image Component
                    <div id="image" class="component-card bg-white rounded-lg shadow-md p-6">
                        <h3 class="text-xl font-semibold text-gray-900 mb-4">Image</h3>
                        <Image 
                            src="https://picsum.photos/300/200".to_string() 
                            alt="Sample image".to_string()
                            class="w-full h-32 object-cover rounded".to_string()
                        />
                    </div>

                    // Video Component
                    <div id="video" class="component-card bg-white rounded-lg shadow-md p-6">
                        <h3 class="text-xl font-semibold text-gray-900 mb-4">Video</h3>
                        <Video 
                            src="https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4".to_string()
                            class="w-full h-32".to_string()
                        />
                    </div>

                    // Audio Component
                    <div id="audio" class="component-card bg-white rounded-lg shadow-md p-6">
                        <h3 class="text-xl font-semibold text-gray-900 mb-4">Audio</h3>
                        <Audio 
                            src="https://sample-videos.com/zip/10/mp3/SampleAudio_0.4mb.mp3".to_string()
                        />
                    </div>
                </div>
            </div>

            // Advanced Components Section
            <div id="advanced-components" class="component-section">
                <div class="text-center mb-8">
                    <h2 class="text-3xl font-bold text-gray-900 mb-2">Advanced Components</h2>
                    <p class="text-lg text-gray-600">Complex UI patterns and interactions</p>
                </div>
                
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    // Accordion Component
                    <div id="accordion" class="component-card bg-white rounded-lg shadow-md p-6">
                        <h3 class="text-xl font-semibold text-gray-900 mb-4">Accordion</h3>
                        <Accordion>
                            <AccordionItem value="item-1".to_string()>
                                <AccordionTrigger>"Section 1"</AccordionTrigger>
                                <AccordionContent>"Content for section 1"</AccordionContent>
                            </AccordionItem>
                        </Accordion>
                    </div>

                    // Tabs Component
                    <div id="tabs" class="component-card bg-white rounded-lg shadow-md p-6">
                        <h3 class="text-xl font-semibold text-gray-900 mb-4">Tabs</h3>
                        <Tabs>
                            <TabsList>
                                <TabsTrigger value="tab1".to_string()>"Tab 1"</TabsTrigger>
                                <TabsTrigger value="tab2".to_string()>"Tab 2"</TabsTrigger>
                            </TabsList>
                            <TabsContent value="tab1".to_string()>"Content for tab 1"</TabsContent>
                            <TabsContent value="tab2".to_string()>"Content for tab 2"</TabsContent>
                        </Tabs>
                    </div>

                    // ContextMenu Component
                    <div id="context-menu" class="component-card bg-white rounded-lg shadow-md p-6">
                        <h3 class="text-xl font-semibold text-gray-900 mb-4">ContextMenu</h3>
                        <ContextMenu>
                            <ContextMenuTrigger>
                                <div class="p-4 border-2 border-dashed border-gray-300 rounded text-center">
                                    "Right-click me"
                                </div>
                            </ContextMenuTrigger>
                            <ContextMenuContent>
                                <ContextMenuItem>"Copy"</ContextMenuItem>
                                <ContextMenuItem>"Paste"</ContextMenuItem>
                            </ContextMenuContent>
                        </ContextMenu>
                    </div>

                    // DropdownMenu Component
                    <div id="dropdown-menu" class="component-card bg-white rounded-lg shadow-md p-6">
                        <h3 class="text-xl font-semibold text-gray-900 mb-4">DropdownMenu</h3>
                        <DropdownMenu>
                            <DropdownMenuTrigger>
                                <Button>"Open Menu"</Button>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent>
                                <DropdownMenuItem>"Item 1"</DropdownMenuItem>
                                <DropdownMenuItem>"Item 2"</DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </div>
                </div>
            </div>

            // Footer
            <div class="text-center py-12 bg-white rounded-lg shadow-md">
                <h2 class="text-2xl font-bold text-gray-900 mb-4">All Components Complete!</h2>
                <p class="text-lg text-gray-600 mb-6">
                    "Explore 20+ production-ready components built with Rust and WebAssembly"
                </p>
                <div class="flex justify-center space-x-4">
                    <a 
                        href="https://github.com/cloud-shuttle/radix-leptos" 
                        target="_blank"
                        class="bg-gray-900 hover:bg-gray-800 text-white px-6 py-3 rounded-lg font-medium transition-colors"
                    >
                        "View on GitHub"
                    </a>
                    <a 
                        href="/docs" 
                        class="bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg font-medium transition-colors"
                    >
                        "Read Documentation"
                    </a>
                </div>
            </div>
        </div>
    }
}

pub fn start_component_showcase() {
    mount_to_body(|| view! { <ComponentShowcase /> });
}
