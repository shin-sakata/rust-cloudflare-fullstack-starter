use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"Hello from Leptos!"</h1>
        <p>"Rust + Cloudflare Workers SPA"</p>
    }
}
