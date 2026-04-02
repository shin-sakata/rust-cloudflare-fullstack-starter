use leptos::prelude::*;

#[component]
pub fn Spinner() -> impl IntoView {
    view! {
        <div class="inline-block w-4 h-4 border-2 border-[var(--color-border-subtle)] border-t-[var(--color-accent-purple)] rounded-full animate-spin"></div>
    }
}
