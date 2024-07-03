use crate::components::LoginForm;
use leptos::{component, view, IntoView};

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <main class="bg-gradient-to-tr from-blue-100 to-blue-50 min-h-screen p-7">
            <h1 class="text-4xl text-center font-serif my-5">My awesome intranet</h1>
            <LoginForm/>
        </main>
    }
}
