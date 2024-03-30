use crate::components::schelling_game::department_funding::storage::get_period_fn::get_period_fn;
use leptos::*;

#[component]
pub fn GetPeriod(department_required_fund_id: u64) -> impl IntoView {
    let period = get_period_fn(department_required_fund_id);
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
