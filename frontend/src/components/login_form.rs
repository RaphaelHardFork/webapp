use crate::components::ErrorAlert;
use crate::Error;

use leptos::{
    component, create_signal, event_target_value, view, IntoView, ReadSignal, SignalGet,
    SignalGetUntracked, SignalSet,
};

#[component]
pub fn LoginForm() -> impl IntoView {
    let (error, set_error) = create_signal::<Option<Error>>(None);
    let (email, set_email) = create_signal::<String>(String::new());
    let (pwd, set_pwd) = create_signal::<String>(String::new());

    view! {
        <div>
            <ErrorAlert error=error.get()/>
            <form>
                <div>
                    <input
                        type="email"
                        placeholder="Email"
                        id="email-input"
                        value=email.get()
                        on:input=move |ev| { set_email.set(event_target_value(&ev)) }
                        prop:value=email
                    />
                </div>
                <div>
                    <input
                        type="password"
                        placeholder="Password"
                        id="pwd-input"
                        value=pwd.get()
                        on:input=move |ev| { set_pwd.set(event_target_value(&ev)) }
                        prop:value=pwd
                    />
                </div>
                <button type="submit" disabled=false>
                    Sign in
                </button>
            </form>
        </div>
    }
}
