use crate::components::schelling_game::positive_externality::storage::get_period_fn::get_period_fn;
use leptos::prelude::*;

#[component]
pub fn GetPeriod(user_to_calculate: String) -> impl IntoView {
    let period = get_period_fn(user_to_calculate);
    let period_value = move || match period() {
        Some(value) => format!("Period name: {:?}", value),
        None => format!(""),
    };
    view! {
        <div>
            <code>{move || period_value()}</code>
        </div>
    }
}
