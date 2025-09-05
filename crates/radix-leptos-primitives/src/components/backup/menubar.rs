
#[derive(Clone, Debug, PartialEq)]
pub enum MenubarSize {
    Small,
    Medium,
    Large,
}

impl Default for MenubarSize {
    fn default() -> Self {
        MenubarSize::Medium
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MenubarItemVariant {
    Default,
    Destructive,
    Disabled,
}

impl Default for MenubarItemVariant {
    fn default() -> Self {
        MenubarItemVariant::Default
    }
}

fn merge_classes(base: &str, custom: Option<String>) -> String {
    let class_value = custom.unwrap_or_default();
    let final_class = format!("{} {}", base, class_value);
    final_class.trim().to_string()
}

#[component]
pub fn Menubar(
    #[prop(optional)] size: Option<MenubarSize>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let size = size.unwrap_or_default();
    let base_classes = match size {
        MenubarSize::Small => "flex h-8 items-center space-x-1 rounded-md border bg-background p-1",
        MenubarSize::Medium => "flex h-10 items-center space-x-1 rounded-md border bg-background p-1",
        MenubarSize::Large => "flex h-12 items-center space-x-1 rounded-md border bg-background p-1",
    };
    
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    view! {
        <div class={final_class} style={style_attr} data-radix-menubar-root="">
            {children()}
        </div>
    }
}

#[component]
pub fn MenubarTrigger(
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);
    let base_classes = "flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm font-medium outline-none focus:bg-accent focus:text-accent-foreground data-[state=open]:bg-accent data-[state=open]:text-accent-foreground disabled:pointer-events-none disabled:opacity-50";
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    let handle_click = move |_| {
        if !disabled {
            if let Some(callback) = on_click {
                callback.run(());
            }
        }
    };

    let handle_keydown = move |event: web_sys::KeyboardEvent| {
        if !disabled && (event.key() == "Enter" || event.key() == " ") {
            event.prevent_default();
            if let Some(callback) = on_click {
                callback.run(());
            }
        }
    };

    view! {
        <button
            class={final_class}
            style={style_attr}
            disabled={disabled}
            on:click=handle_click
            on:keydown=handle_keydown
            data-radix-menubar-trigger=""
        >
            {children()}
        </button>
    }
}

#[component]
pub fn MenubarContent(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let base_classes = "z-50 min-w-[12rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2";
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    view! {
        <div class={final_class} style={style_attr} data-radix-menubar-content="">
            {children()}
        </div>
    }
}

#[component]
pub fn MenubarItem(
    #[prop(optional)] variant: Option<MenubarItemVariant>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);
    
    let base_classes = match variant {
        MenubarItemVariant::Default => "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
        MenubarItemVariant::Destructive => "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 text-destructive focus:text-destructive",
        MenubarItemVariant::Disabled => "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 opacity-50 cursor-not-allowed",
    };
    
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    let handle_click = move |_| {
        if !disabled {
            if let Some(callback) = on_click {
                callback.run(());
            }
        }
    };

    let handle_keydown = move |event: web_sys::KeyboardEvent| {
        if !disabled && (event.key() == "Enter" || event.key() == " ") {
            event.prevent_default();
            if let Some(callback) = on_click {
                callback.run(());
            }
        }
    };

    view! {
        <div
            class={final_class}
            style={style_attr}
            data-disabled={disabled}
            on:click=handle_click
            on:keydown=handle_keydown
            data-radix-menubar-item=""
        >
            {children()}
        </div>
    }
}

#[component]
pub fn MenubarSeparator(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let base_classes = "-mx-1 my-1 h-px bg-muted";
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    view! {
        <div class={final_class} style={style_attr} data-radix-menubar-separator=""></div>
    }
}

#[component]
pub fn MenubarLabel(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let base_classes = "px-2 py-1.5 text-sm font-semibold";
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    view! {
        <div class={final_class} style={style_attr} data-radix-menubar-label="">
            {children()}
        </div>
    }
}

#[component]
pub fn MenubarCheckboxItem(
    #[prop(optional)] checked: Option<ReadSignal<bool>>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] onchecked_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let checked_value = checked.map(|c| c.get()).unwrap_or(false);
    let disabled = disabled.unwrap_or(false);
    let base_classes = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    let checked_signal = checked.clone();
    let handle_click = move |_| {
        if !disabled {
            if let Some(callback) = onchecked_change {
                let current_value = checked_signal.as_ref().map(|c| c.get()).unwrap_or(false);
                callback.run(!current_value);
            }
        }
    };

    let checked_signal = checked.clone();
    let handle_keydown = move |event: web_sys::KeyboardEvent| {
        if !disabled && (event.key() == "Enter" || event.key() == " ") {
            event.prevent_default();
            if let Some(callback) = onchecked_change {
                let current_value = checked_signal.as_ref().map(|c| c.get()).unwrap_or(false);
                callback.run(!current_value);
            }
        }
    };

    view! {
        <div
            class={final_class}
            style={style_attr}
            data-disabled={disabled}
            on:click=handle_click
            on:keydown=handle_keydown
            data-radix-menubar-checkbox-item=""
        >
            <span class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
                {if checked_value {
                    view! { <span class="h-2 w-2 bg-current rounded-sm"></span> }
                }}
            </span>
            <span class="pl-6">{children()}</span>
        </div>
    }
}

#[component]
pub fn MenubarRadioItem(
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] checked: Option<ReadSignal<bool>>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] on_value_change: Option<Callback<String>>,
    children: Children,
) -> impl IntoView {
    let value = value.unwrap_or_default();
    let checked_value = checked.map(|c| c.get()).unwrap_or(false);
    let disabled = disabled.unwrap_or(false);
    let base_classes = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
    let final_class = merge_classes(base_classes, class);
    let style_attr = style.unwrap_or_default();

    let value_clone1 = value.clone();
    let handle_click = move |_| {
        if !disabled {
            if let Some(callback) = on_value_change {
                callback.run(value_clone1.clone());
            }
        }
    };

    let value_clone2 = value.clone();
    let handle_keydown = move |event: web_sys::KeyboardEvent| {
        if !disabled && (event.key() == "Enter" || event.key() == " ") {
            event.prevent_default();
            if let Some(callback) = on_value_change {
                callback.run(value_clone2.clone());
            }
        }
    };

    view! {
        <div
            class={final_class}
            style={style_attr}
            data-disabled={disabled}
            on:click=handle_click
            on:keydown=handle_keydown
            data-radix-menubar-radio-item=""
        >
            <span class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
                {if checked_value {
                    view! { <span class="h-2 w-2 bg-current rounded-full"></span> }
                }}
            </span>
            <span class="pl-6">{children()}</span>
        </div>
    }
}
