use crate::components::transaction::extension_sign_in::sign_in_with_extension;
use leptos::*;

#[component]
pub fn ExtensionSignIn() -> impl IntoView {
    let (name, set_name) = create_signal(String::from(""));
    let (error, set_error) = create_signal(String::from(""));
    let (extrinsic_error, set_extrinsic_error) = create_signal(String::from(""));
    let (extrinsic_success, set_extrinsic_success) = create_signal(String::from(""));

    view! {
        <div>
        <code>{move || format!("{:#?}", name())}</code>
        </div>
    }
}
