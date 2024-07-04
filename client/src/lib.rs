use app::App;
use leptos::mount_to_body;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    // debug WASM
    console_error_panic_hook::set_once();

    // mount app to body
    mount_to_body(App)
}
