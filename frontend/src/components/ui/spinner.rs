use leptos::prelude::*;

#[component]
pub fn Spinner() -> impl IntoView {
    view! {
        <div class="inline-block w-4 h-4 border-2 border-border-subtle border-t-accent-purple rounded-full animate-spin"></div>
    }
}
