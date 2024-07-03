use crate::Error;
use leptos::{component, view, IntoView};
use std::fmt::format;

#[component]
pub fn ErrorAlert(error: Option<Error>) -> impl IntoView {
    if let Some(error) = error {
        let error = format!("Reason: {:?}", error);
        view! { <div>{error}</div> }
    } else {
        // TODO: change this to avoid a blank space
        view! { <div></div> }
    }
}
