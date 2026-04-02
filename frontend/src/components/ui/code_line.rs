use leptos::prelude::*;

#[component]
pub fn CodeLine(text: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center gap-2 px-3 py-1.5 rounded-md bg-[var(--color-surface)]/60 font-mono text-xs">
            <span class="text-[var(--color-accent-purple)]">"$"</span>
            <span class="text-[var(--color-text-secondary)]">{text}</span>
        </div>
    }
}
