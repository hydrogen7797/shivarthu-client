use crate::components::schelling_game::profile_validation::storage::get_period_fn::get_period_fn;
use leptos::*;

#[component]
pub fn GetPeriod(profile_user_account: String) -> impl IntoView {
    let period = get_period_fn(profile_user_account);
    view! {
        <div>
            <code>{move || format!("{:#?}", period())}</code>
        </div>
    }
}
