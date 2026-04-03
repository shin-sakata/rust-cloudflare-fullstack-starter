#![allow(dead_code)]
use leptos::prelude::*;

#[derive(Default, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum BadgeVariant {
    #[default]
    Outline,
    Filled,
    Tint,
}

#[derive(Default, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum BadgeColor {
    #[default]
    Default,
    Success,
}

#[component]
pub fn Badge(
    #[prop(optional)] variant: BadgeVariant,
    #[prop(optional)] color: BadgeColor,
    children: Children,
) -> impl IntoView {
    let base = "inline-flex items-center px-2 py-0.5 rounded-full text-[10px] font-medium tracking-wider uppercase";
    let style = match (variant, color) {
        (BadgeVariant::Filled, BadgeColor::Success) => {
            "bg-accent-green/15 text-accent-green"
        }
        (BadgeVariant::Tint, _) => {
            "bg-accent-purple/15 text-accent-purple"
        }
        (BadgeVariant::Outline, _) => {
            "border border-border-subtle text-text-secondary"
        }
        (BadgeVariant::Filled, BadgeColor::Default) => {
            "bg-accent-blue/15 text-accent-blue"
        }
    };

    view! {
        <span class=format!("{base} {style}")>{children()}</span>
    }
}
