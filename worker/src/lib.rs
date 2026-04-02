use axum::{routing::get, Router};
use http_body_util::BodyExt;
use tower_service::Service;
use worker::*;

fn router() -> Router {
    Router::new().route("/api/health", get(|| async { "ok" }))
}

async fn into_worker_response(resp: axum::response::Response) -> Result<HttpResponse> {
    let (parts, body) = resp.into_parts();
    let bytes = body
        .collect()
        .await
        .map_err(|e| Error::RustError(e.to_string()))?
        .to_bytes();
    let stream = futures_util::stream::once(async move { Ok::<_, String>(bytes.to_vec()) });
    Ok(axum::http::Response::from_parts(parts, Body::from_stream(stream)?))
}

#[event(fetch)]
async fn fetch(req: HttpRequest, _env: Env, _ctx: Context) -> Result<HttpResponse> {
    console_error_panic_hook::set_once();
    into_worker_response(router().call(req).await.unwrap()).await
}
