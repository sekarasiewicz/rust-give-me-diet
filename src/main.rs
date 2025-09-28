mod app;
mod config;

use anyhow::Result;
use app::App;
use leptos::prelude::{Env, LeptosOptions};
use leptos_axum::LeptosRoutes;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<()> {
    // 1) Config from file (ignored in repo)
    let cfg = config::AppConfig::from_file("config/local")?;

    // 2) Minimal Leptos options (SSR only)
    let mut opts = LeptosOptions::builder()
        .output_name("hello-ssr")
        .site_root("target/site")
        .build();

    if cfg.leptos_env.eq_ignore_ascii_case("PROD") {
        opts.env = Env::PROD;
    }

    if cfg.server_port.is_empty() {
        return Err(anyhow::anyhow!("Server port is empty"));
    }

    let routes = leptos_axum::generate_route_list(App);

    // 3) SSR Router
    let app = axum::Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/pkg", ServeDir::new("target/site/pkg"))
        .leptos_routes(&opts, routes, App)
        .with_state(opts.clone());

    // 4) Start server

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", cfg.server_port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
