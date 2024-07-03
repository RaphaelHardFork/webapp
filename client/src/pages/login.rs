use leptos::{component, view, IntoView};

use crate::components::LoginForm;

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div>
            <div>
                <div>
                    <h1 class="text-4xl">My awesome intranet</h1>
                </div>
            </div>
            <div>
                <div>
                    <LoginForm/>
                </div>
            </div>
        </div>
    }
}
