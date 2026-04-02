use leptos_fetch::QueryScopeLocal;
use shared::HealthResponse;

pub fn health_query() -> QueryScopeLocal<(), HealthResponse> {
    QueryScopeLocal::new(async || {
        let resp = gloo_net::http::Request::get("/api/health")
            .send()
            .await
            .unwrap();
        resp.json().await.unwrap()
    })
}
