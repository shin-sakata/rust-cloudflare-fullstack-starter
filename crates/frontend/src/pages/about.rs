use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::ui::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="flex-1 flex items-center justify-center py-20 px-6 relative z-10">
            <div class="max-w-lg w-full flex flex-col items-center gap-10 text-center">
                <div>
                    <h1 class="text-4xl font-black tracking-tighter m-0 bg-gradient-to-br from-accent-blue via-accent-purple to-accent-pink bg-clip-text text-transparent">"Tech Stack"</h1>
                    <p class="text-sm mt-2 text-text-secondary">"Everything compiles to WebAssembly"</p>
                </div>

                <div class="border border-border-subtle rounded-xl overflow-hidden w-full bg-surface-raised">
                    <StackRow label="Frontend" value="Leptos 0.8 (CSR)" />
                    <StackRow label="Backend" value="Axum on CF Workers" />
                    <StackRow label="Data Fetching" value="leptos-fetch" />
                    <StackRow label="Routing" value="leptos_router" />
                    <StackRow label="Styling" value="Tailwind CSS v4" />
                    <StackRow label="Build" value="Trunk + worker-build" />
                    <StackRow label="Dev Tools" value="Nix + process-compose" last=true />
                </div>

                <A href="/" attr:class="no-underline">
                    <Button variant=ButtonVariant::Secondary>"Back to Home"</Button>
                </A>
            </div>
        </div>
    }
}
