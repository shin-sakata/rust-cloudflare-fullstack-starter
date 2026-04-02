use leptos::prelude::*;

#[component]
pub fn StackCard(name: &'static str, desc: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <a
            class="group block border border-[var(--color-border-subtle)] rounded-lg px-4 py-3 no-underline bg-[var(--color-surface-raised)] gradient-border transition-all duration-300 hover:-translate-y-0.5"
            href=href target="_blank" rel="noopener"
        >
            <p class="m-0 text-xs font-semibold text-[var(--color-text-primary)]">{name}</p>
            <p class="m-0 mt-0.5 text-[11px] text-[var(--color-text-muted)]">{desc}</p>
        </a>
    }
}
