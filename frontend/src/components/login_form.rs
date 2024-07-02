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
            <ErrorAlert error=(move || error.get())()/>
            <form>
                <div>
                    <input
                        type="email"
                        placeholder="Email"
                        id="email-input"
                        value=(move || email.get())()
                        on:input=move |ev| { set_email.set(event_target_value(&ev)) }
                        prop:value=email
                    />
                </div>
                <div>
                    <input
                        type="password"
                        placeholder="Password"
                        id="pwd-input"
                        value=(move || pwd.get())()
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

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>; // For tests.

    use leptos::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

    use super::LoginForm;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn create() -> Result<()> {
        // test section
        let document = leptos::document();
        let test_wrapper = document
            .create_element("section")
            .expect("Cannot create document");
        let _ = document.body().unwrap().append_child(&test_wrapper);

        // mount into the DOM
        mount_to(
            test_wrapper.clone().unchecked_into(),
            || view! { <LoginForm/> },
        );

        // extract inputs
        let input = test_wrapper
            .query_selector("#email-input")
            .unwrap()
            .unwrap()
            .unchecked_into::<web_sys::HtmlInputElement>();

        assert_eq!(
            input.placeholder(),
            "Email".to_string(),
            "email placeholder"
        );

        // extract the element
        if let Some(btn_text) = test_wrapper
            .query_selector("button")
            .unwrap()
            .unwrap()
            .text_content()
        {
            assert_eq!(btn_text.trim(), "Sign in".to_string());
        }

        // runtime
        // let runtime = create_runtime();
        // runtime.dispose();

        // clean the browser
        test_wrapper.remove();
        Ok(())
    }

    #[test]
    fn test_logic() -> Result<()> {
        assert_eq!(1, 1, "Whow");
        Ok(())
    }
}

// endregion: --- Tests
