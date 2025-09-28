use leptos::mount::mount_to_body;
use leptos::prelude::ServerFnError;
use leptos::{server, view, wasm_bindgen};

pub mod app;

#[server(Greet, "/api")]
pub async fn greet(name: String) -> Result<String, ServerFnError> {
    Ok(format!("Hello, {}!", name))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    mount_to_body(|| {
        view! { <app::App/> }
    });
}
