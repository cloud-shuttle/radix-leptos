# Component Gallery

A comprehensive showcase of all Radix-Leptos components with live examples and code snippets.

## Table of Contents

- [Layout Components](#layout-components)
- [Form Components](#form-components)
- [Navigation Components](#navigation-components)
- [Data Display Components](#data-display-components)
- [Feedback Components](#feedback-components)
- [Overlay Components](#overlay-components)
- [Media Components](#media-components)
- [Utility Components](#utility-components)

## Layout Components

### Container

```rust
<Container>
    <h1>"Page Title"</h1>
    <p>"Page content goes here"</p>
</Container>
```

**Features:**
- Responsive max-width
- Centered content
- Padding and margins

### Grid

```rust
<Grid columns=3 gap="1rem">
    <GridItem>"Item 1"</GridItem>
    <GridItem>"Item 2"</GridItem>
    <GridItem>"Item 3"</GridItem>
</Grid>
```

**Features:**
- Flexible column layout
- Responsive breakpoints
- Gap control

### Flex

```rust
<Flex direction="row" justify="space-between" align="center">
    <div>"Left content"</div>
    <div>"Right content"</div>
</Flex>
```

**Features:**
- Direction control
- Justification options
- Alignment control

## Form Components

### Input

```rust
<Input
    type="text"
    placeholder="Enter text"
    value=value
    on_change=set_value
/>
```

**Variants:**
- Text, email, password, number
- Search, tel, url
- Date, time, datetime-local

### Textarea

```rust
<Textarea
    placeholder="Enter message"
    rows=4
    value=message
    on_change=set_message
/>
```

**Features:**
- Auto-resize
- Character count
- Validation states

### Select

```rust
<Select
    value=selected_value
    on_value_change=set_selected_value
>
    <SelectTrigger>
        <SelectValue placeholder="Select an option" />
    </SelectTrigger>
    <SelectContent>
        <SelectItem value="option1">"Option 1"</SelectItem>
        <SelectItem value="option2">"Option 2"</SelectItem>
        <SelectItem value="option3">"Option 3"</SelectItem>
    </SelectContent>
</Select>
```

**Features:**
- Single and multi-select
- Search functionality
- Keyboard navigation

### Checkbox

```rust
<Checkbox
    checked=is_checked
    on_change=set_is_checked
    label="Accept terms and conditions"
/>
```

**Features:**
- Indeterminate state
- Custom styling
- Accessibility support

### Radio Group

```rust
<RadioGroup
    value=selected_option
    on_value_change=set_selected_option
>
    <RadioGroupItem value="option1">"Option 1"</RadioGroupItem>
    <RadioGroupItem value="option2">"Option 2"</RadioGroupItem>
    <RadioGroupItem value="option3">"Option 3"</RadioGroupItem>
</RadioGroup>
```

**Features:**
- Single selection
- Keyboard navigation
- Custom styling

### Switch

```rust
<Switch
    checked=is_enabled
    on_change=set_is_enabled
    label="Enable notifications"
/>
```

**Features:**
- Toggle functionality
- Custom styling
- Accessibility support

### Slider

```rust
<Slider
    value=slider_value
    on_change=set_slider_value
    min=0
    max=100
    step=1
/>
```

**Features:**
- Range selection
- Step control
- Custom styling

### Date Picker

```rust
<DatePicker
    value=selected_date
    on_date_select=set_selected_date
    min_date="2024-01-01"
    max_date="2024-12-31"
/>
```

**Features:**
- Date validation
- Range restrictions
- Custom formatting

### Time Picker

```rust
<TimePicker
    value=selected_time
    on_time_select=set_selected_time
    format="24h"
/>
```

**Features:**
- 12/24 hour format
- Time validation
- Custom styling

## Navigation Components

### Tabs

```rust
<Tabs value=active_tab on_value_change=set_active_tab>
    <TabsList>
        <TabsTrigger value="tab1">"Overview"</TabsTrigger>
        <TabsTrigger value="tab2">"Details"</TabsTrigger>
        <TabsTrigger value="tab3">"Settings"</TabsTrigger>
    </TabsList>
    
    <TabsContent value="tab1">
        <div>"Overview content"</div>
    </TabsContent>
    <TabsContent value="tab2">
        <div>"Details content"</div>
    </TabsContent>
    <TabsContent value="tab3">
        <div>"Settings content"</div>
    </TabsContent>
</Tabs>
```

**Features:**
- Keyboard navigation
- Accessibility support
- Custom styling

### Navigation Menu

```rust
<NavigationMenu>
    <NavigationMenuList>
        <NavigationMenuItem>
            <NavigationMenuTrigger>"Products"</NavigationMenuTrigger>
            <NavigationMenuContent>
                <div class="navigation-content">
                    <h3>"Product Categories"</h3>
                    <ul>
                        <li><a href="/electronics">"Electronics"</a></li>
                        <li><a href="/clothing">"Clothing"</a></li>
                        <li><a href="/books">"Books"</a></li>
                    </ul>
                </div>
            </NavigationMenuContent>
        </NavigationMenuItem>
    </NavigationMenuList>
</NavigationMenu>
```

**Features:**
- Dropdown menus
- Keyboard navigation
- Accessibility support

### Breadcrumb

```rust
<Breadcrumb>
    <BreadcrumbList>
        <BreadcrumbItem>
            <BreadcrumbLink href="/">"Home"</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
            <BreadcrumbLink href="/products">"Products"</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
            <BreadcrumbPage>"Current Page"</BreadcrumbPage>
        </BreadcrumbItem>
    </BreadcrumbList>
</Breadcrumb>
```

**Features:**
- Hierarchical navigation
- Custom separators
- Accessibility support

### Pagination

```rust
<Pagination
    current_page=current_page
    total_pages=total_pages
    on_page_change=set_current_page
/>
```

**Features:**
- Page navigation
- Custom styling
- Accessibility support

## Data Display Components

### Table

```rust
<Table>
    <TableHeader>
        <TableRow>
            <TableHead>"Name"</TableHead>
            <TableHead>"Email"</TableHead>
            <TableHead>"Role"</TableHead>
        </TableRow>
    </TableHeader>
    <TableBody>
        {users.into_iter().map(|user| {
            view! {
                <TableRow>
                    <TableCell>{user.name}</TableCell>
                    <TableCell>{user.email}</TableCell>
                    <TableCell>{user.role}</TableCell>
                </TableRow>
            }
        }).collect::<Vec<_>>()}
    </TableBody>
</Table>
```

**Features:**
- Sortable columns
- Responsive design
- Accessibility support

### Card

```rust
<Card>
    <CardHeader>
        <CardTitle>"Card Title"</CardTitle>
        <CardDescription>"Card description"</CardDescription>
    </CardHeader>
    <CardContent>
        <p>"Card content goes here"</p>
    </CardContent>
    <CardFooter>
        <Button>"Action"</Button>
    </CardFooter>
</Card>
```

**Features:**
- Flexible layout
- Custom styling
- Accessibility support

### Badge

```rust
<Badge variant=BadgeVariant::Default>"Default"</Badge>
<Badge variant=BadgeVariant::Secondary>"Secondary"</Badge>
<Badge variant=BadgeVariant::Destructive>"Destructive"</Badge>
<Badge variant=BadgeVariant::Outline>"Outline"</Badge>
```

**Features:**
- Multiple variants
- Custom styling
- Accessibility support

### Avatar

```rust
<Avatar>
    <AvatarImage src="user.jpg" alt="User" />
    <AvatarFallback>"JD"</AvatarFallback>
</Avatar>
```

**Features:**
- Image support
- Fallback text
- Custom styling

### Progress

```rust
<Progress value=progress_value max=100 />
```

**Features:**
- Animated progress
- Custom styling
- Accessibility support

### Skeleton

```rust
<Skeleton width="100%" height="20px" />
<Skeleton width="80%" height="20px" />
<Skeleton width="60%" height="20px" />
```

**Features:**
- Loading states
- Custom dimensions
- Animation control

## Feedback Components

### Alert

```rust
<Alert variant=AlertVariant::Default>
    <AlertTitle>"Alert Title"</AlertTitle>
    <AlertDescription>"Alert description"</AlertDescription>
</Alert>
```

**Variants:**
- Default, Destructive, Warning, Info
- Custom styling
- Accessibility support

### Toast

```rust
<Toast
    title="Toast Title"
    description="Toast description"
    variant=ToastVariant::Default
/>
```

**Features:**
- Auto-dismiss
- Custom positioning
- Accessibility support

### Dialog

```rust
<Dialog open=is_open on_open_change=set_is_open>
    <DialogTrigger>
        <Button>"Open Dialog"</Button>
    </DialogTrigger>
    <DialogContent>
        <DialogHeader>
            <DialogTitle>"Dialog Title"</DialogTitle>
            <DialogDescription>"Dialog description"</DialogDescription>
        </DialogHeader>
        <DialogBody>
            <p>"Dialog content goes here"</p>
        </DialogBody>
        <DialogFooter>
            <Button variant=ButtonVariant::Outline>"Cancel"</Button>
            <Button>"Confirm"</Button>
        </DialogFooter>
    </DialogContent>
</Dialog>
```

**Features:**
- Modal behavior
- Keyboard navigation
- Accessibility support

### Alert Dialog

```rust
<AlertDialog open=is_open on_open_change=set_is_open>
    <AlertDialogTrigger>
        <Button variant=ButtonVariant::Destructive>"Delete"</Button>
    </AlertDialogTrigger>
    <AlertDialogContent>
        <AlertDialogHeader>
            <AlertDialogTitle>"Are you sure?"</AlertDialogTitle>
            <AlertDialogDescription>
                "This action cannot be undone. This will permanently delete your account."
            </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
            <AlertDialogCancel>"Cancel"</AlertDialogCancel>
            <AlertDialogAction>"Delete"</AlertDialogAction>
        </AlertDialogFooter>
    </AlertDialogContent>
</AlertDialog>
```

**Features:**
- Confirmation dialogs
- Destructive actions
- Accessibility support

### Sheet

```rust
<Sheet open=is_open on_open_change=set_is_open>
    <SheetTrigger>
        <Button>"Open Sheet"</Button>
    </SheetTrigger>
    <SheetContent>
        <SheetHeader>
            <SheetTitle>"Sheet Title"</SheetTitle>
            <SheetDescription>"Sheet description"</SheetDescription>
        </SheetHeader>
        <SheetBody>
            <p>"Sheet content goes here"</p>
        </SheetBody>
    </SheetContent>
</Sheet>
```

**Features:**
- Side panels
- Custom positioning
- Accessibility support

## Overlay Components

### Popover

```rust
<Popover open=is_open on_open_change=set_is_open>
    <PopoverTrigger>
        <Button>"Open Popover"</Button>
    </PopoverTrigger>
    <PopoverContent>
        <PopoverHeader>
            <PopoverTitle>"Popover Title"</PopoverTitle>
        </PopoverHeader>
        <PopoverBody>
            <p>"Popover content goes here"</p>
        </PopoverBody>
    </PopoverContent>
</Popover>
```

**Features:**
- Positioning control
- Custom styling
- Accessibility support

### Tooltip

```rust
<Tooltip>
    <TooltipTrigger>
        <Button>"Hover me"</Button>
    </TooltipTrigger>
    <TooltipContent>
        <p>"Tooltip content"</p>
    </TooltipContent>
</Tooltip>
```

**Features:**
- Hover behavior
- Custom positioning
- Accessibility support

### Hover Card

```rust
<HoverCard open=is_open on_open_change=set_is_open>
    <HoverCardTrigger>
        <Button>"Hover me"</Button>
    </HoverCardTrigger>
    <HoverCardContent>
        <HoverCardHeader>
            <HoverCardTitle>"Hover Card Title"</HoverCardTitle>
            <HoverCardDescription>"Hover card description"</HoverCardDescription>
        </HoverCardHeader>
        <HoverCardBody>
            <p>"Hover card content goes here"</p>
        </HoverCardBody>
    </HoverCardContent>
</HoverCard>
```

**Features:**
- Hover behavior
- Custom styling
- Accessibility support

### Context Menu

```rust
<ContextMenu>
    <ContextMenuTrigger>
        <div>"Right-click me"</div>
    </ContextMenuTrigger>
    <ContextMenuContent>
        <ContextMenuItem>"Copy"</ContextMenuItem>
        <ContextMenuItem>"Paste"</ContextMenuItem>
        <ContextMenuSeparator />
        <ContextMenuItem>"Delete"</ContextMenuItem>
    </ContextMenuContent>
</ContextMenu>
```

**Features:**
- Right-click behavior
- Custom styling
- Accessibility support

## Media Components

### Image

```rust
<Image
    src="image.jpg"
    alt="Image description"
    width=400
    height=300
    loading="lazy"
/>
```

**Features:**
- Lazy loading
- Responsive images
- Accessibility support

### Video

```rust
<Video
    src="video.mp4"
    poster="poster.jpg"
    controls=true
    autoplay=false
/>
```

**Features:**
- Custom controls
- Poster images
- Accessibility support

### Audio

```rust
<Audio
    src="audio.mp3"
    controls=true
    autoplay=false
/>
```

**Features:**
- Custom controls
- Playback control
- Accessibility support

## Utility Components

### Separator

```rust
<div>"Content above"</div>
<Separator />
<div>"Content below"</div>
```

**Features:**
- Visual separation
- Custom styling
- Accessibility support

### Scroll Area

```rust
<ScrollArea height="200px">
    <div class="content">
        <p>"Long content that scrolls"</p>
        <p>"More content"</p>
        <p>"Even more content"</p>
    </div>
</ScrollArea>
```

**Features:**
- Custom scrollbars
- Smooth scrolling
- Accessibility support

### Resizable

```rust
<Resizable
    width=300
    height=200
    min_width=200
    min_height=100
    max_width=500
    max_height=400
>
    <div>"Resizable content"</div>
</Resizable>
```

**Features:**
- Resize handles
- Size constraints
- Accessibility support

### Virtual List

```rust
<VirtualList
    items=large_dataset
    item_height=50.0
    container_height=400.0
    overscan=5
    render_item=Callback::new(|item| {
        view! { <div class="virtual-item">{item}</div> }
    })
/>
```

**Features:**
- High performance
- Large datasets
- Custom rendering

## Conclusion

This component gallery showcases the full range of Radix-Leptos components, each designed with:

- **Accessibility**: Full ARIA support and keyboard navigation
- **Performance**: Optimized rendering and memory usage
- **Flexibility**: Customizable styling and behavior
- **Consistency**: Unified design system and API

For more detailed examples, see the [Interactive Examples](interactive-examples.md) and [Advanced Patterns](advanced-patterns.md).
