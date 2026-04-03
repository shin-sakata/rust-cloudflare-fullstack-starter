use leptos::prelude::*;

#[component]
pub fn InfoCard(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="border border-border-subtle rounded-xl p-5 bg-surface-raised transition-all duration-300 text-left hover:border-border-hover hover:shadow-glow">
            <h3 class="m-0 mb-3 text-xs font-semibold tracking-wide uppercase text-text-muted">{title}</h3>
            {children()}
        </div>
    }
}
