use leptos::prelude::*;

#[component]
pub fn CodeLine(text: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center gap-2 px-3 py-1.5 rounded-md bg-surface/60 font-mono text-xs">
            <span class="text-accent-purple">"$"</span>
            <span class="text-text-secondary">{text}</span>
        </div>
    }
}
