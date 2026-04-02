use leptos::prelude::*;

#[component]
pub fn StackRow(label: &'static str, value: &'static str, #[prop(optional)] last: bool) -> impl IntoView {
    let border = if last { "" } else { "border-b border-[var(--color-border-subtle)]" };
    view! {
        <div class=format!("flex justify-between items-center px-5 py-3 {border}")>
            <span class="text-xs font-semibold text-[var(--color-text-primary)]">{label}</span>
            <span class="text-xs text-[var(--color-text-muted)] font-mono">{value}</span>
        </div>
    }
}
