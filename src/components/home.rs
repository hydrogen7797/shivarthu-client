use crate::components::navigation::nav::Nav;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <>
            <Nav/>
            <button on:click={move |_| {
                set_count.update(|count: &mut i32| *count += 1);
            }}>

                "Click me: " {move || count}
            </button>
        </>
    }
}
