use crate::components::schelling_game::profile_validation::apply_staking_period::ApplyStakingPeriod;
use leptos::*;
use leptos_router::*;

#[component]
pub fn ApplyStakingPeriodParams() -> impl IntoView {
    let params = use_params_map();
    
    let profile_user_account = move || {
        params.with(|params| {
            params
                .get("profile_user_account")
                .cloned()
                .unwrap_or_default()
        })
    };
    

    

    let params_value = untrack(move || profile_user_account());

    view! {
        <div>
            <ApplyStakingPeriod profile_user_account=params_value/>
        </div>
    }
}