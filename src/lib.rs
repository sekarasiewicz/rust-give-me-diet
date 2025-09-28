use leptos::mount::mount_to_body;
use leptos::{view, wasm_bindgen};

pub mod app;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    mount_to_body(|| {
        view! { <app::App/> }
    });
}
