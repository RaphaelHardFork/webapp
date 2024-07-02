mod error;
mod login;

pub use error::Error;
pub use login::Login;

use leptos::{component, view, IntoView};
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
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
