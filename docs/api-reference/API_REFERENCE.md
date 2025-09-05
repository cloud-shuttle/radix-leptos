# Radix-Leptos Primitives API Reference

Complete API documentation for all 20+ components in the Radix-Leptos Primitives library.

## Table of Contents

- [Form Components](#form-components)
- [Feedback Components](#feedback-components)
- [Media Components](#media-components)
- [Navigation Components](#navigation-components)
- [Advanced Components](#advanced-components)
- [New Components (v0.7.0)](#new-components-v070)
- [Mobile Touch Components](#mobile-touch-components)

---

## Form Components

### Button

Accessible button component with multiple variants and states.

```rust
#[component]
pub fn Button(
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `variant`: `ButtonVariant` - Visual style variant (Default, Destructive, Outline, Ghost, Link)
- `size`: `ButtonSize` - Size variant (Small, Medium, Large)
- `disabled`: `bool` - Whether the button is disabled
- `on_click`: `Callback<()>` - Click event handler
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Button 
    variant=ButtonVariant::Primary 
    size=ButtonSize::Medium
    on_click=Callback::new(|_| println!("Clicked!"))
    class="bg-blue-500 hover:bg-blue-600"
>
    "Click me!"
</Button>
```

### Checkbox

Accessible checkbox with indeterminate state support.

```rust
#[component]
pub fn Checkbox(
    #[prop(optional)] checked: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_change: Option<Callback<bool>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView
```

**Props:**
- `checked`: `bool` - Whether the checkbox is checked
- `disabled`: `bool` - Whether the checkbox is disabled
- `on_change`: `Callback<bool>` - Change event handler
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Checkbox 
    checked=true 
    on_change=Callback::new(|state| println!("Checkbox: {}", state))
/>
```

### Switch

Toggle switch with smooth animations and accessibility.

```rust
#[component]
pub fn Switch(
    #[prop(optional)] checked: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_change: Option<Callback<bool>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView
```

**Props:**
- `checked`: `bool` - Whether the switch is on
- `disabled`: `bool` - Whether the switch is disabled
- `on_change`: `Callback<bool>` - Change event handler
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Switch 
    checked=true 
    on_change=Callback::new(|state| println!("Switch: {}", state))
/>
```

### RadioGroup

Radio button group with keyboard navigation.

```rust
#[component]
pub fn RadioGroup(
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `value`: `String` - Currently selected value
- `on_change`: `Callback<String>` - Change event handler
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<RadioGroup value="option1" on_change=Callback::new(|value| println!("Selected: {}", value))>
    <RadioGroupItem value="option1" />
    <RadioGroupItem value="option2" />
    <RadioGroupItem value="option3" />
</RadioGroup>
```

### TextInput

Text input field with validation and error states.

```rust
#[component]
pub fn TextInput(
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_input: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView
```

**Props:**
- `value`: `String` - Input value
- `placeholder`: `String` - Placeholder text
- `disabled`: `bool` - Whether the input is disabled
- `on_input`: `Callback<String>` - Input event handler
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<TextInput 
    placeholder="Enter your name..."
    on_input=Callback::new(|value| println!("Input: {}", value))
/>
```

### Select

Dropdown selection with search and keyboard support.

```rust
#[component]
pub fn Select(
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `value`: `String` - Currently selected value
- `on_change`: `Callback<String>` - Change event handler
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Select value="option1" on_change=Callback::new(|value| println!("Selected: {}", value))>
    <SelectTrigger>
        <SelectValue placeholder="Choose an option" />
    </SelectTrigger>
    <SelectContent>
        <SelectItem value="option1">"Option 1"</SelectItem>
        <SelectItem value="option2">"Option 2"</SelectItem>
    </SelectContent>
</Select>
```

---

## Feedback Components

### Toast

Notification system with auto-dismiss and actions.

```rust
#[component]
pub fn Toast(
    #[prop(optional)] variant: Option<ToastVariant>,
    #[prop(optional)] duration: Option<u32>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `variant`: `ToastVariant` - Visual style (Default, Success, Error, Warning)
- `duration`: `u32` - Auto-dismiss duration in milliseconds
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Toast variant=ToastVariant::Success duration=5000>
    <ToastTitle>"Success!"</ToastTitle>
    <ToastDescription>"Your action was completed successfully."</ToastDescription>
</Toast>
```

### Alert

Contextual alerts with multiple variants.

```rust
#[component]
pub fn Alert(
    #[prop(optional)] variant: Option<AlertVariant>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `variant`: `AlertVariant` - Visual style (Default, Info, Warning, Error, Success)
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Alert variant=AlertVariant::Info>
    <AlertTitle>"Information"</AlertTitle>
    <AlertDescription>"This is an informational alert."</AlertDescription>
</Alert>
```

### Badge

Status indicators and labels.

```rust
#[component]
pub fn Badge(
    #[prop(optional)] variant: Option<BadgeVariant>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `variant`: `BadgeVariant` - Visual style (Default, Secondary, Destructive, Outline)
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Badge variant=BadgeVariant::Default>"New"</Badge>
```

### Avatar

User profile images with fallbacks.

```rust
#[component]
pub fn Avatar(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Avatar>
    <AvatarImage src="https://github.com/user.png" alt="User" />
    <AvatarFallback>"JD"</AvatarFallback>
</Avatar>
```

---

## Media Components

### Image

Responsive image display with lazy loading.

```rust
#[component]
pub fn Image(
    #[prop(optional)] src: Option<String>,
    #[prop(optional)] alt: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView
```

**Props:**
- `src`: `String` - Image source URL
- `alt`: `String` - Alt text for accessibility
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Image 
    src="https://example.com/image.jpg" 
    alt="Description"
    class="w-full h-64 object-cover rounded"
/>
```

### Video

Video player with controls and accessibility.

```rust
#[component]
pub fn Video(
    #[prop(optional)] src: Option<String>,
    #[prop(optional)] controls: Option<bool>,
    #[prop(optional)] autoplay: Option<bool>,
    #[prop(optional)] muted: Option<bool>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView
```

**Props:**
- `src`: `String` - Video source URL
- `controls`: `bool` - Show video controls
- `autoplay`: `bool` - Auto-play video
- `muted`: `bool` - Mute video
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Video 
    src="https://example.com/video.mp4"
    controls=true
    class="w-full h-64"
/>
```

### Audio

Audio player with waveform visualization.

```rust
#[component]
pub fn Audio(
    #[prop(optional)] src: Option<String>,
    #[prop(optional)] controls: Option<bool>,
    #[prop(optional)] autoplay: Option<bool>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView
```

**Props:**
- `src`: `String` - Audio source URL
- `controls`: `bool` - Show audio controls
- `autoplay`: `bool` - Auto-play audio
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Audio 
    src="https://example.com/audio.mp3"
    controls=true
/>
```

### Carousel

Image carousel with navigation and indicators.

```rust
#[component]
pub fn Carousel(
    #[prop(optional)] auto_play: Option<bool>,
    #[prop(optional)] loop_carousel: Option<bool>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `auto_play`: `bool` - Auto-play carousel
- `loop_carousel`: `bool` - Loop carousel
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Carousel auto_play=true loop_carousel=true>
    <CarouselContent>
        <CarouselItem>
            <div class="p-1">
                <div class="bg-gray-100 rounded-lg p-4 text-center">"Slide 1"</div>
            </div>
        </CarouselItem>
        <CarouselItem>
            <div class="p-1">
                <div class="bg-gray-100 rounded-lg p-4 text-center">"Slide 2"</div>
            </div>
        </CarouselItem>
    </CarouselContent>
    <CarouselPrevious />
    <CarouselNext />
</Carousel>
```

---

## Navigation Components

### Pagination

Page navigation with configurable layouts.

```rust
#[component]
pub fn Pagination(
    #[prop(optional)] total_pages: Option<u32>,
    #[prop(optional)] current_page: Option<u32>,
    #[prop(optional)] on_page_change: Option<Callback<u32>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView
```

**Props:**
- `total_pages`: `u32` - Total number of pages
- `current_page`: `u32` - Current page number
- `on_page_change`: `Callback<u32>` - Page change handler
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Pagination 
    total_pages=10 
    current_page=1 
    on_page_change=Callback::new(|page| println!("Page: {}", page))
/>
```

### List

Virtualized lists for large datasets.

```rust
#[component]
pub fn List(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<List class="h-64">
    <ListItem>"Item 1"</ListItem>
    <ListItem>"Item 2"</ListItem>
    <ListItem>"Item 3"</ListItem>
</List>
```

### Timeline

Chronological data presentation.

```rust
#[component]
pub fn Timeline(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Timeline>
    <TimelineItem>
        <TimelineSeparator>
            <TimelineDot />
            <TimelineConnector />
        </TimelineSeparator>
        <TimelineContent>
            <TimelineHeading>"Event 1"</TimelineHeading>
            <TimelineDescription>"Description of event 1"</TimelineDescription>
        </TimelineContent>
    </TimelineItem>
</Timeline>
```

---

## Advanced Components

### Accordion

Collapsible content sections.

```rust
#[component]
pub fn Accordion(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Accordion>
    <AccordionItem value="item-1">
        <AccordionTrigger>"Section 1"</AccordionTrigger>
        <AccordionContent>"Content for section 1"</AccordionContent>
    </AccordionItem>
</Accordion>
```

### Tabs

Tabbed navigation with keyboard support.

```rust
#[component]
pub fn Tabs(
    #[prop(optional)] orientation: Option<TabsOrientation>,
    #[prop(optional)] size: Option<TabsSize>,
    #[prop(optional)] variant: Option<TabsVariant>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `orientation`: `TabsOrientation` - Horizontal or vertical orientation
- `size`: `TabsSize` - Size variant (Small, Medium, Large)
- `variant`: `TabsVariant` - Visual style (Default, Pills, Underlined)
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Tabs orientation=TabsOrientation::Horizontal>
    <TabsList>
        <TabsTrigger value="tab1">"Tab 1"</TabsTrigger>
        <TabsTrigger value="tab2">"Tab 2"</TabsTrigger>
    </TabsList>
    <TabsContent value="tab1">"Content for tab 1"</TabsContent>
    <TabsContent value="tab2">"Content for tab 2"</TabsContent>
</Tabs>
```

### Dialog

Modal dialogs with focus management.

```rust
#[component]
pub fn Dialog(
    #[prop(optional)] open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `open`: `bool` - Whether dialog is open
- `on_open_change`: `Callback<bool>` - Open state change handler
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Dialog open=true on_open_change=Callback::new(|open| println!("Dialog: {}", open))>
    <DialogTrigger>
        <Button>"Open Dialog"</Button>
    </DialogTrigger>
    <DialogContent>
        <DialogTitle>"Dialog Title"</DialogTitle>
        <DialogDescription>"Dialog description goes here."</DialogDescription>
    </DialogContent>
</Dialog>
```

### ContextMenu

Right-click context menus.

```rust
#[component]
pub fn ContextMenu(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
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
```

### DropdownMenu

Dropdown menus with submenus.

```rust
#[component]
pub fn DropdownMenu(
    #[prop(optional)] open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `open`: `bool` - Whether menu is open
- `on_open_change`: `Callback<bool>` - Open state change handler
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<DropdownMenu open=false on_open_change=Callback::new(|open| println!("Menu: {}", open))>
    <DropdownMenuTrigger>
        <Button>"Open Menu"</Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent>
        <DropdownMenuItem>"Item 1"</DropdownMenuItem>
        <DropdownMenuItem>"Item 2"</DropdownMenuItem>
    </DropdownMenuContent>
</DropdownMenu>
```

### Menubar

Horizontal menu bars with keyboard navigation.

```rust
#[component]
pub fn Menubar(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Menubar>
    <MenubarMenu>
        <MenubarTrigger>"File"</MenubarTrigger>
        <MenubarContent>
            <MenubarItem>"New"</MenubarItem>
            <MenubarItem>"Open"</MenubarItem>
        </MenubarContent>
    </MenubarMenu>
</Menubar>
```

### ScrollArea

Custom scrollable areas with styling.

```rust
#[component]
pub fn ScrollArea(
    #[prop(optional)] orientation: Option<ScrollAreaOrientation>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `orientation`: `ScrollAreaOrientation` - Scroll direction (Horizontal, Vertical, Both)
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<ScrollArea orientation=ScrollAreaOrientation::Vertical class="h-32 w-full">
    <div class="space-y-2">
        {(1..=20).map(|i| view! {
            <div class="p-2 bg-gray-100 rounded">
                "Item " {i}
            </div>
        })}
    </div>
</ScrollArea>
```

### Table

Data tables with sorting and pagination.

```rust
#[component]
pub fn Table(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Table>
    <TableHeader>
        <TableRow>
            <TableHead>"Name"</TableHead>
            <TableHead>"Email"</TableHead>
        </TableRow>
    </TableHeader>
    <TableBody>
        <TableRow>
            <TableCell>"John Doe"</TableCell>
            <TableCell>"john@example.com"</TableCell>
        </TableRow>
    </TableBody>
</Table>
```

---

## Common Types

### Button Variants
```rust
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Ghost,
    Link,
}
```

### Button Sizes
```rust
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}
```

### Alert Variants
```rust
pub enum AlertVariant {
    Default,
    Info,
    Warning,
    Error,
    Success,
}
```

### Toast Variants
```rust
pub enum ToastVariant {
    Default,
    Success,
    Error,
    Warning,
}
```

### Badge Variants
```rust
pub enum BadgeVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
}
```

### Tabs Orientation
```rust
pub enum TabsOrientation {
    Horizontal,
    Vertical,
}
```

### Tabs Variants
```rust
pub enum TabsVariant {
    Default,
    Pills,
    Underlined,
}
```

### Scroll Area Orientation
```rust
pub enum ScrollAreaOrientation {
    Horizontal,
    Vertical,
    Both,
}
```

---

## Accessibility Features

All components include:

- **ARIA Attributes**: Proper ARIA labels, descriptions, and states
- **Keyboard Navigation**: Full keyboard accessibility
- **Focus Management**: Logical tab order and focus indicators
- **Screen Reader Support**: Semantic HTML and ARIA landmarks
- **Color Contrast**: Meets WCAG 2.1 AA contrast requirements
- **Motion Preferences**: Respects `prefers-reduced-motion`

## Styling

All components accept:
- `class`: Additional CSS classes (Tailwind CSS compatible)
- `style`: Inline styles for custom styling

Components are designed to work seamlessly with Tailwind CSS but can be styled with any CSS framework.

## Performance

- **WebAssembly Powered**: High-performance Rust components
- **Optimized Rendering**: Efficient virtual DOM updates
- **Bundle Size**: Minimal impact on application size
- **Memory Management**: Efficient memory usage patterns

---

## New Components (v0.7.0)

### AlertDialog

Modal dialogs for important actions and confirmations.

```rust
#[component]
pub fn AlertDialog(
    #[prop(optional)] open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `open`: `bool` - Whether dialog is open
- `on_open_change`: `Callback<bool>` - Open state change handler
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<AlertDialog open=true on_open_change=Callback::new(|open| println!("AlertDialog: {}", open))>
    <AlertDialogTrigger>
        <Button>"Delete Account"</Button>
    </AlertDialogTrigger>
    <AlertDialogContent>
        <AlertDialogHeader>
            <AlertDialogTitle>"Are you absolutely sure?"</AlertDialogTitle>
            <AlertDialogDescription>
                "This action cannot be undone. This will permanently delete your account."
            </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
            <AlertDialogCancel>"Cancel"</AlertDialogCancel>
            <AlertDialogAction>"Continue"</AlertDialogAction>
        </AlertDialogFooter>
    </AlertDialogContent>
</AlertDialog>
```

### Sheet

Slide-out panels and drawers for mobile-friendly interfaces.

```rust
#[component]
pub fn Sheet(
    #[prop(optional)] open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] side: Option<SheetSide>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `open`: `bool` - Whether sheet is open
- `on_open_change`: `Callback<bool>` - Open state change handler
- `side`: `SheetSide` - Which side to slide from (Top, Right, Bottom, Left)
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<Sheet open=true on_open_change=Callback::new(|open| println!("Sheet: {}", open))>
    <SheetTrigger>
        <Button>"Open Sheet"</Button>
    </SheetTrigger>
    <SheetContent side=SheetSide::Right>
        <SheetHeader>
            <SheetTitle>"Edit Profile"</SheetTitle>
            <SheetDescription>
                "Make changes to your profile here. Click save when you're done."
            </SheetDescription>
        </SheetHeader>
        <div class="grid gap-4 py-4">
            <div class="grid grid-cols-4 items-center gap-4">
                <Label for="name" class="text-right">"Name"</Label>
                <Input id="name" value="Pedro Duarte" class="col-span-3" />
            </div>
        </div>
        <SheetFooter>
            <SheetClose>
                <Button type="submit">"Save changes"</Button>
            </SheetClose>
        </SheetFooter>
    </SheetContent>
</Sheet>
```

### Skeleton

Loading placeholders with multiple variants for better UX.

```rust
#[component]
pub fn Skeleton(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] width: Option<usize>,
    #[prop(optional)] height: Option<usize>,
    #[prop(optional)] rounded: Option<bool>,
    #[prop(optional)] animated: Option<bool>,
) -> impl IntoView
```

**Props:**
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles
- `width`: `usize` - Width in pixels
- `height`: `usize` - Height in pixels
- `rounded`: `bool` - Whether to apply rounded corners
- `animated`: `bool` - Whether to show loading animation

**Example:**
```rust
<div class="flex items-center space-x-4">
    <Skeleton class="h-12 w-12 rounded-full" />
    <div class="space-y-2">
        <Skeleton class="h-4 w-[250px]" />
        <Skeleton class="h-4 w-[200px]" />
    </div>
</div>
```

### DatePicker

Calendar interface for date selection with validation.

```rust
#[component]
pub fn DatePicker(
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] min_date: Option<String>,
    #[prop(optional)] max_date: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] format: Option<String>,
    #[prop(optional)] locale: Option<String>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] on_validation: Option<Callback<DateValidation>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `value`: `String` - Selected date value
- `placeholder`: `String` - Placeholder text
- `min_date`: `String` - Minimum selectable date
- `max_date`: `String` - Maximum selectable date
- `disabled`: `bool` - Whether the picker is disabled
- `required`: `bool` - Whether the field is required
- `format`: `String` - Date format string
- `locale`: `String` - Locale for date formatting
- `on_change`: `Callback<String>` - Date change handler
- `on_validation`: `Callback<DateValidation>` - Validation handler
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<DatePicker 
    placeholder="Select a date"
    min_date="2024-01-01"
    max_date="2024-12-31"
    format="YYYY-MM-DD"
    on_change=Callback::new(|date| println!("Selected: {}", date))
/>
```

### MultiSelect

Multi-selection dropdown with search functionality.

```rust
#[component]
pub fn MultiSelect(
    #[prop(optional)] options: Option<Vec<SelectOption>>,
    #[prop(optional)] selected: Option<Vec<String>>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] searchable: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_change: Option<Callback<Vec<String>>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `options`: `Vec<SelectOption>` - Available options
- `selected`: `Vec<String>` - Currently selected values
- `placeholder`: `String` - Placeholder text
- `searchable`: `bool` - Whether to enable search
- `disabled`: `bool` - Whether the select is disabled
- `on_change`: `Callback<Vec<String>>` - Selection change handler
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<MultiSelect 
    options=vec![
        SelectOption { value: "react".to_string(), label: "React".to_string() },
        SelectOption { value: "vue".to_string(), label: "Vue".to_string() },
        SelectOption { value: "angular".to_string(), label: "Angular".to_string() },
    ]
    selected=vec!["react".to_string()]
    searchable=true
    placeholder="Select frameworks"
    on_change=Callback::new(|selected| println!("Selected: {:?}", selected))
/>
```

### DataTable

Data tables with sorting, filtering, and pagination capabilities.

```rust
#[component]
pub fn DataTable<T>(
    #[prop(optional)] data: Option<Vec<T>>,
    #[prop(optional)] columns: Option<Vec<TableColumn>>,
    #[prop(optional)] sortable: Option<bool>,
    #[prop(optional)] filterable: Option<bool>,
    #[prop(optional)] paginated: Option<bool>,
    #[prop(optional)] page_size: Option<usize>,
    #[prop(optional)] on_sort: Option<Callback<SortConfig>>,
    #[prop(optional)] on_filter: Option<Callback<FilterConfig>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView
where
    T: Clone + 'static,
```

**Props:**
- `data`: `Vec<T>` - Table data
- `columns`: `Vec<TableColumn>` - Column definitions
- `sortable`: `bool` - Whether columns are sortable
- `filterable`: `bool` - Whether table is filterable
- `paginated`: `bool` - Whether to show pagination
- `page_size`: `usize` - Number of items per page
- `on_sort`: `Callback<SortConfig>` - Sort change handler
- `on_filter`: `Callback<FilterConfig>` - Filter change handler
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

**Example:**
```rust
<DataTable 
    data=users
    columns=vec![
        TableColumn { key: "name".to_string(), label: "Name".to_string(), sortable: true },
        TableColumn { key: "email".to_string(), label: "Email".to_string(), sortable: true },
        TableColumn { key: "role".to_string(), label: "Role".to_string(), sortable: false },
    ]
    sortable=true
    filterable=true
    paginated=true
    page_size=10
    on_sort=Callback::new(|config| println!("Sort: {:?}", config))
/>
```

---

## Mobile Touch Components

### TouchButton

Touch-optimized button with haptic feedback.

```rust
#[component]
pub fn TouchButton(
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    #[prop(optional)] haptic_feedback: Option<bool>,
    #[prop(optional)] touch_delay: Option<u32>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `variant`: `ButtonVariant` - Visual style variant
- `size`: `ButtonSize` - Size variant
- `haptic_feedback`: `bool` - Whether to provide haptic feedback
- `touch_delay`: `u32` - Touch delay in milliseconds
- `disabled`: `bool` - Whether the button is disabled
- `on_click`: `Callback<()>` - Click event handler
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

### SwipeableCard

Card component with swipe gestures for mobile interactions.

```rust
#[component]
pub fn SwipeableCard(
    #[prop(optional)] swipe_threshold: Option<f32>,
    #[prop(optional)] on_swipe_left: Option<Callback<()>>,
    #[prop(optional)] on_swipe_right: Option<Callback<()>>,
    #[prop(optional)] on_swipe_up: Option<Callback<()>>,
    #[prop(optional)] on_swipe_down: Option<Callback<()>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `swipe_threshold`: `f32` - Minimum swipe distance to trigger action
- `on_swipe_left`: `Callback<()>` - Left swipe handler
- `on_swipe_right`: `Callback<()>` - Right swipe handler
- `on_swipe_up`: `Callback<()>` - Up swipe handler
- `on_swipe_down`: `Callback<()>` - Down swipe handler
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

### PullToRefresh

Pull-to-refresh functionality for mobile lists.

```rust
#[component]
pub fn PullToRefresh(
    #[prop(optional)] threshold: Option<f32>,
    #[prop(optional)] on_refresh: Option<Callback<()>>,
    #[prop(optional)] refreshing: Option<bool>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView
```

**Props:**
- `threshold`: `f32` - Pull distance threshold to trigger refresh
- `on_refresh`: `Callback<()>` - Refresh handler
- `refreshing`: `bool` - Whether currently refreshing
- `class`: `String` - Additional CSS classes
- `style`: `String` - Inline styles

---

## Common Types (Updated)

### Sheet Side
```rust
pub enum SheetSide {
    Top,
    Right,
    Bottom,
    Left,
}
```

### Date Validation
```rust
pub struct DateValidation {
    pub is_valid: bool,
    pub error_message: Option<String>,
    pub parsed_date: Option<String>,
}
```

### Select Option
```rust
pub struct SelectOption {
    pub value: String,
    pub label: String,
}
```

### Table Column
```rust
pub struct TableColumn {
    pub key: String,
    pub label: String,
    pub sortable: bool,
}
```

### Sort Config
```rust
pub struct SortConfig {
    pub column: String,
    pub direction: SortDirection,
}

pub enum SortDirection {
    Ascending,
    Descending,
}
```

### Filter Config
```rust
pub struct FilterConfig {
    pub column: String,
    pub value: String,
    pub operator: FilterOperator,
}

pub enum FilterOperator {
    Equals,
    Contains,
    StartsWith,
    EndsWith,
}
```

---

For more detailed examples and usage patterns, see the [Component Showcase](../examples/component_showcase.html) and individual component example pages.
