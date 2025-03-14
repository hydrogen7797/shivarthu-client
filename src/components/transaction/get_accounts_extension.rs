use crate::services::common_services::{get_accounts, Account};
use crate::services::error::ErrorString;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;

async fn get_accounts_result() -> Result<Vec<Account>, ErrorString> {
    let accounts = get_accounts().await;
    accounts
}

#[component]
pub fn GetAccountsExtension(set_account_load: WriteSignal<(String, String)>) -> impl IntoView {
    let (accounts, set_accounts) = signal::<Vec<Account>>(vec![]);
    let (error, set_error) = signal(String::from(""));

    let onclick_button = move |_e: MouseEvent, i| {
        let accounts = accounts();
        let account: &Account = accounts.get(i).unwrap();
        let account_address = account.address.clone();
        let account_source = account.source.clone();
        set_account_load((account_address.clone(), account_source.clone()));
        gloo::console::log!(format!(
            "Account number{}, account address: {}, account source: {}",
            i, account_address, account_source
        ));
    };
    let get_accounts_click = move |_e: MouseEvent| {
        spawn_local({
            async move {
                match get_accounts_result().await {
                    Ok(result) => set_accounts(result),
                    Err(err) => set_error(err.0),
                }
            }
        });
    };
    let accounts_html = move || {
        let html_element = if accounts().is_empty() {
            view! {
                <div>
                    <br />
                    <div class="dark:text-white">
                        {"No Web3 extension accounts found. Install Talisman, SubWallet or the Polkadot.js extension and add an account."}
                    </div>
                </div>
            }.into_any()
        } else {
            view! {
                <div>
                    <br />
                    <div class="dark:text-white">
                        <b>{"Select an account you want to use for signing:"}</b>
                    </div>
                    <div class="flex flex-col gap-4 mx-auto">
                        {move || {
                            accounts()
                                .iter()
                                .enumerate()
                                .map(|(i, account)| {
                                    view! {
                                        <div class="w-full px-4 py-2 rounded-lg bg-yellow-400 text-gray font-medium transition-colors hover:bg-yellow-500 dark:bg-gray-800 dark:text-gray-200 dark:hover:bg-gray-700 mt-2">
                                            <button
                                                on:click={move |e| { onclick_button(e, i) }}
                                                id={account.address.clone()}
                                            >
                                                {account.source.clone()}
                                                {" | "}
                                                {account.name.clone()}
                                                <br />
                                                <small>{account.address.clone()}</small>
                                            </button>
                                        </div>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>

                </div>
            }
            .into_any()
        };
        html_element
    };

    view! {
        <>
            <div class="container py-10 px-10 mx-0 min-w-full flex flex-col items-center">
                <div>
                    <button
                        on:click={get_accounts_click}
                        class="px-4 py-2 rounded-lg bg-blue-600 text-white font-medium transition-colors hover:bg-blue-700 dark:bg-gray-800 dark:text-gray-200 dark:hover:bg-gray-700"
                        id="select-account"
                    >
                        {"=> Select an Account for Signing"}
                    </button>
                </div>
                {move || accounts_html()}
            </div>
        </>
    }
}
