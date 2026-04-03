mod api;
mod components;
mod pages;

use leptos::prelude::*;
use leptos_fetch::QueryClient;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use components::NavBar;
use pages::{About, Home, NotFound};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let prefers_dark = leptos::prelude::window()
        .match_media("(prefers-color-scheme: dark)")
        .ok()
        .flatten()
        .map(|m: web_sys::MediaQueryList| m.matches())
        .unwrap_or(true);
    let is_dark = RwSignal::new(prefers_dark);
    QueryClient::new().provide();

    Effect::new(move |_| {
        let theme = if is_dark.get() { "dark" } else { "light" };
        if let Some(el) = leptos::prelude::document().document_element() {
            let _ = el.set_attribute("data-theme", theme);
        }
    });

    view! {
        <Router>
            <div class="min-h-screen flex flex-col relative">
                <div class="fixed pointer-events-none z-0 top-[-40%] left-1/2 -translate-x-1/2 w-[800px] h-[500px] bg-[radial-gradient(ellipse,var(--color-glow)_0%,transparent_70%)] transition-[background] duration-300"></div>
                <NavBar is_dark=is_dark />
                <Routes fallback=|| view! { <NotFound /> }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/about") view=About />
                </Routes>
            </div>
        </Router>
    }
}
