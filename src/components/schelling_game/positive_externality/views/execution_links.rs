use leptos::prelude::*;

#[component]
pub fn ExecutionLinks(user_to_calculate: String) -> impl IntoView {
    view! {
        <div>
            <p>{user_to_calculate}</p>
        </div>
    }
}
