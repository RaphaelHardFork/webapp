mod error;
mod login;

pub use error::Error;
pub use login::Login;

use leptos::{component, view, IntoView};
use leptos_meta::provide_meta_context;
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
        //bg-gradient-to-tr from-blue-100 to-blue-50
            <main class="background min-h-screen p-7">
                <Routes>
                    <Route path="/" view=Login/>
                    <Route path="/error" view=Error/>
                    <Route path="/*any" view=|| view! { <h1>Not found</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}
