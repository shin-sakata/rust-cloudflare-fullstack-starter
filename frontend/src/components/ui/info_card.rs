use leptos::prelude::*;

#[component]
pub fn InfoCard(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="border border-[var(--color-border-subtle)] rounded-xl p-5 bg-[var(--color-surface-raised)] transition-all duration-300 text-left hover:border-[var(--color-border-hover)] hover:shadow-[0_0_20px_-4px_oklch(0.50_0.15_270/0.3)]">
            <h3 class="m-0 mb-3 text-xs font-semibold tracking-wide uppercase text-[var(--color-text-muted)]">{title}</h3>
            {children()}
        </div>
    }
}
