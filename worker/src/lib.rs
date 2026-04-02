use worker::*;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        .get("/api/health", |_, _| Response::ok("ok"))
        .run(req, env)
        .await
}
