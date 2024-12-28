use leptos::prelude::*;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
use leptos::task::spawn_local;

async fn load_data(count: ReadSignal<i32>, set_count: WriteSignal<i32>) {
    let new_data = count.get_untracked() + 1;
    set_count(new_data);
}

#[component]
pub fn BlockNumber() -> impl IntoView {
    let (count, set_count) = signal(0);

    let Pausable { .. } = use_interval_fn(
        move || {
            spawn_local(async move { load_data(count, set_count).await });
        },
        500,
    );

    view! { <p>{move || count()}</p> }
}
