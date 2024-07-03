mod components;
mod error;
mod pages;

pub use error::{Error, Result};
pub use pages::App;

use leptos::{mount_to_body, view};

pub fn main() {
    // debug WASM
    console_error_panic_hook::set_once();

    // mount App
    mount_to_body(|| {
        view! { <App/> }
    })
}
