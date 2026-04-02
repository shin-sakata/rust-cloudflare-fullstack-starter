use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryScopeLocal};
use leptos_router::{
    components::{Route, Router, Routes, A},
    path,
};
use shared::HealthResponse;
use thaw::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

fn health_query() -> QueryScopeLocal<(), HealthResponse> {
    QueryScopeLocal::new(async || {
        let resp = gloo_net::http::Request::get("/api/health")
            .send()
            .await
            .unwrap();
        resp.json().await.unwrap()
    })
}

#[component]
fn App() -> impl IntoView {
    let theme = RwSignal::new(Theme::dark());
    QueryClient::new().provide();

    view! {
        <ConfigProvider theme=theme>
            <Router>
                <div class="page-root">
                    <NavBar theme=theme />
                    <Routes fallback=|| view! { <NotFound /> }>
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/about") view=About />
                    </Routes>
                </div>
            </Router>
        </ConfigProvider>
    }
}

#[component]
fn NavBar(theme: RwSignal<Theme>) -> impl IntoView {
    let is_dark = Memo::new(move |_| theme.get().name == "dark");

    let toggle = move |_| {
        theme.set(if is_dark.get() {
            Theme::light()
        } else {
            Theme::dark()
        });
    };

    view! {
        <nav class="nav-bar">
            <div class="nav-left">
                <A href="/" attr:style="text-decoration: none; color: inherit; font-size: 16px; font-weight: 700;">
                    "Rust CF Workers"
                </A>
                <Badge appearance=BadgeAppearance::Tint>"SPA"</Badge>
            </div>
            <div class="nav-right">
                <A href="/" attr:class="nav-link">"Home"</A>
                <A href="/about" attr:class="nav-link">"About"</A>
                <Button
                    appearance=ButtonAppearance::Subtle
                    on_click=toggle
                    size=ButtonSize::Small
                >
                    {move || if is_dark.get() { "Light" } else { "Dark" }}
                </Button>
            </div>
        </nav>
    }
}

#[component]
fn Home() -> impl IntoView {
    let client: QueryClient = expect_context();
    let health = client.local_resource(health_query(), || ());

    view! {
        <div class="hero">
            <div class="hero-inner">
                <div>
                    <h1 class="hero-title">"Rust Fullstack\non Cloudflare"</h1>
                    <p class="hero-subtitle">"Leptos + Axum + WASM — all running on the edge"</p>
                </div>

                <div class="badge-row">
                    <Badge appearance=BadgeAppearance::Outline>"Leptos 0.8"</Badge>
                    <Badge appearance=BadgeAppearance::Outline>"Axum"</Badge>
                    <Badge appearance=BadgeAppearance::Outline>"WebAssembly"</Badge>
                    <Badge appearance=BadgeAppearance::Outline>"Thaw UI"</Badge>
                    <Badge appearance=BadgeAppearance::Outline>"leptos-fetch"</Badge>
                </div>

                <div class="card-grid">
                    <div class="info-card">
                        <h3>"API Status"</h3>
                        <Suspense fallback=move || view! { <Spinner size=SpinnerSize::Tiny /> }>
                            {move || Suspend::new(async move {
                                let resp = health.await;
                                view! {
                                    <Flex align=FlexAlign::Center gap=FlexGap::Small>
                                        <Badge appearance=BadgeAppearance::Filled color=BadgeColor::Success>
                                            {resp.status.clone()}
                                        </Badge>
                                        <span style="opacity: 0.5; font-size: 13px; font-family: monospace;">"/api/health"</span>
                                    </Flex>
                                }
                            })}
                        </Suspense>
                    </div>

                    <div class="info-card">
                        <h3>"Quick Start"</h3>
                        <code>
                            "nix develop"<br />
                            "process-compose up"
                        </code>
                    </div>

                    <div class="info-card">
                        <h3>"Frontend"</h3>
                        <p>"Edit "<code>"frontend/src/main.rs"</code>" to modify this page. Hot reload with Trunk."</p>
                    </div>

                    <div class="info-card">
                        <h3>"Backend"</h3>
                        <p>"Edit "<code>"worker/src/lib.rs"</code>" to add API routes with Axum."</p>
                    </div>
                </div>

                <div class="btn-row">
                    <A href="https://book.leptos.dev/" attr:target="_blank" attr:style="text-decoration: none;">
                        <Button appearance=ButtonAppearance::Primary>"Leptos Docs"</Button>
                    </A>
                    <A href="https://developers.cloudflare.com/workers/" attr:target="_blank" attr:style="text-decoration: none;">
                        <Button appearance=ButtonAppearance::Subtle>"CF Workers Docs"</Button>
                    </A>
                    <A href="https://thawui.vercel.app/" attr:target="_blank" attr:style="text-decoration: none;">
                        <Button appearance=ButtonAppearance::Subtle>"Thaw UI Docs"</Button>
                    </A>
                </div>
            </div>
        </div>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div class="hero">
            <div class="hero-inner">
                <div>
                    <h1 class="hero-title">"Tech Stack"</h1>
                    <p class="hero-subtitle">"Everything compiles to WebAssembly"</p>
                </div>

                <div class="info-card" style="width: 100%;">
                    <table class="stack-table">
                        <tr><td>"Frontend"</td><td>"Leptos 0.8 (CSR) + Thaw UI"</td></tr>
                        <tr><td>"Backend"</td><td>"Axum on Cloudflare Workers"</td></tr>
                        <tr><td>"Data Fetching"</td><td>"leptos-fetch"</td></tr>
                        <tr><td>"Routing"</td><td>"leptos_router"</td></tr>
                        <tr><td>"Build"</td><td>"Trunk + worker-build"</td></tr>
                        <tr><td>"Target"</td><td>"wasm32-unknown-unknown"</td></tr>
                        <tr><td>"Dev Tools"</td><td>"Nix + process-compose"</td></tr>
                    </table>
                </div>

                <A href="/" attr:style="text-decoration: none;">
                    <Button appearance=ButtonAppearance::Subtle>"Back to Home"</Button>
                </A>
            </div>
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found">
            <p class="not-found-code">"404"</p>
            <p style="font-size: 18px; opacity: 0.5;">"Page not found"</p>
            <A href="/" attr:style="text-decoration: none;">
                <Button appearance=ButtonAppearance::Primary>"Go Home"</Button>
            </A>
        </div>
    }
}
