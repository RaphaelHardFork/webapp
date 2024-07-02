mod components;
mod error;
mod pages;

pub use error::{Error, Result};

use leptos::{mount_to_body, view};
use pages::App;

fn main() {
    // debug WASM
    console_error_panic_hook::set_once();

    // mount App
    mount_to_body(|| {
        view! { <App/> }
    })
}
