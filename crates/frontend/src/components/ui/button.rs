use leptos::prelude::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
}

#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional)] on_click: Option<Box<dyn Fn(leptos::ev::MouseEvent)>>,
    children: Children,
) -> impl IntoView {
    let base = "inline-flex items-center justify-center gap-1.5 px-5 py-2.5 rounded-lg text-sm font-semibold transition-all duration-200 cursor-pointer no-underline";
    let variant_class = match variant {
        ButtonVariant::Primary => "bg-gradient-to-r from-accent-blue to-accent-purple text-white hover:opacity-90 hover:shadow-lg hover:shadow-accent-purple/20 border-none",
        ButtonVariant::Secondary => "border border-border-subtle text-text-secondary hover:text-text-primary hover:border-border-hover bg-transparent",
        ButtonVariant::Ghost => "text-text-muted hover:text-text-primary bg-transparent border-none hover:bg-surface-overlay",
    };
    let class = format!("{base} {variant_class}");

    if let Some(url) = href {
        view! {
            <a class=class.clone() href=url target="_blank" rel="noopener">{children()}</a>
        }
        .into_any()
    } else {
        view! {
            <button class=class.clone() on:click=move |ev| {
                if let Some(ref handler) = on_click {
                    handler(ev);
                }
            }>{children()}</button>
        }
        .into_any()
    }
}
