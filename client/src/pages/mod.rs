mod error;
mod login;

pub use error::Error;
pub use login::Login;

use leptos::{component, html::Style, view, IntoView};
use leptos_meta::{provide_meta_context, Stylesheet};
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes>
                <Route path="/" view=Login/>
                <Route path="/error" view=Error/>
                <Route path="/*any" view=|| view! { <h1>Not found</h1> }/>
            </Routes>
        </Router>
    }
}
