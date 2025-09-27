mod app;
mod config;

use anyhow::Result;
use app::App;
use leptos::prelude::LeptosOptions;
use leptos::prelude::*;
use leptos_axum::LeptosRoutes;

#[tokio::main]
async fn main() -> Result<()> {
    // 1) Config z pliku (ignorowanego w repo)
    let cfg = config::AppConfig::from_file("config/local")?;

    // 2) Minimalne opcje Leptosa (tylko SSR)
    let mut opts = LeptosOptions::builder()
        .output_name("hello-ssr")
        .site_root("target/site")
        .build();

    if cfg.leptos_env.eq_ignore_ascii_case("PROD") {
        opts.env = Env::PROD;
    }

    let routes = leptos_axum::generate_route_list(App);

    // 3) Router SSR
    let app = axum::Router::new()
        .leptos_routes(&opts, routes, App)
        .with_state(opts.clone());

    // 4) Start serwera

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
