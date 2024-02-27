use leptos::*;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;

async fn load_data(count: ReadSignal<i32>, set_count: WriteSignal<i32>) {
    let new_data = move || count() + 1;
    set_count(new_data());
}

#[component]
pub fn BlockNumber() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let action = create_action(|(count, set_count): &(ReadSignal<i32>, WriteSignal<i32>)| {
        let count = count.clone();
        let set_count = set_count.clone();
        async move { load_data(count, set_count).await }
    });

    let Pausable { .. } = use_interval_fn(
        move || {
            action.dispatch((count, set_count));
        },
    500,
    );

    view! { <p>{move || count()}</p> }
}
