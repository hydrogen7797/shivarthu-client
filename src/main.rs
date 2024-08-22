use leptos::*;
use shivarthu_client::App;
// use wasm_bindgen::JsCast;

fn main() {
    // let app_element = document()
    //     .get_element_by_id("root")
    //     .unwrap()
    //     .dyn_into::<web_sys::HtmlElement>() // Casting to web_sys::HtmlElement
    //     .expect("Element with id 'root' is not an HtmlElement");
    mount_to_body(|| view! { <App/> })
}
