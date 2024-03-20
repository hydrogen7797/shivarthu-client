use crate::components::navigation::nav::Nav;
use crate::components::schelling_game::profile_validation::add_profile_stake_sign_in::SignTransaction;
use crate::services::common_imp::View;
use crate::services::error::ErrorString;
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use crate::components::schelling_game::profile_validation::storage::get_total_fund_for_profile_collected::TotalFundProfileCollected;

#[component]
pub fn AddProfileStake() -> impl IntoView {
    let params = use_params_map();
    let profile_user_account = move || {
        params.with(|params| {
            params
                .get("profile_user_account")
                .cloned()
                .unwrap_or_default()
        })
    };

    let (current_view, set_current_view) = create_signal(View::Form);

    let (profile_stake, set_profile_stake) = create_signal::<Result<u128, ErrorString>>(Ok(0));

    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();

        set_current_view(View::Success);
    };

    let stake_value = move |value: String| {
        let stake = value.parse::<u128>().expect("Invalid input");
        gloo::console::log!(stake);

        set_profile_stake(Ok(stake));
    };

    let render_view = move || match current_view() {
        View::Form => {
            view! {
                <div class="max-w-5xl mx-auto max-md:mx-10">
                    <div>
                        <TotalFundProfileCollected profile_user_account=profile_user_account()/>
                    </div>
                    <br/>
                    <form id="profile-stake-submit-from" on:submit=submit_click>
                        <div class="mb-5">
                            <label
                                for="profile-stake"
                                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            >
                                Profile Stake
                            </label>
                            <input
                                type="number"
                                id="profile-stake"
                                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                required
                                on:input=move |e| stake_value(event_target_value(&e))
                            />
                        </div>
                        <button
                            type="submit"
                            class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        >

                            Submit
                        </button>
                    </form>
                </div>
            }
        }
        View::Success => {
            view! {
                <div>
                    <SignTransaction
                        stake=profile_stake().unwrap()
                        profile_user_account=profile_user_account()
                    />

                </div>
            }
        }
    };

    view! {
        <>
            <Nav/>
            {move || render_view()}
        </>
    }
}
