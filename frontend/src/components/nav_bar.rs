use leptos::prelude::*;
use leptos_router::components::A;

use super::ui::{Button, ButtonVariant};

#[component]
pub fn NavBar(is_dark: RwSignal<bool>) -> impl IntoView {
    let toggle = move |_| {
        is_dark.update(|v| *v = !*v);
    };

    view! {
        <nav class="flex items-center justify-between px-8 py-4 border-b border-[var(--color-border-subtle)] backdrop-blur-xl sticky top-0 z-50 bg-[var(--color-surface)]/80">
            <div class="flex items-center gap-3">
                <A href="/" attr:class="no-underline text-[var(--color-text-primary)] text-sm font-semibold tracking-tight">
                    "Rust CF Workers"
                </A>
            </div>
            <div class="flex items-center gap-6">
                <A href="/" attr:class="no-underline text-xs font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text-primary)] transition-colors duration-200">"Home"</A>
                <A href="/about" attr:class="no-underline text-xs font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text-primary)] transition-colors duration-200">"About"</A>
                <a
                    href="https://github.com/shin-sakata/rust-cloudflare-fullstack-starter"
                    target="_blank"
                    rel="noopener"
                    class="no-underline text-xs font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text-primary)] transition-colors duration-200"
                >
                    "GitHub"
                </a>
                <Button variant=ButtonVariant::Ghost on_click=Box::new(toggle)>
                    {move || if is_dark.get() { "Light" } else { "Dark" }}
                </Button>
            </div>
        </nav>
    }
}
