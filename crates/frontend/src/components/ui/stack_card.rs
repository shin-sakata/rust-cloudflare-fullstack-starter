use leptos::prelude::*;

#[component]
pub fn StackCard(name: &'static str, desc: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <a
            class="group block border border-border-subtle rounded-lg px-4 py-3 no-underline bg-surface-raised gradient-border transition-all duration-300 hover:-translate-y-0.5"
            href=href target="_blank" rel="noopener"
        >
            <p class="m-0 text-xs font-semibold text-text-primary">{name}</p>
            <p class="m-0 mt-0.5 text-[11px] text-text-muted">{desc}</p>
        </a>
    }
}
