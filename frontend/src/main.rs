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
    let is_dark = RwSignal::new(true);
    QueryClient::new().provide();

    view! {
        <Router>
            <div class="min-h-screen flex flex-col relative">
                <div class="hero-glow"></div>
                <NavBar is_dark=is_dark />
                <Routes fallback=|| view! { <NotFound /> }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/about") view=About />
                </Routes>
            </div>
        </Router>
    }
}
