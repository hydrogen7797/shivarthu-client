use crate::components::navigation::nav::Nav;
use crate::components::schelling_game::positive_externality::rpc::juror_selected::JurorSelected;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn JurorSelectedCheck() -> impl IntoView {
    let params = use_params_map();

    let user_to_calculate =
        move || params.with(|params| params.get("user_to_calculate").unwrap_or_default());

    let (check_account, set_check_account) = signal(String::from(""));

    let account = untrack(move || user_to_calculate());

    let on_account = move |ev| {
        let account_value = event_target_value(&ev);
        set_check_account(account_value);
    };

    view! {
        <div>
            <Nav />
            <div class="max-w-5xl mx-auto max-md:mx-10 dark:text-white text-gray-800">
                <h1>Check if an account selected as juror:</h1>
                <br />
                <input
                    type="text"
                    placeholder="Enter account address here"
                    id="juror-address-checking"
                    class="w-full max-w-xs px-4 py-2 text-sm font-normal text-gray-700 placeholder-gray-400 bg-white border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-800 dark:text-gray-200 dark:placeholder-gray-500 dark:border-gray-700"
                    on:input={on_account}
                />
                <br />
                <br />
                <JurorSelected user_to_calculate={account} check_account={check_account} />
            </div>
        </div>
    }
}
