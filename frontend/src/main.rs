use leptos::prelude::*;
use shared::HealthResponse;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

async fn fetch_health() -> String {
    match gloo_net::http::Request::get("/api/health").send().await {
        Ok(response) => match response.json::<HealthResponse>().await {
            Ok(health) => health.status,
            Err(e) => format!("Error: {e}"),
        },
        Err(e) => format!("Error: {e}"),
    }
}

#[component]
fn App() -> impl IntoView {
    let (health, set_health) = signal("loading...".to_string());

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            set_health.set(fetch_health().await);
        });
    });

    view! {
        <h1>"Rust Cloudflare Workers SPA"</h1>
        <p>"API Health: " {health}</p>
    }
}
