mod router;
mod components;
mod js_extension_binding;

use leptos::*;
use router::RouterApp;

#[component]
pub fn App() -> impl IntoView {
    view! {
      <RouterApp/>
    }
}
