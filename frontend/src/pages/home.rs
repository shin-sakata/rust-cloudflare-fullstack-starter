use leptos::prelude::*;
use leptos_fetch::QueryClient;

use crate::api::health_query;
use crate::components::ui::*;

#[component]
pub fn Home() -> impl IntoView {
    let client: QueryClient = expect_context();
    let health = client.local_resource(health_query(), || ());

    view! {
        <div class="flex-1 flex items-center justify-center py-20 px-6 relative z-10">
            <div class="max-w-2xl w-full flex flex-col items-center gap-12 text-center">

                // Hero
                <div class="flex flex-col items-center gap-4">
                    <h1 class="text-6xl max-sm:text-4xl font-black tracking-tighter leading-[1.05] m-0 gradient-text">
                        "Rust Fullstack"<br />"on Cloudflare!"
                    </h1>
                    <p class="text-base max-w-md m-0 text-[var(--color-text-secondary)] leading-relaxed">
                        "Build full-stack apps with Leptos + Axum, compile everything to WebAssembly, and deploy to the edge."
                    </p>
                </div>

                // Status pill
                <Suspense fallback=move || view! {
                    <div class="flex items-center gap-2 px-4 py-2 rounded-full border border-[var(--color-border-subtle)] bg-[var(--color-surface-raised)]">
                        <Spinner />
                        <span class="text-xs text-[var(--color-text-muted)]">"Connecting..."</span>
                    </div>
                }>
                    {move || Suspend::new(async move {
                        let resp = health.await;
                        view! {
                            <div class="flex items-center gap-2.5 px-4 py-2 rounded-full border border-[var(--color-border-subtle)] bg-[var(--color-surface-raised)]">
                                <div class="dot-pulse"></div>
                                <span class="text-xs font-medium text-[var(--color-accent-green)]">{resp.status.clone()}</span>
                                <span class="text-[10px] text-[var(--color-text-muted)] font-mono">"/api/health"</span>
                            </div>
                        }
                    })}
                </Suspense>

                // Cards
                <div class="grid grid-cols-2 max-sm:grid-cols-1 gap-3 w-full">
                    <InfoCard title="Quick Start">
                        <div class="flex flex-col gap-1.5">
                            <CodeLine text="nix develop" />
                            <CodeLine text="process-compose up" />
                        </div>
                    </InfoCard>
                    <InfoCard title="Frontend">
                        <p class="m-0 text-xs text-[var(--color-text-secondary)] leading-relaxed">
                            "Edit "<code class="font-mono text-[var(--color-accent-blue)]">"frontend/src/main.rs"</code>
                            " — hot reload with Trunk"
                        </p>
                    </InfoCard>
                    <InfoCard title="Backend">
                        <p class="m-0 text-xs text-[var(--color-text-secondary)] leading-relaxed">
                            "Edit "<code class="font-mono text-[var(--color-accent-blue)]">"worker/src/lib.rs"</code>
                            " — add API routes with Axum"
                        </p>
                    </InfoCard>
                    <InfoCard title="Styling">
                        <p class="m-0 text-xs text-[var(--color-text-secondary)] leading-relaxed">
                            "Tailwind CSS v4 built in — just use utility classes in "<code class="font-mono text-[var(--color-accent-blue)]">"view!"</code>
                        </p>
                    </InfoCard>
                </div>

                // Powered by
                <div class="w-full flex flex-col gap-5">
                    <p class="text-[11px] font-semibold tracking-[0.2em] uppercase text-center text-[var(--color-text-muted)] m-0">"Powered by"</p>
                    <div class="grid grid-cols-3 max-sm:grid-cols-1 gap-2.5 w-full">
                        <StackCard name="Leptos 0.8" desc="Reactive WASM frontend" href="https://book.leptos.dev/" />
                        <StackCard name="Axum" desc="Rust web framework" href="https://docs.rs/axum/latest/axum/" />
                        <StackCard name="Thaw UI" desc="Component library" href="https://thawui.vercel.app/" />
                        <StackCard name="leptos-fetch" desc="Data fetching & cache" href="https://github.com/zakstucke/leptos-fetch" />
                        <StackCard name="leptos_router" desc="Type-safe routing" href="https://docs.rs/leptos_router/latest/leptos_router/" />
                        <StackCard name="CF Workers" desc="Edge runtime" href="https://developers.cloudflare.com/workers/" />
                    </div>
                </div>

                // CTA
                <div class="flex gap-3 flex-wrap justify-center">
                    <Button href="https://book.leptos.dev/".to_string()>"Get Started"</Button>
                    <Button variant=ButtonVariant::Secondary href="https://developers.cloudflare.com/workers/".to_string()>"Documentation"</Button>
                </div>
            </div>
        </div>
    }
}
