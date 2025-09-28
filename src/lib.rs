use leptos::mount::mount_to_body;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod app;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    mount_to_body(app::App);
}
