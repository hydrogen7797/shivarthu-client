use crate::components::schelling_game::department_funding::apply_staking_period::ApplyStakingPeriod;
use leptos::*;
use leptos_router::*;

#[component]
pub fn ApplyStakingPeriodParams() -> impl IntoView {
    let params = use_params_map();

    let department_required_fund_id = move || {
        params.with(|params| {
            params
                .get("department_required_fund_id")
                .cloned()
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or_default()
        })
    };

    let params_value = untrack(move || department_required_fund_id());

    view! {
        <div>
            <ApplyStakingPeriod department_required_fund_id={params_value}/>
        </div>
    }
}
