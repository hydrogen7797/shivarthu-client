use crate::components::schelling_game::positive_externality::apply_staking_period::ApplyStakingPeriod;
use leptos::*;
use leptos_router::*;

#[component]
pub fn ApplyStakingPeriodParams() -> impl IntoView {
    let params = use_params_map();

    let user_to_calculate =
        move || params.with(|params| params.get("user_to_calculate").cloned().unwrap_or_default());

    let params_value = untrack(move || user_to_calculate());

    view! {
        <div>
            <ApplyStakingPeriod user_to_calculate=params_value/>
        </div>
    }
}
