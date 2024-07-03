mod error;
mod login;

pub use error::Error;
use leptos_meta::{provide_meta_context, Stylesheet};
pub use login::Login;

use leptos::{component, view, IntoView};
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/tailwind.css"/>
        <Router>
            <Routes>
                <Route path="/" view=Login/>
                <Route path="/error" view=Error/>
                <Route path="/*any" view=|| view! { <h1>Not found</h1> }/>
            </Routes>
        </Router>
    }
}
